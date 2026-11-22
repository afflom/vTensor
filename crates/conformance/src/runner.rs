//! Native effects realizing the Prism plans at the imported GGML boundary.

use crate::abi::*;
use std::cell::UnsafeCell;
use std::ffi::{c_char, c_int, c_void};
use std::ptr::{self, null, null_mut};
use std::sync::OnceLock;
use vtensor_pointwise::{
    Adt0 as GraphRequest, Adt1 as Parameters, Adt2 as TensorDescriptor, Adt3 as IndexContext,
    Adt4 as CopyMode, Adt5 as GraphResponse, Adt6 as GraphPlan, Adt7 as WordOperation,
    Adt8 as WordOperand, Adt9 as WordTarget,
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

enum ExecutionPlan {
    Standard(GraphPlan),
    Typed(vtensor_typed::Adt2),
    Quantized(vtensor_matrix_q8::Adt2),
}

fn plan_operation(tensor: &Tensor) -> Result<Option<ExecutionPlan>, BackendError> {
    let output = tensor_metadata(tensor)?;
    let TensorDescriptor::C0(_, _, _, _, _, _, _, _, _, span, _) = &output;
    let operation = u64::try_from(tensor.op).map_err(|_| BackendError::InvalidTensor)?;
    if vtensor_native_predicates::run(vtensor_native_predicates::Adt0::C1(operation, *span)) {
        return Ok(None);
    }
    let source = |slot: usize| {
        // SAFETY: GGML retains each non-null source descriptor during this call.
        unsafe { tensor.src[slot].as_ref() }
            .map(tensor_metadata)
            .transpose()
    };
    let [p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14, p15] =
        tensor.op_params.map(|word| u64::from(word as u32));
    let primary = source(0)?;
    let secondary = source(1)?;
    let base = source(2)?;
    let response = vtensor_pointwise::run(GraphRequest::C0(
        operation,
        Parameters::C0(
            p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14, p15,
        ),
        primary.clone(),
        secondary.clone(),
        base.clone(),
        source(3)?,
        source(4)?,
        source(5)?,
        source(6)?,
        source(7)?,
        source(8)?,
        source(9)?,
        output.clone(),
    ))
    .map_err(|_| BackendError::Overflow)?;
    let metadata = |TensorDescriptor::C0(a, b, c, d, e, f, g, h, i, j, k)| {
        vtensor_typed::Adt1::C0(a, b, c, d, e, f, g, h, i, j, k)
    };
    let typed_plan =
        |request| match vtensor_typed::run(request).map_err(|_| BackendError::Overflow)? {
            vtensor_typed::Adt3::C2(plan) => Ok(Some(ExecutionPlan::Typed(plan))),
            _ => Err(BackendError::InvalidTensor),
        };
    match response {
        GraphResponse::C0(plan @ (GraphPlan::C3(..) | GraphPlan::C4(..))) => {
            Ok(Some(ExecutionPlan::Standard(plan)))
        }
        GraphResponse::C0(GraphPlan::C5(67)) => typed_plan(vtensor_typed::Adt0::C0(
            operation,
            primary.map(metadata),
            secondary.map(metadata),
            metadata(output),
        )),
        GraphResponse::C0(GraphPlan::C5(64)) => normalization_plan(
            primary.as_ref().ok_or(BackendError::InvalidTensor)?,
            secondary.as_ref(),
            &output,
            operation,
            p0,
            p1,
        )
        .map(|plan| Some(ExecutionPlan::Standard(plan))),
        _ => {
            let descriptor = |TensorDescriptor::C0(ty, a, b, c, d, e, f, g, h, _, _)| {
                vtensor_matrix_q8::Adt1::C0(ty, a, b, c, d, e, f, g, h)
            };
            let extra = tensor.src[3..].iter().any(|ptr| !ptr.is_null());
            let w = primary.ok_or(BackendError::InvalidTensor)?;
            if base.is_none() {
                let query = vtensor_typed::Adt0::C3(
                    operation,
                    p0,
                    p1,
                    extra,
                    metadata(w.clone()),
                    secondary.clone().map(metadata),
                    metadata(output.clone()),
                );
                match vtensor_typed::run(query).map_err(|_| BackendError::Overflow)? {
                    vtensor_typed::Adt3::C2(plan) => return Ok(Some(ExecutionPlan::Typed(plan))),
                    vtensor_typed::Adt3::C0 => {}
                    _ => return Err(BackendError::InvalidTensor),
                }
            }
            let a = secondary.ok_or(BackendError::InvalidTensor)?;
            if let Some(base) = base {
                return typed_plan(vtensor_typed::Adt0::C2(
                    operation,
                    extra,
                    metadata(w),
                    metadata(a),
                    metadata(base),
                    metadata(output),
                ));
            }
            let request = vtensor_matrix_q8::Adt0::C0(
                operation,
                extra,
                descriptor(w),
                descriptor(a),
                descriptor(output),
            );
            match vtensor_matrix_q8::run(request).map_err(|_| BackendError::Overflow)? {
                vtensor_matrix_q8::Adt3::C1(plan) => Ok(Some(ExecutionPlan::Quantized(plan))),
                _ => Err(BackendError::InvalidTensor),
            }
        }
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

fn execute_matrix(tensor: &Tensor, plan: GraphPlan) -> Result<(), BackendError> {
    use vtensor_matrix_execution::{Adt0 as Strides, Adt1 as Instruction};
    let GraphPlan::C3(m, k, n, b2, b3, r2, r3, left_count, right_count, _) = plan else {
        return Err(BackendError::InvalidTensor);
    };
    // SAFETY: plan_operation admitted both retained source descriptors.
    let (left, right) = unsafe { (&*tensor.src[0], &*tensor.src[1]) };
    let pointers = [
        graph_tensor_data(left)?,
        graph_tensor_data(right)?,
        graph_tensor_data(tensor)?,
    ];
    let strides = |tensor: &Tensor| {
        let [s0, s1, s2, s3] = tensor.nb.map(|s| s as u64);
        Strides::C0(s0, s1, s2, s3)
    };
    let step = |index| {
        vtensor_matrix_execution::run(
            index,
            m,
            k,
            n,
            b2,
            b3,
            r2,
            r3,
            strides(left),
            strides(right),
            strides(tensor),
        )
        .map_err(|_| BackendError::Overflow)
    };
    let Instruction::C0(steps, total) = step(0)? else {
        return Err(BackendError::InvalidTensor);
    };
    let mut values = workspace(total, 0.0_f32)?;
    let [m, k, n] = [m, k, n].map(|n| n as usize);
    for index in 1..steps {
        match step(index)? {
            // SAFETY: Prism bounds the private workspace slot, foreign pointer
            // slot and byte offset. Native F32 words may be unaligned; byte
            // transfers preserve all encodings without creating foreign borrows.
            Instruction::C1(writing, slot, at, offset) => unsafe {
                let temporary = values.as_mut_ptr().add(at as usize).cast::<u8>();
                let external = pointers[slot as usize].add(offset as usize);
                let (from, to) = if writing {
                    (temporary, external)
                } else {
                    (external, temporary)
                };
                ptr::copy_nonoverlapping(from, to, 4);
            },
            Instruction::C2 => {
                let (left, rest) = values.split_at_mut(left_count as usize);
                let (right, output) = rest.split_at_mut(right_count as usize);
                verdant_tensor::linear::multiply_matrix_rows_f32(m, k, n, left, right, output)
                    .map_err(|_| BackendError::InvalidTensor)?;
            }
            _ => return Err(BackendError::InvalidTensor),
        }
    }
    Ok(())
}

fn execute_quantized_matrix(
    tensor: &Tensor,
    plan: vtensor_matrix_q8::Adt2,
) -> Result<(), BackendError> {
    use verdant_tensor::linear::{
        dot_symmetric_q4_row, dot_symmetric_q8_row, encode_symmetric_q8_block,
    };
    use vtensor_matrix_q8::{run, Adt0 as Request, Adt2 as Plan, Adt3 as Response};
    let Plan::C0(blocks, weight_bytes, rows, _, _, visits, outputs, .., packed) = plan;
    let dot = if packed {
        dot_symmetric_q4_row
    } else {
        dot_symmetric_q8_row
    };
    // SAFETY: graph admission retained both typed GGML source descriptors.
    let (w, a) = unsafe { (&*tensor.src[0], &*tensor.src[1]) };
    let weights = graph_tensor_data(w)?;
    let activations = graph_tensor_data(a)?;
    let destination = graph_tensor_data(tensor)?;
    let mut results = workspace(outputs, 0.0f32)?;
    let mut offsets = workspace(visits, 0u64)?;
    let mut prepared = Vec::new();
    prepared
        .try_reserve_exact(blocks as usize)
        .map_err(|_| BackendError::Allocation)?;
    for (column, (values, offset)) in results
        .chunks_exact_mut(rows as usize)
        .zip(&mut offsets)
        .enumerate()
    {
        let Response::C2(weight_base, activation, dest) =
            run(Request::C1(plan.clone(), column as u64)).map_err(|_| BackendError::Overflow)?
        else {
            return Err(BackendError::InvalidTensor);
        };
        *offset = dest;
        prepared.clear();
        for block in 0..blocks {
            // SAFETY: the certified column lies in the admitted activation span and has
            // exactly this many contiguous 32-lane blocks. GGML views may be unaligned.
            let lanes = unsafe {
                activations
                    .add(activation as usize)
                    .cast::<[f32; 32]>()
                    .add(block as usize)
                    .read_unaligned()
            };
            prepared
                .push(encode_symmetric_q8_block(&lanes).map_err(|_| BackendError::InvalidTensor)?);
        }
        for (row, value) in values.iter_mut().enumerate() {
            let Response::C3(start) = run(Request::C2(plan.clone(), weight_base, row as u64))
                .map_err(|_| BackendError::Overflow)?
            else {
                return Err(BackendError::InvalidTensor);
            };
            let bytes = if weight_bytes == 0 {
                &[]
            } else {
                // SAFETY: Prism bounds the complete contiguous encoded row in the
                // live weight buffer. Writes are staged until all reads finish.
                unsafe {
                    std::slice::from_raw_parts(weights.add(start as usize), weight_bytes as usize)
                }
            };
            *value = dot(bytes, &prepared).map_err(|_| BackendError::InvalidTensor)?;
        }
    }
    for (column, offset) in results.chunks_exact(rows as usize).zip(offsets) {
        // SAFETY: each certified column fits the checked buffer and the private
        // result workspace is disjoint from it. Byte copying permits unaligned views.
        unsafe {
            ptr::copy_nonoverlapping(
                column.as_ptr().cast::<u8>(),
                destination.add(offset as usize),
                std::mem::size_of_val(column),
            );
        }
    }
    Ok(())
}

unsafe fn scalar_word(
    pointer: *mut u8,
    width: u64,
    value: Option<u64>,
) -> Result<u64, BackendError> {
    let width = usize::try_from(width).map_err(|_| BackendError::InvalidTensor)?;
    if !matches!(width, 1 | 2 | 4 | 8) {
        return Err(BackendError::InvalidTensor);
    }
    let mut bytes = value.unwrap_or(0).to_ne_bytes();
    let start = if cfg!(target_endian = "little") {
        0
    } else {
        bytes.len() - width
    };
    let selected = &mut bytes[start..start + width];
    // SAFETY: the caller supplies one admitted scalar region; selected is exactly
    // its validated width. Selecting the native low bytes also preserves big endian words.
    unsafe {
        if value.is_some() {
            ptr::copy_nonoverlapping(selected.as_ptr(), pointer, width);
        } else {
            ptr::copy_nonoverlapping(pointer, selected.as_mut_ptr(), width);
        }
    }
    Ok(u64::from_ne_bytes(bytes))
}

fn execute_word(
    response: GraphResponse,
    pointers: &[*mut u8],
    destination: *mut u8,
    words: &mut [u64],
) -> Result<(), BackendError> {
    let GraphResponse::C7(operation, left, right, target, read_bytes, write_bytes) = response
    else {
        return Err(BackendError::InvalidTensor);
    };
    let read = |operand| match operand {
        WordOperand::C1(index) => words
            .get(index as usize)
            .copied()
            .ok_or(BackendError::InvalidTensor),
        WordOperand::C2(value) => Ok(value),
        operand => {
            let (pointer, offset) = match operand {
                WordOperand::C0(index, offset) => (
                    *pointers
                        .get(index as usize)
                        .ok_or(BackendError::InvalidTensor)?,
                    offset,
                ),
                WordOperand::C3(offset) => (destination, offset),
                _ => return Err(BackendError::InvalidTensor),
            };
            // SAFETY: Prism bounds the selected source or output word and its byte width.
            unsafe { scalar_word(pointer.add(offset as usize), read_bytes, None) }
        }
    };
    let (a, b) = (read(left)?, read(right)?);
    let failure = |overflow| {
        if overflow {
            BackendError::Overflow
        } else {
            BackendError::InvalidTensor
        }
    };
    let value = match operation {
        WordOperation::C0(kind) => verdant_tensor::linear::evaluate_wide(kind, a, b)
            .map_err(|_| BackendError::InvalidTensor)?,
        WordOperation::C1(kind) => Some(u64::from(
            verdant_tensor::linear::evaluate_f32_scalar(
                kind,
                f32::from_bits(a as u32),
                f32::from_bits(b as u32),
            )
            .map_err(|_| BackendError::InvalidTensor)?
            .to_bits(),
        )),
        WordOperation::C2(source, destination, overflow) => {
            verdant_tensor::linear::convert_float_storage(source, destination, a)
                .map_err(|_| failure(overflow))?
        }
        WordOperation::C3(source, destination, overflow) => {
            verdant_tensor::linear::evaluate_float_unary(b, source, destination, a)
                .map_err(|_| failure(overflow))?
        }
        WordOperation::C4(kind, storage, first, second) => {
            verdant_tensor::linear::transform_float_storage(kind, storage, a, first, second)
                .map_err(|_| BackendError::Overflow)?
        }
        WordOperation::C5(kind) => verdant_tensor::linear::evaluate_encoded_word(kind, a, b)
            .map_err(|_| BackendError::InvalidTensor)?,
        WordOperation::C6 => return Err(BackendError::InvalidTensor),
    }
    .ok_or(BackendError::InvalidTensor)?;
    match target {
        WordTarget::C0(index) => {
            *words
                .get_mut(index as usize)
                .ok_or(BackendError::InvalidTensor)? = value
        }
        // SAFETY: Prism bounds each output word, including eight-byte COUNT_EQUAL results.
        WordTarget::C1(offset) => unsafe {
            scalar_word(destination.add(offset as usize), write_bytes, Some(value))?;
        },
        WordTarget::C2 => return Err(BackendError::InvalidTensor),
    }
    Ok(())
}

fn normalization_plan(
    source: &TensorDescriptor,
    secondary: Option<&TensorDescriptor>,
    output: &TensorDescriptor,
    operation: u64,
    p0: u64,
    p1: u64,
) -> Result<GraphPlan, BackendError> {
    let TensorDescriptor::C0(l0, l1, l2, l3, l4, l5, l6, l7, l8, l9, l10) = *source;
    let TensorDescriptor::C0(r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10) = *output;
    let present = secondary.is_some();
    let TensorDescriptor::C0(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10) =
        *secondary.unwrap_or(source);
    let vtensor_norm_plan::Adt0::C1(count, bytes, kind, rows, capacity, parameter) =
        vtensor_norm_plan::run(
            operation, p0, p1, present, l0, l1, l2, l3, l4, l5, l6, l7, l8, l9, l10, r0, r1, r2,
            r3, r4, r5, r6, r7, r8, r9, r10, x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10,
        )
        .map_err(|_| BackendError::Overflow)?
    else {
        return Err(BackendError::InvalidTensor);
    };
    Ok(GraphPlan::C4(
        count,
        bytes,
        IndexContext::C0(l1, l2, l3, l4, l5, l6, l7, l8),
        IndexContext::C0(r1, r2, r3, r4, r5, r6, r7, r8),
        CopyMode::C26(kind, parameter, rows, capacity),
    ))
}

fn normalization_word(
    index: u64,
    source: &IndexContext,
    destination: &IndexContext,
    secondary_strides: [u64; 4],
    mode: &CopyMode,
    flag: u64,
    bytes: u64,
) -> Result<GraphResponse, BackendError> {
    let IndexContext::C0(width, n1, n2, n3, s0, s1, s2, s3) = *source;
    let IndexContext::C0(_, _, _, _, t0, t1, t2, t3) = *destination;
    let [family, operation, a, from, b, right, target, to] = match *mode {
        CopyMode::C26(kind @ (64 | 65), epsilon, rows, capacity) => {
            let vtensor_norm::Adt0::C1(family, operation, a, from, b, right, target, to) =
                vtensor_norm::run(
                    kind, index, width, n1, n2, n3, s0, s1, s2, s3, t0, t1, t2, t3, epsilon, rows,
                    capacity, flag,
                )
                .map_err(|_| BackendError::Overflow)?
            else {
                return Err(BackendError::InvalidTensor);
            };
            [family, operation, a, from, b, right, target, to]
        }
        CopyMode::C26(66, epsilon, rows, capacity) => {
            let [x0, x1, x2, x3] = secondary_strides;
            let vtensor_rms_backward::Adt0::C1(family, operation, a, from, b, right, target, to) =
                vtensor_rms_backward::run(
                    index, width, n1, n2, n3, s0, s1, s2, s3, x0, x1, x2, x3, t0, t1, t2, t3,
                    epsilon, rows, capacity, flag,
                )
                .map_err(|_| BackendError::Overflow)?
            else {
                return Err(BackendError::InvalidTensor);
            };
            [family, operation, a, from, b, right, target, to]
        }
        CopyMode::C24(epsilon) => {
            let vtensor_l2::Adt0::C1(family, operation, a, from, b, right, target, to) =
                vtensor_l2::run(
                    index, width, n1, n2, n3, s0, s1, s2, s3, t0, t1, t2, t3, epsilon, flag,
                )
                .map_err(|_| BackendError::Overflow)?
            else {
                return Err(BackendError::InvalidTensor);
            };
            [family, operation, a, from, b, right, target, to]
        }
        _ => return Err(BackendError::InvalidTensor),
    };
    instruction_response([
        family, operation, 0, 0, a, from, b, right, target, to, bytes, bytes,
    ])
}

fn instruction_response(
    [family, operation, source_type, destination_type, a, from, b, right, target, to, read_bytes, write_bytes]: [u64; 12],
) -> Result<GraphResponse, BackendError> {
    let operand = |kind, at| match kind {
        0 => Ok(WordOperand::C0(0, at)),
        1 => Ok(WordOperand::C1(at)),
        2 => Ok(WordOperand::C2(at)),
        3 => Ok(WordOperand::C3(at)),
        4 => Ok(WordOperand::C0(1, at)),
        _ => Err(BackendError::InvalidTensor),
    };
    Ok(GraphResponse::C7(
        match family {
            0 => WordOperation::C0(operation),
            1 => WordOperation::C1(operation),
            2 => WordOperation::C5(operation),
            3 => WordOperation::C3(operation, operation, false),
            4 => WordOperation::C2(source_type, destination_type, false),
            _ => return Err(BackendError::InvalidTensor),
        },
        operand(a, from)?,
        operand(b, right)?,
        match target {
            0 | 1 => WordTarget::C0(to),
            3 => WordTarget::C1(to),
            _ => return Err(BackendError::InvalidTensor),
        },
        read_bytes,
        write_bytes,
    ))
}

fn execute_typed(tensor: &Tensor, plan: vtensor_typed::Adt2) -> Result<(), BackendError> {
    let vtensor_typed::Adt2::C0(count, capacity, ..) = &plan;
    let mut words = workspace(*capacity, 0_u64)?;
    // SAFETY: admission retained the primary descriptor and each present secondary.
    let (left, secondary) = unsafe { (&*tensor.src[0], tensor.src[1].as_ref()) };
    let primary = graph_tensor_data(left)?;
    let pointers = [primary, secondary.map_or(Ok(primary), graph_tensor_data)?];
    let output = graph_tensor_data(tensor)?;
    for index in 0..*count {
        let instruction =
            vtensor_typed::run(vtensor_typed::Adt0::C1(index, plan.clone(), words[2]))
                .map_err(|_| BackendError::Overflow)?;
        if let vtensor_typed::Adt3::C4(first, second, target) = instruction {
            let operand = |slot: u64| {
                words
                    .get(slot as usize)
                    .map(|word| f32::from_bits(*word as u32))
                    .ok_or(BackendError::InvalidTensor)
            };
            let value = verdant_tensor::linear::swiglu_f32(operand(first)?, operand(second)?)
                .map_err(|_| BackendError::InvalidTensor)?;
            *words
                .get_mut(target as usize)
                .ok_or(BackendError::InvalidTensor)? = u64::from(value.to_bits());
            continue;
        }
        let vtensor_typed::Adt3::C3(
            family,
            operation,
            source_type,
            destination_type,
            a,
            from,
            b,
            right,
            target,
            to,
            read_bytes,
            write_bytes,
        ) = instruction
        else {
            return Err(BackendError::InvalidTensor);
        };
        execute_word(
            instruction_response([
                if family == 0 { 4 } else { family },
                operation,
                source_type,
                destination_type,
                a,
                from,
                b,
                right,
                target,
                to,
                read_bytes,
                write_bytes,
            ])?,
            &pointers,
            output,
            &mut words,
        )?;
    }
    Ok(())
}

fn execute_copy(tensor: &Tensor, plan: GraphPlan) -> Result<(), BackendError> {
    let GraphPlan::C4(count, bytes, source, destination, mode) = plan else {
        return Err(BackendError::InvalidTensor);
    };
    let GraphResponse::C6(mask, literal_type, bits, constant) =
        vtensor_pointwise::run(GraphRequest::C2(mode.clone()))
            .map_err(|_| BackendError::Overflow)?
    else {
        return Err(BackendError::InvalidTensor);
    };
    let mut literal = [0_u8; 8];
    if constant {
        let value = verdant_tensor::linear::convert_float_storage(0, literal_type, bits)
            .map_err(|_| BackendError::Overflow)?
            .ok_or(BackendError::InvalidTensor)?;
        // SAFETY: scalar_word accepts widths of at most eight bytes; literal owns all eight.
        unsafe {
            scalar_word(literal.as_mut_ptr(), bytes, Some(value))?;
        }
    }
    let mut pointers = tensor.src.map(|_| literal.as_mut_ptr());
    if mask >> pointers.len() != 0 {
        return Err(BackendError::InvalidTensor);
    }
    let mut secondary_strides = [0; 4];
    for (slot, pointer) in pointers.iter_mut().enumerate() {
        if mask & (1 << slot) != 0 {
            // SAFETY: GGML retains each descriptor; absent required sources are typed refusals.
            let source = unsafe { tensor.src[slot].as_ref() }.ok_or(BackendError::InvalidTensor)?;
            *pointer = graph_tensor_data(source)?;
            if slot == 1 && matches!(mode, CopyMode::C26(66, ..)) {
                secondary_strides = source.nb.map(|stride| stride as u64);
            }
        }
    }
    let destination_data = graph_tensor_data(tensor)?;
    let mut block = workspace(bytes, 0_u8)?;
    let mut fixed = [0; 3];
    let mut dynamic = match &mode {
        CopyMode::C26(_, _, _, size) if *size > fixed.len() as u64 => {
            Some(workspace(*size, 0_u64)?)
        }
        _ => None,
    };
    let words = dynamic.as_deref_mut().unwrap_or(&mut fixed);
    for index in 0..count {
        let response = match &mode {
            CopyMode::C26(64..=66, ..) | CopyMode::C24(_) => normalization_word(
                index,
                &source,
                &destination,
                secondary_strides,
                &mode,
                words[2],
                bytes,
            )?,
            _ => vtensor_pointwise::run(GraphRequest::C1(
                index,
                source.clone(),
                destination.clone(),
                mode.clone(),
                words[2],
            ))
            .map_err(|_| BackendError::Overflow)?,
        };
        if let GraphResponse::C7(..) = response {
            execute_word(response, &pointers, destination_data, words)?;
            continue;
        }
        if let GraphResponse::C5(kind, from, bytes, to, span, stride) = response {
            // SAFETY: Prism admits both whole tensors and these block spans,
            // refuses source/output aliases, and preflights every index before writes.
            unsafe {
                let input =
                    std::slice::from_raw_parts(pointers[0].add(from as usize), bytes as usize);
                let output = std::slice::from_raw_parts_mut(
                    destination_data.add(to as usize),
                    span as usize,
                );
                verdant_tensor::linear::decode_storage_block(kind, input, output, stride as usize)
                    .map_err(|_| BackendError::InvalidTensor)?;
            }
            continue;
        }
        let GraphResponse::C1(second, from, to) = response else {
            return Err(BackendError::InvalidTensor);
        };
        // SAFETY: Prism bounds both block spans. Capture the source before borrowing an aliased output.
        unsafe {
            let pointer = pointers[usize::from(second)].add(from as usize);
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
            let flags = u64::from((*tensor).flags as u32);
            if !vtensor_native_predicates::run(vtensor_native_predicates::Adt0::C2(flags)) {
                continue;
            }
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
                ExecutionPlan::Standard(plan @ GraphPlan::C3(..)) => execute_matrix(&*tensor, plan),
                ExecutionPlan::Standard(plan @ GraphPlan::C4(..)) => execute_copy(&*tensor, plan),
                ExecutionPlan::Typed(plan) => execute_typed(&*tensor, plan),
                ExecutionPlan::Quantized(plan) => execute_quantized_matrix(&*tensor, plan),
                _ => Err(BackendError::InvalidTensor),
            };
            if result.is_err() {
                return GGML_STATUS_FAILED;
            }
        }
    }
    GGML_STATUS_SUCCESS
}
