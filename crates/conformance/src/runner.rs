//! Native effects realizing the Prism plans at the imported GGML boundary.

use crate::abi::*;
use std::cell::UnsafeCell;
use std::ffi::{c_char, c_int, c_void};
use std::ptr::{self, null, null_mut};
use std::sync::OnceLock;
use vtensor_pointwise::{
    Adt0 as GraphRequest, Adt1 as Parameters, Adt2 as TensorDescriptor, Adt4 as CopyMode,
    Adt5 as GraphResponse, Adt6 as GraphPlan,
};

/// Typed refusals at the native tensor boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BackendError {
    /// The host allocator refused backing memory.
    Allocation,
    /// A planned size cannot be represented.
    Overflow,
    /// A tensor type or allocation state is invalid.
    InvalidTensor,
    /// A byte range exceeds its tensor or backing allocation.
    OutOfBounds,
}

struct State {
    reg: UnsafeCell<Registry>,
    dev: UnsafeCell<Device>,
    buft: UnsafeCell<BufferType>,
    guid: UnsafeCell<[u8; 16]>,
    name: [u8; 8],
    description: [u8; 24],
    alignment: usize,
    device_count: usize,
    device_type: c_int,
    initial_byte: u8,
    caps: [bool; 5],
}

fn state() -> &'static State {
    static ADDRESS: OnceLock<usize> = OnceLock::new();
    let address = *ADDRESS.get_or_init(|| {
        let vtensor_native_config::Adt0::C0(
            api,
            alignment,
            count,
            ty,
            initial,
            name,
            d0,
            d1,
            d2,
            g0,
            g1,
            asynchronous,
            host,
            host_ptr,
            events,
            mmap,
        ) = vtensor_native_config::run();
        let description = std::array::from_fn(|i| [d0, d1, d2][i / 8].to_le_bytes()[i % 8]);
        let guid = std::array::from_fn(|i| [g0, g1][i / 8].to_le_bytes()[i % 8]);
        let mut owner = Box::new(State {
            reg: UnsafeCell::new(Registry {
                api_version: api as c_int,
                iface: RegistryInterface {
                    get_name: Some(reg_name),
                    get_device_count: Some(device_count),
                    get_device: Some(device_at),
                    ..Default::default()
                },
                context: null_mut(),
            }),
            dev: UnsafeCell::new(Device {
                iface: DeviceInterface {
                    get_name: Some(device_name),
                    get_description: Some(description_name),
                    get_memory: Some(memory),
                    get_type: Some(device_type),
                    get_props: Some(properties),
                    init_backend: Some(backend_new),
                    get_buffer_type: Some(buffer_type),
                    buffer_from_host_ptr: Some(from_host),
                    supports_op: Some(supports_operation),
                    supports_buft: Some(supports_buffer_type),
                    ..Default::default()
                },
                reg: null_mut(),
                context: null_mut(),
            }),
            buft: UnsafeCell::new(BufferType {
                iface: BufferTypeInterface {
                    get_name: Some(buft_name),
                    alloc_buffer: Some(buffer_allocate),
                    get_alignment: Some(alignment_of),
                    get_alloc_size: Some(allocation_size),
                    is_host: Some(is_host),
                    ..Default::default()
                },
                device: null_mut(),
                context: null_mut(),
            }),
            guid: UnsafeCell::new(guid),
            name: name.to_le_bytes(),
            description,
            alignment: alignment as usize,
            device_count: count as usize,
            device_type: ty as c_int,
            initial_byte: initial as u8,
            caps: [asynchronous, host, host_ptr, events, mmap],
        });
        let context = ptr::from_mut(&mut *owner).cast::<c_void>();
        // SAFETY: the uniquely owned cells are initialized before publication.
        unsafe {
            (*owner.reg.get()).context = context;
            (*owner.dev.get()).reg = owner.reg.get();
            (*owner.dev.get()).context = context;
            (*owner.buft.get()).device = owner.dev.get();
            (*owner.buft.get()).context = context;
        }
        Box::into_raw(owner) as usize
    });
    // SAFETY: ADDRESS publishes one leaked Box. Immutable metadata never changes;
    // GGML's foreign struct fields are isolated in UnsafeCell.
    unsafe { &*(address as *const State) }
}

