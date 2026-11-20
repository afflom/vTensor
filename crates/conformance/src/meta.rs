//! GGML v3 ABI declarations, checked against the pinned authoritative headers.

use std::ffi::{c_char, c_int, c_void};

pub(crate) type Buf = *mut Buffer;
pub(crate) type Buft = *mut BufferType;
pub(crate) type Dev = *mut Device;
pub(crate) type Reg = *mut Registry;
pub(crate) type BackendPtr = *mut Backend;
pub(crate) type TensorPtr = *mut Tensor;
pub(crate) type Graph = *mut c_void;
pub(crate) type Plan = *mut c_void;
pub(crate) type Event = *mut c_void;

pub(crate) const GGML_STATUS_SUCCESS: c_int = 0;
pub(crate) const GGML_STATUS_FAILED: c_int = -1;

// Shared declarations preserve GGML field order and C layout.
macro_rules! record {
    ($(#[$attribute:meta])* $name:ident { $($field:ident: $ty:ty),* $(,)? }) => {
        $(#[$attribute])*
        #[repr(C)]
        pub(crate) struct $name { $(pub $field: $ty,)* }
    };
}
macro_rules! interface {
    ($name:ident { $($field:ident($($arg:ty),* $(,)?) -> $out:ty;)* }) => {
        record! { #[derive(Default)] $name {
            $($field: Option<unsafe extern "C" fn($($arg),*) -> $out>,)*
        } }
    };
}

record! { Tensor {
    ty: c_int,
    buffer: Buf,
    ne: [i64; 4],
    nb: [usize; 4],
    op: c_int,
    op_params: [i32; 16],
    flags: i32,
    src: [TensorPtr; 10],
    view_src: TensorPtr,
    view_offs: usize,
    data: *mut c_void,
    name: [c_char; 64],
    extra: *mut c_void,
    padding: [c_char; 8],
} }

interface! { BufferTypeInterface {
    get_name(Buft) -> *const c_char;
    alloc_buffer(Buft, usize) -> Buf;
    alloc_buffer_n(Buft, *mut TensorPtr, c_int) -> Buf;
    get_alignment(Buft) -> usize;
    get_max_size(Buft) -> usize;
    get_alloc_size(Buft, *const Tensor) -> usize;
    get_alloc_size_n(Buft, *mut TensorPtr, c_int) -> usize;
    is_host(Buft) -> bool;
} }

record! { BufferType {
    iface: BufferTypeInterface,
    device: Dev,
    context: *mut c_void,
} }

interface! { BufferInterface {
    free_buffer(Buf) -> ();
    get_base(Buf) -> *mut c_void;
    init_tensor(Buf, TensorPtr) -> c_int;
    memset_tensor(Buf, TensorPtr, u8, usize, usize) -> ();
    set_tensor(Buf, TensorPtr, *const c_void, usize, usize) -> ();
    get_tensor(Buf, *const Tensor, *mut c_void, usize, usize) -> ();
    set_tensor_2d(Buf, TensorPtr, *const c_void, usize, usize, usize, usize, usize) -> ();
    get_tensor_2d(Buf, *const Tensor, *mut c_void, usize, usize, usize, usize, usize) -> ();
    cpy_tensor(Buf, *const Tensor, TensorPtr) -> bool;
    clear(Buf, u8) -> ();
    reset(Buf) -> ();
    set_tensor_n(Buf, *const TensorPtr, *const *const c_void, *const usize, *const usize, usize) -> ();
    get_tensor_n(Buf, *const *const Tensor, *const *mut c_void, *const usize, *const usize, usize) -> ();
    cpy_tensor_n(Buf, *const *const Tensor, *const TensorPtr, usize) -> bool;
} }

record! { Buffer {
    iface: BufferInterface,
    buft: Buft,
    context: *mut c_void,
    size: usize,
    usage: c_int,
} }

interface! { BackendInterface {
    get_name(BackendPtr) -> *const c_char;
    free(BackendPtr) -> ();
    set_tensor_async(BackendPtr, TensorPtr, *const c_void, usize, usize) -> ();
    get_tensor_async(BackendPtr, *const Tensor, *mut c_void, usize, usize) -> ();
    set_tensor_2d_async(BackendPtr, TensorPtr, *const c_void, usize, usize, usize, usize, usize) -> ();
    get_tensor_2d_async(BackendPtr, *const Tensor, *mut c_void, usize, usize, usize, usize, usize) -> ();
    cpy_tensor_async(BackendPtr, BackendPtr, *const Tensor, TensorPtr) -> bool;
    synchronize(BackendPtr) -> ();
    graph_plan_create(BackendPtr, *const c_void) -> Plan;
    graph_plan_free(BackendPtr, Plan) -> ();
    graph_plan_update(BackendPtr, Plan, *const c_void) -> ();
    graph_plan_compute(BackendPtr, Plan) -> c_int;
    graph_compute(BackendPtr, Graph) -> c_int;
    event_record(BackendPtr, Event) -> ();
    event_wait(BackendPtr, Event) -> ();
    graph_optimize(BackendPtr, Graph, *mut c_void) -> ();
} }

record! { Backend {
    guid: *mut [u8; 16],
    iface: BackendInterface,
    device: Dev,
    context: *mut c_void,
} }

record! { DeviceCapabilities {
    asynchronous: bool,
    host_buffer: bool,
    buffer_from_host_ptr: bool,
    events: bool,
    mmap_support: bool,
} }

record! { DeviceProperties {
    name: *const c_char,
    description: *const c_char,
    memory_free: usize,
    memory_total: usize,
    ty: c_int,
    device_id: *const c_char,
    caps: DeviceCapabilities,
} }

interface! { DeviceInterface {
    get_name(Dev) -> *const c_char;
    get_description(Dev) -> *const c_char;
    get_memory(Dev, *mut usize, *mut usize) -> ();
    get_type(Dev) -> c_int;
    get_props(Dev, *mut DeviceProperties) -> ();
    init_backend(Dev, *const c_char) -> BackendPtr;
    get_buffer_type(Dev) -> Buft;
    get_host_buffer_type(Dev) -> Buft;
    buffer_from_host_ptr(Dev, *mut c_void, usize, usize) -> Buf;
    supports_op(Dev, *const Tensor) -> bool;
    supports_buft(Dev, Buft) -> bool;
    offload_op(Dev, *const Tensor) -> bool;
    event_new(Dev) -> Event;
    event_free(Dev, Event) -> ();
    event_synchronize(Dev, Event) -> ();
} }

record! { Device {
    iface: DeviceInterface,
    reg: Reg,
    context: *mut c_void,
} }

interface! { RegistryInterface {
    get_name(Reg) -> *const c_char;
    get_device_count(Reg) -> usize;
    get_device(Reg, usize) -> Dev;
    get_proc_address(Reg, *const c_char) -> *mut c_void;
} }

record! { Registry {
    api_version: c_int,
    iface: RegistryInterface,
    context: *mut c_void,
} }

unsafe extern "C" {
    pub(crate) fn ggml_backend_buffer_init(
        buft: Buft,
        iface: BufferInterface,
        context: *mut c_void,
        size: usize,
    ) -> Buf;
    pub(crate) fn ggml_backend_buffer_is_host(buffer: Buf) -> bool;
    pub(crate) fn ggml_graph_n_nodes(graph: Graph) -> c_int;
    pub(crate) fn ggml_graph_node(graph: Graph, index: c_int) -> TensorPtr;
    pub(crate) fn ggml_abort(file: *const c_char, line: c_int, fmt: *const c_char, ...) -> !;
}
