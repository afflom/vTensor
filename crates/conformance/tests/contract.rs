//! The `contract` suite: one test per scenario in `features/suites/contract.feature`.
//!
//! The proofs are checked by the Lean kernel when `just vv` runs
//! `prismpm verify` on `app/`. What this test holds fixed is the claim those
//! proofs are about: the theorem's exact statement and axioms, the six
//! universal policy files they quantify over, and that the verdict the
//! application returns is decided by the function they prove. A weakened
//! statement, a dropped file, an extra axiom, or a verdict decided some other
//! way would still verify, and would no longer be the claim `TC-01` makes.

use std::path::PathBuf;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crates/conformance is two below the root")
        .to_path_buf()
}

/// The body of the top-level declaration named `name`, up to its closing brace.
fn declaration<'a>(model: &'a str, name: &str) -> &'a str {
    let start = model
        .find(&format!("\n  name: {name},\n"))
        .unwrap_or_else(|| panic!("{name} is not declared"));
    let end = model[start..]
        .find("\n}\n")
        .unwrap_or_else(|| panic!("{name} is not closed"));
    &model[start..start + end]
}

/// `TC-01`: The checker answers conformant exactly when every universal policy file's observed digest equals its locked digest
#[test]
fn contract_tc_01() {
    let model = std::fs::read_to_string(root().join("app/src/Contract.lex.tex"))
        .expect("the contract model");

    let decides = declaration(&model, "conformant_decides");
    assert!(
        decides.contains(
            "statement: (((conformant observation) = true) <-> (Conformant observation)),"
        ),
        "TC-01's statement changed: conformant must be true exactly when Conformant holds:\n{decides}"
    );
    assert!(
        decides.contains("parameters: [observation : Observation],")
            && decides.contains("axioms: [propext]"),
        "TC-01 quantifies over every observation and rests on propext only:\n{decides}"
    );

    // The proposition names every universal policy file whose digest
    // `template.lock` binds: all of them but the self-referential lock.
    let contract =
        std::fs::read_to_string(root().join("template-contract.json")).expect("contract");
    let list = contract
        .split("\"universal_policy_paths\":[")
        .nth(1)
        .and_then(|rest| rest.split(']').next())
        .expect("universal_policy_paths");
    let bound: Vec<&str> = list
        .split(',')
        .map(|p| p.trim_matches('"'))
        .filter(|p| *p != "template.lock")
        .collect();
    let proposition = declaration(&model, "Conformant");
    let fields = [
        "devcontainer",
        "bootstrap",
        "agents",
        "verification",
        "prismpmLock",
        "templateContract",
    ];
    assert_eq!(bound.len(), fields.len(), "the contract binds {bound:?}");
    let drifted = declaration(&model, "drifted");
    for (field, path) in fields.iter().zip(&bound) {
        assert!(
            proposition.contains(&format!(
                "(SameDigest (project (project observation {field}) expected) (project (project observation {field}) observed))"
            )),
            "Conformant must require {path} ({field}) to match"
        );
        assert!(
            drifted.contains(&format!("(driftEntry (project observation {field})"))
                && drifted.contains(&format!("\" {path}\"")),
            "a drifted {path} must be named in the verdict"
        );
    }

    // A digest is four 64-bit words, the widest a Nat is compiled to, and
    // two digests are the same only when every word is.
    let same = declaration(&model, "SameDigest");
    for word in ["w0", "w1", "w2", "w3"] {
        assert!(
            same.contains(&format!("((project left {word}) = (project right {word}))")),
            "SameDigest must compare digest word {word}"
        );
    }

    let dispatch = declaration(&model, "dispatch");
    assert!(
        dispatch.contains(
            "(if ((BackendCore::supportsF64Matmul4 true true true) && (conformant (observe value))) then"
        ),
        "dispatch must require both the Prism-supported matrix case and the conformant verdict TC-01 proves"
    );
}

/// `VT-10`: Prism tensor placement matches the imported GGML linear allocator for aligned buffers and refuses invalid alignment, exhausted capacity, and arithmetic overflow
#[test]
fn buffer_allocation_vt_10() {
    tensor_layout::verify_bundle();
    let mut input = String::new();
    let mut cases = Vec::new();
    use std::fmt::Write;
    for alignment in [1_u64, 2, 4, 8, 16, 64, 256, 1024] {
        for offset in [0, alignment, alignment * 3] {
            for capacity in [0, 1, alignment - 1, alignment, alignment + 1, alignment * 4] {
                for size in [
                    0,
                    1,
                    alignment - 1,
                    alignment,
                    alignment + 1,
                    alignment * 2 + 1,
                ] {
                    writeln!(input, "{capacity} {offset} {size} {alignment}").unwrap();
                    cases.push((capacity, offset, size, alignment));
                }
            }
        }
    }
    let reference = ggml_oracle::placements(&root(), &input);
    assert_eq!(reference.len(), cases.len());
    fn outcome(decision: vtensor_allocation::Adt0) -> [u64; 4] {
        match decision {
            vtensor_allocation::Adt0::C2 => [1, 0, 0, 0],
            vtensor_allocation::Adt0::C3(offset, padded, next) => [0, offset, padded, next],
            _ => panic!("valid alignment and offset refused"),
        }
    }
    for (index, ((capacity, offset, size, alignment), expected)) in
        cases.into_iter().zip(reference).enumerate()
    {
        assert_eq!(outcome(vtensor_allocation::run(capacity, offset, size, alignment).unwrap()), expected, "placement {index}: capacity={capacity}, offset={offset}, size={size}, alignment={alignment}");
    }
    for alignment in [0, 3, 6, 63, u64::MAX] {
        assert!(matches!(
            vtensor_allocation::run(64, 0, 1, alignment),
            Ok(vtensor_allocation::Adt0::C0)
        ));
    }
    assert!(matches!(
        vtensor_allocation::run(64, 1, 1, 8),
        Ok(vtensor_allocation::Adt0::C1)
    ));
    assert!(vtensor_allocation::run(u64::MAX, 0, u64::MAX, 2).is_err());
    for ((capacity, offset, size, alignment), expected) in [
        ((u64::MAX, u64::MAX, 0, 1), [0, u64::MAX, 0, u64::MAX]),
        ((u64::MAX, u64::MAX, 1, 1), [1, 0, 0, 0]),
        (
            (u64::MAX, 0, 1, 1_u64 << 63),
            [0, 0, 1_u64 << 63, 1_u64 << 63],
        ),
    ] {
        let decision = vtensor_allocation::run(capacity, offset, size, alignment).unwrap();
        assert_eq!(outcome(decision), expected);
    }
}

/// `VT-11`: GGML discovers the Prism-defined vTensor device and its owned buffers preserve upstream allocation and byte-transfer behavior
#[test]
fn native_backend_vt_11() {
    tensor_layout::verify_bundle();
    tensor_layout::certified_current_model(&root(), "native");
    let inventory = ggml_oracle::read_toml(root().join("model/ggml-declarations.toml"));
    for row in inventory["operation"].as_array().unwrap() {
        let expected = matches!(
            row["symbol"].as_str().unwrap(),
            "GGML_OP_NONE"
                | "GGML_OP_RESHAPE"
                | "GGML_OP_VIEW"
                | "GGML_OP_PERMUTE"
                | "GGML_OP_TRANSPOSE"
        );
        assert_eq!(
            vtensor_native_predicates::run(vtensor_native_predicates::Adt0::C1(
                row["value"].as_integer().unwrap() as u64,
                1
            )),
            expected
        );
    }
    native_backend_oracle::verify(&root(), "buffers");
}

/// `VT-12`: GGML F32 matrix-product graphs execute through Verdant across rectangular shapes, padded storage and batch broadcasting and match the imported GGML CPU reference
#[test]
fn matrix_graphs_vt_12() {
    tensor_layout::certified_current_model(&root(), "matrix");
    tensor_layout::certified_current_model(&root(), "index");
    let inventory = ggml_oracle::read_toml(root().join("model/ggml-declarations.toml"));
    for row in inventory["operation"].as_array().unwrap() {
        let decision = vtensor_matrix::run(
            row["value"].as_integer().unwrap() as u64,
            0,
            0,
            0,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
        )
        .unwrap();
        assert_eq!(
            matches!(decision, vtensor_matrix::Adt0::C2(..)),
            row["symbol"].as_str() == Some("GGML_OP_MUL_MAT")
        );
    }
    assert!(matches!(
        vtensor_matrix::run(28, 0, 0, 0, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1),
        Ok(vtensor_matrix::Adt0::C1)
    ));
    assert!(matches!(
        vtensor_matrix::run(28, 0, 0, 0, 1, 1, 2, 1, 1, 1, 3, 1, 1, 1, 3, 1),
        Ok(vtensor_matrix::Adt0::C1)
    ));
    assert!(vtensor_index::run(u64::MAX, 0, 0, 0, 2, 0, 0, 0).is_err());
    native_backend_oracle::verify(&root(), "matrix");
}

/// `VT-13`: F32 arithmetic and unary graphs execute through Verdant with Prism-defined scalar semantics and broadcasting and match the imported GGML CPU reference
#[test]
fn elementwise_graphs_vt_13() {
    tensor_layout::certified_current_model(&root(), "pointwise");
    native_backend_oracle::verify(&root(), "elementwise");
}

/// `VT-14`: Prism empty-node admission matches the imported GGML CPU no-op behavior for empty copy graphs across every live storage type without accessing operand buffers
#[test]
fn empty_graphs_vt_14() {
    native_backend_oracle::verify(&root(), "empty");
}

/// `VT-15`: Prism plans same-type DUP, CONT and CPY across every live GGML storage type and Verdant moves the admitted blocks with upstream-equivalent logical order and preserved padding
#[test]
fn copy_graphs_vt_15() {
    native_backend_oracle::verify(&root(), "copy");
}

/// `VT-16`: Prism models four-axis REPEAT addressing and Verdant preserves encoded values for every type admitted by the imported GGML CPU repeat oracle
#[test]
fn repeat_graphs_vt_16() {
    native_backend_oracle::verify(&root(), "repeat");
}

/// `VT-17`: Prism selects and addresses CONCAT source blocks on every axis and Verdant execution matches the imported GGML CPU oracle across all live storage types
#[test]
fn concat_graphs_vt_17() {
    native_backend_oracle::verify(&root(), "concat");
}

/// `VT-18`: Prism defines F32 zero and circular padding, reflected padding and signed four-axis roll addressing and Verdant execution matches the imported GGML CPU oracle
#[test]
fn spatial_graphs_vt_18() {
    native_backend_oracle::verify(&root(), "spatial");
}

mod ggml_oracle {

    use sha2::{Digest, Sha256};
    use std::io::Write;
    use std::path::{Path, PathBuf};
    use std::process::{Command, Stdio};

    pub(super) fn read_json(path: impl AsRef<Path>) -> serde_json::Value {
        let path = path.as_ref();
        let bytes =
            std::fs::read(path).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
        serde_json::from_slice(&bytes).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
    }

    pub(super) fn read_toml(path: impl AsRef<Path>) -> toml::Value {
        let path = path.as_ref();
        let text = std::fs::read_to_string(path)
            .unwrap_or_else(|error| panic!("{}: {error}", path.display()));
        toml::from_str(&text).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
    }

    pub(super) fn verify_file(path: impl AsRef<Path>, stamp: impl AsRef<Path>) {
        assert_eq!(file_digest(path), std::fs::read_to_string(stamp).unwrap());
    }

    pub(super) fn file_digest(path: impl AsRef<Path>) -> String {
        let path = path.as_ref();
        digest(&Sha256::digest(
            std::fs::read(path).unwrap_or_else(|error| panic!("{}: {error}", path.display())),
        ))
    }

    pub(super) fn digest(bytes: &[u8]) -> String {
        use std::fmt::Write;
        bytes.iter().fold(String::new(), |mut out, byte| {
            write!(&mut out, "{byte:02x}").expect("string write");
            out
        })
    }

    pub(super) fn compile(root: &Path, source: impl AsRef<Path>, output: impl AsRef<Path>) {
        let source = source.as_ref();
        let c = source.extension().is_some_and(|extension| extension == "c");
        let mut command = Command::new(if c { "cc" } else { "c++" });
        command.args(["-O2", "-ffunction-sections", "-fdata-sections"]);
        command.args(if c {
            &["-std=c11", "-D_GNU_SOURCE"][..]
        } else {
            &["-std=c++17"][..]
        });
        for directory in ["ggml/include", "ggml/src", "ggml/src/ggml-cpu", ""] {
            command.arg("-I").arg(root.join(directory));
        }
        run(command.arg("-c").arg(source).arg("-o").arg(output.as_ref()));
    }

    pub(super) fn run(command: &mut Command) -> Vec<u8> {
        let result = command
            .output()
            .expect("oracle build tool available in SDK");
        assert!(
            result.status.success(),
            "{command:?}\n{}\n{}",
            String::from_utf8_lossy(&result.stdout),
            String::from_utf8_lossy(&result.stderr)
        );
        result.stdout
    }

