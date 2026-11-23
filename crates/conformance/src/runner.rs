//! Native effects realizing the Prism plans at the imported GGML boundary.

use crate::abi::*;
use std::cell::UnsafeCell;
use std::ffi::{c_char, c_int, c_void};
use std::ptr::{self, null, null_mut};
use std::sync::OnceLock;
use verdant_tensor::{linear, TensorComputeError};
use vtensor_matrix_q8::Adt2 as RowProductPlan;
use vtensor_pointwise::{
    Adt0 as GraphRequest, Adt1 as Parameters, Adt10 as GraphPlan, Adt11 as WordOperation,
    Adt12 as WordOperand, Adt13 as WordTarget, Adt2 as TensorDescriptor, Adt3 as IndexContext,
    Adt4 as CopyMode, Adt8 as ConvolutionGeometry, Adt9 as GraphResponse,
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

impl From<TensorComputeError> for BackendError {
    fn from(_: TensorComputeError) -> Self {
        Self::InvalidTensor
    }
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
                    get_proc_address: Some(threadpool::lookup),
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

    fn region(
        &self,
        tensor: &Tensor,
        span: usize,
        offset: usize,
        size: usize,
    ) -> Result<*mut u8, BackendError> {
        use vtensor_buffer_region::Adt0 as Region;
        let decision = vtensor_buffer_region::run(
            self.base as usize as u64,
            self.size as u64,
            tensor.data as usize as u64,
            span as u64,
            offset as u64,
            size as u64,
        )
        .map_err(|_| BackendError::Overflow)?;
        match decision {
            Region::C0 => Ok(null_mut()),
            Region::C1 => Err(BackendError::InvalidTensor),
            Region::C2 => Err(BackendError::OutOfBounds),
            // SAFETY: Prism bounded the whole tensor and requested range inside this backing allocation.
            Region::C3(at) => Ok(unsafe { self.base.add(at as usize) }),
        }
    }
}

fn tensor_metadata(tensor: &Tensor) -> Result<TensorDescriptor, BackendError> {
    let ty = u64::try_from(tensor.ty).map_err(|_| BackendError::InvalidTensor)?;
    let vtensor_types::Adt0::C1(vtensor_types::Adt1::C0(_, block, element)) =
        vtensor_types::run(ty)
    else {
        return Err(BackendError::InvalidTensor);
    };
    // Zero extents are empty. Negative ABI extents cannot inhabit the Prism
    // model's natural-number domain and must fail before buffer admission.
    let [Ok(n0), Ok(n1), Ok(n2), Ok(n3)] = tensor.ne.map(u64::try_from) else {
        return Err(BackendError::InvalidTensor);
    };
    let [s0, s1, s2, s3] = tensor.nb.map(|n| n as u64);
    let layout = vtensor_layout::Adt0::C0(n0, n1, n2, n3, s0, s1, s2, s3, block, element);
    let vtensor_layout::Adt1::C1(size) =
        vtensor_layout::run(layout).map_err(|_| BackendError::Overflow)?
    else {
        return Err(BackendError::InvalidTensor);
    };
    usize::try_from(size).map_err(|_| BackendError::Overflow)?;
    let address = tensor.data as usize as u64;
    Ok(TensorDescriptor::C0(
        ty, n0, n1, n2, n3, s0, s1, s2, s3, size, address,
    ))
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
        tensor_span(tensor).and_then(|size| context.region(tensor, size, 0, size))
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
                let tensor = &*tensor;
                let region = require(context.region(tensor, require(tensor_span(tensor)), offset, size));
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
        let Ok(target) = context.region(&*destination, size, 0, size) else {
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
        context: threadpool::new_context(),
    }))
}

unsafe extern "C" fn backend_free(backend: BackendPtr) {
    // SAFETY: GGML delegates backend destruction to this callback exactly once.
    unsafe {
        threadpool::free_context(backend);
        drop(Box::from_raw(backend));
    }
}

enum ExecutionPlan {
    Standard(GraphPlan),
    Typed(vtensor_typed::Adt2),
    RowProduct(RowProductPlan),
    Rotary(vtensor_rotary::Adt3),
    Attention(Box<vtensor_attention::Adt2>),
    Softmax(Box<vtensor_attention::Adt3>),
}

// Each certified graph bundle renders its own type for the same GGML metadata.
macro_rules! descriptor_for {
    ($construct:path) => {
        |TensorDescriptor::C0(kind, n0, n1, n2, n3, s0, s1, s2, s3, span, address)| {
            $construct(kind, n0, n1, n2, n3, s0, s1, s2, s3, span, address)
        }
    };
}

fn pointwise(request: GraphRequest) -> Result<GraphResponse, BackendError> {
    vtensor_pointwise::run(request).map_err(|_| BackendError::Overflow)
}

impl Tensor {
    fn source(&self, slot: usize) -> Option<&Tensor> {
        // SAFETY: GGML retains every present source descriptor for the graph call.
        unsafe { self.src[slot].as_ref() }
    }

    fn input(&self, slot: usize) -> Result<Option<TensorInput<'_>>, BackendError> {
        self.source(slot).map(TensorInput::new).transpose()
    }
}

fn plan_operation(tensor: &Tensor) -> Result<Option<ExecutionPlan>, BackendError> {
    use BackendError::InvalidTensor;
    let output = tensor_metadata(tensor)?;
    let TensorDescriptor::C0(_, _, _, _, _, _, _, _, _, span, _) = &output;
    let operation = u64::try_from(tensor.op).map_err(|_| InvalidTensor)?;
    if vtensor_native_predicates::run(vtensor_native_predicates::Adt0::C1(operation, *span)) {
        return Ok(None);
    }
    let source = |slot| tensor.source(slot).map(tensor_metadata).transpose();
    let [p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14, p15] =
        tensor.op_params.map(|word| u64::from(word as u32));
    let primary = source(0)?;
    let secondary = source(1)?;
    let base = source(2)?;
    if operation == 43 {
        return attention::softmax_plan(
            tensor,
            primary.ok_or(InvalidTensor)?,
            secondary,
            base,
            output,
        );
    }
    if operation == 71 {
        return attention::plan(
            tensor,
            [primary, secondary, base, source(3)?, source(4)?],
            output,
        );
    }
    if matches!(operation, 45 | 46) {
        return rotary::plan(
            tensor,
            vtensor_rotary::Adt1::C0(p1, p2, p15, p11, p12, p13, p14),
            primary.ok_or(InvalidTensor)?,
            secondary.ok_or(InvalidTensor)?,
            base,
            output,
        )
        .map(|plan| plan.map(ExecutionPlan::Rotary));
    }
    // Extra ABI sources still have to be valid descriptors. The pointwise model consumes three.
    for slot in 3..tensor.src.len() {
        source(slot)?;
    }
    let response = pointwise(GraphRequest::C0(
        operation,
        Parameters::C0(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11),
        primary.clone(),
        secondary.clone(),
        base.clone(),
        output.clone(),
    ))?;
    let metadata = descriptor_for!(vtensor_typed::Adt1::C0);
    let typed_plan =
        |request| match vtensor_typed::run(request).map_err(|_| BackendError::Overflow)? {
            vtensor_typed::Adt3::C2(plan) => Ok(Some(ExecutionPlan::Typed(plan))),
            _ => Err(InvalidTensor),
        };
    match response {
        GraphResponse::C0(plan @ (GraphPlan::C3(..) | GraphPlan::C4(..) | GraphPlan::C6(..))) => {
            Ok(Some(ExecutionPlan::Standard(plan)))
        }
        GraphResponse::C0(GraphPlan::C5(67)) => typed_plan(vtensor_typed::Adt0::C0(
            operation,
            primary.map(metadata),
            secondary.map(metadata),
            metadata(output),
        )),
        _ => {
            let descriptor = |TensorDescriptor::C0(ty, a, b, c, d, e, f, g, h, _, _)| {
                vtensor_matrix_q8::Adt1::C0(ty, a, b, c, d, e, f, g, h)
            };
            let extra = tensor.src[3..].iter().any(|ptr| !ptr.is_null());
            let w = primary.ok_or(InvalidTensor)?;
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
                    _ => return Err(InvalidTensor),
                }
            }
            let a = secondary.ok_or(InvalidTensor)?;
            use vtensor_matrix_q8::Adt0 as Product;
            let request = if let Some(third) = base {
                if operation != 29 {
                    let [left, right, base, out] = [w, a, third, output].map(metadata);
                    return typed_plan(vtensor_typed::Adt0::C2(
                        operation, extra, left, right, base, out,
                    ));
                }
                let [w, a, ids, out] = [w, a, third, output].map(descriptor);
                Product::C3(operation, extra, w, a, ids, out)
            } else {
                let [w, a, out] = [w, a, output].map(descriptor);
                Product::C0(operation, extra, w, a, out)
            };
            match vtensor_matrix_q8::run(request).map_err(|_| BackendError::Overflow)? {
                vtensor_matrix_q8::Adt4::C1(plan) => Ok(Some(ExecutionPlan::RowProduct(plan))),
                _ => Err(InvalidTensor),
            }
        }
    }
}