pub(crate) fn registry() -> Reg {
    state().reg.get()
}

struct BufferContext {
    _owned: Option<Vec<u8>>,
    base: *mut u8,
    size: usize,
}

impl BufferContext {
    fn allocate(size: usize) -> Result<Self, BackendError> {
        let physical = vtensor_native_memory::run(vtensor_native_memory::Adt0::C0(size as u64))
            .map_err(|_| BackendError::Overflow)?;
        let mut owned = workspace(physical, state().initial_byte)?;
        let base = if size == 0 {
            null_mut()
        } else {
            let offset = vtensor_native_memory::run(vtensor_native_memory::Adt0::C1(owned.as_ptr()
                as usize
                as u64))
            .map_err(|_| BackendError::Overflow)? as usize;
            // SAFETY: the certified plan reserves size+alignment-1 bytes and
            // returns an offset below alignment, leaving size logical bytes.
            unsafe { owned.as_mut_ptr().add(offset) }
        };
        Ok(Self {
            _owned: Some(owned),
            base,
            size,
        })
    }

    fn borrowed(base: *mut c_void, size: usize) -> Result<Self, BackendError> {
        if size != 0 && base.is_null() {
            return Err(BackendError::InvalidTensor);
        }
        Ok(Self {
            _owned: None,
            base: base.cast(),
            size,
        })
    }

    fn region(&self, tensor: &Tensor, offset: usize, size: usize) -> Result<*mut u8, BackendError> {
        let span = tensor_span(tensor)?;
        match vtensor_transfer::run(
            true,
            !tensor.data.is_null(),
            span as u64,
            offset as u64,
            size as u64,
        ) {
            vtensor_transfer::Adt0::C1 => return Ok(null_mut()),
            vtensor_transfer::Adt0::C4 => {}
            vtensor_transfer::Adt0::C3 => return Err(BackendError::OutOfBounds),
            _ => return Err(BackendError::InvalidTensor),
        }
        let displacement = (tensor.data as usize)
            .checked_sub(self.base as usize)
            .ok_or(BackendError::OutOfBounds)?;
        if !matches!(
            vtensor_transfer::run(
                true,
                !self.base.is_null(),
                self.size as u64,
                displacement as u64,
                span as u64
            ),
            vtensor_transfer::Adt0::C4
        ) {
            return Err(BackendError::OutOfBounds);
        }
        // SAFETY: both certified admissions bound the whole tensor and the
        // requested subrange. Derive provenance from the actual backing base.
        Ok(unsafe { self.base.add(displacement).add(offset) })
    }
}

fn tensor_metadata(tensor: &Tensor) -> Result<TensorDescriptor, BackendError> {
    let ty = u64::try_from(tensor.ty).map_err(|_| BackendError::InvalidTensor)?;
    let (block, element) = match vtensor_types::run(ty) {
        vtensor_types::Adt0::C1(vtensor_types::Adt1::C0(_, block, element)) => (block, element),
        _ => return Err(BackendError::InvalidTensor),
    };
    // Zero extents are empty. Negative ABI extents cannot inhabit the Prism
    // model's natural-number domain and must fail before buffer admission.
    let [Ok(n0), Ok(n1), Ok(n2), Ok(n3)] = tensor.ne.map(u64::try_from) else {
        return Err(BackendError::InvalidTensor);
    };
    let nb = tensor.nb.map(|n| n as u64);
    let address = tensor.data as usize as u64;
    match vtensor_layout::run(vtensor_layout::Adt0::C0(
        n0, n1, n2, n3, nb[0], nb[1], nb[2], nb[3], block, element,
    ))
    .map_err(|_| BackendError::Overflow)?
    {
        vtensor_layout::Adt1::C1(size) => {
            usize::try_from(size).map_err(|_| BackendError::Overflow)?;
            Ok(TensorDescriptor::C0(
                ty, n0, n1, n2, n3, nb[0], nb[1], nb[2], nb[3], size, address,
            ))
        }
        _ => Err(BackendError::InvalidTensor),
    }
}