    pub(super) fn source_tokens(body: &str) -> Vec<u8> {
        // The extracted body retains its closing brace. Tokenization preserves
        // string literals and code changes while ignoring formatting/comments.
        format!("{{{body}")
            .parse::<proc_macro2::TokenStream>()
            .unwrap()
            .to_string()
            .into_bytes()
    }

    fn verify_source_cache(root: &Path, cache: &Path) {
        let archive = root.join("vendor/llama.cpp-oracle/source.tar.gz");
        let listing =
            String::from_utf8(run(Command::new("tar").arg("-tvzf").arg(&archive))).unwrap();
        let mut inputs = Vec::new();
        for line in listing.lines() {
            let fields: Vec<_> = line.split_whitespace().collect();
            let Some(&name) = fields.get(5) else {
                panic!("unrecognized archive listing: {line}");
            };
            let relevant = name.starts_with("ggml/include/")
                || name.starts_with("ggml/src/ggml-cpu/")
                || name
                    .strip_prefix("ggml/src/")
                    .is_some_and(|tail| !tail.contains('/'));
            if relevant && !name.ends_with('/') {
                assert!(
                    line.starts_with('-') && fields.len() == 6,
                    "non-regular oracle input: {line}"
                );
                inputs.push((name.to_string(), fields[2].parse::<usize>().unwrap()));
            }
        }
        assert!(
            inputs.len() > 100,
            "upstream source inventory was truncated"
        );
        let mut extract = Command::new("tar");
        extract
            .arg("-xOzf")
            .arg(archive)
            .args(inputs.iter().map(|(name, _)| name));
        let authoritative = Sha256::digest(run(&mut extract));
        let mut cached = Sha256::new();
        for (name, length) in inputs {
            let bytes = std::fs::read(cache.join(&name)).unwrap();
            assert_eq!(
                bytes.len(),
                length,
                "cached upstream source differs: {name}"
            );
            cached.update(bytes);
        }
        assert_eq!(
            cached.finalize().as_slice(),
            authoritative.as_slice(),
            "cached GGML reference input differs from the authoritative archive"
        );
    }

    /// Build the oracle once per source, compiler, SDK and build-recipe identity.
    /// A cache hit still verifies the executable digest and executes the oracle.
    pub fn storage_oracle(root: &Path) -> PathBuf {
        static ORACLE: std::sync::OnceLock<PathBuf> = std::sync::OnceLock::new();
        ORACLE.get_or_init(|| build_storage_oracle(root)).clone()
    }

    fn build_storage_oracle(root: &Path) -> PathBuf {
        let imported = root.join("vendor/llama.cpp-oracle");
        let lock = read_toml(imported.join("upstream.toml"));
        let archive =
            std::fs::read(imported.join("source.tar.gz")).expect("upstream source archive");
        let archive_digest = digest(&Sha256::digest(&archive));
        assert_eq!(
            archive_digest,
            lock["archive"][0]["sha256"].as_str().expect("archive pin")
        );
        let mut identity = Sha256::new();
        for input in [
            archive_digest.as_bytes(),
            STORAGE_ORACLE_SOURCE,
            &source_tokens(
                include_str!("contract.rs")
                    .split_once("\nmod ggml_oracle {")
                    .expect("oracle module")
                    .1
                    .split_once("\nmod ggml_inventory {")
                    .expect("module boundary")
                    .0,
            ),
            &run(Command::new("cc").arg("--version")),
            &run(Command::new("c++").arg("--version")),
        ] {
            identity.update((input.len() as u64).to_le_bytes());
            identity.update(input);
        }
        if let Some(inventory) = std::env::var_os("PRISMPM_SDK_INVENTORY") {
            identity.update(std::fs::read(inventory).expect("running SDK inventory"));
        } else {
            identity.update(b"host-development-only");
        }
        identity.update(std::env::consts::ARCH.as_bytes());
        let cache = std::env::var_os("CARGO_TARGET_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|| std::env::temp_dir().join("vtensor-native"))
            .join("ggml-oracle")
            .join(digest(&identity.finalize()));
        let executable = cache.join("storage-oracle");
        if executable.exists() {
            verify_file(&executable, cache.join("executable.sha256"));
            verify_source_cache(root, &cache);
            return executable;
        }
        std::fs::create_dir_all(&cache).expect("oracle cache directory");
        run(Command::new("tar")
            .args(["-xzf"])
            .arg(imported.join("source.tar.gz"))
            .arg("-C")
            .arg(&cache));
        verify_source_cache(root, &cache);
        let source = cache.join("ggml/src");
        let version = std::fs::read_to_string(source.join("ggml-version.h.in"))
            .expect("upstream version template")
            .replace("@GGML_VERSION@", "1.0.0")
            .replace(
                "@GGML_BUILD_COMMIT@",
                lock["commit"].as_str().expect("upstream revision"),
            );
        std::fs::write(cache.join("ggml-version.h"), version).expect("version header");
        std::fs::write(cache.join("storage_oracle.c"), STORAGE_ORACLE_SOURCE)
            .expect("oracle transport");
        for (input, object) in [
            (source.join("ggml.c"), "ggml.o"),
            (source.join("ggml-quants.c"), "quants.o"),
            (source.join("ggml-alloc.c"), "alloc.o"),
            (cache.join("storage_oracle.c"), "transport.o"),
        ] {
            compile(&cache, input, cache.join(object));
        }
        std::fs::write(cache.join("transfer.cpp"), TRANSFER_ORACLE_SOURCE)
            .expect("transfer transport");
        std::fs::write(cache.join("allocation.cpp"), ALLOCATION_ORACLE_SOURCE)
            .expect("allocation transport");
        for (input, object) in [
            (cache.join("allocation.cpp"), "allocation.o"),
            (source.join("ggml-backend.cpp"), "backend.o"),
            (source.join("ggml-backend-meta.cpp"), "meta.o"),
            (cache.join("transfer.cpp"), "transfer.o"),
        ] {
            compile(&cache, input, cache.join(object));
        }
        run(Command::new("c++")
            .arg("-Wl,--gc-sections")
            .arg(cache.join("transfer.o"))
            .arg(cache.join("backend.o"))
            .arg(cache.join("ggml.o"))
            .arg(cache.join("quants.o"))
            .arg("-lm")
            .arg("-o")
            .arg(cache.join("transfer-oracle")));
        std::fs::write(
            cache.join("transfer.sha256"),
            file_digest(cache.join("transfer-oracle")),
        )
        .expect("transfer digest");
        run(Command::new("c++")
            .arg("-Wl,--gc-sections")
            .arg(cache.join("allocation.o"))
            .arg(cache.join("meta.o"))
            .arg(cache.join("alloc.o"))
            .arg(cache.join("backend.o"))
            .arg(cache.join("ggml.o"))
            .arg(cache.join("quants.o"))
            .arg("-lm")
            .arg("-o")
            .arg(cache.join("allocation-oracle")));
        std::fs::write(
            cache.join("allocation.sha256"),
            file_digest(cache.join("allocation-oracle")),
        )
        .expect("allocation digest");
        let objects: serde_json::Map<String, serde_json::Value> =
            ["ggml.o", "quants.o", "alloc.o", "backend.o", "meta.o"]
                .into_iter()
                .map(|name| {
                    (
                        name.to_string(),
                        serde_json::json!(file_digest(cache.join(name))),
                    )
                })
                .collect();
        std::fs::write(
            cache.join("objects.json"),
            serde_json::to_vec(&objects).unwrap(),
        )
        .expect("object bindings");
        let candidate = cache.join("storage-oracle.new");
        run(Command::new("c++")
            .arg("-Wl,--gc-sections")
            .arg(cache.join("transport.o"))
            .arg(cache.join("ggml.o"))
            .arg(cache.join("quants.o"))
            .arg("-lm")
            .arg("-o")
            .arg(&candidate));
        std::fs::write(cache.join("executable.sha256"), file_digest(&candidate))
            .expect("cache digest");
        std::fs::rename(candidate, &executable).expect("publish built oracle");
        executable
    }

    /// Send a batch of tensor descriptions to the actual upstream `ggml_nbytes`.
    pub(super) fn batch(command: &mut Command, input: &str) -> String {
        let mut child = command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("native GGML oracle");
        let mut stdin = child.stdin.take().expect("oracle input");
        // Drain output while writing so large differential batches cannot fill
        // both pipe buffers and block each other.
        let output = std::thread::scope(|scope| {
            let writer = scope.spawn(move || stdin.write_all(input.as_bytes()));
            let output = child.wait_with_output().expect("oracle result");
            writer.join().expect("input writer").expect("write input");
            output
        });
        assert!(
            output.status.success(),
            "{command:?}: {:?}\n{}\n{}",
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8(output.stdout).expect("oracle text")
    }

    /// Query the upstream storage-size functions in one batch.
    pub fn spans(executable: &Path, input: &str) -> Vec<[u64; 3]> {
        batch(&mut Command::new(executable), input)
            .lines()
            .map(|line| {
                let values = line
                    .split_whitespace()
                    .map(|v| v.parse::<u64>().expect("oracle integer"))
                    .collect::<Vec<_>>();
                values.try_into().expect("span, block size, type size")
            })
            .collect()
    }

    /// Query upstream set/get guards without executing tensor computations.
    pub fn transfers(root: &Path, input: &str) -> Vec<u64> {
        let base = storage_oracle(root);
        let executable = base.with_file_name("transfer-oracle");
        verify_file(&executable, base.with_file_name("transfer.sha256"));
        batch(&mut Command::new(&executable), input)
            .lines()
            .map(|line| line.parse().expect("transfer outcome"))
            .collect()
    }

    pub fn placements(root: &Path, input: &str) -> Vec<[u64; 4]> {
        let base = storage_oracle(root);
        let executable = base.with_file_name("allocation-oracle");
        verify_file(&executable, base.with_file_name("allocation.sha256"));
        batch(&mut Command::new(&executable), input)
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|v| v.parse().expect("placement integer"))
                    .collect::<Vec<_>>()
                    .try_into()
                    .expect("placement outcome")
            })
            .collect()
    }

    const ALLOCATION_ORACLE_SOURCE: &[u8] = br##"#include "ggml.h"
#include "ggml-alloc.h"
#include "ggml-backend.h"
#include "ggml-backend-impl.h"
#include <cstdio>
#include <cstdint>
#include <cstring>
#include <memory>
#include <vector>
#include <sys/wait.h>
#include <unistd.h>
static size_t alignment;
static size_t get_alignment(ggml_backend_buffer_type_t) { return alignment; }
static void *get_base(ggml_backend_buffer_t buffer) { return buffer->context; }
static void refusal(const char *message) { _exit(strstr(message, "not enough space in the buffer") ? 10 : 99); }
static void quiet(enum ggml_log_level, const char *, void *) {}
int main() {
    size_t capacity, offset, size;
    for (;;) {
        const int fields = scanf("%zu %zu %zu %zu", &capacity, &offset, &size, &alignment);
        if (fields == EOF) return 0;
        if (fields != 4 || !alignment || (alignment & (alignment - 1)) || alignment > 4096 || capacity > 4096 || size > 4096 || offset > 8192 || offset % alignment) return 2;
        fflush(stdout);
        pid_t child = fork();
        if (child < 0) return 3;
        if (!child) {
            ggml_set_abort_callback(refusal); ggml_log_set(quiet, nullptr);
            std::vector<char> allocation(capacity + alignment + 1);
            void *base = allocation.data(); size_t space = allocation.size();
            if (!std::align(alignment, capacity, base, space)) _exit(99);
            ggml_backend_buffer_type type = {}; type.iface.get_alignment = get_alignment;
            ggml_backend_buffer buffer = {}; buffer.buft = &type; buffer.size = capacity;
            buffer.context = base; buffer.iface.get_base = get_base;
            ggml_tensor tensor = {}; tensor.type = GGML_TYPE_I8;
            for (int i = 0; i < GGML_MAX_DIMS; i++) { tensor.ne[i] = i ? 1 : size; tensor.nb[i] = 1; }
            ggml_tallocr allocator = ggml_tallocr_new(&buffer); allocator.offset = offset;
            const auto status = ggml_tallocr_alloc(&allocator, &tensor);
            if (status != GGML_STATUS_SUCCESS || tensor.buffer != &buffer) _exit(99);
            printf("0 %zu %zu %zu\n", (size_t)((uintptr_t)tensor.data - (uintptr_t)ggml_backend_buffer_get_base(&buffer)), allocator.offset - offset, allocator.offset);
            fflush(stdout); _exit(0);
        }
        int status; if (waitpid(child, &status, 0) != child) return 4;
        if (!WIFEXITED(status) || WEXITSTATUS(status) == 99) return 5;
        if (WEXITSTATUS(status) == 10) printf("1 0 0 0\n");
        else if (WEXITSTATUS(status) != 0) return 6;
    }
}
"##;

    const STORAGE_ORACLE_SOURCE: &[u8] =
        br##"/* Test transport only. Storage semantics are evaluated by unmodified GGML. */
