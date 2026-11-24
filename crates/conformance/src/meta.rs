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

pub(crate) const GGML_MAX_SRC: usize = 10;
pub(crate) const GGML_STATUS_SUCCESS: c_int = 0;
pub(crate) const GGML_STATUS_FAILED: c_int = -1;

// Shared declarations preserve GGML field order and C layout.
macro_rules! record {
    (@field ty) => { "type" };
    (@field asynchronous) => { "async" };
    (@field $field:ident) => { stringify!($field) };
    ($(#[$attribute:meta])* $name:ident $ctype:literal { $($field:ident: $ty:ty),* $(,)? }) => {
        $(#[$attribute])*
        #[repr(C)]
        pub(crate) struct $name { $(pub $field: $ty,)* }
        impl $name {
            const LAYOUT: &'static [(&'static str, usize)] = &[
                (concat!($ctype, ".$size"), size_of::<Self>()),
                (concat!($ctype, ".$align"), align_of::<Self>()),
                $((concat!($ctype, ".", record!(@field $field)), std::mem::offset_of!(Self, $field)),)*
            ];
        }
    };
}
// The native provider and the header layout check use the same ordered callback slots.
macro_rules! interface {
    ($name:ident => $factory:ident $ctype:literal { $($field:ident($($arg:ty),* $(,)?) -> $out:ty => $binding:expr;)* }) => {
        record! { $name $ctype {
            $($field: Option<unsafe extern "C" fn($($arg),*) -> $out>,)*
        } }
        macro_rules! $factory {
            () => { $name { $($field: $binding,)* } };
        }
        pub(crate) use $factory;
    };
}