fn tensor_span(tensor: &Tensor) -> Result<usize, BackendError> {
    let TensorDescriptor::C0(_, _, _, _, _, _, _, _, _, span, _) = tensor_metadata(tensor)?;
    Ok(span as usize)
}

fn require<T>(result: Result<T, BackendError>) -> T {
    result.unwrap_or_else(|error| {
        let message = match error {
            BackendError::Allocation => c"vTensor allocation refused",
            BackendError::Overflow => c"vTensor size overflow",
            BackendError::InvalidTensor => c"vTensor tensor not allocated",
            BackendError::OutOfBounds => c"vTensor out of bounds",
        };
        // SAFETY: GGML's abort interface accepts static NUL-terminated
        // strings. An invalid callback precondition follows its abort ABI.
        unsafe { ggml_abort(c"vTensor".as_ptr(), 0, message.as_ptr()) }
    })
}

// Bind Prism configuration values to GGML's typed query callbacks.
macro_rules! configuration_queries {
    ($($name:ident($owner:ty $(, $arg:ident: $ty:ty)*) -> $result:ty = $value:expr;)*) => {$(
        unsafe extern "C" fn $name(_: $owner $(, $arg: $ty)*) -> $result { $value }
    )*};
}
configuration_queries! {
    reg_name(Reg) -> *const c_char = state().name.as_ptr().cast();
    device_name(Dev) -> *const c_char = state().name.as_ptr().cast();
    buft_name(Buft) -> *const c_char = state().name.as_ptr().cast();
    backend_name(BackendPtr) -> *const c_char = state().name.as_ptr().cast();
    description_name(Dev) -> *const c_char = state().description.as_ptr().cast();
    device_count(Reg) -> usize = state().device_count;
    device_type(Dev) -> c_int = state().device_type;
    buffer_type(Dev) -> Buft = state().buft.get();
    alignment_of(Buft) -> usize = state().alignment;
    is_host(Buft) -> bool = true;
    supports_buffer_type(Dev, buft: Buft) -> bool = buft == state().buft.get();
}

unsafe extern "C" fn device_at(_: Reg, index: usize) -> Dev {
    if vtensor_native_predicates::run(vtensor_native_predicates::Adt0::C0(index as u64)) {
        state().dev.get()
    } else {
        null_mut()
    }
}

unsafe extern "C" fn memory(_: Dev, free: *mut usize, total: *mut usize) {
    // SAFETY: GGML supplies writable result locations. Zero means unreported.
    unsafe {
        free.write(0);
        total.write(0);
    }
}

unsafe extern "C" fn properties(_: Dev, out: *mut DeviceProperties) {
    let s = state();
    // SAFETY: GGML supplies a writable properties object of the imported ABI.
    unsafe {
        out.write(DeviceProperties {
            name: s.name.as_ptr().cast(),
            description: s.description.as_ptr().cast(),
            memory_free: 0,
            memory_total: 0,
            ty: s.device_type,
            device_id: null(),
            caps: DeviceCapabilities {
                asynchronous: s.caps[0],
                host_buffer: s.caps[1],
                buffer_from_host_ptr: s.caps[2],
                events: s.caps[3],
                mmap_support: s.caps[4],
            },
        });
    }
}

unsafe extern "C" fn allocation_size(_: Buft, tensor: *const Tensor) -> usize {
    // SAFETY: GGML supplies a live tensor to the buffer type callback.
    require(tensor_span(unsafe { &*tensor }))
}

fn buffer_interface() -> BufferInterface {
    BufferInterface {
        free_buffer: Some(buffer_free),
        get_base: Some(buffer_base),
        init_tensor: Some(tensor_init),
        memset_tensor: Some(tensor_memset),
        set_tensor: Some(tensor_set),
        get_tensor: Some(tensor_get),
        cpy_tensor: Some(tensor_copy),
        clear: Some(buffer_clear),
        ..Default::default()
    }
}

