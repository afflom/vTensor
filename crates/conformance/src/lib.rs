//! Native GGML integration. Prism owns configuration and planning; Verdant owns tensor computation.

#[path = "meta.rs"]
mod abi;
#[path = "runner.rs"]
mod device;

pub use device::BackendError;

/// Return the process-lifetime vTensor GGML registry.
#[unsafe(no_mangle)]
pub extern "C" fn ggml_backend_vtensor_reg() -> *mut std::ffi::c_void {
    device::registry().cast()
}

/// GGML's dynamic backend loader entry point.
#[unsafe(no_mangle)]
pub extern "C" fn ggml_backend_init() -> *mut std::ffi::c_void {
    ggml_backend_vtensor_reg()
}

macro_rules! sources {
    ($($name:literal => $path:literal),* $(,)?) => {
        /// Inputs embedded by the compiler for native artifact freshness checks.
        pub const SOURCE_FILES: &[&str] = &[$($name),*];
        /// Exact compilation inputs, separated by their paths and NUL bytes.
        pub fn source_snapshot() -> &'static [u8] {
            concat!($($name, "\0", include_str!($path), "\0",)*).as_bytes()
        }
    };
}
sources! {
    "crates/conformance/src/lib.rs" => "lib.rs",
    "crates/conformance/src/meta.rs" => "meta.rs",
    "crates/conformance/src/runner.rs" => "runner.rs",
    "crates/conformance/Cargo.toml" => "../Cargo.toml",
    "Cargo.toml" => "../../../Cargo.toml",
    "Cargo.lock" => "../../../Cargo.lock",
    "model/verdant-development.json" => "../../../model/verdant-development.json",
    ".verdant-engine/quotient-kernel.lock" => "../../../.verdant-engine/quotient-kernel.lock",
    ".verdant-engine/crates/verdant-tensor/src/linear.rs" => "../../../.verdant-engine/crates/verdant-tensor/src/linear.rs",
    "native-kernel.lock" => "../../../native-kernel.lock",
    "matrix-kernel.lock" => "../../../matrix-kernel.lock",
    "index-kernel.lock" => "../../../index-kernel.lock",
    "pointwise-kernel.lock" => "../../../pointwise-kernel.lock",
    "layout-kernel.lock" => "../../../layout-kernel.lock",
    "prismpm.lock" => "../../../prismpm.lock",
    "oracle-import.lock" => "../../../oracle-import.lock",
    "vendor/llama.cpp-oracle/upstream.toml" => "../../../vendor/llama.cpp-oracle/upstream.toml",
}

/// Read-only native build provenance for verification hosts.
#[unsafe(no_mangle)]
pub extern "C" fn vtensor_build_sources() -> *const u8 {
    source_snapshot().as_ptr()
}

/// Byte length of the read-only native build provenance.
#[unsafe(no_mangle)]
pub extern "C" fn vtensor_build_sources_size() -> usize {
    source_snapshot().len()
}

/// Observed native layouts for independent comparison with upstream C headers.
/// These observations describe this compiled library; they are not ABI authority.
pub fn abi_layout() -> Vec<(&'static str, usize)> {
    use abi::*;
    let mut observations = Vec::new();
    macro_rules! record {
        ($ty:ty, $name:literal, $($field:ident => $key:literal),* $(,)?) => {
            observations.push((concat!($name, ".$size"), std::mem::size_of::<$ty>()));
            observations.push((concat!($name, ".$align"), std::mem::align_of::<$ty>()));
            $(observations.push((concat!($name, ".", $key), std::mem::offset_of!($ty, $field)));)*
        };
    }
    record!(Tensor, "ggml_tensor", ty=>"type", buffer=>"buffer", ne=>"ne", nb=>"nb", op=>"op", op_params=>"op_params", flags=>"flags", src=>"src", view_src=>"view_src", view_offs=>"view_offs", data=>"data", name=>"name", extra=>"extra", padding=>"padding");
    record!(BufferTypeInterface, "ggml_backend_buffer_type_i", get_name=>"get_name", alloc_buffer=>"alloc_buffer", alloc_buffer_n=>"alloc_buffer_n", get_alignment=>"get_alignment", get_max_size=>"get_max_size", get_alloc_size=>"get_alloc_size", get_alloc_size_n=>"get_alloc_size_n", is_host=>"is_host");
    record!(BufferType, "ggml_backend_buffer_type", iface=>"iface", device=>"device", context=>"context");
    record!(BufferInterface, "ggml_backend_buffer_i", free_buffer=>"free_buffer", get_base=>"get_base", init_tensor=>"init_tensor", memset_tensor=>"memset_tensor", set_tensor=>"set_tensor", get_tensor=>"get_tensor", set_tensor_2d=>"set_tensor_2d", get_tensor_2d=>"get_tensor_2d", cpy_tensor=>"cpy_tensor", clear=>"clear", reset=>"reset", set_tensor_n=>"set_tensor_n", get_tensor_n=>"get_tensor_n", cpy_tensor_n=>"cpy_tensor_n");
    record!(Buffer, "ggml_backend_buffer", iface=>"iface", buft=>"buft", context=>"context", size=>"size", usage=>"usage");
    record!(BackendInterface, "ggml_backend_i", get_name=>"get_name", free=>"free", set_tensor_async=>"set_tensor_async", get_tensor_async=>"get_tensor_async", set_tensor_2d_async=>"set_tensor_2d_async", get_tensor_2d_async=>"get_tensor_2d_async", cpy_tensor_async=>"cpy_tensor_async", synchronize=>"synchronize", graph_plan_create=>"graph_plan_create", graph_plan_free=>"graph_plan_free", graph_plan_update=>"graph_plan_update", graph_plan_compute=>"graph_plan_compute", graph_compute=>"graph_compute", event_record=>"event_record", event_wait=>"event_wait", graph_optimize=>"graph_optimize");
    record!(Backend, "ggml_backend", guid=>"guid", iface=>"iface", device=>"device", context=>"context");
    record!(DeviceCapabilities, "ggml_backend_dev_caps", asynchronous=>"async", host_buffer=>"host_buffer", buffer_from_host_ptr=>"buffer_from_host_ptr", events=>"events", mmap_support=>"mmap_support");
    record!(DeviceProperties, "ggml_backend_dev_props", name=>"name", description=>"description", memory_free=>"memory_free", memory_total=>"memory_total", ty=>"type", device_id=>"device_id", caps=>"caps");
    record!(DeviceInterface, "ggml_backend_device_i", get_name=>"get_name", get_description=>"get_description", get_memory=>"get_memory", get_type=>"get_type", get_props=>"get_props", init_backend=>"init_backend", get_buffer_type=>"get_buffer_type", get_host_buffer_type=>"get_host_buffer_type", buffer_from_host_ptr=>"buffer_from_host_ptr", supports_op=>"supports_op", supports_buft=>"supports_buft", offload_op=>"offload_op", event_new=>"event_new", event_free=>"event_free", event_synchronize=>"event_synchronize");
    record!(Device, "ggml_backend_device", iface=>"iface", reg=>"reg", context=>"context");
    record!(RegistryInterface, "ggml_backend_reg_i", get_name=>"get_name", get_device_count=>"get_device_count", get_device=>"get_device", get_proc_address=>"get_proc_address");
    record!(Registry, "ggml_backend_reg", api_version=>"api_version", iface=>"iface", context=>"context");
    observations
}
