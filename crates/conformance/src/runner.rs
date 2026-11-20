//! Native effects realizing the Prism plans at the imported GGML boundary.

use crate::abi::*;
use std::cell::UnsafeCell;
use std::ffi::{c_char, c_int, c_void};
use std::ptr::{self, null, null_mut};
use std::sync::OnceLock;
use vtensor_pointwise::Adt0 as GraphPlan;

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
        let physical = usize::try_from(physical).map_err(|_| BackendError::Overflow)?;
        let mut owned = Vec::new();
        owned
            .try_reserve_exact(physical)
            .map_err(|_| BackendError::Allocation)?;
        owned.resize(physical, state().initial_byte);
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

fn tensor_span(tensor: &Tensor) -> Result<usize, BackendError> {
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
    let ne = [n0, n1, n2, n3];
    let nb = tensor.nb.map(|n| n as u64);
    match vtensor_layout::run(vtensor_layout::Adt0::C0(
        ne[0], ne[1], ne[2], ne[3], nb[0], nb[1], nb[2], nb[3], block, element,
    ))
    .map_err(|_| BackendError::Overflow)?
    {
        vtensor_layout::Adt1::C1(size) => usize::try_from(size).map_err(|_| BackendError::Overflow),
        _ => Err(BackendError::InvalidTensor),
    }
}

fn require<T>(result: Result<T, BackendError>) -> T {
    match result {
        Ok(value) => value,
        Err(error) => {
            let message = match error {
                BackendError::Allocation => c"vTensor allocation refused",
                BackendError::Overflow => c"vTensor size overflow",
                BackendError::InvalidTensor => c"vTensor tensor not allocated",
                BackendError::OutOfBounds => c"vTensor out of bounds",
            };
            // SAFETY: GGML's abort interface accepts static NUL-terminated
            // strings. An invalid callback precondition follows its abort ABI.
            unsafe { ggml_abort(c"vTensor".as_ptr(), 0, message.as_ptr()) }
        }
    }
}