fn wrap_buffer(buft: Buft, context: Result<BufferContext, BackendError>) -> Buf {
    let Ok(context) = context else {
        return null_mut();
    };
    let size = context.size;
    // SAFETY: both callers supply GGML's live buffer type and a validated
    // context. GGML owns the wrapper; buffer_free releases only this Rust Box.
    unsafe {
        ggml_backend_buffer_init(
            buft,
            buffer_interface(),
            Box::into_raw(Box::new(context)).cast(),
            size,
        )
    }
}

unsafe extern "C" fn buffer_allocate(buft: Buft, size: usize) -> Buf {
    wrap_buffer(buft, BufferContext::allocate(size))
}

unsafe extern "C" fn from_host(_: Dev, pointer: *mut c_void, size: usize, _: usize) -> Buf {
    wrap_buffer(state().buft.get(), BufferContext::borrowed(pointer, size))
}

unsafe extern "C" fn buffer_free(buffer: Buf) {
    // SAFETY: GGML invokes this callback once before deleting its wrapper.
    unsafe {
        drop(Box::from_raw((*buffer).context.cast::<BufferContext>()));
    }
}

unsafe extern "C" fn buffer_base(buffer: Buf) -> *mut c_void {
    // SAFETY: each live wrapper retains its Rust context until free_buffer.
    unsafe { (*(*buffer).context.cast::<BufferContext>()).base.cast() }
}

unsafe extern "C" fn tensor_init(buffer: Buf, tensor: TensorPtr) -> c_int {
    // SAFETY: GGML provides live buffer/tensor objects; the region checker
    // validates their byte relationship without dereferencing tensor data.
    let result = unsafe {
        let tensor = &*tensor;
        let context = &*(*buffer).context.cast::<BufferContext>();
        tensor_span(tensor).and_then(|size| context.region(tensor, 0, size))
    };
    if result.is_ok() {
        GGML_STATUS_SUCCESS
    } else {
        GGML_STATUS_FAILED
    }
}

unsafe extern "C" fn buffer_clear(buffer: Buf, value: u8) {
    // SAFETY: the buffer owns or borrows this writable logical region. Empty
    // buffers return before passing a null pointer to a Rust memory intrinsic.
    unsafe {
        let context = &*(*buffer).context.cast::<BufferContext>();
        if context.size != 0 {
            ptr::write_bytes(context.base, value, context.size);
        }
    }
}

macro_rules! transfer_callbacks {
    ($($name:ident($tensor:ty, $external:ty) => $effect:expr;)*) => {$(
        unsafe extern "C" fn $name(
            buffer: Buf,
            tensor: $tensor,
            external: $external,
            offset: usize,
            size: usize,
        ) {
            if size == 0 {
                return;
            }
            // SAFETY: GGML retains these descriptors and the external region;
            // certified admissions bound the tensor region. Copy permits aliases.
            unsafe {
                let context = &*(*buffer).context.cast::<BufferContext>();
                let region = require(context.region(&*tensor, offset, size));
                ($effect)(region, external, size);
            }
        }
    )*};
}
transfer_callbacks! {
    tensor_set(TensorPtr, *const c_void) => |region, external: *const c_void, size| ptr::copy(external.cast::<u8>(), region, size);
    tensor_get(*const Tensor, *mut c_void) => |region: *mut u8, external: *mut c_void, size| ptr::copy(region, external.cast::<u8>(), size);
    tensor_memset(TensorPtr, u8) => |region: *mut u8, value, size| ptr::write_bytes(region, value, size);
}

unsafe extern "C" fn tensor_copy(
    buffer: Buf,
    source: *const Tensor,
    destination: TensorPtr,
) -> bool {
    // SAFETY: GGML provides live tensors. Only directly accessible host sources
    // are copied; false is returned before mutation for every unsupported source.
    unsafe {
        let source = &*source;
        let source_buffer = if source.view_src.is_null() {
            source.buffer
        } else {
            (*source.view_src).buffer
        };
        if source_buffer.is_null() || !ggml_backend_buffer_is_host(source_buffer) {
            return false;
        }
        let Ok(size) = tensor_span(source) else {
            return false;
        };
        if tensor_span(&*destination) != Ok(size) || (size != 0 && source.data.is_null()) {
            return false;
        }
        let context = &*(*buffer).context.cast::<BufferContext>();
        let Ok(target) = context.region(&*destination, 0, size) else {
            return false;
        };
        if size != 0 {
            ptr::copy(source.data.cast::<u8>(), target, size);
        }
        true
    }
}