#include "ggml.h"
#include <inttypes.h>
#include <stdio.h>

int main(void) {
    for (;;) {
        int type;
        int64_t ne[4];
        size_t nb[4];
        const int fields = scanf("%d %" SCNd64 " %" SCNd64 " %" SCNd64 " %" SCNd64
                                " %zu %zu %zu %zu",
                                &type, &ne[0], &ne[1], &ne[2], &ne[3],
                                &nb[0], &nb[1], &nb[2], &nb[3]);
        if (fields == EOF) { return 0; }
        if (fields != 9 || type < 0 || type >= GGML_TYPE_COUNT) { return 2; }
        struct ggml_tensor tensor = {0};
        tensor.type = (enum ggml_type) type;
        for (int i = 0; i < GGML_MAX_DIMS; ++i) {
            tensor.ne[i] = ne[i];
            tensor.nb[i] = nb[i];
        }
        printf("%zu %" PRId64 " %zu\n", ggml_nbytes(&tensor),
               ggml_blck_size(tensor.type), ggml_type_size(tensor.type));
    }
}
"##;
    const TRANSFER_ORACLE_SOURCE: &[u8] = br##"#include "ggml.h"
#include "ggml-backend.h"
#include "ggml-backend-impl.h"
#include <cstdio>
#include <cstring>
#include <climits>
#include <vector>
#include <sys/wait.h>
#include <unistd.h>
static int calls;
static void set(ggml_backend_buffer_t, ggml_tensor *, const void *, size_t, size_t) { calls++; }
static void get(ggml_backend_buffer_t, const ggml_tensor *, void *, size_t, size_t) { calls++; }
static void refusal(const char *message) {
    if (strstr(message, "tensor buffer not set")) _exit(10);
    if (strstr(message, "tensor not allocated")) _exit(11);
    if (strstr(message, "out of bounds")) _exit(12);
    _exit(99);
}
int main() {
    unsigned mode, has_buffer, has_data;
    unsigned long long capacity, offset, size;
    for (;;) {
        int fields = scanf("%u %u %u %llu %llu %llu", &mode, &has_buffer, &has_data, &capacity, &offset, &size);
        if (fields == EOF) return 0;
        if (fields != 6 || mode > 3 || has_buffer > 1 || has_data > 1 || capacity > LLONG_MAX) return 2;
        fflush(stdout);
        pid_t child = fork();
        if (child < 0) return 3;
        if (child == 0) {
            ggml_set_abort_callback(refusal);
            ggml_backend_buffer buffer = {};
            buffer.iface.set_tensor = set; buffer.iface.get_tensor = get;
            ggml_tensor tensor = {}, parent = {};
            tensor.type = GGML_TYPE_I8;
            for (int i = 0; i < GGML_MAX_DIMS; ++i) { tensor.ne[i] = i ? 1 : capacity; tensor.nb[i] = 1; }
            std::vector<char> allocation(capacity ? capacity : 1);
            tensor.data = has_data ? allocation.data() : nullptr;
            if (mode >= 2) { tensor.view_src = &parent; parent.buffer = has_buffer ? &buffer : nullptr; }
            else tensor.buffer = has_buffer ? &buffer : nullptr;
            std::vector<char> data(size ? size : 1);
            if (mode % 2 == 0) ggml_backend_tensor_set(&tensor, data.data(), offset, size);
            else ggml_backend_tensor_get(&tensor, data.data(), offset, size);
            _exit(calls ? 1 : 0);
        }
        int status;
        if (waitpid(child, &status, 0) != child) return 4;
        printf("%d\n", WIFEXITED(status) ? WEXITSTATUS(status) : 99);
    }
}
"##;
}

mod ggml_inventory {

    use std::collections::BTreeSet;

    // Refuse unfamiliar enum syntax rather than silently dropping declarations.
    fn declarations(header: &str, name: &str, prefix: &str) -> Vec<(String, i64)> {
        let start = format!("enum {name} {{");
        let body = header.split_once(&start).expect("enum exists").1;
        let body = body.split_once("};").expect("enum closes").0;
        let mut next = 0;
        let mut rows = Vec::new();
        for line in body.lines() {
            let line = line.split("//").next().expect("line").trim();
            if line.is_empty() {
                continue;
            }
            let line = line.strip_suffix(',').expect("one declaration per line");
            let (symbol, value) = match line.split_once('=') {
                Some((symbol, value)) => (
                    symbol.trim(),
                    value.trim().parse::<i64>().expect("literal enum value"),
                ),
                None => (line, next),
            };
            assert!(
                symbol.starts_with(prefix),
                "unexpected declaration: {symbol}"
            );
            assert!(symbol
                .bytes()
                .all(|c| c.is_ascii_alphanumeric() || c == b'_'));
            rows.push((symbol.to_string(), value));
            next = value.checked_add(1).expect("enum value fits");
        }
        assert!(!rows.is_empty());
        assert_eq!(
            rows.iter().map(|r| &r.0).collect::<BTreeSet<_>>().len(),
            rows.len()
        );
        rows
    }

    fn compare(
        inventory: &toml::Value,
        key: &str,
        expected: &[(String, i64)],
    ) -> Result<(), String> {
        let rows = inventory[key]
            .as_array()
            .ok_or("missing declaration array")?;
        let actual: Result<Vec<_>, String> = rows
            .iter()
            .map(|row| {
                let fields = row.as_table().ok_or("declaration is not a table")?;
                if fields.len() != 2 {
                    return Err("declaration must contain exactly symbol and value".into());
                }
                Ok((
                    row["symbol"].as_str().ok_or("missing symbol")?.to_string(),
                    row["value"].as_integer().ok_or("missing numeric value")?,
                ))
            })
            .collect();
        if actual? != expected {
            return Err(format!(
                "{key}: inventory differs from pinned upstream declarations"
            ));
        }
        Ok(())
    }

    /// `VT-04`: The imported GGML operation and tensor type declarations are inventoried exactly without implementation claims
    #[test]
    fn ggml_inventory_vt_04() {
        let root = super::root();
        let header =
            std::fs::read_to_string(root.join("vendor/llama.cpp-oracle/ggml/include/ggml.h"))
                .expect("pinned header");
        let lock =
            super::ggml_oracle::read_toml(root.join("vendor/llama.cpp-oracle/upstream.toml"));
        let inventory = super::ggml_oracle::read_toml(root.join("model/ggml-declarations.toml"));
        assert_eq!(
            inventory["spec"].as_str(),
            Some("vtensor/ggml-declarations/1")
        );
        assert_eq!(
            inventory["authority"].as_str(),
            Some("LLAMA-CPP-GGML-BACKEND")
        );
        assert_eq!(inventory["revision"], lock["commit"]);
        assert_eq!(inventory["source"].as_str(), Some("ggml/include/ggml.h"));
        for (key, name, prefix) in [
            ("operation", "ggml_op", "GGML_OP_"),
            ("tensor_type", "ggml_type", "GGML_TYPE_"),
            ("unary", "ggml_unary_op", "GGML_UNARY_OP_"),
        ] {
            let expected = declarations(&header, name, prefix);
            compare(&inventory, key, &expected).expect("exact upstream declaration inventory");
            // This gate must reject omissions, additions, reordering, numeric drift,
            // and invented implementation claims, including changes to its own comparator.
            for mutation in 0..5 {
                let mut damaged = inventory.clone();
                let rows = damaged[key].as_array_mut().expect("rows");
                match mutation {
                    0 => {
                        rows.remove(0);
                    }
                    1 => rows.push(rows[0].clone()),
                    2 => rows.swap(0, 1),
                    3 => rows[0]["value"] = toml::Value::Integer(-1),
                    _ => {
                        rows[0]
                            .as_table_mut()
                            .expect("table")
                            .insert("implemented".into(), toml::Value::Boolean(true));
                    }
                }
                assert!(
                    compare(&damaged, key, &expected).is_err(),
                    "mutation {mutation} escaped"
                );
            }
        }
    }
}

mod ggml_storage_oracle {

    use super::ggml_oracle;

    /// `VT-05`: The pinned GGML storage oracle executes upstream byte-span semantics for scalar and block-encoded tensors
    #[test]
    fn ggml_storage_oracle_vt_05() {
        let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let executable = ggml_oracle::storage_oracle(&root);
        let output = ggml_oracle::spans(
            &executable,
            concat!(
                "0 1 1 1 1 4 4 4 4\n",        // F32 scalar
                "0 3 2 1 1 4 32 64 64\n",     // F32 padded rows
                "28 2 3 2 2 8 16 48 96\n",    // F64 four-dimensional tensor
                "2 64 2 1 1 18 64 128 128\n", // Q4_0 padded block rows
                "0 3 0 1 1 4 12 12 12\n",     // empty dimension
                "0 -1 2 1 1 4 12 24 24\n",    // upstream empty negative extent
            ),
        );
        assert_eq!(
            output,
            [
                [4, 1, 4],
                [44, 1, 4],
                [192, 1, 8],
                [100, 32, 18],
                [0, 1, 4],
                [0, 1, 4]
            ]
        );
        use std::io::Write;
        let mut child = std::process::Command::new(&executable)
            .stdin(std::process::Stdio::piped())
            .spawn()
            .expect("oracle");
        child
            .stdin
            .take()
            .expect("input")
            .write_all(b"malformed\n")
            .expect("write");
        assert_eq!(child.wait().expect("result").code(), Some(2));

        // Exercise enough input and output to exceed pipe capacity in both
        // directions without starting a separate oracle process for every case.
        let batch = ggml_oracle::spans(&executable, &"0 1 1 1 1 4 4 4 4\n".repeat(8192));
        assert_eq!(batch.len(), 8192);
        assert!(batch.iter().all(|row| *row == [4, 1, 4]));
        let built_at = std::fs::metadata(&executable)
            .expect("oracle metadata")
            .modified()
            .expect("mtime");
        assert_eq!(ggml_oracle::storage_oracle(&root), executable);
        assert_eq!(
            std::fs::metadata(&executable)
                .expect("oracle metadata")
                .modified()
                .expect("mtime"),
            built_at,
            "unchanged oracle was unnecessarily rebuilt"
        );
    }
}

mod oracle_import {

    use super::{ggml_oracle::digest as hex, root};
    use std::collections::BTreeSet;

    use repo_model::Model;
    use sha2::{Digest, Sha256};

    /// `VT-01`: The imported GGML backend contract, implementation interface, and operator oracle match their pinned upstream source
    #[test]
    fn oracle_import_vt_01() {
        let root = root();
        let artifact = root.join("vendor/llama.cpp-oracle");
        let lock = super::ggml_oracle::read_toml(artifact.join("upstream.toml"));

        assert_eq!(
            lock["repository"].as_str(),
            Some("https://github.com/ggml-org/llama.cpp")
        );
        assert_eq!(
            lock["commit"].as_str(),
            Some("9a080f44b503c64f66c84a6274b9d9ee5d5b09d5")
        );

        let files = lock["file"]
            .as_array()
            .expect("the lock records every imported source file");
        let mut paths = BTreeSet::new();
        for entry in files {
            let path = entry["path"].as_str().expect("source path");
            let expected = entry["sha256"].as_str().expect("source SHA-256");
            let bytes = std::fs::read(artifact.join(path))
                .unwrap_or_else(|error| panic!("pinned upstream file {path} is missing: {error}"));
            let observed = hex(&Sha256::digest(bytes));
            assert_eq!(observed, expected, "upstream source drifted: {path}");
            assert!(
                paths.insert(path),
                "upstream source is pinned twice: {path}"
            );
        }

        let required = BTreeSet::from([
            "LICENSE",
            "ggml/include/ggml-backend.h",
            "ggml/include/ggml.h",
            "ggml/include/ggml-alloc.h",
            "ggml/src/ggml-backend-impl.h",
            "tests/test-backend-ops.cpp",
        ]);
        assert_eq!(
        paths, required,
        "the pinned snapshot must include the backend contract, implementation interface, and upstream operation oracle"
    );

        let archives = lock["archive"]
            .as_array()
            .expect("buildable source archive");
        assert_eq!(archives.len(), 1);
        let archive = &archives[0];
        assert_eq!(archive["path"].as_str(), Some("source.tar.gz"));
        assert_eq!(archive["format"].as_str(), Some("git-archive-tar-gzip"));
        assert_eq!(
            archive["paths"]
                .as_array()
                .expect("export paths")
                .iter()
                .map(|p| p.as_str().expect("path"))
                .collect::<Vec<_>>(),
            ["ggml", "tests/test-backend-ops.cpp", "LICENSE"]
        );
        assert_eq!(
            hex(&Sha256::digest(
                std::fs::read(artifact.join("source.tar.gz")).expect("source archive")
            )),
            archive["sha256"].as_str().expect("archive digest")
        );

        let exemption = super::ggml_oracle::read_json(root.join("oracle-import.lock"));
        let expected_files: Vec<_> = files.iter().map(|file| serde_json::json!({"path": format!("vendor/llama.cpp-oracle/{}", file["path"].as_str().expect("path")), "sha256": file["sha256"].as_str().expect("digest")})).collect();
        assert_eq!(exemption["source"]["repository"], "ggml-org/llama.cpp");
        assert_eq!(
            exemption["source"]["commit"].as_str(),
            lock["commit"].as_str()
        );
        assert_eq!(
            exemption["files"],
            serde_json::Value::Array(expected_files),
            "only exact pinned oracle source may be excluded from local implementation accounting"
        );

        let model = Model::load(&root.join("model")).expect("the project model loads");
        model
            .check_artifacts(&root)
            .expect("the authority snapshot is pinned");
        let authority = model
            .authorities
            .authority
            .iter()
            .find(|row| row.id == "LLAMA-CPP-GGML-BACKEND")
            .expect("the imported oracle is registered as an authority");
        assert_eq!(
            authority.artifact.as_deref(),
            Some("vendor/llama.cpp-oracle")
        );
    }
}