fn graph_tensor_data(tensor: &Tensor) -> Result<*mut u8, BackendError> {
    graph_tensor_region(tensor, tensor_span(tensor)?)
}

fn graph_tensor_region(tensor: &Tensor, span: usize) -> Result<*mut u8, BackendError> {
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
        context.region(tensor, span, 0, span)
    }
}

fn reserved<T>(count: usize) -> Result<Vec<T>, BackendError> {
    let mut result = Vec::new();
    result
        .try_reserve_exact(count)
        .map_err(|_| BackendError::Allocation)?;
    Ok(result)
}

fn workspace<T: Clone>(size: u64, initial: T) -> Result<Vec<T>, BackendError> {
    let count = usize::try_from(size).map_err(|_| BackendError::Overflow)?;
    let mut result = reserved(count)?;
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
    let weights = TensorInput::new(left)?;
    let pointers = [
        weights.bytes.as_ptr().cast_mut(),
        graph_tensor_data(right)?,
        graph_tensor_data(tensor)?,
    ];
    let [left_stride, right_stride, output_stride] = [left, right, tensor].map(|t| {
        let [s0, s1, s2, s3] = t.nb.map(|s| s as u64);
        Strides::C0(s0, s1, s2, s3)
    });
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
            left_stride.clone(),
            right_stride.clone(),
            output_stride.clone(),
            left.ty as u64,
            right.ty as u64,
            tensor.op as u64,
        )
        .map_err(|_| BackendError::Overflow)
    };
    let Instruction::C0(steps, total) = step(0)? else {
        return Err(BackendError::InvalidTensor);
    };
    let mut values = workspace(total, 0.0_f32)?;
    let [m, k, n] = [m, k, n].map(|n| n as usize);
    let mut index = 1;
    while index < steps {
        let instruction = step(index)?;
        index += 1;
        match instruction {
            // SAFETY: Prism bounds the private workspace slot, foreign pointer
            // slot and byte offset. Native F32 words may be unaligned; byte
            // transfers preserve all encodings without creating foreign borrows.
            Instruction::C1(writing, slot, at, offset) => unsafe {
                let external = pointers[slot as usize].add(offset as usize).cast::<f32>();
                let word = &mut values[at as usize];
                if writing {
                    external.write_unaligned(*word);
                } else {
                    *word = external.read_unaligned();
                }
            },
            Instruction::C4(slot, at, offset, source, dot_type, width) => {
                // SAFETY: the certified load carries the encoded width and bounded offset.
                let word = unsafe {
                    scalar_word(pointers[slot as usize].add(offset as usize), width, None)?
                };
                values[at as usize] = prepare_float(source, dot_type, word)?;
            }
            Instruction::C5(at, from, kind, bytes, stride, skip) => {
                let block = weights.region(from, bytes as usize)?;
                let dst = &mut values[at as usize..left_count as usize];
                decode_words(kind, block, dst, stride as usize)?;
                index = index.checked_add(skip).ok_or(BackendError::Overflow)?;
            }
            Instruction::C2 => {
                let (left, rest) = values.split_at_mut(left_count as usize);
                let (right, output) = rest.split_at_mut(right_count as usize);
                linear::multiply_matrix_rows_f32(m, k, n, left, right, output)?;
            }
            _ => return Err(BackendError::InvalidTensor),
        }
    }
    Ok(())
}

fn execute_convolution(tensor: &Tensor, geometry: ConvolutionGeometry) -> Result<(), BackendError> {
    let ConvolutionGeometry::C0(
        patches,
        inner,
        channels,
        weight_count,
        tile,
        input_capacity,
        output_capacity,
        kind,
        ..,
    ) = geometry;
    // SAFETY: the certified plan requires both input descriptors and disjoint output storage.
    let (kernel, image) = unsafe { (&*tensor.src[0], &*tensor.src[1]) };
    let (kernel, image) = (graph_tensor_data(kernel)?, graph_tensor_data(image)?);
    let destination = graph_tensor_data(tensor)?;
    let sample = |phase, start, index| {
        let query = GraphRequest::C4(geometry.clone(), phase, start, index as u64);
        match pointwise(query)? {
            GraphResponse::C8(offset) => Ok(offset),
            _ => Err(BackendError::InvalidTensor),
        }
    };
    let count = usize::try_from(weight_count).map_err(|_| BackendError::Overflow)?;
    let mut weights = reserved(count)?;
    for index in 0..count {
        let offset = sample(0, 0, index)?.ok_or(BackendError::InvalidTensor)?;
        // SAFETY: the plan bounds the full kernel word, including unaligned views.
        let value = unsafe {
            let pointer = kernel.add(offset as usize);
            if kind == 0 {
                pointer.cast::<f32>().read_unaligned()
            } else {
                decode_float(kind, u64::from(pointer.cast::<u16>().read_unaligned()))?
            }
        };
        weights.push(value);
    }
    let mut input = workspace(input_capacity, 0.0_f32)?;
    let mut output = workspace(output_capacity, 0.0_f32)?;
    for start in (0..patches).step_by(tile as usize) {
        let rows = tile.min(patches - start) as usize;
        let input = &mut input[..rows * inner as usize];
        for (index, value) in input.iter_mut().enumerate() {
            *value = match sample(1, start, index)? {
                // SAFETY: Prism resolves each interior image coordinate through its actual byte strides.
                Some(at) => unsafe { image.add(at as usize).cast::<f32>().read_unaligned() },
                None => 0.0,
            };
        }
        let output = &mut output[..rows * channels as usize];
        linear::multiply_matrix_rows_f32(
            rows,
            inner as usize,
            channels as usize,
            input,
            &weights,
            output,
        )?;
        for (index, &value) in output.iter().enumerate() {
            let at = sample(2, start, index)?.ok_or(BackendError::InvalidTensor)?;
            // SAFETY: the plan admits the full output span and resolves coordinates in its view.
            unsafe {
                destination
                    .add(at as usize)
                    .cast::<f32>()
                    .write_unaligned(value)
            };
        }
    }
    Ok(())
}

/// The caller supplies a live, disjoint destination spanning the whole row.
unsafe fn write_row<T: Copy>(destination: *mut u8, row: &[T]) {
    // SAFETY: the caller validates the region. Byte copies permit unaligned GGML views.
    unsafe {
        ptr::copy_nonoverlapping(row.as_ptr().cast(), destination, std::mem::size_of_val(row))
    }
}

