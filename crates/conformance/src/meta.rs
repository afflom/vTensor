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

#[repr(C)]
pub(crate) struct Tensor {
    pub ty: c_int,
    pub buffer: Buf,
    pub ne: [i64; 4],
    pub nb: [usize; 4],
    pub op: c_int,
    pub op_params: [i32; 16],
    pub flags: i32,
    pub src: [TensorPtr; 10],
    pub view_src: TensorPtr,
    pub view_offs: usize,
    pub data: *mut c_void,
    pub name: [c_char; 64],
    pub extra: *mut c_void,
    pub padding: [c_char; 8],
}

#[repr(C)]
#[derive(Default)]
pub(crate) struct BufferTypeInterface {
    pub get_name: Option<unsafe extern "C" fn(Buft) -> *const c_char>,
    pub alloc_buffer: Option<unsafe extern "C" fn(Buft, usize) -> Buf>,
    pub alloc_buffer_n: Option<unsafe extern "C" fn(Buft, *mut TensorPtr, c_int) -> Buf>,
    pub get_alignment: Option<unsafe extern "C" fn(Buft) -> usize>,
    pub get_max_size: Option<unsafe extern "C" fn(Buft) -> usize>,
    pub get_alloc_size: Option<unsafe extern "C" fn(Buft, *const Tensor) -> usize>,
    pub get_alloc_size_n: Option<unsafe extern "C" fn(Buft, *mut TensorPtr, c_int) -> usize>,
    pub is_host: Option<unsafe extern "C" fn(Buft) -> bool>,
}

#[repr(C)]
pub(crate) struct BufferType {
    pub iface: BufferTypeInterface,
    pub device: Dev,
    pub context: *mut c_void,
}

#[repr(C)]
#[derive(Default)]
pub(crate) struct BufferInterface {
    pub free_buffer: Option<unsafe extern "C" fn(Buf)>,
    pub get_base: Option<unsafe extern "C" fn(Buf) -> *mut c_void>,
    pub init_tensor: Option<unsafe extern "C" fn(Buf, TensorPtr) -> c_int>,
    pub memset_tensor: Option<unsafe extern "C" fn(Buf, TensorPtr, u8, usize, usize)>,
    pub set_tensor: Option<unsafe extern "C" fn(Buf, TensorPtr, *const c_void, usize, usize)>,
    pub get_tensor: Option<unsafe extern "C" fn(Buf, *const Tensor, *mut c_void, usize, usize)>,
    pub set_tensor_2d: Option<
        unsafe extern "C" fn(Buf, TensorPtr, *const c_void, usize, usize, usize, usize, usize),
    >,
    pub get_tensor_2d: Option<
        unsafe extern "C" fn(Buf, *const Tensor, *mut c_void, usize, usize, usize, usize, usize),
    >,
    pub cpy_tensor: Option<unsafe extern "C" fn(Buf, *const Tensor, TensorPtr) -> bool>,
    pub clear: Option<unsafe extern "C" fn(Buf, u8)>,
    pub reset: Option<unsafe extern "C" fn(Buf)>,
    pub set_tensor_n: Option<
        unsafe extern "C" fn(
            Buf,
            *const TensorPtr,
            *const *const c_void,
            *const usize,
            *const usize,
            usize,
        ),
    >,
    pub get_tensor_n: Option<
        unsafe extern "C" fn(
            Buf,
            *const *const Tensor,
            *const *mut c_void,
            *const usize,
            *const usize,
            usize,
        ),
    >,
    pub cpy_tensor_n:
        Option<unsafe extern "C" fn(Buf, *const *const Tensor, *const TensorPtr, usize) -> bool>,
}

#[repr(C)]
pub(crate) struct Buffer {
    pub iface: BufferInterface,
    pub buft: Buft,
    pub context: *mut c_void,
    pub size: usize,
    pub usage: c_int,
}