unsafe extern "C" fn backend_new(device: Dev, _: *const c_char) -> BackendPtr {
    Box::into_raw(Box::new(Backend {
        guid: state().guid.get(),
        iface: BackendInterface {
            get_name: Some(backend_name),
            free: Some(backend_free),
            graph_compute: Some(graph_compute),
            ..Default::default()
        },
        device,
        context: null_mut(),
    }))
}

unsafe extern "C" fn backend_free(backend: BackendPtr) {
    // SAFETY: GGML delegates backend destruction to this callback exactly once.
    unsafe {
        drop(Box::from_raw(backend));
    }
}

fn plan_operation(tensor: &Tensor) -> Result<Option<GraphPlan>, BackendError> {
    let output = tensor_metadata(tensor)?;
    let TensorDescriptor::C0(_, _, _, _, _, _, _, _, _, span, _) = &output;
    let operation = u64::try_from(tensor.op).map_err(|_| BackendError::InvalidTensor)?;
    if vtensor_native_predicates::run(vtensor_native_predicates::Adt0::C1(operation, *span)) {
        return Ok(None);
    }
    // SAFETY: GGML retains every non-null source descriptor during queries and execution.
    let left = unsafe { tensor.src[0].as_ref() }.ok_or(BackendError::InvalidTensor)?;
    // SAFETY: GGML also retains the optional right descriptor for this call.
    let right = unsafe { tensor.src[1].as_ref() }
        .map(tensor_metadata)
        .transpose()?;
    let [p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14, p15] =
        tensor.op_params.map(|word| u64::from(word as u32));
    let response = vtensor_pointwise::run(GraphRequest::C0(
        operation,
        Parameters::C0(
            p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14, p15,
        ),
        tensor_metadata(left)?,
        right,
        output,
    ))
    .map_err(|_| BackendError::Overflow)?;
    match response {
        GraphResponse::C0(plan @ (GraphPlan::C3(..) | GraphPlan::C4(..))) => Ok(Some(plan)),
        _ => Err(BackendError::InvalidTensor),
    }
}

fn graph_tensor_data(tensor: &Tensor) -> Result<*mut u8, BackendError> {
    let span = tensor_span(tensor)?;
    if span == 0 {
        return Ok(null_mut());
    }
    // SAFETY: a nonempty graph tensor must have a live native buffer. Foreign
    // buffer contexts are refused before interpreting our private context type.
    unsafe {
        if tensor.buffer.is_null() || (*tensor.buffer).buft != state().buft.get() {
            return Err(BackendError::InvalidTensor);
        }
        let context = &*(*tensor.buffer).context.cast::<BufferContext>();
        context.region(tensor, 0, span)
    }
}

fn workspace<T: Clone>(size: u64, initial: T) -> Result<Vec<T>, BackendError> {
    let count = usize::try_from(size).map_err(|_| BackendError::Overflow)?;
    let mut result = Vec::new();
    result
        .try_reserve_exact(count)
        .map_err(|_| BackendError::Allocation)?;
    result.resize(count, initial);
    Ok(result)
}

fn element_offset(tensor: &Tensor, indices: [usize; 4]) -> Result<usize, BackendError> {
    let [i0, i1, i2, i3] = indices.map(|i| i as u64);
    let [s0, s1, s2, s3] = tensor.nb.map(|s| s as u64);
    let offset =
        vtensor_index::run(i0, i1, i2, i3, s0, s1, s2, s3).map_err(|_| BackendError::Overflow)?;
    usize::try_from(offset).map_err(|_| BackendError::Overflow)
}