mod tensor_layout {

    use super::ggml_oracle;

    use vtensor_layout::{run, Adt0, Adt1, Overflow};

    pub(super) fn certified_current_model(root: &std::path::Path, name: &str) {
        assert!(matches!(
            name,
            "layout" | "native" | "matrix" | "index" | "pointwise"
        ));
        let scope = if name == "layout" {
            String::new()
        } else {
            format!("{name}.")
        };
        let project = format!("app/{scope}lexlean.toml");
        let model_lock = format!("app/{scope}lexlean.lock");
        let suffix = match name {
            "native" => "native_config",
            _ => name,
        };
        let primary = format!("vtensor_{suffix}");
        let bundle = format!("generated/{name}");
        let result = super::ggml_oracle::read_json(root.join(format!("{bundle}/result.json")));
        let checked = std::process::Command::new("lexlean")
            .current_dir(root)
            .args([
                "--project",
                &project,
                "--diagnostic-format",
                "json",
                "check",
            ])
            .output()
            .expect("run in the pinned SDK with lexlean available");
        assert!(
            checked.status.success(),
            "{}",
            String::from_utf8_lossy(&checked.stdout)
        );
        let current: serde_json::Value =
            serde_json::from_slice(&checked.stdout).expect("model identities");
        assert_eq!(
            current["source_id"], result["source"]["source_id"],
            "regenerate layout bundle after model changes"
        );
        assert_eq!(current["semantic_id"], result["source"]["semantic_id"]);
        use sha2::{Digest, Sha256};
        let lock_digest = ggml_oracle::file_digest(root.join(model_lock));
        assert_eq!(
            result["source"]["lock_sha256"].as_str(),
            Some(lock_digest.as_str())
        );
        let request =
            super::ggml_oracle::read_json(root.join(format!("app/{name}-production.json")));
        let wanted = request["roots"].as_array().expect("requested roots");
        let exported = result["roots"].as_array().expect("exported roots");
        assert_eq!(exported.len(), wanted.len());
        for (actual, expected) in exported.iter().zip(wanted) {
            assert_eq!(actual["root"], expected["root"]);
            assert_eq!(actual["outputs"].as_array().expect("outputs").len(), 1);
            let output = &actual["outputs"][0];
            assert_eq!(output["target"], expected["targets"][0]);
            assert_eq!(output["package"]["name"], expected["package"]["name"]);
            assert_eq!(output["certification"]["status"], "kernel-checked");
        }
        let kernel_lock = super::ggml_oracle::read_json(root.join(format!("{name}-kernel.lock")));
        assert_eq!(kernel_lock["source"]["result"], result["result_id"]);
        assert_eq!(
            kernel_lock["source"]["attestation"],
            result["verification"]["attestation_id"]
        );
        assert_eq!(
            kernel_lock["output"]["path"],
            format!("{bundle}/packages/rust-core/{primary}/src/lib.rs")
        );
        let bytes =
            std::fs::read(root.join(kernel_lock["output"]["path"].as_str().expect("output path")))
                .expect("generated code");
        assert_eq!(
            kernel_lock["output"]["sha256"],
            ggml_oracle::digest(&Sha256::digest(bytes))
        );
        let certified_files: Vec<_> = result["roots"].as_array().expect("roots").iter()
        .flat_map(|root| root["outputs"].as_array().expect("outputs"))
        .flat_map(|output| output["package"]["files"].as_array().expect("files").iter().filter(|file| file["path"].as_str().expect("path").ends_with(".rs")).chain(std::iter::once(&output["crate"])))
        .map(|file| serde_json::json!({"path": format!("{bundle}/{}", file["path"].as_str().expect("path")), "sha256": file["sha256"]}))
        .collect();
        assert_eq!(
            kernel_lock["files"],
            serde_json::Value::Array(certified_files),
            "generated-code exemptions must exactly match the certified outputs"
        );
        let replay = std::process::Command::new("lexlean")
            .current_dir(root)
            .args(["production", "verify-artifact", &bundle])
            .output()
            .expect("SDK artifact replay");
        assert!(
            replay.status.success(),
            "{}{}",
            String::from_utf8_lossy(&replay.stdout),
            String::from_utf8_lossy(&replay.stderr)
        );
    }

    fn layout(ne: [u64; 4], nb: [u64; 4], block: u64, bytes: u64) -> Adt0 {
        Adt0::C0(
            ne[0], ne[1], ne[2], ne[3], nb[0], nb[1], nb[2], nb[3], block, bytes,
        )
    }

    fn request(output: &mut String, kind: u64, ne: [u64; 4], nb: [u64; 4]) {
        use std::fmt::Write;
        writeln!(
            output,
            "{kind} {} {} {} {} {} {} {} {}",
            ne[0], ne[1], ne[2], ne[3], nb[0], nb[1], nb[2], nb[3]
        )
        .expect("string write");
    }

    struct Fixture {
        executable: std::path::PathBuf,
        kinds: Vec<u64>,
        metadata: Vec<[u64; 3]>,
    }

    fn fixture() -> &'static Fixture {
        static FIXTURE: std::sync::OnceLock<Fixture> = std::sync::OnceLock::new();
        FIXTURE.get_or_init(|| {
            let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
            certified_current_model(&root, "layout");
            let executable = ggml_oracle::storage_oracle(&root);
            let inventory =
                super::ggml_oracle::read_toml(root.join("model/ggml-declarations.toml"));
            let kinds: Vec<_> = inventory["tensor_type"]
                .as_array()
                .expect("types")
                .iter()
                .filter(|row| row["symbol"].as_str() != Some("GGML_TYPE_COUNT"))
                .map(|row| {
                    u64::try_from(row["value"].as_integer().expect("numeric type"))
                        .expect("positive type")
                })
                .collect();
            let mut metadata_requests = String::new();
            for &kind in &kinds {
                request(&mut metadata_requests, kind, [0; 4], [0; 4]);
            }
            let metadata = ggml_oracle::spans(&executable, &metadata_requests);
            assert_eq!(metadata.len(), kinds.len(), "oracle omitted a type");
            Fixture {
                executable,
                kinds,
                metadata,
            }
        })
    }

    pub(super) fn verify_bundle() {
        let _ = fixture();
    }

    /// `VT-06`: The Prism-generated four-dimensional byte-span function matches the pinned GGML storage oracle and rejects invalid blocks and arithmetic overflow
    #[test]
    fn tensor_layout_vt_06() {
        let Fixture {
            executable,
            kinds,
            metadata,
        } = fixture();
        let mut inputs = String::new();
        let mut layouts = Vec::new();
        for (&kind, &[_, block, bytes]) in kinds.iter().zip(metadata) {
            assert!(
                block > 0 && bytes > 0,
                "invalid upstream descriptor for type {kind}"
            );
            for multiples in [1, 2, 4] {
                for rows in [1, 3] {
                    for planes in [1, 2] {
                        for batches in [1, 2] {
                            for padding in [0, 7] {
                                let ne = [block * multiples, rows, planes, batches];
                                let nb0 = bytes + padding;
                                let nb1 = multiples * nb0 + 11;
                                let nb2 = rows * nb1 + 13;
                                let nb = [nb0, nb1, nb2, planes * nb2 + 17];
                                request(&mut inputs, kind, ne, nb);
                                layouts.push(layout(ne, nb, block, bytes));
                            }
                        }
                    }
                }
            }
            for axis in 0..4 {
                let mut ne = [block, 2, 2, 2];
                ne[axis] = 0;
                let nb = [bytes, bytes * 2, bytes * 4, bytes * 8];
                request(&mut inputs, kind, ne, nb);
                layouts.push(layout(ne, nb, block, bytes));
            }
        }
        let expected = ggml_oracle::spans(executable, &inputs);
        assert_eq!(expected.len(), layouts.len(), "oracle omitted a layout");
        for (index, (input, reference)) in layouts.into_iter().zip(expected).enumerate() {
            match run(input).expect("bounded layout fits machine arithmetic") {
                Adt1::C1(bytes) => assert_eq!(bytes, reference[0], "layout case {index}"),
                Adt1::C0 => panic!("valid layout case {index} was rejected"),
            }
        }
        assert!(matches!(run(layout([1; 4], [4; 4], 0, 4)), Ok(Adt1::C0)));
        for (ne, nb, block, bytes) in [
            ([u64::MAX, 1, 1, 1], [8; 4], 1, 8),
            ([u64::MAX, 1, 1, 1], [18; 4], 32, 18),
            ([2, 2, 1, 1], [1, u64::MAX, 1, 1], 1, 1),
        ] {
            assert!(matches!(run(layout(ne, nb, block, bytes)), Err(Overflow)));
        }
        // Empty dimensions must short circuit unsafe arithmetic.
        assert!(matches!(
            run(layout([u64::MAX, 0, 1, 1], [u64::MAX; 4], 1, 8)),
            Ok(Adt1::C1(0))
        ));
    }

    /// `VT-07`: The Prism-generated type catalog matches every declared GGML storage type and refuses removed and out-of-range identifiers
    #[test]
    fn tensor_layout_vt_07() {
        let Fixture {
            kinds, metadata, ..
        } = fixture();
        for (&kind, &[_, block, bytes]) in kinds.iter().zip(metadata) {
            match vtensor_types::run(kind) {
                vtensor_types::Adt0::C1(vtensor_types::Adt1::C0(id, block_size, type_size)) => {
                    assert_eq!(
                        [id, block_size, type_size],
                        [kind, block, bytes],
                        "GGML type {kind}"
                    );
                }
                _ => panic!("declared GGML type {kind} was refused"),
            }
        }
        for kind in (0..=44).chain([45, 255, u64::MAX]) {
            if !kinds.contains(&kind) {
                assert!(
                    matches!(vtensor_types::run(kind), vtensor_types::Adt0::C0),
                    "invalid type {kind} was admitted"
                );
            }
        }
    }
}

mod verdant_api {

    use prism_kernel::supportsF64Matmul4;
    use verdant_tensor::linear::multiply_mat4_cols_exact;
    use verdant_tensor::{verified_unified_tensor_kernels, TensorComputeError};

    const VERDANT_ENGINE_REVISION: &str = "87c3256326b9ec9f8d591855394c0bd0265588ef";

    /// `VT-02`: The Verdant integration exposes the fifteen unified tensor kernel descriptors from its pinned API
    #[test]
    fn verdant_api_vt_02() {
        let root = super::root();
        let lock = super::ggml_oracle::read_toml(root.join("Cargo.lock"));
        let verdant = lock["package"]
            .as_array()
            .expect("Cargo.lock package list exists")
            .iter()
            .find(|package| package["name"].as_str() == Some("verdant-tensor"))
            .expect("Verdant Tensor is resolved in Cargo.lock");
        let source = verdant.get("source")
            .and_then(toml::Value::as_str)
            .expect("Verdant Tensor must resolve to the pinned upstream source; local development overrides are not release evidence");
        assert!(
            source.ends_with(&format!("#{VERDANT_ENGINE_REVISION}")),
            "Verdant Engine must resolve to the pinned origin/main revision: {source}"
        );

        let kernels = verified_unified_tensor_kernels().expect("Verdant verified kernel inventory");
        assert_eq!(kernels.len(), 15);
        for (index, kernel) in kernels.iter().enumerate() {
            assert_eq!(kernel.index, index as u64);
        }
    }