#[repr(C)]
#[derive(Default)]
pub(crate) struct BackendInterface {
    pub get_name: Option<unsafe extern "C" fn(BackendPtr) -> *const c_char>,
    pub free: Option<unsafe extern "C" fn(BackendPtr)>,
    pub set_tensor_async:
        Option<unsafe extern "C" fn(BackendPtr, TensorPtr, *const c_void, usize, usize)>,
    pub get_tensor_async:
        Option<unsafe extern "C" fn(BackendPtr, *const Tensor, *mut c_void, usize, usize)>,
    pub set_tensor_2d_async: Option<
        unsafe extern "C" fn(
            BackendPtr,
            TensorPtr,
            *const c_void,
            usize,
            usize,
            usize,
            usize,
            usize,
        ),
    >,
    pub get_tensor_2d_async: Option<
        unsafe extern "C" fn(
            BackendPtr,
            *const Tensor,
            *mut c_void,
            usize,
            usize,
            usize,
            usize,
            usize,
        ),
    >,
    pub cpy_tensor_async:
        Option<unsafe extern "C" fn(BackendPtr, BackendPtr, *const Tensor, TensorPtr) -> bool>,
    pub synchronize: Option<unsafe extern "C" fn(BackendPtr)>,
    pub graph_plan_create: Option<unsafe extern "C" fn(BackendPtr, *const c_void) -> Plan>,
    pub graph_plan_free: Option<unsafe extern "C" fn(BackendPtr, Plan)>,
    pub graph_plan_update: Option<unsafe extern "C" fn(BackendPtr, Plan, *const c_void)>,
    pub graph_plan_compute: Option<unsafe extern "C" fn(BackendPtr, Plan) -> c_int>,
    pub graph_compute: Option<unsafe extern "C" fn(BackendPtr, Graph) -> c_int>,
    pub event_record: Option<unsafe extern "C" fn(BackendPtr, Event)>,
    pub event_wait: Option<unsafe extern "C" fn(BackendPtr, Event)>,
    pub graph_optimize: Option<unsafe extern "C" fn(BackendPtr, Graph, *mut c_void)>,
}

#[repr(C)]
pub(crate) struct Backend {
    pub guid: *mut [u8; 16],
    pub iface: BackendInterface,
    pub device: Dev,
    pub context: *mut c_void,
}

#[repr(C)]
pub(crate) struct DeviceCapabilities {
    pub asynchronous: bool,
    pub host_buffer: bool,
    pub buffer_from_host_ptr: bool,
    pub events: bool,
    pub mmap_support: bool,
}

#[repr(C)]
pub(crate) struct DeviceProperties {
    pub name: *const c_char,
    pub description: *const c_char,
    pub memory_free: usize,
    pub memory_total: usize,
    pub ty: c_int,
    pub device_id: *const c_char,
    pub caps: DeviceCapabilities,
}

#[repr(C)]
#[derive(Default)]
pub(crate) struct DeviceInterface {
    pub get_name: Option<unsafe extern "C" fn(Dev) -> *const c_char>,
    pub get_description: Option<unsafe extern "C" fn(Dev) -> *const c_char>,
    pub get_memory: Option<unsafe extern "C" fn(Dev, *mut usize, *mut usize)>,
    pub get_type: Option<unsafe extern "C" fn(Dev) -> c_int>,
    pub get_props: Option<unsafe extern "C" fn(Dev, *mut DeviceProperties)>,
    pub init_backend: Option<unsafe extern "C" fn(Dev, *const c_char) -> BackendPtr>,
    pub get_buffer_type: Option<unsafe extern "C" fn(Dev) -> Buft>,
    pub get_host_buffer_type: Option<unsafe extern "C" fn(Dev) -> Buft>,
    pub buffer_from_host_ptr: Option<unsafe extern "C" fn(Dev, *mut c_void, usize, usize) -> Buf>,
    pub supports_op: Option<unsafe extern "C" fn(Dev, *const Tensor) -> bool>,
    pub supports_buft: Option<unsafe extern "C" fn(Dev, Buft) -> bool>,
    pub offload_op: Option<unsafe extern "C" fn(Dev, *const Tensor) -> bool>,
    pub event_new: Option<unsafe extern "C" fn(Dev) -> Event>,
    pub event_free: Option<unsafe extern "C" fn(Dev, Event)>,
    pub event_synchronize: Option<unsafe extern "C" fn(Dev, Event)>,
}

#[repr(C)]
pub(crate) struct Device {
    pub iface: DeviceInterface,
    pub reg: Reg,
    pub context: *mut c_void,
}

#[repr(C)]
#[derive(Default)]
pub(crate) struct RegistryInterface {
    pub get_name: Option<unsafe extern "C" fn(Reg) -> *const c_char>,
    pub get_device_count: Option<unsafe extern "C" fn(Reg) -> usize>,
    pub get_device: Option<unsafe extern "C" fn(Reg, usize) -> Dev>,
    pub get_proc_address: Option<unsafe extern "C" fn(Reg, *const c_char) -> *mut c_void>,
}

#[repr(C)]
pub(crate) struct Registry {
    pub api_version: c_int,
    pub iface: RegistryInterface,
    pub context: *mut c_void,
}

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