fn execute_matrix(tensor: &Tensor, plan: GraphPlan) -> Result<(), BackendError> {
    let GraphPlan::C3(m, k, n, b2, b3, r2, r3, left_count, right_count, output_count) = plan else {
        return Err(BackendError::InvalidTensor);
    };
    // SAFETY: plan_operation admitted both retained source descriptors.
    let (left, right) = unsafe { (&*tensor.src[0], &*tensor.src[1]) };
    let (left_data, right_data, output_data) = (
        graph_tensor_data(left)?,
        graph_tensor_data(right)?,
        graph_tensor_data(tensor)?,
    );
    let mut left_values = workspace(left_count, 0.0)?;
    let mut right_values = workspace(right_count, 0.0)?;
    let mut output_values = workspace(output_count, 0.0)?;
    let (m, k, n) = (m as usize, k as usize, n as usize);
    for i3 in 0..b3 as usize {
        for i2 in 0..b2 as usize {
            let (l2, l3) = (i2 / r2 as usize, i3 / r3 as usize);
            for row in 0..m {
                for inner in 0..k {
                    let offset = element_offset(left, [inner, row, l2, l3])?;
                    // SAFETY: certified span admission bounds every modeled element
                    // offset for these extents. Unaligned F32 views are supported.
                    left_values[row * k + inner] =
                        unsafe { ptr::read_unaligned(left_data.add(offset).cast::<f32>()) };
                }
            }
            for inner in 0..k {
                for column in 0..n {
                    let offset = element_offset(right, [inner, column, i2, i3])?;
                    // SAFETY: the validated right span bounds this modeled address.
                    right_values[inner * n + column] =
                        unsafe { ptr::read_unaligned(right_data.add(offset).cast::<f32>()) };
                }
            }
            verdant_tensor::linear::multiply_matrix_rows_f32(
                m,
                k,
                n,
                &left_values,
                &right_values,
                &mut output_values,
            )
            .map_err(|_| BackendError::InvalidTensor)?;
            for row in 0..m {
                for column in 0..n {
                    let offset = element_offset(tensor, [row, column, i2, i3])?;
                    // SAFETY: the output span bounds this modeled address. Raw
                    // stores avoid manufacturing Rust borrows over foreign aliases.
                    unsafe {
                        ptr::write_unaligned(
                            output_data.add(offset).cast::<f32>(),
                            output_values[row * n + column],
                        );
                    }
                }
            }
        }
    }
    Ok(())
}

fn storage_word(src: u64, dst: u64, bits: u64, mode: &CopyMode) -> Result<[u8; 4], BackendError> {
    let word = match mode {
        CopyMode::C13(kind, ty, a, b) => {
            verdant_tensor::linear::transform_float_storage(*kind, *ty, bits, *a, *b)
                .map_err(|_| BackendError::Overflow)
        }
        CopyMode::C14(kind, source, destination) => {
            verdant_tensor::linear::evaluate_float_unary(*kind, *source, *destination, bits)
                .map_err(|_| BackendError::Overflow)
        }
        _ => verdant_tensor::linear::convert_float_storage(src, dst, bits)
            .map_err(|_| BackendError::Overflow),
    }?
    .ok_or(BackendError::InvalidTensor)?;
    Ok(storage_bytes(word, dst))
}

fn storage_bytes(word: u64, storage: u64) -> [u8; 4] {
    if storage == 0 {
        (word as u32).to_ne_bytes()
    } else {
        let [a, b] = (word as u16).to_ne_bytes();
        [a, b, 0, 0]
    }
}

unsafe fn read_word(pointer: *const u8, storage: u64) -> u64 {
    // SAFETY: callers provide one admitted word of the selected storage width.
    unsafe {
        if storage == 0 {
            u64::from(ptr::read_unaligned(pointer.cast::<u32>()))
        } else {
            u64::from(ptr::read_unaligned(pointer.cast::<u16>()))
        }
    }
}