fn decode_words(
    kind: u64,
    source: &[u8],
    destination: &mut [f32],
    stride: usize,
) -> Result<(), BackendError> {
    // SAFETY: all f32 bit patterns are valid and the mutable slice owns these bytes.
    let bytes = unsafe {
        std::slice::from_raw_parts_mut(
            destination.as_mut_ptr().cast(),
            std::mem::size_of_val(destination),
        )
    };
    Ok(linear::decode_storage_block(kind, source, bytes, stride)?)
}

struct TensorInput<'a> {
    bytes: &'a [u8],
    ty: c_int,
    ne: [i64; 4],
    nb: [usize; 4],
}

impl<'a> TensorInput<'a> {
    fn new(t: &'a Tensor) -> Result<Self, BackendError> {
        let span = tensor_span(t)?;
        let bytes = if span == 0 {
            &[]
        } else {
            // SAFETY: GGML retains this checked native region throughout graph execution.
            unsafe { std::slice::from_raw_parts(graph_tensor_region(t, span)?, span) }
        };
        Ok(Self {
            bytes,
            ty: t.ty,
            ne: t.ne,
            nb: t.nb,
        })
    }

    fn region(&self, offset: u64, count: usize) -> Result<&[u8], BackendError> {
        if count == 0 {
            return Ok(&[]);
        }
        let offset = usize::try_from(offset).map_err(|_| BackendError::Overflow)?;
        self.bytes
            .get(offset..)
            .and_then(|b| b.get(..count))
            .ok_or(BackendError::OutOfBounds)
    }
}

fn execute_row_products(
    execution: &threadpool::Execution,
    tensor: &Tensor,
    plan: RowProductPlan,
) -> Result<(), BackendError> {
    use linear::{dot_grouped_q2_row, encode_grouped_q8_block};
    use RowProductPlan::C0;
    let C0(_, weight_bytes, ..) = plan;
    let symmetric = linear::encode_symmetric_q8_block::<32>;
    macro_rules! packed {
        ($encode:expr, $dot:path) => {
            execute_block_rows(
                execution,
                0,
                tensor,
                plan,
                |x| Ok($encode(x)?),
                |source, at, a, _| Ok($dot(source.region(at, weight_bytes as usize)?, a)?),
            )
        };
    }
    match plan {
        C0(inner, .., Some(precision), _) => execute_block_rows(
            execution,
            inner,
            tensor,
            plan,
            |&[x]| {
                if precision == 0 {
                    Ok(x)
                } else {
                    prepare_float(0, precision, x.to_bits().into())
                }
            },
            |source, at, activations, weights| {
                source.decode(at, weights)?;
                let mut value = [0.0];
                linear::multiply_matrix_rows_f32(
                    1,
                    weights.len(),
                    1,
                    weights,
                    activations,
                    &mut value,
                )?;
                Ok(value[0])
            },
        ),
        C0(.., None, 2) => packed!(symmetric, linear::dot_symmetric_q4_row),
        C0(.., None, 8) => packed!(symmetric, linear::dot_symmetric_q8_row),
        C0(.., None, 10) => packed!(encode_grouped_q8_block, dot_grouped_q2_row),
        _ => Err(BackendError::InvalidTensor),
    }
}

fn execute_block_rows<const LANES: usize, Block: Sync>(
    execution: &threadpool::Execution,
    scratch_width: u64,
    tensor: &Tensor,
    plan: RowProductPlan,
    encode: impl Fn(&[f32; LANES]) -> Result<Block, BackendError>,
    dot: impl Fn(&TensorInput, u64, &[Block], &mut [f32]) -> Result<f32, BackendError> + Sync,
) -> Result<(), BackendError> {
    use vtensor_matrix_q8::{run, Adt0 as Request, Adt2 as Plan, Adt4 as Response};
    let Plan::C0(blocks, _, rows, _, _, visits, outputs, ..) = plan;
    // SAFETY: graph admission retained both typed GGML source descriptors.
    let (w, a) = unsafe { (&*tensor.src[0], &*tensor.src[1]) };
    let weights = TensorInput::new(w)?;
    let indices = tensor.input(2)?;
    let request = |query| run(query).map_err(|_| BackendError::Overflow);
    let activations = graph_tensor_data(a)?;
    let destination = graph_tensor_data(tensor)?;
    let mut results = workspace(outputs, 0.0f32)?;
    let mut offsets = workspace(visits, 0u64)?;
    let mut prepared = reserved(usize::try_from(blocks).map_err(|_| BackendError::Overflow)?)?;
    for (column, (values, offset)) in results
        .chunks_exact_mut(rows as usize)
        .zip(&mut offsets)
        .enumerate()
    {
        let column = column as u64;
        let mut address = request(Request::C1(plan.clone(), column))?;
        if let Response::C4(at) = address {
            let source = indices.as_ref().ok_or(BackendError::InvalidTensor)?;
            let expert = u32::from_ne_bytes(source.region(at, 4)?.try_into().unwrap());
            address = request(Request::C4(plan.clone(), column, expert.into()))?;
        }
        let (weight_base, activation, dest) = match address {
            Response::C2(w, a, o) => (w, a, o),
            Response::C5(dest) => {
                *offset = dest;
                continue;
            }
            _ => return Err(BackendError::InvalidTensor),
        };
        *offset = dest;
        prepared.clear();
        for block in 0..blocks {
            // SAFETY: the certified column lies in the admitted activation span and has
            // exactly this many contiguous blocks. GGML views may be unaligned.
            let lanes = unsafe {
                activations
                    .add(activation as usize)
                    .cast::<[f32; LANES]>()
                    .add(block as usize)
                    .read_unaligned()
            };
            prepared.push(encode(&lanes)?);
        }
        execution.fill(values, |first, chunk| {
            let mut scratch = workspace(scratch_width, 0.0_f32)?;
            for (index, value) in chunk.iter_mut().enumerate() {
                let row = first + index;
                let Response::C3(start) =
                    request(Request::C2(plan.clone(), weight_base, row as u64))?
                else {
                    return Err(BackendError::InvalidTensor);
                };
                *value = dot(&weights, start, &prepared, &mut scratch)?;
            }
            Ok(())
        })?;
    }
    for (column, offset) in results.chunks_exact(rows as usize).zip(offsets) {
        // SAFETY: each certified column fits the checked buffer and the private
        // result workspace is disjoint from it. Byte copying permits unaligned views.
        unsafe { write_row(destination.add(offset as usize), column) }
    }
    Ok(())
}

unsafe fn scalar_word(
    pointer: *mut u8,
    width: u64,
    value: Option<u64>,
) -> Result<u64, BackendError> {
    macro_rules! access {
        ($ty:ty) => {
            match value {
                Some(word) => {
                    // SAFETY: the admitted write span contains this entire native word.
                    unsafe { pointer.cast::<$ty>().write_unaligned(word as $ty) };
                    word
                }
                // SAFETY: the admitted read span covers this width, including unaligned views.
                None => u64::from(unsafe { pointer.cast::<$ty>().read_unaligned() }),
            }
        };
    }
    Ok(match width {
        1 => access!(u8),
        2 => access!(u16),
        4 => access!(u32),
        8 => access!(u64),
        _ => return Err(BackendError::InvalidTensor),
    })
}

fn convert(from: u64, to: u64, word: u64) -> Result<u64, BackendError> {
    linear::convert_float_storage(from, to, word)
        .map_err(|_| BackendError::Overflow)?
        .ok_or(BackendError::InvalidTensor)
}

