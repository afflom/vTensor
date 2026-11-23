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
        /// Named compiler inputs used to validate native test artifacts against disk.
        pub const SOURCE_INPUTS: &[(&str, &[u8])] = &[$(($name, include_bytes!($path))),*];
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
    "native-kernel.lock" => "../../../native-kernel.lock",
    "matrix-kernel.lock" => "../../../matrix-kernel.lock",
    "index-kernel.lock" => "../../../index-kernel.lock",
    "norm-kernel.lock" => "../../../norm-kernel.lock",
    "attention-kernel.lock" => "../../../attention-kernel.lock",
    "rotary-kernel.lock" => "../../../rotary-kernel.lock",
    "typed-kernel.lock" => "../../../typed-kernel.lock",
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

pub use abi::abi_layout;