macro_rules! layouts {
    ($( $declare:ident! { $(#[$attr:meta])* $name:ident $(=> $factory:ident)? $ctype:literal { $($fields:tt)* } } )+) => {
        $($declare! { $(#[$attr])* $name $(=> $factory)? $ctype { $($fields)* } })+
        /// Observed native layouts for independent comparison with the imported C headers.
        pub fn abi_layout() -> Vec<(&'static str, usize)> { [$($name::LAYOUT),+].concat() }
    };
}

layouts! {
record! { Tensor "ggml_tensor" {
    ty: c_int,
    buffer: Buf,
    ne: [i64; 4],
    nb: [usize; 4],
    op: c_int,
    op_params: [i32; 16],
    flags: i32,
    src: [TensorPtr; GGML_MAX_SRC],
    view_src: TensorPtr,
    view_offs: usize,
    data: *mut c_void,
    name: [c_char; 64],
    extra: *mut c_void,
    padding: [c_char; 8],
} }

interface! { BufferTypeInterface => buffer_type_interface "ggml_backend_buffer_type_i" {
    get_name(Buft) -> *const c_char => Some(buft_name);
    alloc_buffer(Buft, usize) -> Buf => Some(buffer_allocate);
    alloc_buffer_n(Buft, *mut TensorPtr, c_int) -> Buf => None;
    get_alignment(Buft) -> usize => Some(alignment_of);
    get_max_size(Buft) -> usize => None;
    get_alloc_size(Buft, *const Tensor) -> usize => Some(allocation_size);
    get_alloc_size_n(Buft, *mut TensorPtr, c_int) -> usize => None;
    is_host(Buft) -> bool => Some(is_host);
} }

record! { BufferType "ggml_backend_buffer_type" {
    iface: BufferTypeInterface,
    device: Dev,
    context: *mut c_void,
} }

interface! { BufferInterface => buffer_interface "ggml_backend_buffer_i" {
    free_buffer(Buf) -> () => Some(buffer_free);
    get_base(Buf) -> *mut c_void => Some(buffer_base);
    init_tensor(Buf, TensorPtr) -> c_int => Some(tensor_init);
    memset_tensor(Buf, TensorPtr, u8, usize, usize) -> () => Some(tensor_memset);
    set_tensor(Buf, TensorPtr, *const c_void, usize, usize) -> () => Some(tensor_set);
    get_tensor(Buf, *const Tensor, *mut c_void, usize, usize) -> () => Some(tensor_get);
    set_tensor_2d(Buf, TensorPtr, *const c_void, usize, usize, usize, usize, usize) -> () => None;
    get_tensor_2d(Buf, *const Tensor, *mut c_void, usize, usize, usize, usize, usize) -> () => None;
    cpy_tensor(Buf, *const Tensor, TensorPtr) -> bool => Some(tensor_copy);
    clear(Buf, u8) -> () => Some(buffer_clear);
    reset(Buf) -> () => None;
    set_tensor_n(Buf, *const TensorPtr, *const *const c_void, *const usize, *const usize, usize) -> () => None;
    get_tensor_n(Buf, *const *const Tensor, *const *mut c_void, *const usize, *const usize, usize) -> () => None;
    cpy_tensor_n(Buf, *const *const Tensor, *const TensorPtr, usize) -> bool => None;
} }

record! { Buffer "ggml_backend_buffer" {
    iface: BufferInterface,
    buft: Buft,
    context: *mut c_void,
    size: usize,
    usage: c_int,
} }

interface! { BackendInterface => backend_interface "ggml_backend_i" {
    get_name(BackendPtr) -> *const c_char => Some(backend_name);
    free(BackendPtr) -> () => Some(backend_free);
    set_tensor_async(BackendPtr, TensorPtr, *const c_void, usize, usize) -> () => None;
    get_tensor_async(BackendPtr, *const Tensor, *mut c_void, usize, usize) -> () => None;
    set_tensor_2d_async(BackendPtr, TensorPtr, *const c_void, usize, usize, usize, usize, usize) -> () => None;
    get_tensor_2d_async(BackendPtr, *const Tensor, *mut c_void, usize, usize, usize, usize, usize) -> () => None;
    cpy_tensor_async(BackendPtr, BackendPtr, *const Tensor, TensorPtr) -> bool => None;
    synchronize(BackendPtr) -> () => None;
    graph_plan_create(BackendPtr, *const c_void) -> Plan => Some(graph_plan_create);
    graph_plan_free(BackendPtr, Plan) -> () => Some(graph_plan_free);
    graph_plan_update(BackendPtr, Plan, *const c_void) -> () => None;
    graph_plan_compute(BackendPtr, Plan) -> c_int => Some(graph_plan_compute);
    graph_compute(BackendPtr, Graph) -> c_int => Some(graph_compute);
    event_record(BackendPtr, Event) -> () => None;
    event_wait(BackendPtr, Event) -> () => None;
    graph_optimize(BackendPtr, Graph, *mut c_void) -> () => None;
} }

record! { Backend "ggml_backend" {
    guid: *mut [u8; 16],
    iface: BackendInterface,
    device: Dev,
    context: *mut c_void,
} }

record! { DeviceCapabilities "ggml_backend_dev_caps" {
    asynchronous: bool,
    host_buffer: bool,
    buffer_from_host_ptr: bool,
    events: bool,
    mmap_support: bool,
} }

record! { DeviceProperties "ggml_backend_dev_props" {
    name: *const c_char,
    description: *const c_char,
    memory_free: usize,
    memory_total: usize,
    ty: c_int,
    device_id: *const c_char,
    caps: DeviceCapabilities,
} }

interface! { DeviceInterface => device_interface "ggml_backend_device_i" {
    get_name(Dev) -> *const c_char => Some(device_name);
    get_description(Dev) -> *const c_char => Some(description_name);
    get_memory(Dev, *mut usize, *mut usize) -> () => Some(memory);
    get_type(Dev) -> c_int => Some(device_type);
    get_props(Dev, *mut DeviceProperties) -> () => Some(properties);
    init_backend(Dev, *const c_char) -> BackendPtr => Some(backend_new);
    get_buffer_type(Dev) -> Buft => Some(buffer_type);
    get_host_buffer_type(Dev) -> Buft => None;
    buffer_from_host_ptr(Dev, *mut c_void, usize, usize) -> Buf => Some(from_host);
    supports_op(Dev, *const Tensor) -> bool => Some(supports_operation);
    supports_buft(Dev, Buft) -> bool => Some(supports_buffer_type);
    offload_op(Dev, *const Tensor) -> bool => None;
    event_new(Dev) -> Event => None;
    event_free(Dev, Event) -> () => None;
    event_synchronize(Dev, Event) -> () => None;
} }

record! { Device "ggml_backend_device" {
    iface: DeviceInterface,
    reg: Reg,
    context: *mut c_void,
} }

record! { #[derive(Clone)] ThreadpoolParams "ggml_threadpool_params" {
    cpumask: [bool; 512],
    n_threads: c_int,
    prio: c_int,
    poll: u32,
    strict_cpu: bool,
    paused: bool,
} }

interface! { RegistryInterface => registry_interface "ggml_backend_reg_i" {
    get_name(Reg) -> *const c_char => Some(reg_name);
    get_device_count(Reg) -> usize => Some(device_count);
    get_device(Reg, usize) -> Dev => Some(device_at);
    get_proc_address(Reg, *const c_char) -> *mut c_void => Some(threadpool::lookup);
} }

record! { Registry "ggml_backend_reg" {
    api_version: c_int,
    iface: RegistryInterface,
    context: *mut c_void,
} }
}

unsafe extern "C" {
    pub(crate) fn ggml_backend_buffer_init(
        buft: Buft,
        iface: BufferInterface,
        context: *mut c_void,
        size: usize,
    ) -> Buf;
    pub(crate) fn ggml_backend_buft_is_host(buft: Buft) -> bool;
    pub(crate) fn ggml_graph_n_nodes(graph: Graph) -> c_int;
    pub(crate) fn ggml_graph_nodes(graph: Graph) -> *const TensorPtr;
    pub(crate) fn ggml_abort(file: *const c_char, line: c_int, fmt: *const c_char, ...) -> !;
}