fn execute_word(
    response: GraphResponse,
    storage: u64,
    pointers: &[*mut u8; 4],
    words: &mut [u64; 3],
) -> Result<(), BackendError> {
    let GraphResponse::C4(kind, left_slot, left_from, right_slot, right_from, target, to) =
        response
    else {
        return Err(BackendError::InvalidTensor);
    };
    let read = |slot, from| match slot {
        // Prism emits only literal scratch indices 0, 1 and 2, using the
        // same admitted bounds as the pointer-slot bindings below.
        1 => Ok(words[from as usize]),
        2 => Ok(from),
        slot => {
            let index = match slot {
                0 => 0,
                3 => 2,
                4 => 1,
                5 => 3,
                _ => return Err(BackendError::InvalidTensor),
            };
            // SAFETY: Prism bounds every selected word and its storage width.
            Ok(unsafe { read_word(pointers[index].add(from as usize), storage) })
        }
    };
    let (a, b) = (read(left_slot, left_from)?, read(right_slot, right_from)?);
    let value = match kind {
        0..=3 => verdant_tensor::linear::evaluate_wide(kind, a, b)
            .map_err(|_| BackendError::InvalidTensor)?,
        4 | 5 | 8..=15 => Some(u64::from(
            verdant_tensor::linear::evaluate_f32_scalar(
                match kind {
                    4 => 0,
                    5 => 3,
                    _ => kind - 8,
                },
                f32::from_bits(a as u32),
                f32::from_bits(b as u32),
            )
            .map_err(|_| BackendError::InvalidTensor)?
            .to_bits(),
        )),
        6 => verdant_tensor::linear::convert_float_storage(storage, 0, a)
            .map_err(|_| BackendError::InvalidTensor)?,
        7 => verdant_tensor::linear::convert_float_storage(0, storage, a)
            .map_err(|_| BackendError::InvalidTensor)?,
        16..=19 => verdant_tensor::linear::evaluate_encoded_word(kind - 16, a, b)
            .map_err(|_| BackendError::InvalidTensor)?,
        _ => return Err(BackendError::InvalidTensor),
    }
    .ok_or(BackendError::InvalidTensor)?;
    match target {
        1 => words[to as usize] = value,
        // SAFETY: Prism emits destination writes only for its admitted logical
        // output words, after all required source reads. Padding is untouched.
        3 => unsafe {
            let encoded = storage_bytes(value, storage);
            ptr::copy_nonoverlapping(
                encoded.as_ptr(),
                pointers[2].add(to as usize),
                if storage == 0 { 4 } else { 2 },
            );
        },
        // SAFETY: Prism's I64 COUNT_EQUAL plan admits this scalar word.
        6 => unsafe { ptr::write_unaligned(pointers[2].add(to as usize).cast::<u64>(), value) },
        _ => return Err(BackendError::InvalidTensor),
    }
    Ok(())
}