    /// `VT-03`: vTensor delegates fixed 4x4 F64 matrix multiplication to Verdant Tensor and preserves column-major results
    #[test]
    fn verdant_api_vt_03() {
        assert!(supportsF64Matmul4(true, true, true));
        for (type_is_f64, lhs_is_4x4, rhs_is_4x4) in [
            (false, true, true),
            (true, false, true),
            (true, true, false),
        ] {
            assert!(!supportsF64Matmul4(type_is_f64, lhs_is_4x4, rhs_is_4x4));
        }

        let lhs = [
            [1.0, 2.0, 0.0, 1.0],
            [0.0, 1.0, 1.0, 0.0],
            [1.0, 0.0, 1.0, 0.0],
            [2.0, 1.0, 0.0, 1.0],
        ];
        let rhs = [
            [2.0, 0.0, 1.0, 0.0],
            [1.0, 3.0, 0.0, 1.0],
            [0.0, 1.0, 2.0, 0.0],
            [1.0, 0.0, 1.0, 1.0],
        ];
        let expected: [[f64; 4]; 4] = std::array::from_fn(|column| {
            std::array::from_fn(|row| {
                (0..4)
                    .map(|inner| lhs[inner][row] * rhs[column][inner])
                    .sum()
            })
        });

        assert_eq!(
            multiply_mat4_cols_exact(&lhs, &rhs).expect("Verdant matrix multiplication"),
            expected
        );

        let non_finite = [[f64::NAN; 4]; 4];
        assert!(matches!(
            multiply_mat4_cols_exact(&non_finite, &rhs),
            Err(TensorComputeError::NonFiniteTensorEntry { operand: "mat4" })
        ));
    }
}

/// `VT-09`: Prism transfer admission matches upstream GGML set and get guards and rejects overflowing ranges
#[test]
fn buffer_transfer_vt_09() {
    tensor_layout::verify_bundle();
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut input = String::new();
    let mut cases = Vec::new();
    use std::fmt::Write;
    for mode in 0..4 {
        for has_buffer in [false, true] {
            for has_data in [false, true] {
                for capacity in [0_u64, 1, 8, 64] {
                    for offset in [0, 1, capacity, capacity + 1] {
                        for size in [0, 1, capacity, capacity + 1] {
                            writeln!(
                                input,
                                "{mode} {} {} {capacity} {offset} {size}",
                                u8::from(has_buffer),
                                u8::from(has_data)
                            )
                            .unwrap();
                            cases.push((has_buffer, has_data, capacity, offset, size));
                        }
                    }
                }
            }
        }
    }
    let reference = ggml_oracle::transfers(&root, &input);
    assert_eq!(reference.len(), cases.len());
    fn outcome(decision: vtensor_transfer::Adt0) -> u64 {
        match decision {
            vtensor_transfer::Adt0::C0 => 10,
            vtensor_transfer::Adt0::C1 => 0,
            vtensor_transfer::Adt0::C2 => 11,
            vtensor_transfer::Adt0::C3 => 12,
            vtensor_transfer::Adt0::C4 => 1,
        }
    }
    for (index, ((buffer, data, capacity, offset, size), expected)) in
        cases.into_iter().zip(reference).enumerate()
    {
        assert_eq!(
            outcome(vtensor_transfer::run(buffer, data, capacity, offset, size)),
            expected,
            "transfer {index}"
        );
    }
    // The native unsigned-add assertion can wrap for invalid requests. The
    // modeled range check deliberately refuses them without wrapping.
    assert_eq!(
        outcome(vtensor_transfer::run(true, true, 64, u64::MAX, 2)),
        12
    );
    assert_eq!(
        outcome(vtensor_transfer::run(true, true, u64::MAX, u64::MAX, 1)),
        12
    );
    assert_eq!(
        outcome(vtensor_transfer::run(true, false, 0, u64::MAX, 0)),
        0
    );
    assert_eq!(outcome(vtensor_transfer::run(false, false, 0, 0, 0)), 10);
}

mod native_backend_oracle {
    use super::ggml_oracle::{compile, digest, file_digest, run, storage_oracle, verify_file};
    use sha2::{Digest, Sha256};
    use std::fmt::Write;
    use std::path::Path;
    use std::process::Command;

    pub(super) fn verify(root: &Path, mode: &str) {
        static ACCESS: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let _access = ACCESS.lock().unwrap_or_else(|error| error.into_inner());
        let oracle = storage_oracle(root);
        let common = oracle.parent().unwrap();
        let objects = super::ggml_oracle::read_json(common.join("objects.json"));
        for name in ["ggml.o", "quants.o", "alloc.o", "backend.o", "meta.o"] {
            assert_eq!(objects[name], file_digest(common.join(name)));
        }
        let executable = std::env::current_exe().unwrap();
        let library = executable
            .parent()
            .unwrap()
            .join("libvtensor_backend.so")
            .canonicalize()
            .expect("Cargo builds the native shared library");
        let layouts = vtensor_backend::abi_layout();
        let mut driver = String::from(HEADER);
        for &(key, _) in &layouts {
            let (ty, field) = key.rsplit_once('.').unwrap();
            let expression = match field {
                "$size" => format!("sizeof({ty})"),
                "$align" => format!("alignof({ty})"),
                _ => format!("offsetof({ty}, {field})"),
            };
            writeln!(
                driver,
                "printf(\"ABI {key} %zu\\n\", (size_t)({expression}));"
            )
            .unwrap();
        }
        driver.push_str(BODY);
        let mut identity = Sha256::new();
        identity.update(library.as_os_str().as_encoded_bytes());
        // The host dynamically loads this path; live provenance checks bind its contents.
        identity.update(driver.as_bytes());
        identity.update(common.as_os_str().as_encoded_bytes());
        identity.update(super::ggml_oracle::source_tokens(
            include_str!("contract.rs")
                .split_once("\nmod native_backend_oracle {")
                .unwrap()
                .1,
        ));
        let cache = common.join("native").join(digest(&identity.finalize()));
        std::fs::create_dir_all(&cache).unwrap();
        let program = cache.join("native-oracle");
        if !program.exists() {
            std::fs::write(cache.join("driver.cpp"), &driver).unwrap();
            let source_objects = [
                ("ggml-backend-reg.cpp", "reg.o"),
                ("ggml-backend-dl.cpp", "dl.o"),
                ("ggml-threading.cpp", "threading.o"),
            ];
            let frontend = common
                .join("native-frontend")
                .join(digest(&Sha256::digest(format!("{source_objects:?}"))));
            std::fs::create_dir_all(&frontend).unwrap();
            for (source, object) in source_objects {
                let object_path = frontend.join(object);
                if !object_path.exists() {
                    compile(common, common.join("ggml/src").join(source), &object_path);
                    std::fs::write(
                        frontend.join(format!("{object}.sha256")),
                        file_digest(&object_path),
                    )
                    .unwrap();
                }
                verify_file(&object_path, frontend.join(format!("{object}.sha256")));
            }
            compile(common, cache.join("driver.cpp"), cache.join("driver.o"));
            let build_list =
                std::fs::read_to_string(common.join("ggml/src/ggml-cpu/CMakeLists.txt")).unwrap();
            let mut cpu_sources: Vec<String> = build_list
                .split_once("list (APPEND GGML_CPU_SOURCES")
                .unwrap()
                .1
                .split_once(')')
                .unwrap()
                .0
                .split_whitespace()
                .filter(|name| name.ends_with(".c") || name.ends_with(".cpp"))
                .map(str::to_string)
                .collect();
            let architecture = match std::env::consts::ARCH {
                "x86_64" => "x86",
                "aarch64" => "arm",
                other => panic!("unmodeled oracle architecture: {other}"),
            };
            cpu_sources.extend([
                format!("ggml-cpu/arch/{architecture}/quants.c"),
                format!("ggml-cpu/arch/{architecture}/repack.cpp"),
            ]);
            let cpu = common
                .join("reference-cpu")
                .join(digest(&Sha256::digest(format!("{cpu_sources:?}"))));
            std::fs::create_dir_all(&cpu).unwrap();
            let mut cpu_objects = Vec::new();
            for source in cpu_sources {
                let object = cpu.join(format!("{}.o", source.replace('/', "_")));
                if !object.exists() {
                    compile(
                        common,
                        common.join("ggml/src").join(source),
                        object.with_extension("new"),
                    );
                    std::fs::write(
                        object.with_extension("sha256"),
                        file_digest(object.with_extension("new")),
                    )
                    .unwrap();
                    std::fs::rename(object.with_extension("new"), &object).unwrap();
                }
                verify_file(&object, object.with_extension("sha256"));
                cpu_objects.push(object);
            }
            let mut linker = Command::new("c++");
            linker.args(["-Wl,--gc-sections", "-Wl,--export-dynamic"]);
            linker.arg(cache.join("driver.o"));
            for name in ["reg.o", "dl.o", "threading.o"] {
                linker.arg(frontend.join(name));
            }
            for name in ["ggml.o", "quants.o", "alloc.o", "backend.o", "meta.o"] {
                linker.arg(common.join(name));
            }
            linker.args(cpu_objects);
            linker
                .arg(&library)
                .arg(format!(
                    "-Wl,-rpath,{}",
                    library.parent().unwrap().display()
                ))
                .args(["-lm", "-ldl", "-pthread", "-o"])
                .arg(cache.join("native-oracle.new"));
            run(&mut linker);
            std::fs::write(
                cache.join("program.sha256"),
                file_digest(cache.join("native-oracle.new")),
            )
            .unwrap();
            std::fs::rename(cache.join("native-oracle.new"), &program).unwrap();
        }
        verify_file(&program, cache.join("program.sha256"));
        let declarations = super::ggml_oracle::read_toml(root.join("model/ggml-declarations.toml"));
        let kinds: Vec<_> = declarations["tensor_type"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|row| row["symbol"].as_str() != Some("GGML_TYPE_COUNT"))
            .map(|row| row["value"].as_integer().unwrap())
            .collect();
        let input = kinds
            .iter()
            .filter(|_| {
                matches!(
                    mode,
                    "buffers" | "empty" | "copy" | "repeat" | "concat" | "spatial"
                )
            })
            .map(|kind| format!("{kind}\n"))
            .collect::<String>();
        let snapshot = cache.join("loaded-source.snapshot");
        let mut expected = Vec::new();
        for file in vtensor_backend::SOURCE_FILES {
            expected.extend_from_slice(file.as_bytes());
            expected.push(0);
            expected.extend_from_slice(&std::fs::read(root.join(file)).unwrap());
            expected.push(0);
        }
        assert_eq!(
            digest(&Sha256::digest(vtensor_backend::source_snapshot())),
            digest(&Sha256::digest(&expected)),
            "Rust test library is stale"
        );
        let expected_snapshot = cache.join("expected-source.snapshot");
        std::fs::write(&expected_snapshot, &expected).unwrap();
        let text = super::ggml_oracle::batch(
            Command::new(&program)
                .arg(&snapshot)
                .arg(&library)
                .arg(&expected_snapshot)
                .arg(mode),
            &input,
        );
        let observed: std::collections::BTreeMap<_, _> = text
            .lines()
            .map(|line| {
                let fields: Vec<_> = line.split_whitespace().collect();
                assert_eq!(fields.len(), 3, "unexpected native record: {line}");
                (fields[1].to_string(), fields[2].parse::<usize>().unwrap())
            })
            .collect();
        assert_eq!(
            observed.len(),
            layouts.len() + 1,
            "native oracle omitted or duplicated ABI observations"
        );
        for (key, value) in layouts {
            assert_eq!(observed.get(key), Some(&value), "ABI layout {key}");
        }
        assert_eq!(
            observed.get("cases"),
            Some(
                &(match mode {
                    "matrix" => 640,
                    "elementwise" => 120,
                    "buffers" => kinds.len() * 3,
                    "empty" => kinds.len() * 24,
                    "copy" => kinds.len() * 96,
                    "repeat" => 160,
                    "concat" => kinds.len() * 288,
                    "spatial" => 2256,
                    _ => panic!("unknown oracle mode"),
                })
            )
        );
        assert_eq!(
            file_digest(snapshot),
            digest(&Sha256::digest(expected)),
            "loaded native library does not match current source"
        );
    }