// Bind Prism configuration values to GGML's typed query callbacks.
macro_rules! configuration_queries {
    ($($name:ident($owner:ty) -> $result:ty = $value:expr;)*) => {$(
        unsafe extern "C" fn $name(_: $owner) -> $result { $value }
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
}

unsafe extern "C" fn device_at(_: Reg, index: usize) -> Dev {
    if vtensor_native_predicates::run(vtensor_native_predicates::Adt0::C0(index as u64)) {
        state().dev.get()
    } else {
        null_mut()
    }
}

unsafe extern "C" fn supports_buffer_type(_: Dev, buft: Buft) -> bool {
    buft == state().buft.get()
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

unsafe extern "C" fn buffer_allocate(buft: Buft, size: usize) -> Buf {
    let Ok(context) = BufferContext::allocate(size) else {
        return null_mut();
    };
    // SAFETY: GGML allocates its own wrapper; the free callback releases only
    // our Rust context. This preserves ownership across the two allocators.
    unsafe {
        ggml_backend_buffer_init(
            buft,
            buffer_interface(),
            Box::into_raw(Box::new(context)).cast(),
            size,
        )
    }
}

unsafe extern "C" fn from_host(_: Dev, pointer: *mut c_void, size: usize, _: usize) -> Buf {
    let Ok(context) = BufferContext::borrowed(pointer, size) else {
        return null_mut();
    };
    // SAFETY: the caller owns a writable host region until buffer destruction.
    // Its bytes are borrowed and are neither initialized nor freed here.
    unsafe {
        ggml_backend_buffer_init(
            state().buft.get(),
            buffer_interface(),
            Box::into_raw(Box::new(context)).cast(),
            size,
        )
    }
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

unsafe extern "C" fn tensor_set(
    buffer: Buf,
    tensor: TensorPtr,
    source: *const c_void,
    offset: usize,
    size: usize,
) {
    if size == 0 {
        return;
    }
    // SAFETY: GGML requires a readable source of size bytes; the certified
    // admissions validate the destination. copy permits overlapping transfers.
    unsafe {
        let context = &*(*buffer).context.cast::<BufferContext>();
        let destination = require(context.region(&*tensor, offset, size));
        ptr::copy(source.cast::<u8>(), destination, size);
    }
}

unsafe extern "C" fn tensor_get(
    buffer: Buf,
    tensor: *const Tensor,
    destination: *mut c_void,
    offset: usize,
    size: usize,
) {
    if size == 0 {
        return;
    }
    // SAFETY: GGML requires a writable destination of size bytes; the certified
    // admissions validate the source. No slice borrow crosses the foreign call.
    unsafe {
        let context = &*(*buffer).context.cast::<BufferContext>();
        let source = require(context.region(&*tensor, offset, size));
        ptr::copy(source, destination.cast::<u8>(), size);
    }
}

unsafe extern "C" fn tensor_memset(
    buffer: Buf,
    tensor: TensorPtr,
    value: u8,
    offset: usize,
    size: usize,
) {
    if size == 0 {
        return;
    }
    // SAFETY: the certified admissions establish this writable tensor region.
    unsafe {
        let context = &*(*buffer).context.cast::<BufferContext>();
        let destination = require(context.region(&*tensor, offset, size));
        ptr::write_bytes(destination, value, size);
    }
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
    let span = tensor_span(tensor)?;
    let operation = u64::try_from(tensor.op).map_err(|_| BackendError::InvalidTensor)?;
    if vtensor_native_predicates::run(vtensor_native_predicates::Adt0::C1(operation, span as u64)) {
        return Ok(None);
    }
    if tensor.src[0].is_null() {
        return Err(BackendError::InvalidTensor);
    }
    // SAFETY: GGML retains every non-null source descriptor during queries and execution.
    let (left, right) = unsafe { (&*tensor.src[0], tensor.src[1].as_ref()) };
    tensor_span(left)?;
    if let Some(right) = right {
        tensor_span(right)?;
    }
    let [Ok(k), Ok(m), Ok(l2), Ok(l3)] = left.ne.map(u64::try_from) else {
        return Err(BackendError::InvalidTensor);
    };
    let [Ok(rk), Ok(n), Ok(b2), Ok(b3)] = right.map_or([1; 4], |r| r.ne).map(u64::try_from) else {
        return Err(BackendError::InvalidTensor);
    };
    let [Ok(om), Ok(on), Ok(o2), Ok(o3)] = tensor.ne.map(u64::try_from) else {
        return Err(BackendError::InvalidTensor);
    };
    let pointwise = vtensor_pointwise::run(
        operation,
        tensor.op_params[0] as u64,
        left.ty as u64,
        right.map_or(0, |r| r.ty as u64),
        tensor.ty as u64,
        right.is_some(),
        k,
        m,
        l2,
        l3,
        rk,
        n,
        b2,
        b3,
        om,
        on,
        o2,
        o3,
    )
    .map_err(|_| BackendError::Overflow)?;
    match pointwise {
        GraphPlan::C2(..) | GraphPlan::C3(..) => Ok(Some(pointwise)),
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

fn float_workspace(size: u64) -> Result<Vec<f32>, BackendError> {
    let count = usize::try_from(size).map_err(|_| BackendError::Overflow)?;
    let mut result = Vec::new();
    result
        .try_reserve_exact(count)
        .map_err(|_| BackendError::Allocation)?;
    result.resize(count, 0.0);
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
    let mut left_values = float_workspace(left_count)?;
    let mut right_values = float_workspace(right_count)?;
    let mut output_values = float_workspace(output_count)?;
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

fn execute_pointwise(tensor: &Tensor, plan: GraphPlan) -> Result<(), BackendError> {
    let GraphPlan::C2(kind, unary, n0, n1, n2, n3, _) = plan else {
        return Err(BackendError::InvalidTensor);
    };
    // SAFETY: the admitted plan requires a live left source and, for binary operations, a right source.
    let left = unsafe { &*tensor.src[0] };
    let left_data = graph_tensor_data(left)?;
    let output_data = graph_tensor_data(tensor)?;
    let right = if unary {
        None
    } else {
        // SAFETY: Prism admitted a binary operation with a retained right source.
        Some(unsafe { &*tensor.src[1] })
    };
    let right_data = right.map(graph_tensor_data).transpose()?;
    for i3 in 0..n3 {
        for i2 in 0..n2 {
            for i1 in 0..n1 {
                for i0 in 0..n0 {
                    let indices = [i0 as usize, i1 as usize, i2 as usize, i3 as usize];
                    let left_offset = element_offset(left, indices)?;
                    let output_offset = element_offset(tensor, indices)?;
                    // SAFETY: modeled extents and byte spans bound every unaligned F32 read.
                    let a =
                        unsafe { ptr::read_unaligned(left_data.add(left_offset).cast::<f32>()) };
                    let b = if let (Some(right), Some(data)) = (right, right_data) {
                        let [r0, r1, r2, r3] = right.ne.map(|n| n as u64);
                        let [s0, s1, s2, s3] = right.nb.map(|n| n as u64);
                        let offset = vtensor_repeat_offset::run(
                            i0, i1, i2, i3, r0, r1, r2, r3, s0, s1, s2, s3,
                        )
                        .map_err(|_| BackendError::Overflow)?;
                        let offset = usize::try_from(offset).map_err(|_| BackendError::Overflow)?;
                        // SAFETY: Prism's repeat planner bounds these coordinates by the right extents.
                        unsafe { ptr::read_unaligned(data.add(offset).cast::<f32>()) }
                    } else {
                        0.0
                    };
                    let value = verdant_tensor::linear::evaluate_f32_scalar(kind, a, b)
                        .map_err(|_| BackendError::InvalidTensor)?;
                    // SAFETY: the admitted output span bounds this modeled address.
                    unsafe {
                        ptr::write_unaligned(output_data.add(output_offset).cast::<f32>(), value);
                    }
                }
            }
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
                GraphPlan::C2(..) => execute_pointwise(&*tensor, plan),
                _ => Err(BackendError::InvalidTensor),
            };
            if result.is_err() {
                return GGML_STATUS_FAILED;
            }
        }
    }
    GGML_STATUS_SUCCESS
}