fn execute_copy(tensor: &Tensor, plan: GraphPlan) -> Result<(), BackendError> {
    let GraphPlan::C4(count, bytes, source, destination, mode) = plan else {
        return Err(BackendError::InvalidTensor);
    };
    // SAFETY: the admitted plan retains every required source descriptor.
    let source_data = |slot: usize| graph_tensor_data(unsafe { &*tensor.src[slot] });
    // Typed host word bindings preserve native byte order; conversion is Verdant's.
    let literal = match mode {
        CopyMode::C9(ty, bits) => storage_word(0, ty, bits, &mode)?,
        _ => [0; 4],
    };
    let left_data = match mode {
        CopyMode::C9(..) => null_mut(),
        _ => source_data(0)?,
    };
    let right_data = match mode {
        CopyMode::C2(..)
        | CopyMode::C6(..)
        | CopyMode::C10(_, false, _)
        | CopyMode::C18(..)
        | CopyMode::C19(..)
        | CopyMode::C20(..) => source_data(1)?,
        _ => literal.as_ptr().cast_mut(),
    };
    let destination_data = graph_tensor_data(tensor)?;
    let pointers = [
        left_data,
        right_data,
        destination_data,
        literal.as_ptr().cast_mut(),
    ];
    let mut block = workspace(bytes, 0_u8)?;
    let mut words = [0; 3];
    for index in 0..count {
        let request = match mode {
            CopyMode::C17 => GraphRequest::C2(index, source.clone(), destination.clone(), words[2]),
            CopyMode::C19(_, ref indices) => GraphRequest::C3(
                index,
                source.clone(),
                destination.clone(),
                indices.clone(),
                words[2],
            ),
            CopyMode::C20(kind, ref indices) => GraphRequest::C4(
                kind,
                index,
                source.clone(),
                destination.clone(),
                indices.clone(),
                words[2],
            ),
            _ => GraphRequest::C1(index, source.clone(), destination.clone(), mode.clone()),
        };
        let response = vtensor_pointwise::run(request).map_err(|_| BackendError::Overflow)?;
        if let GraphResponse::C4(..) = response {
            let storage = match mode {
                CopyMode::C16(_, storage) => storage,
                CopyMode::C19(storage, _) if matches!(response, GraphResponse::C4(6, ..)) => {
                    storage
                }
                _ => 0,
            };
            execute_word(response, storage, &pointers, &mut words)?;
            continue;
        }
        if let GraphResponse::C5(kind, from, bytes, to, span, stride) = response {
            // SAFETY: Prism admits both whole tensors and these block spans,
            // refuses source/output aliases, and preflights every index before writes.
            unsafe {
                let input =
                    std::slice::from_raw_parts(left_data.add(from as usize), bytes as usize);
                let output = std::slice::from_raw_parts_mut(
                    destination_data.add(to as usize),
                    span as usize,
                );
                verdant_tensor::linear::decode_storage_block(kind, input, output, stride as usize)
                    .map_err(|_| BackendError::InvalidTensor)?;
            }
            continue;
        }
        let encoded;
        // SAFETY: Prism bounds the selected source word/block and destination.
        // Capture source bytes before borrowing a possibly aliased destination.
        unsafe {
            let (pointer, to): (*const u8, u64) = match response {
                GraphResponse::C1(second, from, to) => {
                    (pointers[usize::from(second)].add(from as usize), to)
                }
                GraphResponse::C3(source_type, destination_type, from, to) => {
                    let pointer = pointers[0].add(from as usize);
                    let word = read_word(pointer, source_type);
                    encoded = storage_word(source_type, destination_type, word, &mode)?;
                    (encoded.as_ptr(), to)
                }
                _ => return Err(BackendError::InvalidTensor),
            };
            ptr::copy_nonoverlapping(pointer, block.as_mut_ptr(), block.len());
            let target =
                std::slice::from_raw_parts_mut(destination_data.add(to as usize), block.len());
            verdant_tensor::memory::copy_storage_block(&block, target)
                .map_err(|_| BackendError::InvalidTensor)?;
        }
    }
    Ok(())
}

unsafe extern "C" fn supports_operation(_: Dev, tensor: *const Tensor) -> bool {
    // SAFETY: GGML supplies a live tensor descriptor for this query.
    plan_operation(unsafe { &*tensor }).is_ok()
}

unsafe extern "C" fn graph_compute(_: BackendPtr, graph: Graph) -> c_int {
    let mut plans = Vec::new();
    // SAFETY: GGML retains this graph and all descriptors for the synchronous
    // call. Preflight plans every numerical node once before any computation.
    unsafe {
        for index in 0..ggml_graph_n_nodes(graph) {
            let tensor = ggml_graph_node(graph, index);
            match plan_operation(&*tensor) {
                Ok(Some(plan)) => {
                    if plans.try_reserve(1).is_err() {
                        return GGML_STATUS_FAILED;
                    }
                    plans.push((tensor, plan));
                }
                Ok(None) => {}
                Err(_) => return GGML_STATUS_FAILED,
            }
        }
        for (tensor, plan) in plans {
            let result = match plan {
                GraphPlan::C3(..) => execute_matrix(&*tensor, plan),
                GraphPlan::C4(..) => execute_copy(&*tensor, plan),
                _ => Err(BackendError::InvalidTensor),
            };
            if result.is_err() {
                return GGML_STATUS_FAILED;
            }
        }
    }
    GGML_STATUS_SUCCESS
}