fn decode_float(storage: u64, word: u64) -> Result<f32, BackendError> {
    let bits = if storage == 43 {
        linear::evaluate_small_float(3, word)
            .map_err(|_| BackendError::Overflow)?
            .ok_or(BackendError::InvalidTensor)?
    } else {
        convert(storage, 0, word)?
    };
    Ok(f32::from_bits(bits as u32))
}

fn prepare_float(source: u64, dot_type: u64, word: u64) -> Result<f32, BackendError> {
    if source == dot_type || dot_type == 0 {
        decode_float(source, word)
    } else {
        decode_float(dot_type, convert(source, dot_type, word)?)
    }
}

// The F32 GGML profile returns an unconditional positive zero below its lower
// cutoff. The gated operation must preserve that result even for a negative gate.
fn gaussian_table(word: u64) -> Result<Option<f32>, BackendError> {
    match pointwise(GraphRequest::C5(word))? {
        GraphResponse::C9(Some(0)) => Ok(None),
        GraphResponse::C9(Some(bits)) => Ok(Some(f32::from_bits(bits as u32))),
        GraphResponse::C9(None) => {
            let input = prepare_float(0, 1, word)?;
            let value = linear::gelu_f32(input)?;
            Ok(Some(prepare_float(0, 1, u64::from(value.to_bits()))?))
        }
        _ => Err(BackendError::InvalidTensor),
    }
}

fn gated_value(variant: u64, storage: u64, input: f32, gate: f32) -> Result<f32, BackendError> {
    let word = u64::from(input.to_bits());
    let activation = match variant {
        0 => match linear::evaluate_encoded_word(0, word, 0)
            .map_err(|_| BackendError::InvalidTensor)?
        {
            Some(2) => input,
            Some(_) => return Ok(0.0),
            None => return Err(BackendError::InvalidTensor),
        },
        1 if storage == 0 => match gaussian_table(word)? {
            Some(value) => value,
            None => return Ok(0.0),
        },
        1 => linear::gelu_f32(input)?,
        2 => return Ok(linear::swiglu_f32(input, gate)?),
        5 => linear::gelu_quick_f32(input)?,
        _ => return Err(BackendError::InvalidTensor),
    };
    let activation = if storage == 1 && variant != 0 {
        prepare_float(0, 1, u64::from(activation.to_bits()))?
    } else {
        activation
    };
    Ok(linear::evaluate_f32_scalar(2, activation, gate)?)
}

fn execute_word(
    response: GraphResponse,
    pointers: &[*mut u8],
    destination: *mut u8,
    words: &mut [u64],
) -> Result<(), BackendError> {
    use BackendError::InvalidTensor;
    let GraphResponse::C7(operation, left, right, target, read_bytes, write_bytes) = response
    else {
        return Err(InvalidTensor);
    };
    let read = |operand| match operand {
        WordOperand::C1(index) => words.get(index as usize).copied().ok_or(InvalidTensor),
        WordOperand::C2(value) => Ok(value),
        operand => {
            let (pointer, offset) = match operand {
                WordOperand::C0(index, offset) => {
                    (*pointers.get(index as usize).ok_or(InvalidTensor)?, offset)
                }
                WordOperand::C3(offset) => (destination, offset),
                _ => return Err(InvalidTensor),
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
            InvalidTensor
        }
    };
    let float = |word: u64| f32::from_bits(word as u32);
    let value = match operation {
        WordOperation::C0(kind) => linear::evaluate_wide(kind, a, b)?,
        WordOperation::C1(kind) => Some(u64::from(
            linear::evaluate_f32_scalar(kind, float(a), float(b))?.to_bits(),
        )),
        WordOperation::C2(source, destination, overflow) => {
            linear::convert_float_storage(source, destination, a).map_err(|_| failure(overflow))?
        }
        WordOperation::C3(source, destination, overflow) => {
            linear::evaluate_float_unary(b, source, destination, a)
                .map_err(|_| failure(overflow))?
        }
        WordOperation::C4(kind, storage, first, second) => {
            linear::transform_float_storage(kind, storage, a, first, second)
                .map_err(|_| BackendError::Overflow)?
        }
        WordOperation::C5(kind) => {
            linear::evaluate_encoded_word(kind, a, b).map_err(|_| InvalidTensor)?
        }
        WordOperation::C6 => return Err(InvalidTensor),
        WordOperation::C7(kind, source, destination) => {
            let input = decode_float(source, a)?;
            let result = match kind {
                13 => linear::natural_log_f32(input),
                14 => linear::sin_cos_f32(input).map(|pair| pair.0),
                15 => linear::sin_cos_f32(input).map(|pair| pair.1),
                16 => linear::exponential_f32(input),
                17 => linear::sigmoid_f32(input),
                18 => linear::silu_f32(input),
                19 => linear::tanh_f32(input),
                20 if source == 0 => Ok(gaussian_table(a)?.unwrap_or_default()),
                20 => linear::gelu_f32(input),
                21 => linear::gelu_quick_f32(input),
                _ => return Err(InvalidTensor),
            }
            .map_err(|_| BackendError::Overflow)?;
            Some(convert(0, destination, u64::from(result.to_bits()))?)
        }
    }
    .ok_or(InvalidTensor)?;
    match target {
        WordTarget::C0(index) => *words.get_mut(index as usize).ok_or(InvalidTensor)? = value,
        // SAFETY: Prism bounds each output word, including eight-byte COUNT_EQUAL results.
        WordTarget::C1(offset) => unsafe {
            scalar_word(destination.add(offset as usize), write_bytes, Some(value))?;
        },
        WordTarget::C2 => return Err(InvalidTensor),
    }
    Ok(())
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
    use vtensor_norm_dispatch::{Adt0 as Selection, Adt1 as Instruction};
    let selection = match *mode {
        CopyMode::C24(epsilon) => Selection::C0(epsilon),
        CopyMode::C26(kind, epsilon, rows, capacity) => {
            Selection::C1(kind, epsilon, rows, capacity)
        }
        _ => return Err(BackendError::InvalidTensor),
    };
    let [x0, x1, x2, x3] = secondary_strides;
    let Instruction::C1(family, operation, a, from, b, right, target, to) =
        vtensor_norm_dispatch::run(
            selection, index, width, n1, n2, n3, s0, s1, s2, s3, x0, x1, x2, x3, t0, t1, t2, t3,
            flag,
        )
        .map_err(|_| BackendError::Overflow)?
    else {
        return Err(BackendError::InvalidTensor);
    };
    pointwise(GraphRequest::C3(
        false, family, operation, 0, 0, a, from, b, right, target, to, bytes, bytes,
    ))
}

fn execute_typed(tensor: &Tensor, plan: vtensor_typed::Adt2) -> Result<(), BackendError> {
    let vtensor_typed::Adt2::C0(count, capacity, ..) = &plan;
    let mut words = workspace(*capacity, 0_u64)?;
    let mut ranker = None;
    // SAFETY: admission retained the primary descriptor and each present secondary.
    let (left, secondary) = unsafe { (&*tensor.src[0], tensor.src[1].as_ref()) };
    let primary = graph_tensor_data(left)?;
    let pointers = [primary, secondary.map_or(Ok(primary), graph_tensor_data)?];
    let output = graph_tensor_data(tensor)?;
    for index in 0..*count {
        let instruction =
            vtensor_typed::run(vtensor_typed::Adt0::C1(index, plan.clone(), words[2]))
                .map_err(|_| BackendError::Overflow)?;
        if let vtensor_typed::Adt3::C5(from, to, width, count, descending) = instruction {
            let rank = match &mut ranker {
                Some(rank) => rank,
                None => ranker.insert(linear::RowRankingF32::new(
                    width as usize,
                    count as usize,
                    descending,
                )?),
            };
            // SAFETY: Prism admits aligned binary32 lanes, disjoint output storage and the full row spans.
            let input = unsafe {
                std::slice::from_raw_parts(primary.add(from as usize).cast::<f32>(), width as usize)
            };
            let indices = rank.select(input)?;
            // SAFETY: the selected prefix fits this certified contiguous GGML I32 row.
            unsafe { write_row(output.add(to as usize), indices) };
            continue;
        }
        if let vtensor_typed::Adt3::C4(variant, storage, first, second, target) = instruction {
            let operand = |slot: u64| {
                words
                    .get(slot as usize)
                    .map(|word| f32::from_bits(*word as u32))
                    .ok_or(BackendError::InvalidTensor)
            };
            let value = gated_value(variant, storage, operand(first)?, operand(second)?)?;
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
            pointwise(GraphRequest::C3(
                true,
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
            ))?,
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
        pointwise(GraphRequest::C2(mode.clone()))?
    else {
        return Err(BackendError::InvalidTensor);
    };
    let mut literal = [0_u8; 8];
    if constant {
        let value = convert(0, literal_type, bits)?;
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
            let source = tensor.source(slot).ok_or(BackendError::InvalidTensor)?;
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
            _ => pointwise(GraphRequest::C1(
                index,
                source.clone(),
                destination.clone(),
                mode.clone(),
                words[2],
            ))?,
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
                linear::decode_storage_block(kind, input, output, stride as usize)?;
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
            verdant_tensor::memory::copy_storage_block(&block, target)?;
        }
    }
    Ok(())
}

unsafe extern "C" fn supports_operation(_: Dev, tensor: *const Tensor) -> bool {
    // SAFETY: GGML supplies a live tensor descriptor for this query.
    plan_operation(unsafe { &*tensor }).is_ok()
}

unsafe extern "C" fn graph_compute(backend: BackendPtr, graph: Graph) -> c_int {
    // SAFETY: the backend and its dispatch settings remain live for this graph call.
    let Ok(execution) = (unsafe { threadpool::execution(backend) }) else {
        return GGML_STATUS_FAILED;
    };
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
                ExecutionPlan::Standard(GraphPlan::C6(geometry)) => {
                    execute_convolution(&*tensor, geometry)
                }
                ExecutionPlan::Softmax(plan) => attention::softmax(&*tensor, *plan),
                ExecutionPlan::Attention(plan) => attention::execute(&*tensor, *plan),
                ExecutionPlan::Rotary(plan) => rotary::execute(&*tensor, plan),
                ExecutionPlan::Typed(plan) => execute_typed(&*tensor, plan),
                ExecutionPlan::RowProduct(plan) => execute_row_products(&execution, &*tensor, plan),
                _ => Err(BackendError::InvalidTensor),
            };
            if result.is_err() {
                return GGML_STATUS_FAILED;
            }
        }
    }
    GGML_STATUS_SUCCESS
}

