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

impl BackendError {
    fn invalid<T>(_: T) -> Self {
        Self::InvalidTensor
    }
    fn overflow<T>(_: T) -> Self {
        Self::Overflow
    }
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

// Generated Verdant operations have distinct overflow types and share the GGML refusal.
macro_rules! checked_word {
    ($value:expr) => {
        $value.ok().flatten().ok_or(BackendError::InvalidTensor)
    };
}

// A plan interpreter expects a specific Prism reply or a specific set of source slots.
// Every mismatched constructor has the same public invalid-tensor result.
macro_rules! tensor_fields {
    ($fields:pat = $value:expr) => {
        let $fields = $value else {
            return Err(BackendError::InvalidTensor);
        };
    };
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
                iface: registry_interface!(),
                context: null_mut(),
            }),
            dev: UnsafeCell::new(Device {
                iface: device_interface!(),
                reg: null_mut(),
                context: null_mut(),
            }),
            buft: UnsafeCell::new(BufferType {
                iface: buffer_type_interface!(),
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
            .map_err(BackendError::overflow)?;
        let mut owned = workspace(physical, state().initial_byte)?;
        let base = if size == 0 {
            null_mut()
        } else {
            let offset = vtensor_native_memory::run(vtensor_native_memory::Adt0::C1(owned.as_ptr()
                as usize
                as u64))
            .map_err(BackendError::overflow)? as usize;
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
}

fn tensor_metadata(tensor: &Tensor) -> Result<TensorDescriptor, BackendError> {
    tensor_layout(tensor).map(|(metadata, ..)| metadata)
}

fn tensor_layout(tensor: &Tensor) -> Result<(TensorDescriptor, usize, usize), BackendError> {
    let ty = u64::try_from(tensor.ty).map_err(BackendError::invalid)?;
    let reply = vtensor_types::run(ty);
    tensor_fields!(vtensor_types::Adt0::C1(vtensor_types::Adt1::C0(_, block, element)) = reply);
    // Zero extents are empty. Negative ABI extents cannot inhabit the Prism
    // model's natural-number domain and must fail before buffer admission.
    tensor_fields!([Ok(n0), Ok(n1), Ok(n2), Ok(n3)] = tensor.ne.map(u64::try_from));
    let [s0, s1, s2, s3] = tensor.nb.map(|n| n as u64);
    let layout = vtensor_layout::Adt0::C0(n0, n1, n2, n3, s0, s1, s2, s3, block, element);
    let reply = vtensor_layout::run(layout).map_err(BackendError::overflow)?;
    tensor_fields!(vtensor_layout::Adt1::C1(size) = reply);
    usize::try_from(size).map_err(BackendError::overflow)?;
    let address = tensor.data as usize as u64;
    Ok((
        TensorDescriptor::C0(ty, n0, n1, n2, n3, s0, s1, s2, s3, size, address),
        block as usize,
        element as usize,
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
}

unsafe extern "C" fn supports_buffer_type(_: Dev, buft: Buft) -> bool {
    // SAFETY: GGML retains the type during the callback. Its host predicate guarantees standard layout.
    !buft.is_null()
        && vtensor_native_predicates::run(vtensor_native_predicates::Adt0::C6(
            buft == state().buft.get(),
            unsafe { ggml_backend_buft_is_host(buft) },
        ))
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
            buffer_interface!(),
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
    // SAFETY: GGML retains the descriptor and the supplied backing buffer during initialization.
    let tensor = unsafe { &*tensor };
    tensor_span(tensor)
        .and_then(|size| host_region(buffer, tensor, size, 0, size))
        .map_or(GGML_STATUS_FAILED, |_| GGML_STATUS_SUCCESS)
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
                let tensor = &*tensor;
                let region = require(host_region(buffer, tensor, require(tensor_span(tensor)), offset, size));
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
        let (source, destination) = (&*source, &*destination);
        let (Ok(size), Ok(target_size)) = (tensor_span(source), tensor_span(destination)) else {
            return false;
        };
        if size != target_size {
            return false;
        }
        let origin = source.view_src.as_ref().unwrap_or(source).buffer;
        let (Ok(from), Ok(to)) = (
            host_region(origin, source, size, 0, size),
            host_region(buffer, destination, size, 0, size),
        ) else {
            return false;
        };
        if size != 0 {
            ptr::copy(from, to, size);
        }
        true
    }
}

unsafe extern "C" fn backend_new(device: Dev, _: *const c_char) -> BackendPtr {
    Box::into_raw(Box::new(Backend {
        guid: state().guid.get(),
        iface: backend_interface!(),
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
    Callback(CallbackParams),
    Delay(std::time::Duration),
    Standard(GraphPlan),
    Typed(vtensor_typed::Adt2),
    RowProduct(RowProductPlan),
    Rotary(vtensor_rotary::Adt3),
    Attention(Box<vtensor_attention::Adt2>),
    Softmax(Box<vtensor_attention::Adt3>),
    SparseMask(Box<vtensor_attention::Adt4>),
    Indexer(Box<vtensor_attention::Adt5>),
    Recurrent(Box<vtensor_attention::Adt6>),
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
    vtensor_pointwise::run(request).map_err(BackendError::overflow)
}

impl Tensor {
    fn views_start_of(&self, source: &Tensor) -> bool {
        let backing = std::ptr::NonNull::new(source.view_src).unwrap_or_else(|| source.into());
        self.view_src == backing.as_ptr()
            && self.view_offs == source.view_offs
            && (self.data.is_null() || source.data.is_null() || self.data == source.data)
    }

    fn source(&self, slot: usize) -> Option<&Tensor> {
        // SAFETY: GGML retains every present source descriptor for the graph call.
        unsafe { self.src[slot].as_ref() }
    }

    fn map_sources<'a, T, const N: usize>(
        &'a self,
        first: usize,
        bind: impl Fn(&'a Tensor) -> Result<T, BackendError>,
    ) -> Result<[Option<T>; N], BackendError> {
        let mut inputs = [const { None }; N];
        for (slot, input) in inputs.iter_mut().enumerate() {
            *input = self.source(first + slot).map(&bind).transpose()?;
        }
        Ok(inputs)
    }

    fn inputs<const N: usize>(&self) -> Result<[Option<TensorInput<'_>>; N], BackendError> {
        self.map_sources(0, TensorInput::new)
    }
}

// GGML stores the three fields of each custom operation in op_params using memcpy.
#[derive(Clone, Copy)]
#[repr(C)]
struct CallbackParams {
    function: *const c_void,
    tasks: c_int,
    user: *mut c_void,
}

impl CallbackParams {
    fn read(tensor: &Tensor) -> Self {
        // SAFETY: op_params has 64 bytes and GGML uses this repr(C) prefix for all four callbacks.
        unsafe { tensor.op_params.as_ptr().cast::<Self>().read_unaligned() }
    }

    fn plan(tensor: &Tensor) -> Result<Option<ExecutionPlan>, BackendError> {
        let params = Self::read(tensor);
        let mask = tensor
            .src
            .iter()
            .enumerate()
            .fold(0, |mask, (i, p)| mask | (u64::from(!p.is_null()) << i));
        let view = !tensor.view_src.is_null();
        let compatible = tensor.source(0).is_some_and(|a| {
            a.ty == tensor.ty && a.ne == tensor.ne && (!view || tensor.views_start_of(a))
        });
        let request = vtensor_native_predicates::Adt0::C5(
            tensor.op as u64,
            mask,
            u64::from(params.tasks as u32),
            !params.function.is_null(),
            view,
            compatible,
        );
        vtensor_native_predicates::run(request)
            .then_some(Some(ExecutionPlan::Callback(params)))
            .ok_or(BackendError::InvalidTensor)
    }

    fn invoke(
        self,
        tensor: TensorPtr,
        width: usize,
        execution: &threadpool::Execution,
    ) -> Result<(), BackendError> {
        // SAFETY: graph_compute retains each descriptor and its allocation until fill joins all callback tasks.
        let descriptor = unsafe { &*tensor };
        graph_tensor_data(descriptor)?;
        descriptor.map_sources::<_, GGML_MAX_SRC>(0, graph_tensor_data)?;
        let mut tasks = workspace(width as u64, ())?;
        let (dst, function, user, operation) = (
            tensor as usize,
            self.function as usize,
            self.user as usize,
            descriptor.op,
        );
        execution.fill(&mut tasks, |first, chunk| {
            let dst = dst as TensorPtr;
            let user = user as *mut c_void;
            for task in first..first+chunk.len() {
                let (ith, nth) = (task as c_int, width as c_int);
                // SAFETY: the constructor fixes the callback ABI. The host retains the function and
                // user data for the synchronous submission and owns synchronization between its tasks.
                unsafe {
                    macro_rules! operand { ($index:expr) => { *const Tensor }; }
                    macro_rules! call {
                        ($($src:expr),*) => {{
                            let callback: unsafe extern "C" fn(TensorPtr, $(operand!($src),)* c_int, c_int, *mut c_void) =
                                std::mem::transmute(function);
                            callback(dst, $((*dst).src[$src],)* ith, nth, user);
                        }};
                    }
                    match operation {
                        90 => call!(0),
                        91 => call!(0, 1),
                        92 => call!(0, 1, 2),
                        93 => call!(),
                        _ => return Err(BackendError::InvalidTensor),
                    }
                }
            }
            Ok(())
        })
    }
}

fn graph_node_tasks(t: &Tensor, limit: usize) -> Result<usize, BackendError> {
    let size = |shape: &[i64]| shape.iter().fold(1u64, |n, &v| n.saturating_mul(v as u64));
    let rows = t.source(0).map_or(0, |input| size(&input.ne[1..]));
    let heads = t.source(1).map_or(0, |input| input.ne[1] as u64);
    const TASK_WORD: usize = std::mem::offset_of!(CallbackParams, tasks) / size_of::<i32>();
    let requested = t.op_params[TASK_WORD] as u32;
    let request = vtensor_native_memory::Adt0::C4(
        t.op as u64,
        u64::from(t.op_params[0] as u32),
        size(&t.ne),
        rows,
        heads,
        requested.into(),
        limit as u64,
    );
    vtensor_native_memory::run(request)
        .map(|n| n as usize)
        .map_err(BackendError::overflow)
}

fn plan_operation(tensor: &Tensor) -> Result<Option<ExecutionPlan>, BackendError> {
    use BackendError::InvalidTensor;
    let output = tensor_metadata(tensor)?;
    let TensorDescriptor::C0(_, _, _, _, _, _, _, _, _, span, _) = &output;
    let operation = u64::try_from(tensor.op).map_err(BackendError::invalid)?;
    use vtensor_native_route::Adt0 as Route;
    let route = vtensor_native_route::run(operation, *span);
    match route {
        Route::C0 => return Ok(None),
        Route::C1 => return CallbackParams::plan(tensor),
        Route::C2 => {
            let source = tensor.source(0).ok_or(InvalidTensor)?;
            let layout = |t: &Tensor| (t.ty, t.ne, t.nb, t.data);
            let micros = u64::from(tensor.op_params[0] as u32);
            let request = vtensor_native_predicates::Adt0::C4(
                operation,
                micros,
                layout(tensor) == layout(source),
                tensor.views_start_of(source),
            );
            let delay = ExecutionPlan::Delay(std::time::Duration::from_micros(micros));
            return vtensor_native_predicates::run(request)
                .then_some(Some(delay))
                .ok_or(InvalidTensor);
        }
        Route::C3 => return attention::plan(tensor, operation, output),
        Route::C4 => return rotary::plan(tensor, output),
        Route::C5 | Route::C6 | Route::C7 => {}
    }
    let [p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, ..] =
        tensor.op_params.map(|word| u64::from(word as u32));
    let [primary, secondary, base, fourth, fifth, sixth, seventh] =
        tensor.map_sources(0, tensor_metadata)?;
    let has_source_from = |slot: usize| tensor.src[slot..].iter().any(|source| !source.is_null());
    let weight = tensor.source(0);
    let response = pointwise(GraphRequest::C0(
        operation,
        Parameters::C0(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11),
        primary.clone(),
        secondary.clone(),
        base.clone(),
        fourth,
        fifth,
        sixth,
        seventh,
        has_source_from(7),
        weight.map_or(0, |t| u64::from(t.flags as u32)),
        weight.is_some_and(|t| tensor.views_start_of(t)),
        output.clone(),
    ))?;
    let metadata = descriptor_for!(vtensor_typed::Adt1::C0);
    let typed_plan = |request| match vtensor_typed::run(request).map_err(BackendError::overflow)? {
        vtensor_typed::Adt3::C2(plan) => Ok(Some(ExecutionPlan::Typed(plan))),
        _ => Err(InvalidTensor),
    };
    let full = matches!(response, GraphResponse::C0(GraphPlan::C7));
    match response {
        GraphResponse::C0(GraphPlan::C8) => Err(InvalidTensor),
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
            let extra = has_source_from(3);
            let w = primary.ok_or(InvalidTensor)?;
            // MUL_MAT owns the ordinary row-product route. Other two-input fallbacks use the typed planner.
            if base.is_none() && !matches!(route, Route::C5) {
                let [left, out] = [w, output].map(metadata);
                let right = secondary.map(metadata);
                return typed_plan(vtensor_typed::Adt0::C3(
                    operation, p0, p1, p2, p3, extra, left, right, out,
                ));
            }
            let a = secondary.ok_or(InvalidTensor)?;
            use vtensor_matrix_q8::Adt0 as Product;
            let request = if let Some(third) = base {
                if !matches!(route, Route::C6) {
                    let [left, right, base, out] = [w, a, third, output].map(metadata);
                    return typed_plan(vtensor_typed::Adt0::C2(
                        operation, extra, left, right, base, out,
                    ));
                }
                let [w, a, ids, out] = [w, a, third, output].map(descriptor);
                Product::C3(operation, extra, w, a, ids, out, full)
            } else {
                let [w, a, out] = [w, a, output].map(descriptor);
                Product::C0(operation, extra, w, a, out, full)
            };
            match vtensor_matrix_q8::run(request).map_err(BackendError::overflow)? {
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
    host_region(tensor.buffer, tensor, span, 0, span)
}

fn host_region(
    handle: Buf,
    tensor: &Tensor,
    span: usize,
    offset: usize,
    size: usize,
) -> Result<*mut u8, BackendError> {
    use vtensor_buffer_region::Adt0 as Region;
    // SAFETY: GGML retains the buffer for this call. Only ordinary host layouts reach get_base.
    let buffer = unsafe { handle.as_ref() }.ok_or(BackendError::InvalidTensor)?;
    // SAFETY: a live GGML buffer retains its type descriptor for the same lifetime.
    if !unsafe { supports_buffer_type(state().dev.get(), buffer.buft) } {
        return Err(BackendError::InvalidTensor);
    }
    let base = match buffer.iface.get_base {
        // SAFETY: the callback belongs to this live host buffer. Prism bounds the returned address.
        Some(get) if span != 0 => unsafe { get(handle) }.cast::<u8>(),
        _ => null_mut(),
    };
    let decision = vtensor_buffer_region::run(
        base as usize as u64,
        buffer.size as u64,
        tensor.data as usize as u64,
        span as u64,
        offset as u64,
        size as u64,
    )
    .map_err(BackendError::overflow)?;
    match decision {
        Region::C0 => Ok(null_mut()),
        Region::C1 => Err(BackendError::InvalidTensor),
        Region::C2 => Err(BackendError::OutOfBounds),
        // SAFETY: Prism bounded the whole tensor and requested range inside this backing allocation.
        Region::C3(at) => Ok(unsafe { base.add(at as usize) }),
    }
}

fn reserved<T>(count: usize) -> Result<Vec<T>, BackendError> {
    let mut result = Vec::new();
    result.try_reserve_exact(count)?;
    Ok(result)
}

fn workspace<T: Clone>(size: u64, initial: T) -> Result<Vec<T>, BackendError> {
    let count = usize::try_from(size).map_err(BackendError::overflow)?;
    let mut result = reserved(count)?;
    result.resize(count, initial);
    Ok(result)
}

fn rounded_dot<'a>(
    [product, widen, add, narrow]: [u64; 4],
    weights: &[f32],
    activations: impl Iterator<Item = &'a f32>,
) -> Result<f32, BackendError> {
    let wide = |op, a, b| checked_word!(linear::evaluate_wide(op, a, b));
    let mut sum = 0;
    for (&weight, &activation) in weights.iter().zip(activations) {
        let term = linear::evaluate_f32_scalar(product, weight, activation)?;
        sum = wide(add, sum, wide(widen, term.to_bits().into(), 0)?)?;
    }
    Ok(f32::from_bits(wide(narrow, sum, 0)? as u32))
}

fn execute_matrix(tensor: &Tensor, plan: GraphPlan) -> Result<(), BackendError> {
    use vtensor_matrix_execution::{Adt0 as Strides, Adt1 as Instruction};
    tensor_fields!(GraphPlan::C3(m, k, n, b2, b3, r2, r3, left_count, right_count, _) = plan);
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
        .map_err(BackendError::overflow)
    };
    tensor_fields!(Instruction::C0(steps, total) = step(0)?);
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
                decode_block(kind, block, float_bytes(dst), stride as usize)?;
                index = index.checked_add(skip).ok_or(BackendError::Overflow)?;
            }
            Instruction::C2 | Instruction::C6(..) => {
                let (left, rest) = values.split_at_mut(left_count as usize);
                let (right, output) = rest.split_at_mut(right_count as usize);
                if let Instruction::C6(product, widen, add, narrow) = instruction {
                    let stages = [product, widen, add, narrow];
                    for (index, value) in output.iter_mut().enumerate() {
                        let start = index / n * k;
                        *value = rounded_dot(
                            stages,
                            &left[start..start + k],
                            right.iter().skip(index % n).step_by(n),
                        )?;
                    }
                } else {
                    linear::multiply_matrix_rows_f32(m, k, n, left, right, output)?;
                }
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
    tensor_fields!([Some(kernel), Some(image)] = tensor.inputs()?);
    let destination = graph_tensor_data(tensor)?;
    let sample = |phase, start, index| {
        let query = GraphRequest::C4(geometry.clone(), phase, start, index as u64);
        tensor_fields!(GraphResponse::C8(offset) = pointwise(query)?);
        Ok(offset)
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
    let value = match kind {
        0 | 1 | 30 | 43 => {
            // SAFETY: scalar_word reads exactly the supplied slice length and checks its width.
            let bits =
                unsafe { scalar_word(source.as_ptr().cast_mut(), source.len() as u64, None)? };
            decode_float(kind, bits)?
        }
        26 => linear::signed_word_f32(i32::from_ne_bytes(
            source.try_into().map_err(BackendError::invalid)?,
        ))?,
        _ => return Ok(linear::decode_storage_block(kind, source, output, stride)?),
    };
    *output.first_chunk_mut().ok_or(BackendError::OutOfBounds)? = value.to_ne_bytes();
    Ok(())
}

fn float_bytes(values: &mut [f32]) -> &mut [u8] {
    // SAFETY: the borrow is exclusive and every binary32 bit pattern is valid.
    unsafe {
        std::slice::from_raw_parts_mut(values.as_mut_ptr().cast(), std::mem::size_of_val(values))
    }
}

struct ScalarCast(WordOperation, Option<WordOperation>);
impl ScalarCast {
    fn new(source: c_int, destination: c_int) -> Result<Self, BackendError> {
        let reply = pointwise(GraphRequest::C8(source as u64, destination as u64))?;
        tensor_fields!(GraphResponse::C12(stage, tail) = reply);
        Ok(Self(stage, tail))
    }

    fn apply(&self, input: u64) -> Result<u64, BackendError> {
        let first = word_value(self.0.clone(), input, 0)?;
        self.1
            .as_ref()
            .map_or(Ok(first), |next| word_value(next.clone(), first, 0))
    }
}

struct TensorInput<'a> {
    scalar_read: Option<ScalarCast>,
    block_lanes: usize,
    block_bytes: usize,
    bytes: &'a [u8],
    ty: c_int,
    ne: [i64; 4],
    nb: [usize; 4],
}

impl<'a> TensorInput<'a> {
    fn new(t: &'a Tensor) -> Result<Self, BackendError> {
        let (TensorDescriptor::C0(.., span, _), block_lanes, block_bytes) = tensor_layout(t)?;
        let span = span as usize;
        let bytes = if span == 0 {
            &[]
        } else {
            // SAFETY: GGML retains this checked native region throughout graph execution.
            unsafe { std::slice::from_raw_parts(graph_tensor_region(t, span)?, span) }
        };
        Ok(Self {
            scalar_read: None,
            bytes,
            block_lanes,
            block_bytes,
            ty: t.ty,
            ne: t.ne,
            nb: t.nb,
        })
    }

    fn region(&self, offset: u64, count: usize) -> Result<&[u8], BackendError> {
        if count == 0 {
            return Ok(&[]);
        }
        let offset = usize::try_from(offset).map_err(BackendError::overflow)?;
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

// A packed GGML format can have a Verdant dot product before it gains an encoder.
macro_rules! packed_formats {
    ($( $id:literal => $dot:expr $(, $store:ident)?; )+) => {
        fn packed_weight(kind: u64) -> Option<PackedWeight> {
            use linear::{AffineWeightEncoding::*, BlockFloatEncoding::*, SymmetricWeightEncoding::*};
            use linear::{CodebookEncoding::*, GroupedWeightEncoding as GroupedFormat};
            use PackedWeight::{Affine, Grouped, Symmetric};
            match kind { $($id => Some($dot),)+ _ => None }
        }
        fn weight_encoder(kind: c_int) -> Option<linear::PackedWeightEncoding> {
            match kind { $($( $id => Some(linear::PackedWeightEncoding::$store), )?)+ _ => None }
        }
    };
}
packed_formats! {
    2 => Symmetric(linear::dot_symmetric_q4_row), CenteredFour;
    3 => Affine(|w, a| linear::dot_affine_weight_row(w, a, FourBit)), AffineFour;
    6 => Symmetric(|w, a| linear::dot_encoded_symmetric_row(w, a, SignedFive)), CenteredFive;
    7 => Affine(|w, a| linear::dot_affine_weight_row(w, a, FiveBit)), AffineFive;
    8 => Symmetric(linear::dot_symmetric_q8_row), SymmetricEight;
    10 => Grouped(GroupedFormat::AffineTwo), GroupedAffineTwo;
    11 => Grouped(GroupedFormat::SignedThree), GroupedSignedThree;
    12 => Grouped(GroupedFormat::AffineFour), GroupedAffineFour;
    13 => Grouped(GroupedFormat::AffineFive), GroupedAffineFive;
    14 => Grouped(GroupedFormat::SignedSix), GroupedSignedSix;
    16 => Grouped(GroupedFormat::Codebook(IndexedSigns)), ImportanceTwoGroupedParity;
    17 => Grouped(GroupedFormat::Codebook(ExtendedIndex)), ImportanceTwoParity;
    18 => Grouped(GroupedFormat::Codebook(FourLaneIndexedSigns)), ImportanceThreeCompact;
    19 => Grouped(GroupedFormat::Codebook(ShiftedGrid)), ImportanceOneShifted;
    20 => Symmetric(|w, a| linear::dot_encoded_symmetric_row(w, a, NonlinearFour)), NonlinearFour;
    21 => Grouped(GroupedFormat::Codebook(FourLaneExplicitSigns)), ImportanceThree;
    22 => Grouped(GroupedFormat::Codebook(ExplicitSigns)), ImportanceTwo;
    23 => Grouped(GroupedFormat::NonlinearFour), GroupedNonlinearFour;
    29 => Grouped(GroupedFormat::Codebook(DistributedScale)), ImportanceOneDistributed;
    34 => Grouped(GroupedFormat::PackedTrits), PackedTrits;
    35 => Grouped(GroupedFormat::TwoBitTernary), TwoBitTernary;
    39 => Symmetric(|w, a| linear::dot_block_float_row(w, a, ExponentOnly)), ExponentOnlyFour;
    40 => Symmetric(|w, a| linear::dot_block_float_row(w, a, GroupedE4M3)), UnsignedScaleFour;
    41 => Symmetric(linear::dot_binary_sign_row), BinarySign;
    42 => Symmetric(linear::dot_two_bit_row), CenteredTwo;
}

fn block_field<const N: usize>(bytes: &mut &[u8]) -> Result<[u8; N], BackendError> {
    let (field, tail) = bytes
        .split_first_chunk()
        .ok_or(BackendError::InvalidTensor)?;
    *bytes = tail;
    Ok(*field)
}

fn symmetric_activation(mut bytes: &[u8]) -> Result<linear::SymmetricQ8Block<32>, BackendError> {
    Ok(linear::SymmetricQ8Block {
        scale: u16::from_ne_bytes(block_field(&mut bytes)?),
        codes: block_field(&mut bytes)?.map(|byte| byte as i8),
    })
}

fn affine_activation(mut bytes: &[u8]) -> Result<linear::AffineQ8Block<32>, BackendError> {
    let scale = u16::from_ne_bytes(block_field(&mut bytes)?);
    let scaled_sum = u16::from_ne_bytes(block_field(&mut bytes)?);
    Ok(linear::AffineQ8Block {
        symmetric: linear::SymmetricQ8Block {
            scale,
            codes: block_field(&mut bytes)?.map(|byte| byte as i8),
        },
        scaled_sum,
    })
}

fn grouped_activation(mut bytes: &[u8]) -> Result<linear::GroupedQ8Block, BackendError> {
    let scale = f32::from_ne_bytes(block_field(&mut bytes)?);
    let codes = block_field(&mut bytes)?.map(|byte| byte as i8);
    tensor_fields!((words, []) = bytes.as_chunks::<2>());
    let sums: &[[u8; 2]; 16] = words.try_into().map_err(BackendError::invalid)?;
    Ok(linear::GroupedQ8Block {
        scale,
        codes,
        sums: sums.map(i16::from_ne_bytes),
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
        ($encode:expr, $read:expr, $dot:expr) => {
            execute_block_rows(
                execution,
                0,
                tensor,
                plan,
                |input, at, lanes| {
                    if input.block_lanes == 1 {
                        input.decode(at, lanes)?;
                        Ok($encode(lanes)?)
                    } else {
                        $read(input.region(at, input.block_bytes)?)
                    }
                },
                |source, at, a, _| Ok(($dot)(source.region(at, weight_bytes as usize)?, a)?),
            )
        };
    }
    match plan {
        C0(inner, .., Some(precision), kind) => {
            use vtensor_matrix_q8::{Adt0 as Request, Adt4 as Response};
            let stages = match vtensor_matrix_q8::run(Request::C5(kind))
                .map_err(BackendError::overflow)?
            {
                Response::C6(product, widen, add, narrow) => Some([product, widen, add, narrow]),
                Response::C0 => None,
                _ => return Err(BackendError::InvalidTensor),
            };
            execute_block_rows(
                execution,
                inner,
                tensor,
                plan,
                |input, at, lanes: &mut [f32; 1]| {
                    input.decode(at, lanes)?;
                    prepare_float(0, precision, lanes[0].to_bits().into())
                },
                |source, at, activations, weights| source.dot(at, activations, weights, stages),
            )
        }
        C0(.., None, kind) => match packed_weight(kind).ok_or(BackendError::InvalidTensor)? {
            PackedWeight::Grouped(kind) => packed!(
                linear::encode_grouped_q8_block,
                grouped_activation,
                |w, a| { linear::dot_grouped_weight_row(w, a, kind) }
            ),
            PackedWeight::Symmetric(dot) => {
                packed!(linear::encode_symmetric_q8_block, symmetric_activation, dot)
            }
            PackedWeight::Affine(dot) => {
                packed!(linear::encode_affine_q8_block, affine_activation, dot)
            }
        },
    }
}

fn execute_block_rows<const LANES: usize, Block: Sync>(
    execution: &threadpool::Execution,
    scratch_width: u64,
    tensor: &Tensor,
    plan: RowProductPlan,
    prepare: impl Fn(&TensorInput, u64, &mut [f32; LANES]) -> Result<Block, BackendError>,
    dot: impl Fn(&TensorInput, u64, &[Block], &mut [f32]) -> Result<f32, BackendError> + Sync,
) -> Result<(), BackendError> {
    use vtensor_matrix_q8::{run, Adt0 as Request, Adt2 as Plan, Adt4 as Response};
    use BackendError::{InvalidTensor, Overflow};
    let Plan::C0(blocks, _, rows, _, _, visits, outputs, ..) = plan;
    tensor_fields!([Some(weights), Some(activations), indices] = tensor.inputs()?);
    let request = |query| run(query).map_err(BackendError::overflow);
    let destination = graph_tensor_data(tensor)?;
    let mut results = workspace(outputs, 0.0f32)?;
    let mut offsets = reserved(usize::try_from(visits).map_err(BackendError::overflow)?)?;
    if !LANES.is_multiple_of(activations.block_lanes) {
        return Err(InvalidTensor);
    }
    let stride = (LANES / activations.block_lanes)
        .checked_mul(activations.nb[0])
        .ok_or(Overflow)? as u64;
    let mut lanes = [0.0; LANES];
    let mut prepared = reserved(usize::try_from(blocks).map_err(BackendError::overflow)?)?;
    for (column, values) in (0..visits).zip(results.chunks_exact_mut(rows as usize)) {
        let mut address = request(Request::C1(plan.clone(), column))?;
        if let Response::C4(at) = address {
            let source = indices.as_ref().ok_or(InvalidTensor)?;
            let expert = u32::from_ne_bytes(source.word(at)?);
            address = request(Request::C4(plan.clone(), column, expert.into()))?;
        }
        let (weight_base, activation, dest) = match address {
            Response::C2(w, a, o) => (w, a, o),
            Response::C5(dest) => {
                offsets.push(dest);
                continue;
            }
            _ => return Err(InvalidTensor),
        };
        offsets.push(dest);
        prepared.clear();
        let mut from = activation;
        for _ in 0..blocks {
            prepared.push(prepare(&activations, from, &mut lanes)?);
            from = from.checked_add(stride).ok_or(Overflow)?;
        }
        execution.fill(values, |first, chunk| {
            let mut scratch = workspace(scratch_width, 0.0_f32)?;
            for (row, value) in (first..).zip(chunk) {
                let reply = request(Request::C2(plan.clone(), weight_base, row as u64))?;
                tensor_fields!(Response::C3(start) = reply);
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
        (0, 43) => linear::evaluate_small_float(4, word).map_err(BackendError::overflow)?,
        (43, 0) => linear::evaluate_small_float(3, word).map_err(BackendError::overflow)?,
        _ => linear::convert_float_storage(from, to, word).map_err(BackendError::overflow)?,
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
            .map_err(BackendError::invalid),
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

fn vector_swiglu(input: f32, gate: f32) -> Result<f32, BackendError> {
    use vtensor_vector_swiglu::Adt0::{C0 as Arithmetic, C1 as Literal, C2 as ResultSlot};
    let mut slots = [0u64; 30];
    slots[0] = input.to_bits().into();
    slots[1] = gate.to_bits().into();
    for stage in 0..slots.len() {
        let step = vtensor_vector_swiglu::run(stage as u64, slots[5], slots[6], slots[25])
            .map_err(BackendError::overflow)?;
        let read = |operand: u64| slots.get(operand as usize).copied().unwrap_or(operand);
        let word = match step {
            Arithmetic(kind, a, b) => word_value(WordOperation::C1(kind), read(a), read(b))?,
            Literal(word) => word,
            ResultSlot(slot) => {
                return slots
                    .get(slot as usize)
                    .map(|word| f32::from_bits(*word as u32))
                    .ok_or(BackendError::InvalidTensor)
            }
        };
        *slots
            .get_mut(stage + 3)
            .ok_or(BackendError::InvalidTensor)? = word;
    }
    Err(BackendError::InvalidTensor)
}

fn gated_value(variant: u64, storage: u64, input: f32, gate: f32) -> Result<f32, BackendError> {
    let word = u64::from(input.to_bits());
    let activation = match variant {
        0 if checked_word!(linear::evaluate_encoded_word(0, word, 0))? != 2 => return Ok(0.0),
        0 => input,
        1 if storage == 0 => match gaussian_table(word)? {
            Some(value) => value,
            None => return Ok(0.0),
        },
        1 => linear::gelu_f32(input)?,
        2 => return Ok(linear::swiglu_f32(input, gate)?),
        7 => return vector_swiglu(input, gate),
        4 => linear::gaussian_error_f32(input, true)?,
        5 => linear::gelu_quick_f32(input)?,
        _ => return Err(BackendError::InvalidTensor),
    };
    let activation = if storage == 1 && matches!(variant, 1 | 5) {
        prepare_float(0, 1, u64::from(activation.to_bits()))?
    } else {
        activation
    };
    Ok(linear::evaluate_f32_scalar(2, activation, gate)?)
}

fn signed_field(a: u64, sign_bit: u64, extension: u64, limit: u64) -> Result<u64, BackendError> {
    let field = |word, factor, divisor, modulus| {
        checked_word!(linear::extract_packed_field(word, factor, divisor, modulus))
    };
    if extension == 0 {
        field(a, 1, 1, limit)
    } else {
        let sign = field(a, 1, sign_bit, 2)?;
        let high = field(sign, extension, 1, limit)?;
        checked_word!(linear::evaluate_encoded_word(2, a, high))
    }
}

fn word_value(operation: WordOperation, a: u64, b: u64) -> Result<u64, BackendError> {
    use BackendError::InvalidTensor;
    let failure = |overflow| {
        if overflow {
            BackendError::Overflow
        } else {
            InvalidTensor
        }
    };
    let float = |word: u64| f32::from_bits(word as u32);
    match operation {
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
                .map_err(BackendError::overflow)?
        }
        WordOperation::C5(kind) => return checked_word!(linear::evaluate_encoded_word(kind, a, b)),
        WordOperation::C8(kind) => return checked_word!(linear::evaluate_small_float(kind, a)),
        WordOperation::C9 => return checked_word!(linear::truncate_signed_word_f32(a)),
        WordOperation::C10(sign_bit, extension, limit) => {
            return signed_field(a, sign_bit, extension, limit)
        }
        WordOperation::C11(source, limit) => {
            let value = checked_word!(linear::truncate_signed_word_f32(convert(source, 0, a)?))?;
            return checked_word!(linear::extract_packed_field(value, 1, 1, limit));
        }
        WordOperation::C12(sign_bit, extension, destination) => {
            let word = signed_field(a, sign_bit, extension, 1_u64 << 32)?;
            let value = linear::signed_word_f32(word as i32)?;
            Some(convert(0, destination, u64::from(value.to_bits()))?)
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
                24 => linear::gaussian_error_f32(input, true),
                _ => return Err(InvalidTensor),
            }
            .map_err(BackendError::overflow)?;
            Some(convert(0, destination, u64::from(result.to_bits()))?)
        }
    }
    .ok_or(InvalidTensor)
}

fn execute_word(
    response: GraphResponse,
    pointers: &[*mut u8],
    destination: *mut u8,
    words: &mut [u64],
) -> Result<(), BackendError> {
    use BackendError::InvalidTensor;
    tensor_fields!(
        GraphResponse::C7(operation, left, right, target, read_bytes, write_bytes) = response
    );
    let source = |slot: u64| pointers.get(slot as usize).copied().ok_or(InvalidTensor);
    let read = |operand| {
        let (pointer, offset) = match operand {
            WordOperand::C0(slot, offset) => (source(slot)?, offset),
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
    let value = word_value(operation, read(left)?, read(right)?)?;
    let (pointer, offset) = match target {
        WordTarget::C0(index) => {
            *words.get_mut(index as usize).ok_or(InvalidTensor)? = value;
            return Ok(());
        }
        WordTarget::C1(offset) => (destination, offset),
        WordTarget::C2 => return Err(InvalidTensor),
        WordTarget::C3(slot, offset) => (source(slot)?, offset),
    };
    // SAFETY: Prism bounds each store in the output or a bound mutable optimizer state.
    unsafe { scalar_word(pointer.add(offset as usize), write_bytes, Some(value))? };
    Ok(())
}

fn execute_typed(tensor: &Tensor, plan: vtensor_typed::Adt2) -> Result<(), BackendError> {
    use vtensor_typed::Adt3::{C3 as Word, C4 as Gate, C5 as Rank, C6 as Encode, C7 as BinaryRow};
    use BackendError::InvalidTensor;
    let vtensor_typed::Adt2::C0(count, capacity, importance, operation, .., row_arithmetic, width) =
        &plan;
    let mut words = workspace(*capacity, 0_u64)?;
    let mut ranker = None;
    let (left, secondary) = (tensor.source(0).ok_or(InvalidTensor)?, tensor.source(1));
    use linear::RowRankingF32;
    // Word instructions can update their input in place. Only staged row conversion
    // retains an immutable input slice throughout the instruction loop.
    let source = (*width != 0).then(|| TensorInput::new(left)).transpose()?;
    let mut right = secondary
        .filter(|_| *row_arithmetic)
        .map(TensorInput::new)
        .transpose()?;
    if let Some(input) = right.as_mut().filter(|input| (24..=28).contains(&input.ty)) {
        input.scalar_read = Some(ScalarCast::new(input.ty, 0)?);
    }
    let pointer = |tensor, input: Option<&TensorInput>| match input {
        Some(input) => Ok(input.bytes.as_ptr().cast_mut()),
        None => graph_tensor_data(tensor),
    };
    let primary = pointer(left, source.as_ref())?;
    let pointers = [
        primary,
        secondary.map_or(Ok(primary), |t| pointer(t, right.as_ref()))?,
    ];
    let mut offsets = Vec::new();
    let mut encoded = Vec::new();
    let (TensorDescriptor::C0(.., span, _), lanes, block_bytes) = tensor_layout(tensor)?;
    let scalar_store = (source.is_some() && lanes == 1)
        .then(|| ScalarCast::new(0, tensor.ty))
        .transpose()?;
    let uniform = importance
        .map(|word| workspace(lanes as u64, f32::from_bits(word as u32)))
        .transpose()?;
    let output = graph_tensor_region(tensor, span as usize)?;
    let kind = weight_encoder(tensor.ty);
    let mut values = workspace(*width, 0.0)?;
    let mut right_values = workspace(right.as_ref().map_or(0, |input| input.ne[0] as u64), 0.0)?;
    for index in 0..*count {
        let instruction =
            vtensor_typed::run(vtensor_typed::Adt0::C1(index, plan.clone(), words[2]))
                .map_err(BackendError::overflow)?;
        match instruction {
            Encode(from, to) | BinaryRow(from, _, to) => {
                let source = source.as_ref().ok_or(InvalidTensor)?;
                source.decode(from, &mut values)?;
                if let BinaryRow(_, offset, _) = instruction {
                    let right = right.as_ref().ok_or(InvalidTensor)?;
                    right.decode(offset, &mut right_values)?;
                    for (value, rhs) in values.iter_mut().zip(right_values.iter().cycle()) {
                        *value = linear::evaluate_f32_scalar(*operation, *value, *rhs)?;
                    }
                }
                let row_start = encoded.len();
                let row_bytes = (values.len() / lanes)
                    .checked_mul(tensor.nb[0])
                    .ok_or(BackendError::Overflow)?;
                encoded.try_reserve(row_bytes)?;
                for block_values in values.chunks_exact(lanes) {
                    let start = encoded.len();
                    match (tensor.ty, &scalar_store) {
                        (9, _) => {
                            let row = linear::encode_affine_q8_block::<32>(
                                block_values.try_into().map_err(|_| InvalidTensor)?,
                            )?;
                            encoded.extend(row.symmetric.scale.to_ne_bytes());
                            encoded.extend(row.scaled_sum.to_ne_bytes());
                            encoded.extend(row.symmetric.codes.map(|code| code as u8));
                        }
                        (15, _) => {
                            let row = linear::encode_grouped_q8_block(
                                block_values.try_into().map_err(|_| InvalidTensor)?,
                            )?;
                            encoded.extend(row.scale.to_ne_bytes());
                            encoded.extend(row.codes.map(|code| code as u8));
                            encoded.extend(row.sums.into_iter().flat_map(i16::to_ne_bytes));
                        }
                        (_, Some(cast)) => {
                            let word = cast.apply(block_values[0].to_bits().into())?;
                            encoded.resize(start + block_bytes, 0);
                            // SAFETY: the appended scratch block contains exactly block_bytes bytes.
                            unsafe {
                                scalar_word(
                                    encoded[start..].as_mut_ptr(),
                                    block_bytes as u64,
                                    Some(word),
                                )?
                            };
                        }
                        _ => {
                            let kind = kind.ok_or(InvalidTensor)?;
                            encoded.resize(start + tensor.nb[0], 0);
                            let block = &mut encoded[start..];
                            if let Some(weights) = uniform.as_deref() {
                                linear::encode_importance_weight_block(
                                    block_values,
                                    Some(weights),
                                    kind,
                                    block,
                                )?;
                            } else {
                                linear::encode_weight_block(block_values, kind, block)?;
                            }
                        }
                    }
                    if encoded.len() - start != tensor.nb[0] {
                        return Err(InvalidTensor);
                    }
                }
                offsets.try_reserve(1)?;
                offsets.push((to, row_start..encoded.len()));
            }
            Rank(from, to, width, count, descending) => {
                let (width, count) = (width as usize, count as usize);
                let rank = ranker
                    .get_or_insert_with(|| RowRankingF32::new(width, count, descending))
                    .as_mut()
                    .map_err(BackendError::invalid)?;
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
    for (offset, range) in offsets {
        // SAFETY: Prism bounded each complete block in the destination. All reads and
        // numerical conversions have finished, including any overlapping source views.
        unsafe { write_row(output.add(offset as usize), &encoded[range]) };
    }
    Ok(())
}

fn execute_copy(tensor: &Tensor, plan: GraphPlan) -> Result<(), BackendError> {
    use BackendError::InvalidTensor;
    tensor_fields!(GraphPlan::C4(count, bytes, source, destination, mode) = plan);
    let reply = pointwise(GraphRequest::C2(mode.clone()))?;
    tensor_fields!(GraphResponse::C6(mask, literal_type, bits, constant, capacity) = reply);
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
    for (slot, pointer) in pointers.iter_mut().enumerate() {
        if mask & (1 << slot) != 0 {
            let source = tensor.source(slot).ok_or(InvalidTensor)?;
            *pointer = graph_tensor_data(source)?;
        }
    }
    // SAFETY: both decoder instructions carry a complete source block bounded by Prism.
    let encoded = |from: u64, bytes: u64| unsafe {
        std::slice::from_raw_parts(pointers[0].add(from as usize), bytes as usize)
    };
    let destination_data = graph_tensor_data(tensor)?;
    let mut block = workspace(bytes, 0_u8)?;
    let mut fixed = [0; 3];
    let mut dynamic = (capacity > fixed.len() as u64)
        .then(|| workspace(capacity, 0_u64))
        .transpose()?;
    let words = dynamic.as_deref_mut().unwrap_or(&mut fixed);
    let [x0, x1, x2, x3] = match mode {
        CopyMode::C26(66, ..) => tensor.source(1).ok_or(InvalidTensor)?.nb.map(|v| v as u64),
        _ => [0; 4],
    };
    let mut steps = 0..count;
    while let Some(index) = steps.next() {
        let response = pointwise(GraphRequest::C6(
            index,
            source.clone(),
            destination.clone(),
            mode.clone(),
            words[2],
            words[1],
            bytes,
            x0,
            x1,
            x2,
            x3,
        ))?;
        match response {
            GraphResponse::C11(next) if (steps.start..=count).contains(&next) => steps.start = next,
            GraphResponse::C7(..) => execute_word(response, &pointers, destination_data, words)?,
            // SAFETY: Prism preflights every index and bounds the complete block and output lanes.
            GraphResponse::C10(kind, from, width, to, lanes, stride) => unsafe {
                let mut decoded = [0.0; 256];
                let row = decoded.get_mut(..lanes as usize).ok_or(InvalidTensor)?;
                decode_block(kind, encoded(from, width), float_bytes(row), 4)?;
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

// GGML retains the node array for the call. A persistent plan copies the array and
// borrows its tensors, so clearing the graph builder cannot change the captured topology.
unsafe fn graph_nodes<'a>(graph: Graph) -> &'a [TensorPtr] {
    // SAFETY: the caller retains a valid GGML graph for the returned borrow.
    let count = unsafe { ggml_graph_n_nodes(graph) } as usize;
    if count == 0 {
        return &[];
    }
    // SAFETY: a nonempty GGML graph exposes count initialized entries for its retained lifetime.
    unsafe { std::slice::from_raw_parts(ggml_graph_nodes(graph), count) }
}

unsafe extern "C" fn graph_plan_create(_: BackendPtr, graph: *const c_void) -> Plan {
    // SAFETY: the caller supplies the live source graph and retains the tensors until plan release.
    let nodes = unsafe { graph_nodes(graph.cast_mut()) };
    let Ok(mut owned) = reserved(nodes.len()) else {
        return null_mut();
    };
    owned.extend_from_slice(nodes);
    Box::into_raw(Box::new(owned)).cast()
}

unsafe extern "C" fn graph_plan_free(_: BackendPtr, plan: Plan) {
    if !plan.is_null() {
        // SAFETY: a non-null handle is the live allocation returned by create.
        unsafe { drop(Box::from_raw(plan.cast::<Vec<TensorPtr>>())) };
    }
}

unsafe extern "C" fn graph_plan_compute(backend: BackendPtr, plan: Plan) -> c_int {
    // SAFETY: the handle and its tensors live for this synchronous submission.
    let Some(nodes) = (unsafe { plan.cast::<Vec<TensorPtr>>().as_ref() }) else {
        return GGML_STATUS_FAILED;
    };
    execute_graph(backend, nodes).map_or(GGML_STATUS_FAILED, |_| GGML_STATUS_SUCCESS)
}

unsafe extern "C" fn graph_compute(backend: BackendPtr, graph: Graph) -> c_int {
    // SAFETY: GGML retains the graph and its tensors until this call returns.
    execute_graph(backend, unsafe { graph_nodes(graph) })
        .map_or(GGML_STATUS_FAILED, |_| GGML_STATUS_SUCCESS)
}

fn execute_graph(backend: BackendPtr, nodes: &[TensorPtr]) -> Result<(), BackendError> {
    let mut plans = Vec::new();
    let mut has_callbacks = false;
    // SAFETY: GGML retains this graph and all descriptors for the synchronous
    // call. Preflight plans every numerical node once before any computation.
    unsafe {
        for &tensor in nodes {
            let flags = u64::from((*tensor).flags as u32);
            if !vtensor_native_predicates::run(vtensor_native_predicates::Adt0::C2(flags)) {
                continue;
            }
            if let Some(plan) = plan_operation(&*tensor)? {
                plans.try_reserve(1)?;
                has_callbacks |= matches!(plan, ExecutionPlan::Callback(_));
                plans.push((tensor, plan));
            }
        }
        let execution = threadpool::execution(backend, has_callbacks)?;
        let mut callback_width = 1;
        if has_callbacks {
            // The pinned CPU plans one team for the whole graph, including nodes that
            // request more tasks than a callback. Width only matters for host callbacks.
            for &node in nodes {
                if callback_width == execution.threads {
                    break;
                }
                let n = graph_node_tasks(&*node, execution.threads)?;
                callback_width = callback_width.max(n);
            }
        }
        for (tensor, plan) in plans {
            match plan {
                ExecutionPlan::Callback(callback) => {
                    callback.invoke(tensor, callback_width, &execution)
                }
                ExecutionPlan::Standard(plan @ GraphPlan::C3(..)) => execute_matrix(&*tensor, plan),
                ExecutionPlan::Standard(plan @ GraphPlan::C4(..)) => execute_copy(&*tensor, plan),
                ExecutionPlan::Standard(GraphPlan::C6(geometry)) => {
                    execute_convolution(&*tensor, geometry)
                }
                ExecutionPlan::Delay(duration) => {
                    std::thread::sleep(duration);
                    Ok(())
                }
                ExecutionPlan::Recurrent(plan) => attention::recurrent(&execution, &*tensor, *plan),
                ExecutionPlan::Indexer(plan) => attention::indexer(&*tensor, *plan),
                ExecutionPlan::SparseMask(plan) => attention::sparse_mask(&*tensor, *plan),
                ExecutionPlan::Softmax(plan) => attention::softmax(&*tensor, *plan),
                ExecutionPlan::Attention(plan) => attention::execute(&*tensor, *plan),
                ExecutionPlan::Rotary(plan) => rotary::execute(&*tensor, plan),
                ExecutionPlan::Typed(plan) => execute_typed(&*tensor, plan),
                ExecutionPlan::RowProduct(plan) => execute_row_products(&execution, &*tensor, plan),
                _ => Err(BackendError::InvalidTensor),
            }?;
        }
    }
    Ok(())
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
        vtensor_rotary::run(query).map_err(BackendError::overflow)
    }

    pub(super) fn plan(
        tensor: &Tensor,
        output: TensorDescriptor,
    ) -> Result<Option<super::ExecutionPlan>, BackendError> {
        let descriptor = descriptor_for!(Descriptor::C0);
        tensor_fields!(
            [Some(source), Some(positions), factors] =
                tensor.map_sources(0, |t| super::tensor_metadata(t).map(descriptor))?
        );
        let p = tensor.op_params.map(|word| u64::from(word as u32));
        match request(Request::C0(
            tensor.op as u64,
            tensor.src[3..].iter().any(|v| !v.is_null()),
            Layout::C0(p[1], p[2], p[15], p[11], p[12], p[13], p[14]),
            source,
            positions,
            factors,
            descriptor(output),
        ))? {
            Response::C2(plan) => Ok(Some(super::ExecutionPlan::Rotary(plan))),
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
        .map_err(BackendError::invalid)
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
            let reply = request(Request::C3(layout.clone(), lane as u64))?;
            tensor_fields!(Response::C6(axis, reset) = reply);
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
                    let reply = request(Request::C1(plan.clone(), head, token, batch))?;
                    tensor_fields!(Response::C3(src, dst) = reply);
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
        decode_block, float_bytes, graph_tensor_data, workspace, BackendError, Tensor,
        TensorDescriptor, TensorInput,
    };
    use verdant_tensor::linear::{
        self, evaluate_f32_scalar, multiply_matrix_rows_f32, WeightedMeanF32,
    };
    use vtensor_attention::{
        Adt0 as Request, Adt1 as Descriptor, Adt2 as Plan, Adt3 as SoftmaxPlan, Adt4 as SparsePlan,
        Adt5 as IndexerPlan, Adt6 as RecurrentPlan, Adt7 as Response,
    };
    use BackendError::{InvalidTensor, Overflow};

    fn request(query: Request) -> Result<Response, BackendError> {
        vtensor_attention::run(query).map_err(BackendError::overflow)
    }

    pub(super) fn recurrent(
        execution: &super::threadpool::Execution,
        tensor: &Tensor,
        plan: RecurrentPlan,
    ) -> Result<(), BackendError> {
        let RecurrentPlan::C0(.., streams, cells, capacity, written, selective) = plan;
        tensor_fields!(
            [Some(a), Some(b), Some(c), Some(d), Some(e), Some(f), ids] = tensor.inputs()?
        );
        let [q, k, v, gate, gain, initial] = if selective {
            [e, f, b, d, c, a]
        } else {
            [a, b, c, d, e, f]
        };
        let [width, heads, tokens, _] = v.ne.map(|n| n as u64);
        let mut staged = workspace(written, 0.0f32)?;
        let address = staged.as_mut_ptr().expose_provenance();
        let stage = |offset: Option<u64>, row: &[f32]| {
            if let Some(at) = offset {
                let base = std::ptr::with_exposed_provenance_mut::<u8>(address);
                // SAFETY: Prism assigns each (head, batch) disjoint complete score and state rows
                // inside this prefix. The allocation stays fixed until every worker has joined.
                unsafe { super::write_row(base.add(at as usize), row) };
            }
        };
        let mut jobs = workspace(streams, ())?;
        execution.fill(&mut jobs, |start, chunk| {
            let mut query = workspace(q.ne[0] as u64, 0.0f32)?;
            let mut key = workspace(k.ne[0] as u64, 0.0f32)?;
            let mut value = workspace(width, 0.0f32)?;
            let mut scores = workspace(width, 0.0f32)?;
            let mut coefficients = workspace(gate.ne[0] as u64, 0.0f32)?;
            let mut state = workspace(cells, 0.0f32)?;
            let mut scratch = workspace(capacity, 0.0f32)?;
            for stream in start..start + chunk.len() {
                let stream = stream as u64;
                let (head, batch) = (stream % heads, stream / heads);
                let query_start = if let Some(ids) = &ids {
                    let index =
                        u32::from_ne_bytes(ids.word(batch.checked_mul(4).ok_or(Overflow)?)?);
                    Request::C14(plan.clone(), head, batch, index.into())
                } else {
                    Request::C11(plan.clone(), head, batch)
                };
                let reply = request(query_start)?;
                tensor_fields!(Response::C13(start, empty_snapshot) = reply);
                initial.decode(start, &mut state)?;
                stage(empty_snapshot, &state);
                for token in 0..tokens {
                    let reply = request(Request::C12(plan.clone(), head, batch, token))?;
                    tensor_fields!(Response::C14(a, b, c, d, e, out, snapshot) = reply);
                    q.decode(a, &mut query)?;
                    k.decode(b, &mut key)?;
                    v.decode(c, &mut value)?;
                    gate.decode(d, &mut coefficients)?;
                    let rate = f32::from_ne_bytes(gain.word(e)?);
                    if selective {
                        let input = linear::SelectiveStateInput {
                            drive: &value,
                            time_step: rate,
                            transition: &coefficients,
                            injection: &query,
                            readout: &key,
                        };
                        linear::selective_state_step_f32(
                            input,
                            &mut state,
                            &mut scores,
                            &mut scratch,
                        )?;
                    } else {
                        let input = linear::GatedDeltaInput {
                            query: &query,
                            key: &key,
                            value: &value,
                            log_decay: &coefficients,
                            gain: rate,
                        };
                        linear::gated_delta_step_f32(input, &mut state, &mut scores, &mut scratch)?;
                    }
                    stage(Some(out), &scores);
                    stage(snapshot, &state);
                }
            }
            Ok(())
        })?;
        let output = graph_tensor_data(tensor)?;
        // SAFETY: publish only the certified prefix, after every index and every worker succeeds.
        unsafe { super::write_row(output, &staged) };
        Ok(())
    }

    pub(super) fn indexer(tensor: &Tensor, plan: IndexerPlan) -> Result<(), BackendError> {
        tensor_fields!([Some(query), Some(key), Some(weights), Some(mask)] = tensor.inputs()?);
        let offsets = |token, head, stream, key| {
            let query = Request::C10(plan.clone(), token, head, stream, key);
            tensor_fields!(Response::C11(q, k, w, m, out) = request(query)?);
            Ok([q, k, w, m, out])
        };
        let [width, heads, tokens, streams] = query.ne.map(|n| n as u64);
        let keys = key.ne[2] as u64;
        let matrix = |rows: u64, cols| workspace(rows.checked_mul(cols).ok_or(Overflow)?, 0.0_f32);
        let mut key_row = workspace(width, 0.0_f32)?;
        let mut transposed_keys = matrix(width, keys)?;
        let mut queries = matrix(heads, width)?;
        let mut dots = matrix(heads, keys)?;
        let mut head_weights = workspace(heads, 0.0_f32)?;
        let mut summed = workspace(keys, 0.0_f32)?;
        let mut bias = workspace(keys, 0.0_f32)?;
        let destination = graph_tensor_data(tensor)?;
        for stream in 0..streams {
            for column in 0..keys {
                let [_, at, ..] = offsets(0, 0, stream, column)?;
                key.decode(at, &mut key_row)?;
                for (row, &value) in transposed_keys
                    .chunks_exact_mut(keys as usize)
                    .zip(&key_row)
                {
                    row[column as usize] = value;
                }
            }
            for token in 0..tokens {
                let [_, _, w, m, out] = offsets(token, 0, stream, 0)?;
                weights.decode(w, &mut head_weights)?;
                mask.decode(m, &mut bias)?;
                for (head, row) in queries.chunks_exact_mut(width as usize).enumerate() {
                    let [q, ..] = offsets(token, head as u64, stream, 0)?;
                    query.decode(q, row)?;
                }
                multiply_matrix_rows_f32(
                    heads as usize,
                    width as usize,
                    keys as usize,
                    &queries,
                    &transposed_keys,
                    &mut dots,
                )?;
                for dot in &mut dots {
                    let word = linear::evaluate_float_unary(4, 0, 0, dot.to_bits().into())?
                        .ok_or(InvalidTensor)?;
                    *dot = f32::from_bits(word as u32);
                }
                multiply_matrix_rows_f32(
                    1,
                    heads as usize,
                    keys as usize,
                    &head_weights,
                    &dots,
                    &mut summed,
                )?;
                for (sum, &mask) in summed.iter_mut().zip(&bias) {
                    *sum = evaluate_f32_scalar(0, *sum, mask)?;
                }
                // SAFETY: Prism bounds this contiguous result row and excludes all source aliases.
                unsafe { super::write_row(destination.add(out as usize), &summed) };
            }
        }
        Ok(())
    }

    pub(super) fn sparse_mask(tensor: &Tensor, plan: SparsePlan) -> Result<(), BackendError> {
        use verdant_tensor::memory::copy_storage_block;
        let SparsePlan::C0(_, _, _, _, _, _, width, excluded, _) = plan;
        tensor_fields!([Some(mask), Some(top), score, blocks, extra] = tensor.inputs()?);
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
                    let reply = request(Request::C5(plan.clone(), token, head, batch))?;
                    tensor_fields!(
                        Response::C6(mask_row, top_row, score_row, extra_row, out) = reply
                    );
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
                            .map_err(BackendError::overflow)?
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
        let [mask, sink] = tensor.map_sources(1, TensorInput::new)?;
        let SoftmaxPlan::C0(_, _, _, scale, bias) = &plan;
        let scale = f32::from_bits(*scale as u32);
        let bias = bias.map(|bits| f32::from_bits(bits as u32));
        let mut row = workspace(tensor.ne[0] as u64, 0.0_f32)?;
        let mut mask_row = workspace(if mask.is_some() { row.len() as u64 } else { 0 }, 0.0_f32)?;
        let scalar =
            |operation, a, b| evaluate_f32_scalar(operation, a, b).map_err(BackendError::overflow);
        for head in 0..tensor.ne[2] as u64 {
            let slope = head_slope(bias, tensor.ne[2], head)?.unwrap_or(1.0);
            for batch in 0..tensor.ne[3] as u64 {
                for token in 0..tensor.ne[1] as u64 {
                    let reply = request(Request::C3(plan.clone(), token, head, batch))?;
                    tensor_fields!(Response::C4(from, moff, soff, to) = reply);
                    let bytes = float_bytes(&mut row);
                    // SAFETY: the admitted dense row fits the live input. Stage it before the in-place write.
                    let input = unsafe {
                        std::slice::from_raw_parts(source.add(from as usize), bytes.len())
                    };
                    bytes.copy_from_slice(input);
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
        let [a, b, c, d, e, rows, ids] =
            tensor.map_sources(0, |t| super::tensor_metadata(t).map(descriptor))?;
        let trailing = tensor.src[7..].iter().any(|p| !p.is_null());
        let [p0, p1, p2, p3, ..] = tensor.op_params.map(|w| u64::from(w as u32));
        let slots = [a, b, c, d, e, rows];
        let output = descriptor(output);
        let query = if let Some(ids) = ids {
            tensor_fields!([Some(a), Some(b), Some(c), Some(d), Some(e), Some(f)] = slots);
            Request::C13(op, trailing, p0, a, b, c, d, e, f, ids, output)
        } else {
            let [a, b, c, d, e, f] = slots;
            Request::C8(op, trailing, p0, p1, p2, p3, a, b, c, d, e, f, output)
        };
        Ok(Some(match request(query)? {
            Response::C1(p) => ExecutionPlan::Attention(Box::new(p)),
            Response::C3(p) => ExecutionPlan::Softmax(Box::new(p)),
            Response::C5(p) => ExecutionPlan::SparseMask(Box::new(p)),
            Response::C10(p) => ExecutionPlan::Indexer(Box::new(p)),
            Response::C12(p) => ExecutionPlan::Recurrent(Box::new(p)),
            _ => return Err(InvalidTensor),
        }))
    }

    impl TensorInput<'_> {
        pub(super) fn dot(
            &self,
            offset: u64,
            activations: &[f32],
            weights: &mut [f32],
            stages: Option<[u64; 4]>,
        ) -> Result<f32, BackendError> {
            self.decode(offset, weights)?;
            if let Some(stages) = stages {
                return super::rounded_dot(stages, weights, activations.iter());
            }
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
                float_bytes(out).copy_from_slice(bytes);
                return Ok(());
            }
            if !out.len().is_multiple_of(self.block_lanes) {
                return Err(InvalidTensor);
            }
            let mut at = offset;
            for (index, block) in out.chunks_exact_mut(self.block_lanes).enumerate() {
                if index > 0 {
                    at = at.checked_add(self.nb[0] as u64).ok_or(Overflow)?;
                }
                let encoded = self.region(at, self.block_bytes)?;
                if let Some(cast) = &self.scalar_read {
                    // SAFETY: the checked scalar slice contains one full stored word.
                    let raw = unsafe {
                        super::scalar_word(encoded.as_ptr().cast_mut(), encoded.len() as u64, None)?
                    };
                    block[0] = f32::from_bits(cast.apply(raw)? as u32);
                } else {
                    decode_block(
                        self.ty as u64,
                        encoded,
                        float_bytes(block),
                        size_of::<f32>(),
                    )?;
                }
            }
            Ok(())
        }
    }

    use super::BlockDot;
    use linear::{AffineQ8Block, SymmetricQ8Block};

    enum KeyRows {
        Scalar(Vec<f32>, [u64; 4]),
        Grouped(linear::GroupedWeightEncoding, Vec<linear::GroupedQ8Block>),
        Symmetric(BlockDot<SymmetricQ8Block<32>>, Vec<SymmetricQ8Block<32>>),
        Affine(BlockDot<AffineQ8Block<32>>, Vec<AffineQ8Block<32>>),
    }

    fn prepare_blocks<const LANES: usize, B>(
        query: &[f32],
        target: &mut Vec<B>,
        encode: impl Fn(&[f32; LANES]) -> Result<B, verdant_tensor::TensorComputeError>,
    ) -> Result<(), BackendError> {
        tensor_fields!((blocks, []) = query.as_chunks::<LANES>());
        target.clear();
        for block in blocks {
            target.push(encode(block)?);
        }
        Ok(())
    }

    pub(super) fn execute(tensor: &Tensor, plan: Plan) -> Result<(), BackendError> {
        tensor_fields!([Some(q), Some(k), Some(v), mask, sink, indices] = tensor.inputs()?);
        let shape = q.ne;
        let Plan::C0(.., columns, head_bias, cap) = &plan;
        let columns = usize::try_from(*columns).map_err(BackendError::overflow)?;
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
        use super::PackedWeight::{Affine, Grouped, Symmetric};
        let blocks = query.len() / 32;
        let mut keys = match super::packed_weight(k.ty as u64) {
            Some(Symmetric(dot)) => KeyRows::Symmetric(dot, super::reserved(blocks)?),
            Some(Affine(dot)) => KeyRows::Affine(dot, super::reserved(blocks)?),
            Some(Grouped(kind)) => KeyRows::Grouped(kind, super::reserved(query.len() / 256)?),
            None => {
                let response = request(Request::C15(k.ty as u64))?;
                tensor_fields!(Response::C15(product, widen, add, narrow) = response);
                let stages = [product, widen, add, narrow];
                KeyRows::Scalar(workspace(query.len() as u64, 0.0)?, stages)
            }
        };
        let mut value = workspace(tensor.ne[0] as u64, 0.0)?;
        let output = graph_tensor_data(tensor)?;
        let [cap, head_bias] = [cap, head_bias].map(|word| word.map(|b| f32::from_bits(b as u32)));
        let scalar = |op, a, b| evaluate_f32_scalar(op, a, b).map_err(BackendError::overflow);
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
                    let reply = request(Request::C1(plan.clone(), token, head, batch))?;
                    tensor_fields!(
                        Response::C2(qoff, koff, voff, moff, soff, dst, ks, vs, dot_type) = reply
                    );
                    q.decode(qoff, &mut query)?;
                    // GGML prepares the query in the key's dot-product storage once per head.
                    match (&mut keys, dot_type) {
                        (KeyRows::Symmetric(_, blocks), 8) => {
                            prepare_blocks(&query, blocks, linear::encode_symmetric_q8_block)?
                        }
                        (KeyRows::Affine(_, blocks), 9) => {
                            prepare_blocks(&query, blocks, linear::encode_affine_q8_block)?
                        }
                        (KeyRows::Grouped(_, blocks), 15) => {
                            prepare_blocks(&query, blocks, linear::encode_grouped_q8_block)?
                        }
                        (KeyRows::Scalar(..), 0) => {}
                        (KeyRows::Scalar(..), 30) => {
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
                        let packed = || k.blocks(key_offset, query.len());
                        let dot = match &mut keys {
                            KeyRows::Symmetric(dot, blocks) => dot(packed()?, blocks)?,
                            KeyRows::Affine(dot, blocks) => dot(packed()?, blocks)?,
                            KeyRows::Grouped(kind, blocks) => {
                                linear::dot_grouped_weight_row(packed()?, blocks, *kind)?
                            }
                            KeyRows::Scalar(key, stages) => {
                                k.dot(key_offset, &query, key, Some(*stages))?
                            }
                        };
                        v.decode(voff + row * vs, &mut value)?;
                        let mut scaled = scalar(2, dot, scale)?;
                        if let Some(cap) = cap {
                            scaled = scalar(2, cap, linear::tanh_f32(scaled)?)?;
                        }
                        let score = scalar(0, scaled, bias)?;
                        mean.push(score, Some(&value))
                            .map_err(BackendError::overflow)?;
                    }
                    if let Some(sink) = &sink {
                        mean.push(f32::from_ne_bytes(sink.word(soff)?), None)
                            .map_err(BackendError::overflow)?;
                    }
                    let row = mean.finish().map_err(BackendError::overflow)?;
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
        pub(super) threads: usize,
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
            .map_err(BackendError::overflow)? as usize;
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
    pub(super) unsafe fn execution(
        backend: BackendPtr,
        callbacks: bool,
    ) -> Result<Execution, BackendError> {
        // SAFETY: graph_compute retains its live backend for the entire submission.
        let mut settings = unsafe { &*(*backend).context.cast::<Context>() }
            .lock()
            .map_err(BackendError::invalid)?;
        // Unconfigured GGML CPU callbacks observe GGML_DEFAULT_N_THREADS.
        let default = match (callbacks, settings.attached.is_some()) {
            (true, _) => 4,
            (_, true) => usize::MAX,
            _ => 1,
        };
        let limit = settings.limit.unwrap_or(default);
        if settings.attached.is_none() && limit > 1 && settings.private.is_none() {
            settings.private = Some(Pool::create(ThreadpoolParams {
                n_threads: limit as c_int,
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
            threads = limit.min(active.workers.len() + 1);
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