    const HEADER: &str = r##"#include "ggml.h"
#include "ggml-backend.h"
#include "ggml-backend-impl.h"
#include "ggml-alloc.h"
#include "ggml-cpu.h"
#include <cmath>
#include <algorithm>
#include <cstddef>
#include <cstdio>
#include <cstring>
#include <cstdint>
#include <fstream>
#include <iterator>
#include <filesystem>
#include <limits>
#include <vector>
#include <dlfcn.h>
#include <sys/wait.h>
#include <unistd.h>
extern "C" ggml_backend_reg_t ggml_backend_vtensor_reg();
extern "C" ggml_backend_reg_t ggml_backend_init();
extern "C" const unsigned char * vtensor_build_sources();
extern "C" size_t vtensor_build_sources_size();
#define CHECK(x) do { if (!(x)) { fprintf(stderr, "native check line %d: %s\n", __LINE__, #x); return 2; } } while (0)
static void refusal(const char *message) { _exit(strstr(message, "out of bounds") ? 12 : 99); }
static int current_kind = -1, current_layout = -1;
static void unexpected_abort(const char *message) { fprintf(stderr, "upstream abort at type %d layout %d: %s\n", current_kind, current_layout, message); _exit(99); }
static ggml_tensor * canonical_copy(ggml_context *ctx, ggml_tensor *input) {
    const int64_t block=ggml_blck_size(input->type), count=ggml_nelements(input)/block;
    ggml_tensor *target=ggml_new_tensor_2d(ctx,input->type,block,count); target->nb[1]*=2; target->nb[2]=target->nb[1]*count; target->nb[3]=target->nb[2];
    return ggml_reshape_4d(ctx,ggml_cont(ctx,ggml_cpy(ctx,input,target)),input->ne[0],input->ne[1],input->ne[2],input->ne[3]);
}
int main(int argc, char **argv) {
    CHECK(argc == 5);
    Dl_info loaded = {};
    CHECK(dladdr((void *)&ggml_backend_vtensor_reg, &loaded) != 0);
    CHECK(std::filesystem::canonical(loaded.dli_fname) == std::filesystem::canonical(argv[2]));
    std::ifstream expected_file(argv[3], std::ios::binary);
    CHECK(expected_file.good());
    const std::vector<unsigned char> expected_source((std::istreambuf_iterator<char>(expected_file)), std::istreambuf_iterator<char>());
    CHECK(expected_source.size() == vtensor_build_sources_size());
    CHECK(!memcmp(expected_source.data(), vtensor_build_sources(), expected_source.size()));
    std::ofstream provenance(argv[1], std::ios::binary);
    provenance.write((const char *)vtensor_build_sources(), vtensor_build_sources_size());
    provenance.close(); CHECK(provenance.good());
"##;

    const BODY: &str = r##"
    ggml_set_abort_callback(unexpected_abort);
    ggml_backend_reg_t reg = ggml_backend_vtensor_reg();
    CHECK(reg && ggml_backend_init() == reg && reg->api_version == GGML_BACKEND_API_VERSION);
    ggml_backend_register(reg);
    CHECK(ggml_backend_reg_count() == 1 && ggml_backend_dev_count() == 1);
    CHECK(ggml_backend_reg_by_name("vTensor") == reg);
    CHECK(ggml_backend_reg_dev_count(reg) == 1);
    ggml_backend_dev_t dev = ggml_backend_reg_dev_get(reg, 0);
    CHECK(dev && ggml_backend_dev_by_name("vTensor") == dev);
    CHECK(reg->iface.get_device(reg, 1) == nullptr);
    ggml_backend_dev_props props = {}; ggml_backend_dev_get_props(dev, &props);
    CHECK(!strcmp(props.name, "vTensor") && !strcmp(props.description, "Prism / Verdant Tensor"));
    CHECK(props.type == GGML_BACKEND_DEVICE_TYPE_CPU && !props.caps.async && props.caps.buffer_from_host_ptr && props.caps.mmap_support && !props.caps.events && !props.caps.host_buffer);
    ggml_backend_t backend = ggml_backend_dev_init(dev, nullptr); CHECK(backend);
    CHECK(!strcmp(ggml_backend_name(backend), "vTensor"));
    ggml_backend_buffer_type_t buft = ggml_backend_dev_buffer_type(dev);
    CHECK(ggml_backend_buft_is_host(buft) && ggml_backend_dev_supports_buft(dev, buft));
    const size_t align = ggml_backend_buft_get_alignment(buft); CHECK(align == 64);
    const bool repeating=!strcmp(argv[4], "repeat"), concatenating=!strcmp(argv[4], "concat");
    const bool spatial=!strcmp(argv[4], "spatial");
    if (!strcmp(argv[4], "copy") || repeating || concatenating || spatial) {
      ggml_backend_t reference=ggml_backend_cpu_init(); CHECK(reference);
      ggml_backend_cpu_set_n_threads(reference,1);
      size_t cases=0, refused=0; int kind;
      while (scanf("%d",&kind)==1) for(int operation=0;operation<(concatenating?4:repeating?1:3);operation++) {
      const std::vector<int> extents=(concatenating || (spatial && operation==0))?std::vector<int>{0,1,3}:spatial?std::vector<int>{2,3}:std::vector<int>{1,3};
      const std::vector<int> batches_list=(spatial && operation==2)?std::vector<int>{2,3}:std::vector<int>{1,2};
      for(int pattern=0;pattern<(spatial?(operation==0?22:operation==1?4:17):1);pattern++) for(int layout=0;layout<4;layout++)
      for(int units : extents) for(int rows : extents) for(int batches : batches_list) {
        if((spatial && kind!=GGML_TYPE_F32) || (repeating && kind!=GGML_TYPE_F32 && kind!=GGML_TYPE_F16 && kind!=GGML_TYPE_BF16 && kind!=GGML_TYPE_I16 && kind!=GGML_TYPE_I32)) continue;
        ggml_init_params params={8*1024*1024,nullptr,true}; ggml_context *ctx=ggml_init(params); CHECK(ctx);
        const auto type=(ggml_type)kind; const int64_t block=ggml_blck_size(type); const size_t bytes=ggml_type_size(type);
        ggml_tensor *input=ggml_new_tensor_4d(ctx,type,block*units,rows,2,batches);
        int64_t right_ne[]={input->ne[0],input->ne[1],input->ne[2],input->ne[3]}; right_ne[operation%4]=layout==3?0:right_ne[operation%4]+(operation==0?block:1);
        ggml_tensor *right=concatenating?ggml_new_tensor(ctx,type,4,right_ne):nullptr;
        for(ggml_tensor *operand : {input,right}) if(operand && (layout==1 || layout==2)) {
          operand->nb[0]=bytes*(layout==2?2:1); operand->nb[1]=operand->nb[0]*(operand->ne[0]/block)+bytes;
          operand->nb[2]=operand->nb[1]*operand->ne[1]+bytes; operand->nb[3]=operand->nb[2]*operand->ne[2]+bytes;
        }
        const int64_t blocks=ggml_nelements(input)/block;
        ggml_tensor *target=layout==3?ggml_new_tensor_4d(ctx,type,block,blocks,1,1):ggml_dup_tensor(ctx,input);
        target->nb[1]+=bytes; target->nb[2]=target->nb[1]*target->ne[1]+bytes; target->nb[3]=target->nb[2]*target->ne[2]+bytes;
        if(repeating) target=ggml_new_tensor_4d(ctx,type,input->ne[0]*2,input->ne[1]*(layout&1?3:1),input->ne[2]*(layout&2?2:1),input->ne[3]*2);
        auto spatial_op = [&](ggml_tensor *a, bool reference) -> ggml_tensor * {
          int args[8]={};
          if(operation==0) {
            for(int axis=0;axis<4;axis++) {const int crop=std::min<int64_t>(a->ne[axis],1); int l=0,r=0;
              switch(pattern/2) {
                case 1:l=1;r=2;break; case 2:l=2;break; case 3:r=2;break;
                case 4:l=-crop;break; case 5:r=-crop;break; case 6:l=-crop;r=2;break; case 7:l=2;r=-crop;break;
                case 8:l=INT32_MAX;r=-INT32_MAX;break; case 9:l=-INT32_MAX;r=INT32_MAX;break;
                case 10:l=a->ne[axis]>0?INT32_MIN:-INT32_MAX;r=INT32_MAX;break;
              } args[axis*2]=l;args[axis*2+1]=r;
            }
            if(reference && pattern%2) for(int axis=0;axis<4;axis++) if(a->ne[axis]) {
              const int64_t l=args[axis*2]%a->ne[axis], r=int64_t(args[axis*2])+args[axis*2+1]-l;
              GGML_ASSERT(r>=INT32_MIN && r<=INT32_MAX); args[axis*2]=l;args[axis*2+1]=r;
            }
            return pattern%2?ggml_pad_ext_circular(ctx,a,args[0],args[1],args[2],args[3],args[4],args[5],args[6],args[7]):ggml_pad_ext(ctx,a,args[0],args[1],args[2],args[3],args[4],args[5],args[6],args[7]);
          }
          if(operation==1) return ggml_pad_reflect_1d(ctx,a,pattern&1?a->ne[0]-1:0,pattern&2?a->ne[0]-1:0);
          for(int axis=0;axis<4;axis++) args[axis]=pattern?(((pattern-1)&(1<<axis))?-1:1)*(a->ne[axis]-1):0;
          return ggml_roll(ctx,a,args[0],args[1],args[2],args[3]);
        };
        ggml_tensor *result=spatial?spatial_op(operation?ggml_cont(ctx,input):input,false):concatenating?ggml_concat(ctx,input,right,operation):repeating?ggml_repeat(ctx,input,target):operation==0?ggml_dup(ctx,input):(operation==1?(layout==3?ggml_cont_1d(ctx,input,ggml_nelements(input)):ggml_cont(ctx,input)):ggml_cpy(ctx,input,target));
        ggml_tensor *expected_tensor=spatial?spatial_op(canonical_copy(ctx,input),true):concatenating?ggml_concat(ctx,canonical_copy(ctx,input),canonical_copy(ctx,right),operation):repeating?ggml_repeat(ctx,canonical_copy(ctx,input),target):canonical_copy(ctx,input);
        if(repeating || concatenating || spatial) {result->nb[1]+=bytes;result->nb[2]=result->nb[1]*result->ne[1]+bytes;result->nb[3]=result->nb[2]*result->ne[2]+bytes;}
        ggml_backend_buffer_t buffer=ggml_backend_alloc_ctx_tensors(ctx,backend); CHECK(buffer || (!ggml_nbytes(input)&&(!right||!ggml_nbytes(right))&&!ggml_nbytes(result)));
        std::vector<unsigned char> output(ggml_nbytes(result),0xA5), expected(ggml_nbytes(expected_tensor));
        for(ggml_tensor *operand : {input,right}) if(operand) {
          std::vector<unsigned char> initial(ggml_nbytes(operand));
          for(size_t i=0;i<initial.size();i++) initial[i]=(i*37+kind*13+layout+(operand==right?71:0))%256;
          if(!initial.empty()) ggml_backend_tensor_set(operand,initial.data(),0,initial.size());
        }
        if(!output.empty()) ggml_backend_tensor_set(result,output.data(),0,output.size());
        ggml_cgraph *graph=ggml_new_graph(ctx); ggml_build_forward_expand(graph,result);
        if(spatial && operation==0 && pattern%2 && ggml_is_empty(input) && !ggml_is_empty(result)) {
          CHECK(!ggml_backend_dev_supports_op(dev,result)); CHECK(ggml_backend_graph_compute(backend,graph)==GGML_STATUS_FAILED);
          std::vector<unsigned char> unchanged(output.size()); ggml_backend_tensor_get(result,unchanged.data(),0,unchanged.size()); CHECK(unchanged==output);
          ggml_backend_buffer_free(buffer); ggml_free(ctx); cases++;refused++;continue;
        }
        ggml_cgraph *oracle_graph=ggml_new_graph(ctx); ggml_build_forward_expand(oracle_graph,expected_tensor);
        CHECK(ggml_backend_graph_compute(reference,oracle_graph)==GGML_STATUS_SUCCESS);
        if(!expected.empty()) ggml_backend_tensor_get(expected_tensor,expected.data(),0,expected.size());
        if(kind==0 && pattern==0 && units==(spatial && operation?2:repeating?3:1) && rows==(spatial && operation?2:1) && batches==(spatial && operation==2?2:1) && layout==0) {
          const int64_t saved_extent=result->ne[0]; result->ne[0]=saved_extent+(repeating?1:block); CHECK(!ggml_backend_dev_supports_op(dev,result));
          CHECK(ggml_backend_graph_compute(backend,graph)==GGML_STATUS_FAILED); result->ne[0]=saved_extent;
          ggml_tensor *saved=result->src[concatenating || (!spatial && operation==2)?1:0]; result->src[concatenating || (!spatial && operation==2)?1:0]=nullptr;
          CHECK(!ggml_backend_dev_supports_op(dev,result));
          CHECK(ggml_backend_graph_compute(backend,graph)==GGML_STATUS_FAILED); result->src[concatenating || (!spatial && operation==2)?1:0]=saved;
          if(concatenating) {const int32_t axis=4; memcpy(result->op_params,&axis,4); CHECK(!ggml_backend_dev_supports_op(dev,result));
            CHECK(ggml_backend_graph_compute(backend,graph)==GGML_STATUS_FAILED); memcpy(result->op_params,&operation,4);
            right->type=GGML_TYPE_F16; CHECK(!ggml_backend_dev_supports_op(dev,result)); CHECK(ggml_backend_graph_compute(backend,graph)==GGML_STATUS_FAILED); right->type=type;
            const int other=(operation+1)%4; right->ne[other]++; CHECK(!ggml_backend_dev_supports_op(dev,result));
            CHECK(ggml_backend_graph_compute(backend,graph)==GGML_STATUS_FAILED); right->ne[other]--; }
          if(spatial) {const int32_t bad=operation==2?input->ne[0]:-1; memcpy(result->op_params,&bad,4); CHECK(!ggml_backend_dev_supports_op(dev,result));
            CHECK(ggml_backend_graph_compute(backend,graph)==GGML_STATUS_FAILED); memset(result->op_params,0,4);}
          std::vector<unsigned char> unchanged(output.size()); ggml_backend_tensor_get(result,unchanged.data(),0,unchanged.size()); CHECK(unchanged==output);
        }
        CHECK(ggml_backend_dev_supports_op(dev,result)); CHECK(ggml_backend_graph_compute(backend,graph)==GGML_STATUS_SUCCESS);
        std::vector<unsigned char> actual(output.size()); if(!actual.empty()) ggml_backend_tensor_get(result,actual.data(),0,actual.size());
        size_t index=0;
        for(int64_t i3=0;i3<result->ne[3];i3++) for(int64_t i2=0;i2<result->ne[2];i2++)
        for(int64_t i1=0;i1<result->ne[1];i1++) for(int64_t i0=0;i0<result->ne[0]/block;i0++,index++) {
          const size_t offset=i0*result->nb[0]+i1*result->nb[1]+i2*result->nb[2]+i3*result->nb[3];
          memcpy(output.data()+offset,expected.data()+index*bytes,bytes);
        }
        if(actual!=output) {
          const size_t at=std::mismatch(actual.begin(),actual.end(),output.begin()).first-actual.begin();
          fprintf(stderr,"copy mismatch type=%d op=%d layout=%d pattern=%d shape=%d,%d,2,%d byte=%zu actual=%u expected=%u\n",kind,operation,layout,pattern,units,rows,batches,at,actual[at],output[at]); return 2;
        }
        ggml_backend_buffer_free(buffer); ggml_free(ctx); cases++;
      }
      }
      CHECK(refused==(spatial?200:0));
      ggml_backend_free(reference); ggml_backend_free(backend); printf("RESULT cases %zu\n",cases);return 0;
    }
    if (!strcmp(argv[4], "empty")) {
      ggml_backend_t reference=ggml_backend_cpu_init(); CHECK(reference);
      ggml_backend_cpu_set_n_threads(reference,1);
      size_t cases=0; int kind;
      while (scanf("%d",&kind)==1) for (int axis=0;axis<4;axis++) for (int64_t scale : {int64_t(2),int64_t(INT32_MAX)}) {
        ggml_init_params params={8*1024*1024,nullptr,true};
        ggml_context *ctx=ggml_init(params); CHECK(ctx);
        int64_t ne[]={ggml_blck_size((ggml_type)kind)*2,3,2,2};
        ne[axis==3?2:3]=scale; ne[axis]=0;
        ggml_tensor *input=ggml_new_tensor(ctx,(ggml_type)kind,4,ne);
        ggml_tensor *copy=ggml_cpy(ctx,input,ggml_dup_tensor(ctx,input));
        for (ggml_tensor *result : {ggml_dup(ctx,input),ggml_cont(ctx,input),copy}) {
          CHECK(ggml_is_empty(result) && !result->data && !input->data);
          ggml_cgraph *graph=ggml_new_graph(ctx); ggml_build_forward_expand(graph,result);
          CHECK(ggml_graph_n_nodes(graph)==1);
          CHECK(ggml_backend_graph_compute(reference,graph)==GGML_STATUS_SUCCESS);
          CHECK(ggml_backend_dev_supports_op(dev,result));
          CHECK(ggml_backend_graph_compute(backend,graph)==GGML_STATUS_SUCCESS);
          CHECK(!result->data && !input->data); cases++;
        }
        ggml_free(ctx);
      }
      ggml_backend_free(reference); ggml_backend_free(backend);
      printf("RESULT cases %zu\n",cases); return 0;
    }
    if (!strcmp(argv[4], "elementwise")) {
      ggml_backend_t reference = ggml_backend_cpu_init(); CHECK(reference);
      ggml_backend_cpu_set_n_threads(reference, 1);
      size_t cases = 0;
      const ggml_op expected_ops[] = {
        GGML_OP_ADD, GGML_OP_SUB, GGML_OP_MUL, GGML_OP_DIV, GGML_OP_SQRT, GGML_OP_SQR, GGML_OP_UNARY, GGML_OP_UNARY
      };
      const uint32_t edges[] = {0,0x80000000,0x3F800000,0xBF800000,0x7F800000,0xFF800000,0x7FC00001,1,
        0x7F7FFFFF,0x00800000,0x007FFFFF,0x3F800001,0x3F000000,0x00800001,0x80000001,0x7F800001};
      for (int kind=0; kind<8; kind++) for (int width : {1,17,257}) for (int padded : {0,1})
      for (int broadcast=0; broadcast<(kind<4?4:1); broadcast++) {
        ggml_init_params params = {8*1024*1024, nullptr, true};
        ggml_context *ctx = ggml_init(params); CHECK(ctx);
        ggml_tensor *left=ggml_new_tensor_4d(ctx, GGML_TYPE_F32,width,3,2,2);
        const int64_t r0=(broadcast==1||broadcast==3)?1:width;
        const int64_t r1=(broadcast==1||broadcast==2)?1:3;
        const int64_t r2=broadcast?1:2, r3=(broadcast==1||broadcast==2)?1:2;
        ggml_tensor *right=kind<4?ggml_new_tensor_4d(ctx,GGML_TYPE_F32,r0,r1,r2,r3):nullptr;
        for (ggml_tensor *operand : {left,right}) if (operand) {
          operand->nb[1]=(operand->ne[0]+padded)*sizeof(float);
          operand->nb[2]=operand->nb[1]*operand->ne[1]+padded*sizeof(float);
          operand->nb[3]=operand->nb[2]*operand->ne[2]+padded*sizeof(float);
        }
        auto operation = [&](ggml_tensor *a, ggml_tensor *b) -> ggml_tensor * {
          switch(kind) {
            case 0: return ggml_add(ctx,a,b);
            case 1: return ggml_sub(ctx,a,b);
            case 2: return ggml_mul(ctx,a,b);
            case 3: return ggml_div(ctx,a,b);
            case 4: return ggml_sqrt(ctx,a);
            case 5: return ggml_sqr(ctx,a);
            case 6: return ggml_neg(ctx,a);
            case 7: return ggml_abs(ctx,a);
            default: return nullptr;
          }
        };
        ggml_tensor *result=operation(left,right);
        ggml_tensor *reference_result=operation(ggml_cont(ctx,left),right?ggml_cont(ctx,right):nullptr);
        CHECK(result && result->op==expected_ops[kind]);
        if (kind>=6) CHECK(ggml_get_unary_op(result)==(kind==6?GGML_UNARY_OP_NEG:GGML_UNARY_OP_ABS));
        ggml_backend_buffer_t buffer=ggml_backend_alloc_ctx_tensors(ctx,backend); CHECK(buffer);
        for (ggml_tensor *operand : {left,right}) if (operand) {
          std::vector<unsigned char> bytes(ggml_nbytes(operand),0xA5);
          size_t index=0;
          for (int64_t i3=0;i3<operand->ne[3];i3++) for (int64_t i2=0;i2<operand->ne[2];i2++)
          for (int64_t i1=0;i1<operand->ne[1];i1++) for (int64_t i0=0;i0<operand->ne[0];i0++,index++) {
            uint32_t bits=index<16?edges[(index*(operand==left?1:5)+(operand==left?0:3))%16]:uint32_t(index*2654435761U+(operand==left?0x9E3779B9U:0x85EBCA6BU));
            memcpy(bytes.data()+i0*operand->nb[0]+i1*operand->nb[1]+i2*operand->nb[2]+i3*operand->nb[3],&bits,4);
          }
          ggml_backend_tensor_set(operand,bytes.data(),0,bytes.size());
        }
        ggml_cgraph *graph=ggml_new_graph(ctx); ggml_build_forward_expand(graph,result);
        CHECK(ggml_backend_dev_supports_op(dev,result));
        ggml_cgraph *reference_graph=ggml_new_graph(ctx); ggml_build_forward_expand(reference_graph,reference_result);
        CHECK(ggml_backend_dev_supports_op(ggml_backend_get_device(reference),reference_result));
        CHECK(ggml_backend_graph_compute(reference,reference_graph)==GGML_STATUS_SUCCESS);
        std::vector<uint32_t> expected(ggml_nelements(result)),actual(expected.size());
        ggml_backend_tensor_get(reference_result,expected.data(),0,ggml_nbytes(reference_result));
        ggml_backend_tensor_memset(result,0xA5,0,ggml_nbytes(result));
        CHECK(ggml_backend_graph_compute(backend,graph)==GGML_STATUS_SUCCESS);
        ggml_backend_tensor_get(result,actual.data(),0,ggml_nbytes(result));
        for(size_t i=0;i<expected.size();i++) {
          float e,a; memcpy(&e,&expected[i],4); memcpy(&a,&actual[i],4);
          if (!(std::isnan(e)&&std::isnan(a)) && expected[i]!=actual[i]) {
            fprintf(stderr,"elementwise mismatch kind=%d width=%d padded=%d repeat=%d element=%zu expected=%08x actual=%08x\n",kind,width,padded,broadcast,i,expected[i],actual[i]); return 2;
          }
        }
        if (width==1 && padded==0 && broadcast==0 && (kind==0 || kind==6)) {
          auto refused_without_writes = [&]() -> bool {
            if (ggml_backend_dev_supports_op(dev,result)) return false;
            ggml_backend_tensor_memset(result,0xA5,0,ggml_nbytes(result));
            if (ggml_backend_graph_compute(backend,graph)!=GGML_STATUS_FAILED) return false;
            std::vector<unsigned char> bytes(ggml_nbytes(result));
            ggml_backend_tensor_get(result,bytes.data(),0,bytes.size());
            return std::all_of(bytes.begin(),bytes.end(),[](unsigned char b){ return b==0xA5; });
          };
          if (kind==0) {
            result->src[1]=nullptr; CHECK(refused_without_writes()); result->src[1]=right;
            right->ne[1]=2; CHECK(refused_without_writes()); right->ne[1]=3;
            result->ne[1]=2; CHECK(!ggml_backend_dev_supports_op(dev,result)); result->ne[1]=3;
            const auto saved_type=left->type; left->type=GGML_TYPE_COUNT;
            CHECK(refused_without_writes()); left->type=saved_type;
          } else {
            const int32_t bad_variant=GGML_UNARY_OP_COUNT;
            int32_t saved_variant; memcpy(&saved_variant,result->op_params,sizeof(saved_variant));
            memcpy(result->op_params,&bad_variant,sizeof(bad_variant)); CHECK(refused_without_writes());
            memcpy(result->op_params,&saved_variant,sizeof(saved_variant));
          }
        }
        ggml_backend_buffer_free(buffer); ggml_free(ctx); cases++;
      }
      CHECK(ggml_backend_reg_count()==1 && ggml_backend_dev_count()==1);
      ggml_backend_free(reference); ggml_backend_free(backend);
      printf("RESULT cases %zu\n",cases); return 0;
    }
    if (!strcmp(argv[4], "matrix")) {
      ggml_backend_t reference = ggml_backend_cpu_init(); CHECK(reference);
      ggml_backend_cpu_set_n_threads(reference, 1);
      size_t matrix_cases = 0; bool negatives_checked = false;
      const float edges[][2] = {{INFINITY,0.0f}, {INFINITY,-1.0f}, {NAN,1.0f}, {-0.0f,1.0f},
        {std::numeric_limits<float>::denorm_min(),1.0f}, {std::numeric_limits<float>::min(),0.5f},
        {std::numeric_limits<float>::max(),2.0f}, {-INFINITY,-INFINITY}};
      const int batches[][4] = {{1,1,1,1}, {1,1,3,2}, {2,1,4,2}, {2,2,4,4}};
      for (int rows : {0,1,2,5}) for (int columns : {0,1,3,7}) for (int inner : {0,1,2,7,33})
      for (int padded : {0,1}) for (const auto &batch : batches) {
        ggml_init_params params = {8*1024*1024, nullptr, true};
        ggml_context *ctx = ggml_init(params); CHECK(ctx);
        ggml_tensor *left = ggml_new_tensor_4d(ctx, GGML_TYPE_F32, inner, rows, batch[0], batch[1]);
        ggml_tensor *right = ggml_new_tensor_4d(ctx, GGML_TYPE_F32, inner, columns, batch[2], batch[3]);
        for (ggml_tensor *operand : {left, right}) {
          operand->nb[1] = (std::max(inner, 1)+padded)*sizeof(float);
          operand->nb[2] = operand->nb[1]*operand->ne[1]+padded*sizeof(float);
          operand->nb[3] = operand->nb[2]*operand->ne[2]+padded*sizeof(float);
        }
        ggml_tensor *product = ggml_mul_mat(ctx, left, right); CHECK(product);
        ggml_backend_buffer_t buffer = ggml_backend_alloc_ctx_tensors(ctx, backend);
        CHECK(buffer || (!ggml_nbytes(left) && !ggml_nbytes(right) && !ggml_nbytes(product)));
        for (ggml_tensor *operand : {left, right}) {
          std::vector<unsigned char> bytes(ggml_nbytes(operand), 0xA5);
          for (int64_t i3=0; i3<operand->ne[3]; i3++) for (int64_t i2=0; i2<operand->ne[2]; i2++)
          for (int64_t i1=0; i1<operand->ne[1]; i1++) for (int64_t i0=0; i0<operand->ne[0]; i0++) {
            float value = float((i0*7+i1*11+i2*3+i3*5+(operand==left?1:2))%19-9)/4.0f;
            if (rows==1 && columns==1 && inner==1) value=edges[padded*4+(&batch-batches)][operand==left?0:1];
            memcpy(bytes.data()+i0*operand->nb[0]+i1*operand->nb[1]+i2*operand->nb[2]+i3*operand->nb[3], &value, sizeof(value));
          }
          if (!bytes.empty()) ggml_backend_tensor_set(operand, bytes.data(), 0, bytes.size());
        }
        ggml_cgraph *graph = ggml_new_graph(ctx); ggml_build_forward_expand(graph, product);
        CHECK(ggml_backend_dev_supports_op(dev, product));
        CHECK(ggml_backend_graph_compute(reference, graph) == GGML_STATUS_SUCCESS);
        std::vector<float> expected(ggml_nelements(product)), actual(expected.size());
        if (ggml_nbytes(product)) {
          ggml_backend_tensor_get(product, expected.data(), 0, ggml_nbytes(product));
          ggml_backend_tensor_memset(product, 0xA5, 0, ggml_nbytes(product));
        }
        if (!negatives_checked && ggml_nbytes(product) && ggml_nbytes(left)) {
          negatives_checked = true;
          product->ne[0]++;
          CHECK(!ggml_backend_dev_supports_op(dev, product));
          CHECK(ggml_backend_graph_compute(backend, graph) == GGML_STATUS_FAILED);
          product->ne[0]--;
          ggml_tensor *saved_left = product->src[0]; product->src[0] = nullptr;
          CHECK(!ggml_backend_dev_supports_op(dev, product));
          CHECK(ggml_backend_graph_compute(backend, graph) == GGML_STATUS_FAILED);
          product->src[0] = saved_left;
          ggml_backend_buffer_t saved_buffer = product->buffer; product->buffer = nullptr;
          CHECK(ggml_backend_graph_compute(backend, graph) == GGML_STATUS_FAILED);
          product->buffer = saved_buffer;
          ggml_backend_buffer_t foreign = ggml_backend_cpu_buffer_from_ptr(left->data, ggml_nbytes(left)); CHECK(foreign);
          saved_buffer = left->buffer; left->buffer = foreign;
          CHECK(ggml_backend_graph_compute(backend, graph) == GGML_STATUS_FAILED);
          left->buffer = saved_buffer; ggml_backend_buffer_free(foreign);
          ggml_tensor *unsupported = ggml_dup(ctx, product);
          ggml_cgraph *bad_graph = ggml_new_graph(ctx); ggml_build_forward_expand(bad_graph, unsupported);
          unsupported->op = GGML_OP_COUNT;
          CHECK(ggml_backend_graph_compute(backend, bad_graph) == GGML_STATUS_FAILED);
          std::vector<unsigned char> unchanged(ggml_nbytes(product));
          ggml_backend_tensor_get(product, unchanged.data(), 0, unchanged.size());
          CHECK(std::all_of(unchanged.begin(), unchanged.end(), [](unsigned char b){return b==0xA5;}));
        }
        CHECK(ggml_backend_graph_compute(backend, graph) == GGML_STATUS_SUCCESS);
        if (ggml_nbytes(product)) ggml_backend_tensor_get(product, actual.data(), 0, ggml_nbytes(product));
        for (size_t i=0; i<expected.size(); i++) CHECK((std::isnan(expected[i]) && std::isnan(actual[i])) || expected[i] == actual[i]);
        ggml_backend_buffer_free(buffer); ggml_free(ctx); matrix_cases++;
      }
      CHECK(negatives_checked);
      CHECK(ggml_backend_reg_count() == 1 && ggml_backend_dev_count() == 1);
      ggml_backend_free(reference); ggml_backend_free(backend);
      printf("RESULT cases %zu\n", matrix_cases); return 0;
    }
    ggml_backend_buffer_t empty = ggml_backend_alloc_buffer(backend, 0);
    CHECK(empty && !ggml_backend_buffer_get_base(empty) && ggml_backend_buffer_get_size(empty) == 0);
    ggml_backend_buffer_clear(empty, 127); ggml_backend_buffer_free(empty);
    CHECK(!ggml_backend_alloc_buffer(backend, std::numeric_limits<size_t>::max()));
    CHECK(!ggml_backend_alloc_buffer(backend, (size_t)std::numeric_limits<intptr_t>::max()+1));
    CHECK(!ggml_backend_dev_buffer_from_host_ptr(dev, nullptr, 1, 1));
    size_t cases = 0; int kind;
    while (scanf("%d", &kind) == 1) {
      for (int layout = 0; layout < 3; layout++) {
        current_kind = kind; current_layout = layout;
        ggml_tensor tensor = {}; tensor.type = (ggml_type)kind;
        const int64_t block = ggml_blck_size(tensor.type);
        const size_t element = ggml_type_size(tensor.type);
        tensor.ne[0] = layout == 2 ? 0 : block*3; tensor.ne[1] = 2; tensor.ne[2] = 2; tensor.ne[3] = 1;
        tensor.nb[0] = element; tensor.nb[1] = element*3 + (layout == 1 ? element : 0);
        tensor.nb[2] = tensor.nb[1]*2 + (layout == 1 ? element : 0); tensor.nb[3] = tensor.nb[2]*2;
        const size_t size = ggml_nbytes(&tensor);
        CHECK(ggml_backend_buft_get_alloc_size(buft, &tensor) == size);
        ggml_backend_buffer_t buffer = ggml_backend_alloc_buffer(backend, size+align); CHECK(buffer);
        ggml_tallocr allocator = ggml_tallocr_new(buffer);
        CHECK(ggml_tallocr_alloc(&allocator, &tensor) == GGML_STATUS_SUCCESS);
        CHECK(tensor.buffer == buffer && (uintptr_t)tensor.data % align == 0);
        std::vector<unsigned char> expected(size), output(size), input(size);
        ggml_backend_tensor_get(&tensor, output.data(), 0, size);
        CHECK(output == expected);
        ggml_backend_buffer_clear(buffer, 0x41); std::fill(expected.begin(), expected.end(), 0x41);
        ggml_backend_tensor_get(&tensor, output.data(), 0, size); CHECK(output == expected);
        for (size_t i=0; i<size; i++) input[i] = (i*37 + cases) % 256;
        ggml_backend_tensor_set(&tensor, input.data(), 0, size); expected = input;
        if (size) {
          const size_t at = size/3, length = std::min<size_t>(5, size-at);
          ggml_backend_tensor_memset(&tensor, 0x23, at, length);
          std::fill(expected.begin()+at, expected.begin()+at+length, 0x23);
          ggml_backend_tensor_set(&tensor, input.data(), at, length);
          std::copy(input.begin(), input.begin()+length, expected.begin()+at);
          const unsigned char columns[] = {0x73, 0x35, 0x11};
          ggml_backend_tensor_set_2d(&tensor, columns, 0, 1, 3, 2, 1);
          for (size_t i=0; i<3; i++) expected[i*2] = columns[i];
          unsigned char gathered[3] = {};
          ggml_backend_tensor_get_2d(&tensor, gathered, 0, 1, 3, 2, 1);
          CHECK(!memcmp(gathered, columns, 3));
          ggml_tensor view = {}; view.type = GGML_TYPE_I8; view.ne[0] = std::min<size_t>(7,size-1);
          for (int d=1; d<4; d++) view.ne[d]=1;
          for (int d=0; d<4; d++) view.nb[d]=1;
          view.view_src = &tensor; view.view_offs = 1;
          CHECK(ggml_backend_view_init(&view) == GGML_STATUS_SUCCESS);
          std::vector<unsigned char> viewed(view.ne[0]);
          ggml_backend_tensor_get(&view, viewed.data(), 0, viewed.size());
          CHECK(std::equal(viewed.begin(), viewed.end(), expected.begin()+1));
          std::fill(viewed.begin(), viewed.end(), 0x37);
          ggml_backend_tensor_set(&view, viewed.data(), 0, viewed.size());
          std::copy(viewed.begin(), viewed.end(), expected.begin()+1);
        }
        ggml_backend_tensor_get(&tensor, output.data(), 0, size); CHECK(output == expected);
        ggml_tensor copy = tensor; copy.buffer = nullptr; copy.data = nullptr;
        ggml_backend_buffer_t other = ggml_backend_alloc_buffer(backend, size+align); CHECK(other);
        ggml_tallocr second = ggml_tallocr_new(other);
        CHECK(ggml_tallocr_alloc(&second, &copy) == GGML_STATUS_SUCCESS);
        ggml_backend_tensor_copy(&tensor, &copy);
        ggml_backend_tensor_get(&copy, output.data(), 0, size); CHECK(output == expected);
        std::vector<unsigned char> host(size ? size : 1, 0x29);
        ggml_backend_buffer_t borrowed = ggml_backend_dev_buffer_from_host_ptr(dev, host.data(), size, size); CHECK(borrowed);
        ggml_tensor mapped = tensor; mapped.buffer = nullptr; mapped.data = nullptr;
        CHECK(ggml_backend_tensor_alloc(borrowed, &mapped, ggml_backend_buffer_get_base(borrowed)) == GGML_STATUS_SUCCESS);
        ggml_backend_tensor_get(&mapped, output.data(), 0, size);
        CHECK(std::all_of(output.begin(), output.end(), [](unsigned char b){return b==0x29;}));
        ggml_backend_buffer_free(borrowed); host[0] = 0x66;
        ggml_tensor invalid = tensor; invalid.type = (ggml_type)4;
        CHECK(buffer->iface.init_tensor(buffer, &invalid) == GGML_STATUS_FAILED);
        invalid = tensor; invalid.ne[0] = -1;
        CHECK(buffer->iface.init_tensor(buffer, &invalid) == GGML_STATUS_FAILED);
        if (size) {
          fflush(stdout); const pid_t child = fork(); CHECK(child >= 0);
          if (!child) { ggml_set_abort_callback(refusal); unsigned char byte=0; buffer->iface.set_tensor(buffer, &tensor, &byte, size, 1); _exit(99); }
          int status; CHECK(waitpid(child, &status, 0) == child);
          CHECK(WIFEXITED(status) && WEXITSTATUS(status) == 12);
        }
        ggml_backend_buffer_free(other); ggml_backend_buffer_free(buffer); cases++;
      }
    }
    ggml_init_params params = {8*1024*1024, nullptr, true};
    ggml_context *ctx = ggml_init(params); CHECK(ctx);
    ggml_tensor *input = ggml_new_tensor_1d(ctx, GGML_TYPE_F32, 4);
    ggml_backend_buffer_t graph_buffer = ggml_backend_alloc_buffer(backend, 128); CHECK(graph_buffer);
    ggml_tallocr graph_allocator = ggml_tallocr_new(graph_buffer);
    CHECK(ggml_tallocr_alloc(&graph_allocator, input) == GGML_STATUS_SUCCESS);
    ggml_tensor *reshape = ggml_reshape_2d(ctx, input, 2, 2);
    CHECK(ggml_backend_view_init(reshape) == GGML_STATUS_SUCCESS);
    ggml_cgraph *graph = ggml_new_graph(ctx); ggml_build_forward_expand(graph, reshape);
    CHECK(ggml_backend_dev_supports_op(dev, reshape));
    CHECK(ggml_backend_graph_compute(backend, graph) == GGML_STATUS_SUCCESS);
    ggml_tensor *invalid_op = ggml_dup(ctx, input);
    ggml_cgraph *unsupported = ggml_new_graph(ctx); ggml_build_forward_expand(unsupported, invalid_op);
    invalid_op->op = GGML_OP_COUNT;
    CHECK(!ggml_backend_dev_supports_op(dev, invalid_op));
    CHECK(ggml_backend_graph_compute(backend, unsupported) == GGML_STATUS_FAILED);
    ggml_backend_buffer_free(graph_buffer); ggml_free(ctx); ggml_backend_free(backend);
    printf("RESULT cases %zu\n", cases); return 0;
}
"##;
}
