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
    Adt12 as WordOperand, Adt13 as WordTarget, Adt2 as TensorDescriptor, Adt4 as CopyMode,
    Adt8 as ConvolutionGeometry, Adt9 as GraphResponse,
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

impl From<std::collections::TryReserveError> for BackendError {
    fn from(_: std::collections::TryReserveError) -> Self {
        Self::Allocation
    }
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
    tensor_layout(tensor).map(|(metadata, _)| metadata)
}

fn tensor_layout(tensor: &Tensor) -> Result<(TensorDescriptor, usize), BackendError> {
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
    Ok((
        TensorDescriptor::C0(ty, n0, n1, n2, n3, s0, s1, s2, s3, size, address),
        block as usize,
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
    result.map_or(GGML_STATUS_FAILED, |_| GGML_STATUS_SUCCESS)
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
    SparseMask(Box<vtensor_attention::Adt4>),
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

    fn source_metadata(&self, slot: usize) -> Result<Option<TensorDescriptor>, BackendError> {
        self.source(slot).map(tensor_metadata).transpose()
    }

    fn inputs<const N: usize>(&self) -> Result<[Option<TensorInput<'_>>; N], BackendError> {
        let mut inputs = [const { None }; N];
        for (slot, input) in inputs.iter_mut().enumerate() {
            *input = self.input(slot)?;
        }
        Ok(inputs)
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
    if matches!(operation, 43 | 71 | 88) {
        return attention::plan(tensor, operation, output);
    }
    let [p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14, p15] =
        tensor.op_params.map(|word| u64::from(word as u32));
    let primary = tensor.source_metadata(0)?;
    let secondary = tensor.source_metadata(1)?;
    let base = tensor.source_metadata(2)?;
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
    let mut extra = false;
    for slot in 3..tensor.src.len() {
        extra |= tensor.source_metadata(slot)?.is_some();
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
            let w = primary.ok_or(InvalidTensor)?;
            // MUL_MAT owns the ordinary row-product route. Other two-input fallbacks use the typed planner.
            if base.is_none() && operation != 28 {
                return typed_plan(vtensor_typed::Adt0::C3(
                    operation,
                    p0,
                    p1,
                    p2,
                    p3,
                    extra,
                    metadata(w),
                    secondary.map(metadata),
                    metadata(output),
                ));
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
    result.try_reserve_exact(count)?;
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
    use BackendError::InvalidTensor;
    let ConvolutionGeometry::C0(
        patches,
        inner,
        channels,
        weight_count,
        tile,
        input_capacity,
        output_capacity,
        overlap_cells,
        ..,
    ) = geometry;
    let [Some(kernel), Some(image)] = tensor.inputs()? else {
        return Err(InvalidTensor);
    };
    let destination = graph_tensor_data(tensor)?;
    let sample = |phase, start, index| {
        let query = GraphRequest::C4(geometry.clone(), phase, start, index as u64);
        match pointwise(query)? {
            GraphResponse::C8(offset) => Ok(offset),
            _ => Err(InvalidTensor),
        }
    };
    let mut weights = workspace(weight_count, 0.0_f32)?;
    for (index, weight) in weights.iter_mut().enumerate() {
        let offset = sample(0, 0, index)?.ok_or(InvalidTensor)?;
        kernel.decode(offset, std::slice::from_mut(weight))?;
    }
    let mut overlap = workspace(overlap_cells, 0.0_f32)?;
    let (inner, channels) = (inner as usize, channels as usize);
    let mut input = workspace(input_capacity, 0.0_f32)?;
    let mut output = workspace(output_capacity, 0.0_f32)?;
    for start in (0..patches).step_by(tile as usize) {
        let rows = tile.min(patches - start) as usize;
        let input = &mut input[..rows * inner];
        for (index, value) in input.iter_mut().enumerate() {
            *value = match sample(1, start, index)? {
                Some(at) => f32::from_ne_bytes(image.word(at)?),
                None => 0.0,
            };
        }
        let output = &mut output[..rows * channels];
        linear::multiply_matrix_rows_f32(rows, inner, channels, input, &weights, output)?;
        for (index, &value) in output.iter().enumerate() {
            let at = sample(2, start, index)?.ok_or(InvalidTensor)?;
            if overlap.is_empty() {
                // SAFETY: direct plans resolve each byte offset inside the retained destination.
                unsafe { write_row(destination.add(at as usize), std::slice::from_ref(&value)) };
            } else {
                // Transposed plans return the contiguous output cell receiving this dot product.
                let cell = overlap.get_mut(at as usize).ok_or(InvalidTensor)?;
                *cell = linear::evaluate_f32_scalar(0, *cell, value)?;
            }
        }
    }
    if !overlap.is_empty() {
        // SAFETY: Prism admitted a disjoint contiguous output spanning the completed rows.
        unsafe { write_row(destination, &overlap) };
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

fn decode_block(
    kind: u64,
    source: &[u8],
    output: &mut [u8],
    stride: usize,
) -> Result<(), BackendError> {
    if kind == 26 {
        let word = source.try_into().map_err(|_| BackendError::InvalidTensor)?;
        let lane = output
            .first_chunk_mut::<4>()
            .ok_or(BackendError::OutOfBounds)?;
        *lane = linear::signed_word_f32(i32::from_ne_bytes(word))?.to_ne_bytes();
        return Ok(());
    }
    Ok(linear::decode_storage_block(kind, source, output, stride)?)
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
    decode_block(kind, source, bytes, stride)
}

struct TensorInput<'a> {
    block_lanes: usize,
    bytes: &'a [u8],
    ty: c_int,
    ne: [i64; 4],
    nb: [usize; 4],
}

impl<'a> TensorInput<'a> {
    fn new(t: &'a Tensor) -> Result<Self, BackendError> {
        let (TensorDescriptor::C0(.., span, _), block_lanes) = tensor_layout(t)?;
        let span = span as usize;
        let bytes = if span == 0 {
            &[]
        } else {
            // SAFETY: GGML retains this checked native region throughout graph execution.
            unsafe { std::slice::from_raw_parts(graph_tensor_region(t, span)?, span) }
        };
        Ok(Self {
            bytes,
            block_lanes,
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

type BlockDot<B> = fn(&[u8], &[B]) -> Result<f32, TensorComputeError>;

#[derive(Clone, Copy)]
enum PackedWeight {
    Symmetric(BlockDot<linear::SymmetricQ8Block<32>>),
    Grouped(linear::GroupedWeightEncoding),
    Affine(BlockDot<linear::AffineQ8Block<32>>),
}

fn packed_weight(kind: u64) -> Option<PackedWeight> {
    use linear::{AffineWeightEncoding::*, BlockFloatEncoding::*, SymmetricWeightEncoding::*};
    use linear::{CodebookEncoding::*, GroupedWeightEncoding as GroupedFormat};
    use PackedWeight::{Affine, Grouped, Symmetric};
    Some(match kind {
        2 => Symmetric(linear::dot_symmetric_q4_row),
        3 => Affine(|w, a| linear::dot_affine_weight_row(w, a, FourBit)),
        6 => Symmetric(|w, a| linear::dot_encoded_symmetric_row(w, a, SignedFive)),
        7 => Affine(|w, a| linear::dot_affine_weight_row(w, a, FiveBit)),
        8 => Symmetric(linear::dot_symmetric_q8_row),
        10 => Grouped(GroupedFormat::AffineTwo),
        11 => Grouped(GroupedFormat::SignedThree),
        12 => Grouped(GroupedFormat::AffineFour),
        13 => Grouped(GroupedFormat::AffineFive),
        14 => Grouped(GroupedFormat::SignedSix),
        16 => Grouped(GroupedFormat::Codebook(IndexedSigns)),
        17 => Grouped(GroupedFormat::Codebook(ExtendedIndex)),
        18 => Grouped(GroupedFormat::Codebook(FourLaneIndexedSigns)),
        21 => Grouped(GroupedFormat::Codebook(FourLaneExplicitSigns)),
        22 => Grouped(GroupedFormat::Codebook(ExplicitSigns)),
        20 => Symmetric(|w, a| linear::dot_encoded_symmetric_row(w, a, NonlinearFour)),
        23 => Grouped(GroupedFormat::NonlinearFour),
        34 => Grouped(GroupedFormat::PackedTrits),
        35 => Grouped(GroupedFormat::TwoBitTernary),
        39 => Symmetric(|w, a| linear::dot_block_float_row(w, a, ExponentOnly)),
        40 => Symmetric(|w, a| linear::dot_block_float_row(w, a, GroupedE4M3)),
        41 => Symmetric(linear::dot_binary_sign_row),
        42 => Symmetric(linear::dot_two_bit_row),
        _ => return None,
    })
}

fn execute_row_products(
    execution: &threadpool::Execution,
    tensor: &Tensor,
    plan: RowProductPlan,
) -> Result<(), BackendError> {
    use RowProductPlan::C0;
    let C0(_, weight_bytes, ..) = plan;
    macro_rules! packed {
        ($encode:expr, $dot:expr) => {
            execute_block_rows(
                execution,
                0,
                tensor,
                plan,
                |x| Ok($encode(x)?),
                |source, at, a, _| Ok(($dot)(source.region(at, weight_bytes as usize)?, a)?),
            )
        };
    }
    match plan {
        C0(inner, .., Some(precision), _) => execute_block_rows(
            execution,
            inner,
            tensor,
            plan,
            |&[x]| prepare_float(0, precision, x.to_bits().into()),
            |source, at, activations, weights| source.dot(at, activations, weights),
        ),
        C0(.., None, kind) => match packed_weight(kind).ok_or(BackendError::InvalidTensor)? {
            PackedWeight::Grouped(kind) => packed!(linear::encode_grouped_q8_block, |w, a| {
                linear::dot_grouped_weight_row(w, a, kind)
            }),
            PackedWeight::Symmetric(dot) => packed!(linear::encode_symmetric_q8_block, dot),
            PackedWeight::Affine(dot) => packed!(linear::encode_affine_q8_block, dot),
        },
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
    use BackendError::{InvalidTensor, Overflow};
    let Plan::C0(blocks, _, rows, _, _, visits, outputs, ..) = plan;
    let [Some(weights), Some(activations), indices] = tensor.inputs()? else {
        return Err(InvalidTensor);
    };
    let request = |query| run(query).map_err(|_| Overflow);
    let destination = graph_tensor_data(tensor)?;
    let mut results = workspace(outputs, 0.0f32)?;
    let mut offsets = workspace(visits, 0u64)?;
    let stride = LANES.checked_mul(activations.nb[0]).ok_or(Overflow)? as u64;
    let mut lanes = [0.0; LANES];
    let mut prepared = reserved(usize::try_from(blocks).map_err(|_| Overflow)?)?;
    for (column, (values, offset)) in results
        .chunks_exact_mut(rows as usize)
        .zip(&mut offsets)
        .enumerate()
    {
        let column = column as u64;
        let mut address = request(Request::C1(plan.clone(), column))?;
        if let Response::C4(at) = address {
            let source = indices.as_ref().ok_or(InvalidTensor)?;
            let expert = u32::from_ne_bytes(source.word(at)?);
            address = request(Request::C4(plan.clone(), column, expert.into()))?;
        }
        let (weight_base, activation, dest) = match address {
            Response::C2(w, a, o) => (w, a, o),
            Response::C5(dest) => {
                *offset = dest;
                continue;
            }
            _ => return Err(InvalidTensor),
        };
        *offset = dest;
        prepared.clear();
        let mut from = activation;
        for _ in 0..blocks {
            activations.decode(from, &mut lanes)?;
            prepared.push(encode(&lanes)?);
            from = from.checked_add(stride).ok_or(Overflow)?;
        }
        execution.fill(values, |first, chunk| {
            let mut scratch = workspace(scratch_width, 0.0_f32)?;
            for (row, value) in (first..).zip(chunk) {
                let Response::C3(start) =
                    request(Request::C2(plan.clone(), weight_base, row as u64))?
                else {
                    return Err(InvalidTensor);
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
    match (from, to) {
        (43, 0) => linear::evaluate_small_float(3, word).map_err(|_| BackendError::Overflow)?,
        _ => linear::convert_float_storage(from, to, word).map_err(|_| BackendError::Overflow)?,
    }
    .ok_or(BackendError::InvalidTensor)
}

fn decode_float(storage: u64, word: u64) -> Result<f32, BackendError> {
    convert(storage, 0, word).map(|bits| f32::from_bits(bits as u32))
}

fn prepare_float(source: u64, dot_type: u64, word: u64) -> Result<f32, BackendError> {
    match (source, dot_type) {
        (0, 0) => u32::try_from(word)
            .map(f32::from_bits)
            .map_err(|_| BackendError::InvalidTensor),
        _ if source == dot_type || dot_type == 0 => decode_float(source, word),
        _ => decode_float(dot_type, convert(source, dot_type, word)?),
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
    let read = |operand| {
        let (pointer, offset) = match operand {
            WordOperand::C0(index, offset) => {
                (*pointers.get(index as usize).ok_or(InvalidTensor)?, offset)
            }
            WordOperand::C1(index) => {
                return words.get(index as usize).copied().ok_or(InvalidTensor)
            }
            WordOperand::C2(value) => return Ok(value),
            WordOperand::C3(offset) => (destination, offset),
            WordOperand::C4 => return Err(InvalidTensor),
        };
        // SAFETY: Prism bounds the selected source or output word and its byte width.
        unsafe { scalar_word(pointer.add(offset as usize), read_bytes, None) }
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
        WordOperation::C8(kind) => {
            linear::evaluate_small_float(kind, a).map_err(|_| InvalidTensor)?
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
                22 => linear::softplus_f32(input),
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

fn execute_typed(tensor: &Tensor, plan: vtensor_typed::Adt2) -> Result<(), BackendError> {
    use vtensor_typed::Adt3::{C3 as Word, C4 as Gate, C5 as Rank, C6 as Encode};
    use BackendError::InvalidTensor;
    let vtensor_typed::Adt2::C0(count, capacity, ..) = &plan;
    let mut words = workspace(*capacity, 0_u64)?;
    let mut ranker = None;
    // SAFETY: admission retained the primary descriptor and each present secondary.
    let (left, secondary) = unsafe { (&*tensor.src[0], tensor.src[1].as_ref()) };
    use linear::PackedWeightEncoding::*;
    let kind = match tensor.ty {
        2 => Some(CenteredFour),
        3 => Some(AffineFour),
        6 => Some(CenteredFive),
        7 => Some(AffineFive),
        8 => Some(SymmetricEight),
        34 => Some(PackedTrits),
        35 => Some(TwoBitTernary),
        41 => Some(BinarySign),
        42 => Some(CenteredTwo),
        _ => None,
    };
    // Word instructions can update their input in place. Only staged block encoding
    // retains an immutable input slice throughout the instruction loop.
    let source = kind.map(|_| TensorInput::new(left)).transpose()?;
    let primary = match &source {
        Some(input) => input.bytes.as_ptr().cast_mut(),
        None => graph_tensor_data(left)?,
    };
    let pointers = [primary, secondary.map_or(Ok(primary), graph_tensor_data)?];
    let mut offsets = Vec::new();
    let mut encoded = Vec::new();
    let (TensorDescriptor::C0(.., span, _), lanes) = tensor_layout(tensor)?;
    let output = graph_tensor_region(tensor, span as usize)?;
    let mut values = workspace(if kind.is_some() { lanes as u64 } else { 0 }, 0.0)?;
    for index in 0..*count {
        let instruction =
            vtensor_typed::run(vtensor_typed::Adt0::C1(index, plan.clone(), words[2]))
                .map_err(|_| BackendError::Overflow)?;
        match instruction {
            Encode(from, to) => {
                let (input, kind) = source.as_ref().zip(kind).ok_or(InvalidTensor)?;
                input.decode(from, &mut values)?;
                let start = encoded.len();
                encoded.try_reserve(tensor.nb[0])?;
                encoded.resize(start + tensor.nb[0], 0);
                linear::encode_weight_block(values.as_slice(), kind, &mut encoded[start..])?;
                offsets.try_reserve(1)?;
                offsets.push(to);
            }
            Rank(from, to, width, count, descending) => {
                let width = width as usize;
                let rank = match &mut ranker {
                    Some(rank) => rank,
                    None => ranker.insert(linear::RowRankingF32::new(
                        width,
                        count as usize,
                        descending,
                    )?),
                };
                // SAFETY: Prism admits aligned binary32 lanes, disjoint output storage and the full row spans.
                let input =
                    unsafe { std::slice::from_raw_parts(primary.add(from as usize).cast(), width) };
                let indices = rank.select(input)?;
                // SAFETY: the selected prefix fits this certified contiguous GGML I32 row.
                unsafe { write_row(output.add(to as usize), indices) };
            }
            Gate(variant, storage, first, second, target) => {
                let operand = |slot: u64| {
                    words
                        .get(slot as usize)
                        .map(|word| f32::from_bits(*word as u32))
                        .ok_or(InvalidTensor)
                };
                let value = gated_value(variant, storage, operand(first)?, operand(second)?)?;
                *words.get_mut(target as usize).ok_or(InvalidTensor)? = u64::from(value.to_bits());
            }
            // The typed planner and the pointwise interpreter export separate Rust enums
            // for the same word fields. Bind the ordering once across this package boundary.
            Word(family, op, src, dst, left, from, right, at, target, to, read, write) => {
                let request = GraphRequest::C3(
                    true, family, op, src, dst, left, from, right, at, target, to, read, write,
                );
                execute_word(pointwise(request)?, &pointers, output, &mut words)?;
            }
            _ => return Err(InvalidTensor),
        }
    }
    for (offset, block) in offsets.into_iter().zip(encoded.chunks(tensor.nb[0].max(1))) {
        // SAFETY: Prism bounded each complete block in the destination. All reads and
        // numerical conversions have finished, including any overlapping source views.
        unsafe { write_row(output.add(offset as usize), block) };
    }
    Ok(())
}

fn execute_copy(tensor: &Tensor, plan: GraphPlan) -> Result<(), BackendError> {
    use BackendError::InvalidTensor;
    let GraphPlan::C4(count, bytes, source, destination, mode) = plan else {
        return Err(InvalidTensor);
    };
    let GraphResponse::C6(mask, literal_type, bits, constant) =
        pointwise(GraphRequest::C2(mode.clone()))?
    else {
        return Err(InvalidTensor);
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
        return Err(InvalidTensor);
    }
    let mut secondary_strides = [0; 4];
    for (slot, pointer) in pointers.iter_mut().enumerate() {
        if mask & (1 << slot) != 0 {
            let source = tensor.source(slot).ok_or(InvalidTensor)?;
            *pointer = graph_tensor_data(source)?;
            if slot == 1 && matches!(mode, CopyMode::C26(66, ..)) {
                secondary_strides = source.nb.map(|stride| stride as u64);
            }
        }
    }
    // SAFETY: both decoder instructions carry a complete source block bounded by Prism.
    let encoded = |from: u64, bytes: u64| unsafe {
        std::slice::from_raw_parts(pointers[0].add(from as usize), bytes as usize)
    };
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
        let [x0, x1, x2, x3] = secondary_strides;
        let response = pointwise(GraphRequest::C6(
            index,
            source.clone(),
            destination.clone(),
            mode.clone(),
            words[2],
            bytes,
            x0,
            x1,
            x2,
            x3,
        ))?;
        match response {
            GraphResponse::C7(..) => execute_word(response, &pointers, destination_data, words)?,
            // SAFETY: Prism preflights every index and bounds the complete block and output lanes.
            GraphResponse::C10(kind, from, width, to, lanes, stride) => unsafe {
                let mut decoded = [0.0; 256];
                let row = decoded.get_mut(..lanes as usize).ok_or(InvalidTensor)?;
                decode_words(kind, encoded(from, width), row, 4)?;
                let base = destination_data.add(to as usize);
                for (i, &value) in row.iter().enumerate() {
                    let lane = base.add(i * stride as usize).cast::<f32>();
                    let sum = linear::evaluate_f32_scalar(0, lane.read_unaligned(), value)?;
                    lane.write_unaligned(sum);
                }
            },
            // SAFETY: the certified source and destination spans are complete and disjoint.
            GraphResponse::C5(kind, from, bytes, to, span, stride) => unsafe {
                let output = std::slice::from_raw_parts_mut(
                    destination_data.add(to as usize),
                    span as usize,
                );
                decode_block(kind, encoded(from, bytes), output, stride as usize)?;
            },
            // SAFETY: capture the admitted source bytes before borrowing an aliased output.
            GraphResponse::C1(second, from, to) => unsafe {
                let pointer = pointers[usize::from(second)].add(from as usize);
                ptr::copy_nonoverlapping(pointer, block.as_mut_ptr(), block.len());
                let target =
                    std::slice::from_raw_parts_mut(destination_data.add(to as usize), block.len());
                verdant_tensor::memory::copy_storage_block(&block, target)?;
            },
            _ => return Err(InvalidTensor),
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
                ExecutionPlan::SparseMask(plan) => attention::sparse_mask(&*tensor, *plan),
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
        Adt0 as Request, Adt1 as Descriptor, Adt2 as Plan, Adt3 as SoftmaxPlan, Adt4 as SparsePlan,
        Adt5 as Response,
    };
    use BackendError::{InvalidTensor, Overflow};

    fn request(query: Request) -> Result<Response, BackendError> {
        vtensor_attention::run(query).map_err(|_| Overflow)
    }

    pub(super) fn sparse_mask(tensor: &Tensor, plan: SparsePlan) -> Result<(), BackendError> {
        use verdant_tensor::memory::copy_storage_block;
        let SparsePlan::C0(_, _, _, _, _, _, width, excluded, _) = plan;
        let [Some(mask), Some(top), score, blocks, extra] = tensor.inputs()? else {
            return Err(InvalidTensor);
        };
        let output = graph_tensor_data(tensor)?;
        let row_bytes = (tensor.ne[0] as u64).checked_mul(width).ok_or(Overflow)?;
        if row_bytes == 0 {
            return Ok(());
        }
        let mut template = workspace(row_bytes, 0_u8)?;
        let mut word = [0; size_of::<u32>()];
        // SAFETY: the certified mask plan selects a two- or four-byte native word.
        unsafe {
            super::scalar_word(word.as_mut_ptr(), width, Some(excluded))?;
        }
        for cell in template.chunks_exact_mut(width as usize) {
            cell.copy_from_slice(&word[..width as usize]);
        }
        let offset =
            |role, index, lane| match request(Request::C6(plan.clone(), role, index, lane))? {
                Response::C7(at) => Ok(Some(at)),
                Response::C8 => Ok(None),
                _ => Err(InvalidTensor),
            };
        let read = |input: &TensorInput, base: u64, at: u64| -> Result<u64, BackendError> {
            Ok(u32::from_ne_bytes(input.word(base.checked_add(at).ok_or(Overflow)?)?) as u64)
        };
        for batch in 0..tensor.ne[3] as u64 {
            for head in 0..tensor.ne[2] as u64 {
                for token in 0..tensor.ne[1] as u64 {
                    let Response::C6(mask_row, top_row, score_row, extra_row, out) =
                        request(Request::C5(plan.clone(), token, head, batch))?
                    else {
                        return Err(InvalidTensor);
                    };
                    // SAFETY: Prism admits the entire dense row and separates it from each input.
                    let target = unsafe {
                        std::slice::from_raw_parts_mut(output.add(out as usize), template.len())
                    };
                    copy_storage_block(&template, target)?;
                    let mut keep = |index| -> Result<(), BackendError> {
                        if let Some(at) = offset(4, index, 0)? {
                            let src = mask.region(
                                mask_row.checked_add(at).ok_or(Overflow)?,
                                width as usize,
                            )?;
                            let start = at as usize;
                            copy_storage_block(
                                src,
                                target
                                    .get_mut(start..start + width as usize)
                                    .ok_or(InvalidTensor)?,
                            )?;
                        }
                        Ok(())
                    };
                    for j in 0..top.ne[0] as u64 {
                        let pick = read(&top, top_row, offset(0, j, 0)?.ok_or(InvalidTensor)?)?;
                        let Some(score_at) = offset(2, pick, 0)? else {
                            continue;
                        };
                        if let Some(scores) = &score {
                            let value = read(scores, score_row, score_at)?;
                            if linear::evaluate_encoded_word(
                                0,
                                value,
                                u64::from(f32::NEG_INFINITY.to_bits()),
                            )
                            .map_err(|_| Overflow)?
                                != Some(2)
                            {
                                continue;
                            }
                        }
                        if let Some(cells) = &blocks {
                            for lane in 0..cells.ne[0] as u64 {
                                keep(read(
                                    cells,
                                    0,
                                    offset(3, pick, lane)?.ok_or(InvalidTensor)?,
                                )?)?;
                            }
                        } else {
                            keep(pick)?;
                        }
                    }
                    if let Some(cells) = &extra {
                        for j in 0..cells.ne[0] as u64 {
                            keep(read(
                                cells,
                                extra_row,
                                offset(1, j, 0)?.ok_or(InvalidTensor)?,
                            )?)?;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn head_slope(
        bias: Option<f32>,
        heads: i64,
        head: u64,
    ) -> Result<Option<f32>, super::TensorComputeError> {
        bias.map(|value| linear::attention_head_slope_f32(value, heads as u32, head as u32))
            .transpose()
    }

    pub(super) fn softmax(tensor: &Tensor, plan: SoftmaxPlan) -> Result<(), BackendError> {
        let source = graph_tensor_data(tensor.source(0).ok_or(InvalidTensor)?)?;
        let output = graph_tensor_data(tensor)?;
        let mask = tensor.input(1)?;
        let sink = tensor.input(2)?;
        let SoftmaxPlan::C0(_, _, _, scale, bias) = &plan;
        let scale = f32::from_bits(*scale as u32);
        let bias = bias.map(|bits| f32::from_bits(bits as u32));
        let mut row = workspace(tensor.ne[0] as u64, 0.0_f32)?;
        let mut mask_row = workspace(if mask.is_some() { row.len() as u64 } else { 0 }, 0.0_f32)?;
        let scalar = |operation, a, b| evaluate_f32_scalar(operation, a, b).map_err(|_| Overflow);
        for head in 0..tensor.ne[2] as u64 {
            let slope = head_slope(bias, tensor.ne[2], head)?.unwrap_or(1.0);
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
                    let extra = sink
                        .as_ref()
                        .map(|s| s.word(soff).map(f32::from_ne_bytes))
                        .transpose()?;
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
        op: u64,
        output: TensorDescriptor,
    ) -> Result<Option<super::ExecutionPlan>, BackendError> {
        use super::ExecutionPlan;
        let descriptor = descriptor_for!(Descriptor::C0);
        let [a, b, c, d, e, rows] =
            [0, 1, 2, 3, 4, 5].map(|slot| tensor.source_metadata(slot).map(|v| v.map(descriptor)));
        let [p0, p1, p2, p3, ..] = tensor.op_params.map(|w| u64::from(w as u32));
        let trailing = tensor.src[6..].iter().any(|p| !p.is_null());
        let query = Request::C8(
            op,
            trailing,
            p0,
            p1,
            p2,
            p3,
            a?,
            b?,
            c?,
            d?,
            e?,
            rows?,
            descriptor(output),
        );
        Ok(Some(match request(query)? {
            Response::C1(p) => ExecutionPlan::Attention(Box::new(p)),
            Response::C3(p) => ExecutionPlan::Softmax(Box::new(p)),
            Response::C5(p) => ExecutionPlan::SparseMask(Box::new(p)),
            _ => return Err(InvalidTensor),
        }))
    }

    impl TensorInput<'_> {
        pub(super) fn dot(
            &self,
            offset: u64,
            activations: &[f32],
            weights: &mut [f32],
        ) -> Result<f32, BackendError> {
            self.decode(offset, weights)?;
            let mut value = [0.0];
            multiply_matrix_rows_f32(1, weights.len(), 1, weights, activations, &mut value)?;
            Ok(value[0])
        }

        pub(super) fn word<const N: usize>(&self, offset: u64) -> Result<[u8; N], BackendError> {
            Ok(self.region(offset, N)?.try_into().unwrap())
        }

        fn blocks(&self, offset: u64, lanes: usize) -> Result<&[u8], BackendError> {
            if !lanes.is_multiple_of(self.block_lanes) {
                return Err(InvalidTensor);
            }
            let bytes = (lanes / self.block_lanes)
                .checked_mul(self.nb[0])
                .ok_or(Overflow)?;
            self.region(offset, bytes)
        }

        pub(super) fn decode(&self, offset: u64, out: &mut [f32]) -> Result<(), BackendError> {
            if self.ty == 0 && (self.nb[0] == size_of::<f32>() || out.len() == 1) {
                let bytes = self.region(offset, size_of_val(out))?;
                // SAFETY: out exclusively owns the full destination and every binary32 word is valid.
                unsafe { super::write_row(out.as_mut_ptr().cast(), bytes) };
                return Ok(());
            }
            if self.block_lanes > 1 {
                let source = self.blocks(offset, out.len())?;
                for (input, block) in source
                    .chunks_exact(self.nb[0])
                    .zip(out.chunks_exact_mut(self.block_lanes))
                {
                    super::decode_words(self.ty as u64, input, block, 4)?;
                }
                return Ok(());
            }
            // A scalar read has no inter-element stride. The Prism offset already selects its lane.
            let width = match (out.len(), self.ty) {
                (1, 1 | 30) => size_of::<u16>(),
                (1, 43) => 1,
                _ => self.nb[0],
            };
            let bytes = out.len().checked_mul(width).ok_or(Overflow)?;
            for (bytes, value) in self.region(offset, bytes)?.chunks_exact(width).zip(out) {
                *value = match (self.ty as u64, bytes) {
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

    use super::BlockDot;
    use linear::{AffineQ8Block, SymmetricQ8Block};

    enum KeyRows {
        Scalar(Vec<f32>),
        Symmetric(BlockDot<SymmetricQ8Block<32>>, Vec<SymmetricQ8Block<32>>),
        Affine(BlockDot<AffineQ8Block<32>>, Vec<AffineQ8Block<32>>),
    }

    fn prepare_blocks<B>(
        query: &[f32],
        target: &mut Vec<B>,
        encode: impl Fn(&[f32; 32]) -> Result<B, verdant_tensor::TensorComputeError>,
    ) -> Result<(), BackendError> {
        let (blocks, tail) = query.as_chunks::<32>();
        if !tail.is_empty() {
            return Err(InvalidTensor);
        }
        target.clear();
        for block in blocks {
            target.push(encode(block)?);
        }
        Ok(())
    }

    pub(super) fn execute(tensor: &Tensor, plan: Plan) -> Result<(), BackendError> {
        let [Some(q), Some(k), Some(v), mask, sink, indices] = tensor.inputs()? else {
            return Err(InvalidTensor);
        };
        let shape = q.ne;
        let Plan::C0(.., columns, head_bias, cap) = &plan;
        let columns = usize::try_from(*columns).map_err(|_| Overflow)?;
        // Validate all cache indices before writing the first output row.
        let cells = indices
            .map(|indices| {
                let (words, tail) = indices.bytes.as_chunks::<4>();
                if !tail.is_empty() || Some(words.len()) != columns.checked_mul(shape[3] as usize) {
                    return Err(InvalidTensor);
                }
                let mut cells = super::reserved(words.len())?;
                for &word in words {
                    let raw = u32::from_ne_bytes(word);
                    cells.push(match request(Request::C9(raw.into(), k.ne[1] as u64))? {
                        Response::C9(cell) => Some(cell),
                        Response::C8 => None,
                        _ => return Err(InvalidTensor),
                    });
                }
                Ok(cells)
            })
            .transpose()?;
        let mut query = workspace(shape[0] as u64, 0.0)?;
        use super::PackedWeight::{Affine, Symmetric};
        let blocks = query.len() / 32;
        let mut keys = match super::packed_weight(k.ty as u64) {
            Some(Symmetric(dot)) => KeyRows::Symmetric(dot, super::reserved(blocks)?),
            Some(Affine(dot)) => KeyRows::Affine(dot, super::reserved(blocks)?),
            _ => KeyRows::Scalar(workspace(query.len() as u64, 0.0)?),
        };
        let mut value = workspace(tensor.ne[0] as u64, 0.0)?;
        let output = graph_tensor_data(tensor)?;
        let [cap, head_bias] = [cap, head_bias].map(|word| word.map(|b| f32::from_bits(b as u32)));
        let scalar = |op, a, b| evaluate_f32_scalar(op, a, b).map_err(|_| Overflow);
        let mut scale = f32::from_bits(tensor.op_params[0] as u32);
        if let Some(cap) = cap {
            scale = scalar(3, scale, cap)?;
        }
        for head in 0..shape[2] as u64 {
            let slope = head_slope(head_bias, shape[2], head)?;
            let slope_word = u64::from(slope.unwrap_or(1.0).to_bits());
            for batch in 0..shape[3] as u64 {
                let cells = cells
                    .as_ref()
                    .map(|v| &v[batch as usize * columns..][..columns]);
                for token in 0..shape[1] as u64 {
                    let Response::C2(qoff, koff, voff, moff, soff, dst, ks, vs, dot_type) =
                        request(Request::C1(plan.clone(), token, head, batch))?
                    else {
                        return Err(InvalidTensor);
                    };
                    q.decode(qoff, &mut query)?;
                    // GGML prepares the query in the key's dot-product storage once per head.
                    match (&mut keys, dot_type) {
                        (KeyRows::Symmetric(_, blocks), 8) => {
                            prepare_blocks(&query, blocks, linear::encode_symmetric_q8_block)?
                        }
                        (KeyRows::Affine(_, blocks), 9) => {
                            prepare_blocks(&query, blocks, linear::encode_affine_q8_block)?
                        }
                        (KeyRows::Scalar(_), 0) => {}
                        (KeyRows::Scalar(_), 30) => {
                            for word in &mut query {
                                *word = super::prepare_float(0, dot_type, word.to_bits().into())?;
                            }
                        }
                        _ => return Err(InvalidTensor),
                    }
                    let mut mean = WeightedMeanF32::new(value.len());
                    for column in 0..columns {
                        let Some(row) = cells.map_or(Some(column as u64), |v| v[column]) else {
                            continue;
                        };
                        let bias = match &mask {
                            Some(m) => {
                                let half = u16::from_ne_bytes(
                                    m.word(moff + column as u64 * m.nb[0] as u64)?,
                                );
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
                        let dot = match &mut keys {
                            KeyRows::Symmetric(dot, blocks) => {
                                dot(k.blocks(key_offset, query.len())?, blocks)?
                            }
                            KeyRows::Affine(dot, blocks) => {
                                dot(k.blocks(key_offset, query.len())?, blocks)?
                            }
                            KeyRows::Scalar(key) => k.dot(key_offset, &query, key)?,
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
                        mean.push(f32::from_ne_bytes(sink.word(soff)?), None)
                            .map_err(|_| Overflow)?;
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