mod rotary {
    //! GGML rotary graph transport. Prism supplies addresses and Verdant computes coefficients and values.

    use super::{
        convert, decode_float, graph_tensor_data, scalar_word, workspace, BackendError, Tensor,
        TensorDescriptor,
    };
    use verdant_tensor::linear::{
        rotate_pair_f32, signed_word_f32, RotaryParameters, RotarySpectrum,
    };
    use vtensor_rotary::{
        Adt0 as Request, Adt1 as Layout, Adt2 as Descriptor, Adt3 as Plan, Adt4 as Response,
    };

    fn request(query: Request) -> Result<Response, BackendError> {
        vtensor_rotary::run(query).map_err(|_| BackendError::Overflow)
    }

    pub(super) fn plan(
        tensor: &Tensor,
        layout: Layout,
        source: TensorDescriptor,
        positions: TensorDescriptor,
        factors: Option<TensorDescriptor>,
        output: TensorDescriptor,
    ) -> Result<Option<Plan>, BackendError> {
        let descriptor = descriptor_for!(Descriptor::C0);
        match request(Request::C0(
            tensor.op as u64,
            tensor.src[3..].iter().any(|v| !v.is_null()),
            layout,
            descriptor(source),
            descriptor(positions),
            factors.map(descriptor),
            descriptor(output),
        ))? {
            Response::C2(plan) => Ok(Some(plan)),
            Response::C0 => Ok(None),
            _ => Err(BackendError::InvalidTensor),
        }
    }

    fn spectrum(tensor: &Tensor) -> Result<RotarySpectrum, BackendError> {
        let p = &tensor.op_params;
        let value = |i: usize| f32::from_bits(p[i] as u32);
        RotarySpectrum::new(RotaryParameters {
            dimensions: p[1] as u32,
            context: p[4],
            base: value(5),
            frequency_scale: value(6),
            extension: value(7),
            magnitude: value(8),
            rotations: [value(9), value(10)],
            truncate: p[3] == 0,
        })
        .map_err(|_| BackendError::InvalidTensor)
    }

    pub(super) fn execute(tensor: &Tensor, plan: Plan) -> Result<(), BackendError> {
        let Plan::C0(width, pairs, channels, heads, tokens, batches, layout, _, _) = &plan;
        // SAFETY: graph admission requires data and positions and validates the optional factor descriptor.
        let (source, positions, factors) =
            unsafe { (&*tensor.src[0], &*tensor.src[1], tensor.src[2].as_ref()) };
        let (input, output) = (graph_tensor_data(source)?, graph_tensor_data(tensor)?);
        let axis_count = (positions.ne[0] as u64)
            .checked_div(*tokens)
            .ok_or(BackendError::InvalidTensor)? as usize;
        let positions = graph_tensor_data(positions)?;
        let factors = factors.map(graph_tensor_data).transpose()?;
        let spectrum = spectrum(tensor)?;
        let format = source.ty as u64;
        let mut coefficients = workspace(*pairs, (0_f32, 0_f32))?;
        let mut schedule = workspace(*pairs, (0_usize, None, 1_f32))?;
        for (lane, slot) in schedule.iter_mut().enumerate() {
            let Response::C6(axis, reset) = request(Request::C3(layout.clone(), lane as u64))?
            else {
                return Err(BackendError::InvalidTensor);
            };
            let factor = match factors {
                Some(pointer) => {
                    // SAFETY: Prism admitted this contiguous F32 factor word.
                    f32::from_bits(unsafe { scalar_word(pointer.add(lane * 4), 4, None)? } as u32)
                }
                None => 1.0,
            };
            *slot = (
                axis as usize,
                reset.checked_sub(1).map(|axis| axis as usize),
                factor,
            );
        }
        let mut lanes = workspace(channels / 2, (None, 0, 0))?;
        for (index, item) in lanes.iter_mut().enumerate() {
            *item = match request(Request::C2(plan.clone(), index as u64))? {
                Response::C4(slot, a, b) => (Some(slot as usize), a as usize, b as usize),
                Response::C5(a, b) => (None, a as usize, b as usize),
                _ => return Err(BackendError::InvalidTensor),
            };
        }
        for token in 0..*tokens {
            let mut base = [0.0; 4];
            for (axis, value) in base.iter_mut().enumerate().take(axis_count) {
                // SAFETY: Prism bounded the contiguous position vector for every admitted axis.
                let word = unsafe {
                    scalar_word(
                        positions.add((axis * *tokens as usize + token as usize) * 4),
                        4,
                        None,
                    )?
                };
                *value = signed_word_f32(word as i32)?;
            }
            let mut phase = base;
            for (lane, (weight, &(axis, reset, factor))) in
                coefficients.iter_mut().zip(&schedule).enumerate()
            {
                if let Some(reset) = reset {
                    phase[reset] = base[reset];
                }
                *weight = spectrum.weights(phase[axis], factor, lane as u32, tensor.op == 46)?;
                for value in phase.iter_mut().take(axis_count) {
                    *value = spectrum.advance(*value)?;
                }
            }
            for batch in 0..*batches {
                for head in 0..*heads {
                    let Response::C3(src, dst) =
                        request(Request::C1(plan.clone(), head, token, batch))?
                    else {
                        return Err(BackendError::InvalidTensor);
                    };
                    for &(slot, a, b) in &lanes {
                        // SAFETY: the certified plan bounds both lane addresses in these admitted rows.
                        // Both loads precede either store so an identical source/output view is supported.
                        unsafe {
                            let read =
                                |i: usize| scalar_word(input.add(src as usize + i), *width, None);
                            let (x, y) = (read(a)?, read(b)?);
                            let (x, y) = match slot {
                                Some(slot) => {
                                    let pair = (decode_float(format, x)?, decode_float(format, y)?);
                                    let (x, y) = rotate_pair_f32(pair, coefficients[slot])?;
                                    (
                                        convert(0, format, u64::from(x.to_bits()))?,
                                        convert(0, format, u64::from(y.to_bits()))?,
                                    )
                                }
                                None => (x, y),
                            };
                            scalar_word(output.add(dst as usize + a), *width, Some(x))?;
                            scalar_word(output.add(dst as usize + b), *width, Some(y))?;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

mod attention {
    use super::{
        graph_tensor_data, workspace, BackendError, Tensor, TensorDescriptor, TensorInput,
    };
    use verdant_tensor::linear::{
        self, evaluate_f32_scalar, multiply_matrix_rows_f32, WeightedMeanF32,
    };
    use vtensor_attention::{
        Adt0 as Request, Adt1 as Descriptor, Adt2 as Plan, Adt3 as SoftmaxPlan, Adt4 as Response,
    };
    use BackendError::{InvalidTensor, Overflow};

    fn request(query: Request) -> Result<Response, BackendError> {
        vtensor_attention::run(query).map_err(|_| Overflow)
    }

    pub(super) fn softmax_plan(
        tensor: &Tensor,
        source: TensorDescriptor,
        mask: Option<TensorDescriptor>,
        sink: Option<TensorDescriptor>,
        output: TensorDescriptor,
    ) -> Result<Option<super::ExecutionPlan>, BackendError> {
        let descriptor = descriptor_for!(Descriptor::C0);
        match request(Request::C2(
            tensor.op as u64,
            tensor.src[3..].iter().any(|p| !p.is_null()),
            u64::from(tensor.op_params[0] as u32),
            u64::from(tensor.op_params[1] as u32),
            descriptor(source),
            mask.map(descriptor),
            sink.map(descriptor),
            descriptor(output),
        ))? {
            Response::C3(plan) => Ok(Some(super::ExecutionPlan::Softmax(Box::new(plan)))),
            _ => Err(InvalidTensor),
        }
    }

    pub(super) fn softmax(tensor: &Tensor, plan: SoftmaxPlan) -> Result<(), BackendError> {
        let source = graph_tensor_data(tensor.source(0).ok_or(InvalidTensor)?)?;
        let output = graph_tensor_data(tensor)?;
        // GGML ignores the inner stride of a singleton axis. The Prism plan guarantees
        // contiguous scalar rows, so decode using their physical storage widths.
        let scalar_input = |slot| -> Result<Option<TensorInput<'_>>, BackendError> {
            Ok(tensor.input(slot)?.map(|mut input| {
                input.nb[0] = if input.ty == 0 {
                    size_of::<f32>()
                } else {
                    size_of::<u16>()
                };
                input
            }))
        };
        let mask = scalar_input(1)?;
        let sink = scalar_input(2)?;
        let SoftmaxPlan::C0(_, _, _, scale, bias) = &plan;
        let scale = f32::from_bits(*scale as u32);
        let mut row = workspace(tensor.ne[0] as u64, 0.0_f32)?;
        let mut mask_row = workspace(if mask.is_some() { row.len() as u64 } else { 0 }, 0.0_f32)?;
        let scalar = |operation, a, b| evaluate_f32_scalar(operation, a, b).map_err(|_| Overflow);
        for head in 0..tensor.ne[2] as u64 {
            let slope = bias
                .map(|bits| {
                    linear::attention_head_slope_f32(
                        f32::from_bits(bits as u32),
                        tensor.ne[2] as u32,
                        head as u32,
                    )
                })
                .transpose()?
                .unwrap_or(1.0);
            for batch in 0..tensor.ne[3] as u64 {
                for token in 0..tensor.ne[1] as u64 {
                    let Response::C4(from, moff, soff, to) =
                        request(Request::C3(plan.clone(), token, head, batch))?
                    else {
                        return Err(InvalidTensor);
                    };
                    // SAFETY: the admitted dense row fits the live input. Stage it before the in-place write.
                    unsafe {
                        std::ptr::copy_nonoverlapping(
                            source.add(from as usize),
                            row.as_mut_ptr().cast(),
                            std::mem::size_of_val(row.as_slice()),
                        )
                    };
                    for value in &mut row {
                        *value = scalar(2, *value, scale)?;
                    }
                    if let Some(mask) = &mask {
                        mask.decode(moff, &mut mask_row)?;
                        for (value, &weight) in row.iter_mut().zip(&mask_row) {
                            *value = scalar(0, *value, scalar(2, slope, weight)?)?;
                        }
                    }
                    let extra = if let Some(sink) = &sink {
                        let mut score = [0.0];
                        sink.decode(soff, &mut score)?;
                        Some(score[0])
                    } else {
                        None
                    };
                    linear::normalize_exponential_f32(&mut row, extra)?;
                    // SAFETY: Prism bound this row to the dense destination, disjoint from mask and sink.
                    unsafe { super::write_row(output.add(to as usize), &row) }
                }
            }
        }
        Ok(())
    }

    pub(super) fn plan(
        tensor: &Tensor,
        sources: [Option<TensorDescriptor>; 5],
        output: TensorDescriptor,
    ) -> Result<Option<super::ExecutionPlan>, BackendError> {
        let [Some(query), Some(key), Some(value), mask, sink] = sources else {
            return Err(InvalidTensor);
        };
        let word = |index: usize| u64::from(tensor.op_params[index] as u32);
        let descriptor = descriptor_for!(Descriptor::C0);
        match request(Request::C0(
            tensor.op as u64,
            tensor.src[5..].iter().any(|p| !p.is_null()),
            word(1),
            word(2),
            word(3),
            descriptor(query),
            descriptor(key),
            descriptor(value),
            mask.map(descriptor),
            sink.map(descriptor),
            descriptor(output),
        ))? {
            Response::C1(p) => Ok(Some(super::ExecutionPlan::Attention(Box::new(p)))),
            _ => Err(InvalidTensor),
        }
    }

    impl TensorInput<'_> {
        fn half(&self, offset: u64) -> Result<u16, BackendError> {
            Ok(u16::from_ne_bytes(
                self.region(offset, 2)?.try_into().unwrap(),
            ))
        }

        fn blocks(&self, offset: u64, count: usize) -> Result<&[u8], BackendError> {
            let bytes = count.checked_mul(self.nb[0]).ok_or(Overflow)?;
            self.region(offset, bytes)
        }

        pub(super) fn decode(&self, offset: u64, out: &mut [f32]) -> Result<(), BackendError> {
            let width = self.nb[0];
            if matches!(self.ty, 2 | 8) {
                let (blocks, tail) = out.as_chunks_mut::<32>();
                if !tail.is_empty() {
                    return Err(InvalidTensor);
                }
                let source = self.blocks(offset, blocks.len())?;
                for (input, block) in source.chunks_exact(width).zip(blocks) {
                    super::decode_words(self.ty as u64, input, block, 4)?;
                }
                return Ok(());
            }
            for (bytes, value) in self.blocks(offset, out.len())?.chunks_exact(width).zip(out) {
                *value = match (self.ty as u64, bytes) {
                    (0, &[a, b, c, d]) => f32::from_ne_bytes([a, b, c, d]),
                    (43, &[byte]) => super::decode_float(43, byte.into())?,
                    (kind @ (1 | 30), &[a, b]) => {
                        super::decode_float(kind, u16::from_ne_bytes([a, b]).into())?
                    }
                    _ => return Err(InvalidTensor),
                };
            }
            Ok(())
        }
    }

    pub(super) fn execute(tensor: &Tensor, plan: Plan) -> Result<(), BackendError> {
        let [Some(q), Some(k), Some(v), mask, sink] = [
            tensor.input(0)?,
            tensor.input(1)?,
            tensor.input(2)?,
            tensor.input(3)?,
            tensor.input(4)?,
        ] else {
            return Err(InvalidTensor);
        };
        let shape = q.ne;
        let key_rows = k.ne[1] as u64;
        let mut query = workspace(shape[0] as u64, 0.0)?;
        let packed_product = match k.ty {
            2 => Some(linear::dot_symmetric_q4_row::<32> as fn(&[u8], &[_]) -> _),
            8 => Some(linear::dot_symmetric_q8_row::<32> as _),
            _ => None,
        };
        let (mut key, mut packed) = if packed_product.is_some() {
            (Vec::new(), super::reserved(query.len() / 32)?)
        } else {
            (workspace(shape[0] as u64, 0.0)?, Vec::new())
        };
        let mut value = workspace(tensor.ne[0] as u64, 0.0)?;
        let output = graph_tensor_data(tensor)?;
        let Plan::C0(.., head_bias, cap) = &plan;
        let decode = |bits: u64| f32::from_bits(bits as u32);
        let cap = cap.map(decode);
        let head_bias = head_bias.map(decode);
        let scalar = |op, a, b| evaluate_f32_scalar(op, a, b).map_err(|_| Overflow);
        let mut scale = f32::from_bits(tensor.op_params[0] as u32);
        if let Some(cap) = cap {
            scale = scalar(3, scale, cap)?;
        }
        for head in 0..shape[2] as u64 {
            let slope = head_bias
                .map(|bias| linear::attention_head_slope_f32(bias, shape[2] as u32, head as u32))
                .transpose()?;
            let slope_word = u64::from(slope.unwrap_or(1.0).to_bits());
            for batch in 0..shape[3] as u64 {
                for token in 0..shape[1] as u64 {
                    let Response::C2(qoff, koff, voff, moff, soff, dst, ks, vs, dot_type) =
                        request(Request::C1(plan.clone(), token, head, batch))?
                    else {
                        return Err(InvalidTensor);
                    };
                    q.decode(qoff, &mut query)?;
                    // GGML prepares the query in the key's dot-product storage once per head.
                    if dot_type == 8 {
                        packed.clear();
                        for block in query.as_chunks::<32>().0 {
                            packed.push(linear::encode_symmetric_q8_block(block)?);
                        }
                    } else if dot_type != 0 {
                        for word in &mut query {
                            *word = super::prepare_float(0, dot_type, word.to_bits().into())?;
                        }
                    }
                    let mut mean = WeightedMeanF32::new(value.len());
                    for row in 0..key_rows {
                        let bias = match &mask {
                            Some(m) => {
                                let half = m.half(moff + row * m.nb[0] as u64)?;
                                if vtensor_attention_mask::run(u64::from(half), slope_word) {
                                    continue;
                                }
                                let word = super::decode_float(1, half.into())?;
                                match slope {
                                    Some(slope) => scalar(2, slope, word)?,
                                    None => word,
                                }
                            }
                            None => 0.0,
                        };
                        let key_offset = koff + row * ks;
                        let dot = if let Some(product) = packed_product {
                            product(k.blocks(key_offset, packed.len())?, &packed)?
                        } else {
                            k.decode(key_offset, &mut key)?;
                            let mut dot = [0.0];
                            multiply_matrix_rows_f32(1, key.len(), 1, &key, &query, &mut dot)?;
                            dot[0]
                        };
                        v.decode(voff + row * vs, &mut value)?;
                        let mut scaled = scalar(2, dot, scale)?;
                        if let Some(cap) = cap {
                            scaled = scalar(2, cap, linear::tanh_f32(scaled)?)?;
                        }
                        let score = scalar(0, scaled, bias)?;
                        mean.push(score, Some(&value)).map_err(|_| Overflow)?;
                    }
                    if let Some(sink) = &sink {
                        let mut score = [0.0];
                        sink.decode(soff, &mut score)?;
                        mean.push(score[0], None).map_err(|_| Overflow)?;
                    }
                    let row = mean.finish().map_err(|_| Overflow)?;
                    // SAFETY: the Prism plan bounds this contiguous F32 row. Verdant owns its separate workspace.
                    unsafe { super::write_row(output.add(dst as usize), &row) }
                }
            }
        }
        Ok(())
    }
}

mod threadpool {
    use super::{BackendError, BackendPtr, Reg, ThreadpoolParams};
    use std::ffi::{c_char, c_int, c_void, CStr};
    use std::ptr::null_mut;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{mpsc, Arc, Mutex};
    use std::thread::{self, JoinHandle};

    type Job = Box<dyn FnOnce() + Send>;
    type Status = Result<(), BackendError>;

    struct Worker {
        send: Option<mpsc::Sender<Job>>,
        thread: Option<JoinHandle<()>>,
    }

    impl Drop for Worker {
        fn drop(&mut self) {
            self.send.take();
            if let Some(thread) = self.thread.take() {
                let _ = thread.join();
            }
        }
    }

    struct Pool {
        workers: Vec<Worker>,
        paused: Arc<AtomicBool>,
        params: ThreadpoolParams,
    }

    #[cfg(target_os = "linux")]
    fn configure(params: &ThreadpoolParams, placement: usize) {
        unsafe extern "C" {
            fn sched_setaffinity(pid: c_int, bytes: usize, mask: *const usize) -> c_int;
            fn sched_setscheduler(pid: c_int, policy: c_int, priority: *const c_int) -> c_int;
        }
        if params.prio != 0 {
            let level = (i64::from(params.prio) + 1) as u64;
            if let Ok(priority) = vtensor_native_memory::run(vtensor_native_memory::Adt0::C3(level))
            {
                let priority = priority as c_int;
                let policy = if params.prio < 0 { 3 } else { 1 };
                // SAFETY: pid zero selects this thread and Linux sched_param contains one int.
                unsafe { sched_setscheduler(0, policy, &priority) };
            }
        }
        let enabled = || {
            params
                .cpumask
                .iter()
                .enumerate()
                .filter_map(|(i, &on)| on.then_some(i))
        };
        let count = enabled().count();
        if count == 0 {
            return;
        }
        let mut mask = [0usize; 512 / usize::BITS as usize];
        let mut select = |cpu: usize| {
            mask[cpu / usize::BITS as usize] |= 1 << (cpu % usize::BITS as usize);
        };
        if params.strict_cpu {
            if let Some(cpu) = enabled().nth(placement % count) {
                select(cpu);
            }
        } else {
            enabled().for_each(select);
        }
        // SAFETY: the kernel reads exactly the supplied number of mask bytes.
        unsafe { sched_setaffinity(0, std::mem::size_of_val(&mask), mask.as_ptr()) };
    }

    #[cfg(not(target_os = "linux"))]
    fn configure(_: &ThreadpoolParams, _: usize) {}

    impl Pool {
        fn create(params: ThreadpoolParams) -> Result<Arc<Self>, BackendError> {
            let valid = vtensor_native_predicates::Adt0::C3(
                params.n_threads as u64,
                (i64::from(params.prio) + 1) as u64,
                u64::from(params.poll),
            );
            if !vtensor_native_predicates::run(valid) {
                return Err(BackendError::InvalidTensor);
            }
            let mut workers = Vec::new();
            let paused = Arc::new(AtomicBool::new(params.paused));
            for index in 0..params.n_threads as usize - 1 {
                let (send, receive) = mpsc::channel::<Job>();
                let idle = Arc::clone(&paused);
                let settings = params.clone();
                let thread = thread::Builder::new()
                    .name(format!("vTensor-{index}"))
                    .spawn(move || {
                        configure(&settings, index);
                        while let Ok(job) = receive.recv() {
                            job();
                            for _ in 0..settings.poll {
                                if idle.load(Ordering::Relaxed) {
                                    break;
                                }
                                if let Ok(next) = receive.try_recv() {
                                    next();
                                } else {
                                    std::hint::spin_loop();
                                }
                            }
                        }
                    })
                    .map_err(|_| BackendError::Allocation)?;
                workers.push(Worker {
                    send: Some(send),
                    thread: Some(thread),
                });
            }
            if !params.paused {
                configure(&params, workers.len());
            }
            Ok(Arc::new(Self {
                workers,
                paused,
                params,
            }))
        }
    }

    // A returning graph must have exclusive ownership of its outputs again, even
    // when a worker refuses a job or the submitting thread unwinds.
    struct Batch(mpsc::Receiver<Status>);
    impl Drop for Batch {
        fn drop(&mut self) {
            for _ in &self.0 {}
        }
    }

    pub(super) struct Execution {
        pool: Option<Arc<Pool>>,
        threads: usize,
    }

    impl Execution {
        pub(super) fn fill<T: Send, F: Fn(usize, &mut [T]) -> Status + Sync>(
            &self,
            values: &mut [T],
            fill: F,
        ) -> Status {
            let Some(pool) = &self.pool else {
                return fill(0, values);
            };
            if self.threads == 1 || values.len() < 2 {
                return fill(0, values);
            }
            let width = vtensor_native_memory::run(vtensor_native_memory::Adt0::C2(
                self.threads as u64,
                values.len() as u64,
            ))
            .map_err(|_| BackendError::Overflow)? as usize;
            let mut chunks = values.chunks_mut(width);
            let first = chunks.next().ok_or(BackendError::InvalidTensor)?;
            let (send, receive) = mpsc::channel();
            let batch = Batch(receive);
            let function = &fill;
            let submit = move || {
                for (i, (worker, chunk)) in pool.workers.iter().zip(chunks).enumerate() {
                    let done = send.clone();
                    let task: Box<dyn FnOnce() + Send + '_> = Box::new(move || {
                        let status = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                            function((i + 1) * width, chunk)
                        }))
                        .unwrap_or(Err(BackendError::InvalidTensor));
                        let _ = done.send(status);
                    });
                    // SAFETY: each chunk is exclusive. Batch drains until all job senders
                    // are destroyed, including on early return or unwind. Both the slice
                    // and the borrowed function outlive Batch and every invocation.
                    let job =
                        unsafe { std::mem::transmute::<Box<dyn FnOnce() + Send + '_>, Job>(task) };
                    worker
                        .send
                        .as_ref()
                        .ok_or(BackendError::Allocation)?
                        .send(job)
                        .map_err(|_| BackendError::Allocation)?;
                }
                drop(send);
                function(0, first)
            };
            let mut result = submit();
            for status in &batch.0 {
                result = result.and(status);
            }
            result
        }
    }

    #[derive(Default)]
    pub(super) struct Settings {
        attached: Option<Arc<Pool>>,
        private: Option<Arc<Pool>>,
        limit: Option<usize>,
    }
    type Context = Mutex<Settings>;

    pub(super) fn new_context() -> *mut c_void {
        Box::into_raw(Box::<Context>::default()).cast()
    }
    pub(super) unsafe fn free_context(backend: BackendPtr) {
        // SAFETY: backend_new initialized this context, and GGML destroys it once.
        unsafe { drop(Box::from_raw((*backend).context.cast::<Context>())) }
    }
    pub(super) unsafe fn execution(backend: BackendPtr) -> Result<Execution, BackendError> {
        // SAFETY: graph_compute retains its live backend for the entire submission.
        let mut settings = unsafe { &*(*backend).context.cast::<Context>() }
            .lock()
            .map_err(|_| BackendError::InvalidTensor)?;
        let limit = settings.limit;
        if settings.attached.is_none() && limit.unwrap_or(1) > 1 && settings.private.is_none() {
            settings.private = Some(Pool::create(ThreadpoolParams {
                n_threads: limit.unwrap_or(1) as c_int,
                cpumask: [false; 512],
                prio: 0,
                poll: 0,
                strict_cpu: false,
                paused: false,
            })?);
        }
        let pool = settings
            .attached
            .as_ref()
            .or(settings.private.as_ref())
            .cloned();
        let mut threads = 1;
        if let Some(active) = &pool {
            threads = limit.unwrap_or(usize::MAX).min(active.workers.len() + 1);
            if active.paused.swap(false, Ordering::Relaxed) {
                configure(&active.params, active.workers.len());
            }
        }
        Ok(Execution { pool, threads })
    }

    unsafe extern "C" fn create(params: *const ThreadpoolParams) -> *mut c_void {
        // SAFETY: GGML supplies a readable params object for threadpool_new.
        let Some(params) = (unsafe { params.as_ref() }) else {
            return null_mut();
        };
        Pool::create(params.clone()).map_or(null_mut(), |pool| Box::into_raw(Box::new(pool)).cast())
    }
    unsafe extern "C" fn release(handle: *mut c_void) {
        if !handle.is_null() {
            // SAFETY: the caller releases the handle returned by create exactly once.
            unsafe { drop(Box::from_raw(handle.cast::<Arc<Pool>>())) }
        }
    }
    unsafe extern "C" fn attach(backend: BackendPtr, handle: *mut c_void) {
        // SAFETY: the registry supplies its own backend and either a live pool or null.
        let (settings, next) = unsafe {
            (
                &*(*backend).context.cast::<Context>(),
                handle.cast::<Arc<Pool>>().as_ref().cloned(),
            )
        };
        let mut state = settings.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(old) = &state.attached {
            if next.as_ref().is_none_or(|p| !Arc::ptr_eq(p, old)) {
                old.paused.store(true, Ordering::Relaxed);
            }
        }
        state.attached = next;
    }
    unsafe extern "C" fn set_threads(backend: BackendPtr, count: c_int) {
        let request = vtensor_native_predicates::Adt0::C3(count as u64, 1, 0);
        if vtensor_native_predicates::run(request) {
            // SAFETY: GGML passes a live backend created by this registry.
            let mut state = unsafe { &*(*backend).context.cast::<Context>() }
                .lock()
                .unwrap_or_else(|e| e.into_inner());
            if state.limit != Some(count as usize) {
                state.limit = Some(count as usize);
                state.private.take();
            }
        }
    }
    pub(super) unsafe extern "C" fn lookup(_: Reg, name: *const c_char) -> *mut c_void {
        if name.is_null() {
            return null_mut();
        }
        // SAFETY: the imported GGML registry lookup takes a NUL-terminated name.
        match unsafe { CStr::from_ptr(name) }.to_bytes() {
            b"ggml_threadpool_new" => create as *mut c_void,
            b"ggml_threadpool_free" => release as *mut c_void,
            b"ggml_backend_cpu_set_threadpool" => attach as *mut c_void,
            b"ggml_backend_set_n_threads" => set_threads as *mut c_void,
            _ => null_mut(),
        }
    }
}
