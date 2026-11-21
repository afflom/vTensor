//! The `contract` suite: one test per scenario in `features/suites/contract.feature`.
//!
//! The proofs are checked by the Lean kernel when `just vv` runs
//! `prismpm verify` on `app/`. What this test holds fixed is the claim those
//! proofs are about: the theorem's exact statement and axioms, the six
//! universal policy files they quantify over, and that the verdict the
//! application returns is decided by the function they prove. A weakened
//! statement, a dropped file, an extra axiom, or a verdict decided some other
//! way would still verify, and would no longer be the claim `TC-01` makes.

use repo_model::repo_root as root;

/// The body of the top-level declaration named `name`, up to its closing brace.
fn declaration<'a>(model: &'a str, name: &str) -> &'a str {
    let (_, body) = model
        .split_once(&format!("\n  name: {name},\n"))
        .unwrap_or_else(|| panic!("{name} is not declared"));
    body.split_once("\n}\n")
        .unwrap_or_else(|| panic!("{name} is not closed"))
        .0
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
    let contract = ggml_oracle::read_json(root().join("template-contract.json"));
    let bound: Vec<_> = contract["universal_policy_paths"]
        .as_array()
        .expect("universal_policy_paths")
        .iter()
        .map(|path| path.as_str().expect("policy path"))
        .filter(|path| *path != "template.lock")
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
        let capacities = [0, 1, alignment - 1, alignment, alignment + 1, alignment * 4];
        let mut sizes = capacities;
        *sizes.last_mut().unwrap() = alignment * 2 + 1;
        for [offset, capacity, size] in
            ggml_oracle::product([&[0, alignment, alignment * 3], &capacities, &sizes])
        {
            writeln!(input, "{capacity} {offset} {size} {alignment}").unwrap();
            cases.push((capacity, offset, size, alignment));
        }
    }
    assert_eq!(cases.len(), 864);
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
    // Prism proves metadataClassification for every opcode and span.
    native_backend_oracle::verify(&root(), "buffers", 108);
}

/// `VT-12`: GGML F32 matrix-product graphs execute through Verdant across rectangular shapes, padded storage and batch broadcasting and match the imported GGML CPU reference
#[test]
fn matrix_graphs_vt_12() {
    tensor_layout::certified_current_model(&root(), "matrix");
    tensor_layout::certified_current_model(&root(), "index");
    // The certified scalarDispatchForEveryOpcode theorem covers every Nat opcode.
    assert!(matches!(
        vtensor_matrix::run(28, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1),
        Ok(vtensor_matrix::Adt0::C2(..))
    ));
    // Preserve both malformed inner-dimension and indivisible batch cases.
    for (k, b2, r2, o2) in [(2, 1, 1, 1), (1, 2, 3, 3)] {
        assert!(matches!(
            vtensor_matrix::run(28, 0, 0, 0, k, 1, b2, 1, 1, 1, r2, 1, 1, 1, o2, 1),
            Ok(vtensor_matrix::Adt0::C1)
        ));
    }
    assert!(vtensor_index::run(u64::MAX, 0, 0, 0, 2, 0, 0, 0).is_err());
    native_backend_oracle::verify(&root(), "matrix", 1280);
}

/// `VT-13`: F32 arithmetic and unary graphs execute through Verdant with Prism-defined scalar semantics and broadcasting and match the imported GGML CPU reference
#[test]
fn elementwise_graphs_vt_13() {
    tensor_layout::certified_current_model(&root(), "pointwise");
    native_backend_oracle::verify(&root(), "elementwise", 120);
}

/// `VT-14`: Prism empty-node admission matches the imported GGML CPU no-op behavior for empty copy graphs across every live storage type without accessing operand buffers
#[test]
fn empty_graphs_vt_14() {
    native_backend_oracle::verify(&root(), "empty", 864);
}

/// `VT-15`: Prism plans same-type DUP, CONT and CPY across every live GGML storage type and Verdant moves the admitted blocks with upstream-equivalent logical order and preserved padding
#[test]
fn copy_graphs_vt_15() {
    native_backend_oracle::verify(&root(), "copy", 3456);
}

/// `VT-16`: Prism models four-axis REPEAT addressing and Verdant preserves encoded values for every type admitted by the imported GGML CPU repeat oracle
#[test]
fn repeat_graphs_vt_16() {
    native_backend_oracle::verify(&root(), "repeat", 160);
}

/// `VT-17`: Prism selects and addresses CONCAT source blocks on every axis and Verdant execution matches the imported GGML CPU oracle across all live storage types
#[test]
fn concat_graphs_vt_17() {
    native_backend_oracle::verify(&root(), "concat", 10368);
}

/// `VT-18`: Prism defines F32 zero and circular padding, reflected padding and signed four-axis roll addressing and Verdant execution matches the imported GGML CPU oracle
#[test]
fn spatial_graphs_vt_18() {
    native_backend_oracle::verify(&root(), "spatial", 2256);
}

/// `VT-19`: Prism plans SET and ACC initialization, bounded update views and aliases and Verdant execution matches the imported GGML CPU oracle for supported float and integer storage
#[test]
fn update_graphs_vt_19() {
    native_backend_oracle::verify(&root(), "update", 4608);
}

/// `VT-20`: Prism models four-axis REPEAT_BACK initialization and accumulation order and Verdant execution matches the imported GGML F32 CPU oracle across strided and empty tensors
#[test]
fn fold_graphs_vt_20() {
    native_backend_oracle::verify(&root(), "fold", 672);
}

/// `VT-21`: Prism defines batched DIAG placement and Verdant preserves diagonal encodings and positive-zero off-diagonal values for every type supported by the imported GGML CPU oracle
#[test]
fn diagonal_graphs_vt_21() {
    native_backend_oracle::verify(&root(), "diagonal", 384);
}

/// `VT-22`: Prism defines all four TRI variants and Verdant preserves retained encodings and zeroes masked elements for every type supported by the imported GGML CPU oracle
#[test]
fn triangular_graphs_vt_22() {
    native_backend_oracle::verify(&root(), "triangular", 1920);
}

/// `VT-23`: Prism plans copied and in-place FILL and Verdant converts and transfers F32 and F16 constants with upstream-equivalent encodings and preserved padding
#[test]
fn fill_graphs_vt_23() {
    native_backend_oracle::verify(&root(), "fill", 15360);
}

/// `VT-24`: Prism plans mixed F32 F16 and BF16 casts and copies by logical element order and Verdant conversion matches the imported GGML CPU oracle across reshaped strided and empty layouts
#[test]
fn cast_graphs_vt_24() {
    native_backend_oracle::verify(&root(), "cast", 1920);
}

/// `VT-25`: Prism models window partition padding and window join logical indexing and Verdant transfers match the imported GGML CPU oracle for independent strided empty and bounded window inputs
#[test]
fn window_graphs_vt_25() {
    native_backend_oracle::verify(&root(), "window", 3024);
}

/// `VT-26`: Prism plans scale bias clamp and leaky ReLU graphs and Verdant matches the imported GGML CPU kernels for every supported float storage type across copied in-place strided and empty tensors
#[test]
fn transform_graphs_vt_26() {
    native_backend_oracle::verify(&root(), "transform", 18816);
}

/// `VT-27`: Prism plans thirteen unary operations for all five supported GGML float source destination pairs and Verdant matches copied in-place strided and empty CPU graphs with exact finite outputs and preserved padding
#[test]
fn unary_graphs_vt_27() {
    native_backend_oracle::verify(&root(), "unary", 4992);
}

/// `VT-28`: Prism models row-local F32 cumulative sums with exact first-word preservation and Verdant matches the imported GGML CPU scan across strided and empty graphs
#[test]
fn scan_graphs_vt_28() {
    native_backend_oracle::verify(&root(), "scan", 528);
}

/// `VT-29`: Prism models SUM SUM_ROWS and MEAN accumulation precision rounding and empty reductions and Verdant matches every supported float type against the imported GGML CPU oracle
#[test]
fn reduction_graphs_vt_29() {
    native_backend_oracle::verify(&root(), "reduction", 4080);
}

/// `VT-30`: Prism models ARGMAX last-tie and NaN state and logical COUNT_EQUAL across all four tensor axes and Verdant matches imported GGML CPU row operations with typed refusals and exact integer outputs
#[test]
fn row_graphs_vt_30() {
    use vtensor_pointwise::{Adt0, Adt1, Adt2, Adt5, Adt6};
    for (operation, ty, output_type) in [(13, 0, 0), (18, 26, 27)] {
        let empty = Adt2::C0(ty, 0, 1 << 40, 1 << 40, 1 << 40, 4, 0, 0, 0, 0, 0);
        let output = Adt2::C0(output_type, 1, 1, 1, 1, 8, 8, 8, 8, 8, 0);
        let params = Adt1::C0(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
        assert!(matches!(
            vtensor_pointwise::run(Adt0::C0(
                operation,
                params,
                Some(empty.clone()),
                Some(empty),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                output
            )),
            Ok(Adt5::C0(Adt6::C4(2, ..)))
        ));
    }
    native_backend_oracle::verify(&root(), "rows", 2264);
}

/// `VT-31`: Prism models indexed F16 and F32 row-gradient accumulation into F32 and Verdant matches imported GGML GET_ROWS_BACK graphs with duplicate indices empty tensors and atomic invalid-index refusals
#[test]
fn row_gradient_graphs_vt_31() {
    native_backend_oracle::verify(&root(), "row-gradient", 3360);
}

/// `VT-32`: Prism models four-dimensional indexed row gathering and every executable GGML CPU source format and Verdant decodes the packed blocks with imported authoritative codebooks while preserving integer words padding empty tensors and atomic invalid-index refusals
#[test]
fn row_gather_graphs_vt_32() {
    native_backend_oracle::verify(&root(), "row-gather", 384);
}

/// `VT-33`: Prism models GGML relative-position word selection and Verdant preserves F16 and BF16 source encodings in F16 outputs across strided empty and higher-dimensional source layouts
#[test]
fn relative_position_graphs_vt_33() {
    native_backend_oracle::verify(&root(), "relative-position", 192);
}

/// `VT-34`: Prism models three-source indexed row addition and Verdant matches GGML ADD_ID arithmetic and exact minus-one row copies across strides empty tensors and atomic invalid-index refusals
#[test]
fn indexed_add_graphs_vt_34() {
    native_backend_oracle::verify(&root(), "indexed-add", 164);
}

/// `VT-35`: Prism generates all matrix fixture dimensions layouts and input words matching the complete frozen prior oracle corpus while GGML independently computes every expected result
#[test]
fn fixture_generation_vt_35() {
    native_backend_oracle::fixture_input(0);
    use vtensor_fixtures::Adt0 as Request;
    for (family, request) in [
        (1, Request::C0(0)),
        (0, Request::C0(4)),
        (0, Request::C1(1280, 0)),
        (0, Request::C1(0, 24)),
        (0, Request::C2(0, 2, 0)),
        (0, Request::C3(0, 0, 0)),
    ] {
        assert_eq!(vtensor_fixtures::run(family, request).unwrap(), None);
    }
}

/// `VT-36`: Prism generates every arithmetic fixture descriptor and input word while preserving the complete frozen broadcasting and exceptional-word corpus and the independent GGML CPU numerical oracle
#[test]
fn arithmetic_fixture_generation_vt_36() {
    native_backend_oracle::fixture_input(1);
    for query in [
        (1, 120, 0, 0),
        (1, 0, 0, 24),
        (2, 0, 2, 0),
        (3, 96, 1, 0),
        (4, 0, 0, 0),
    ] {
        assert_eq!(
            vtensor_arithmetic::run(query.0, query.1, query.2, query.3).unwrap(),
            None
        );
    }
}

mod ggml_oracle {

    use sha2::{Digest, Sha256};
    use std::io::Write;
    use std::path::{Path, PathBuf};
    use std::process::{Command, Stdio};

    pub(super) fn product<const N: usize>(
        domains: [&[u64]; N],
    ) -> impl Iterator<Item = [u64; N]> + '_ {
        (0..domains.iter().map(|d| d.len()).product()).map(move |mut ordinal| {
            let mut row = [0; N];
            for axis in (0..N).rev() {
                row[axis] = domains[axis][ordinal % domains[axis].len()];
                ordinal /= domains[axis].len();
            }
            row
        })
    }

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
        repo_model::artifact::sha256_hex(
            &std::fs::read(path).unwrap_or_else(|error| panic!("{}: {error}", path.display())),
        )
    }

    pub(super) use repo_model::artifact::digest_hex as digest;

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
        for (name, bytes) in [
            ("storage_oracle.c", STORAGE_ORACLE_SOURCE),
            ("transfer.cpp", TRANSFER_ORACLE_SOURCE),
            ("allocation.cpp", ALLOCATION_ORACLE_SOURCE),
        ] {
            std::fs::write(cache.join(name), bytes).expect("oracle transport");
        }
        for (input, object) in [
            (source.join("ggml.c"), "ggml.o"),
            (source.join("ggml-quants.c"), "quants.o"),
            (source.join("ggml-alloc.c"), "alloc.o"),
            (cache.join("storage_oracle.c"), "storage.o"),
            (cache.join("allocation.cpp"), "allocation.o"),
            (source.join("ggml-backend.cpp"), "backend.o"),
            (source.join("ggml-backend-meta.cpp"), "meta.o"),
            (cache.join("transfer.cpp"), "transfer.o"),
        ] {
            compile(&cache, input, cache.join(object));
        }
        let objects: serde_json::Map<String, serde_json::Value> =
            ["ggml.o", "quants.o", "alloc.o", "backend.o", "meta.o"]
                .into_iter()
                .map(|name| (name.into(), file_digest(cache.join(name)).into()))
                .collect();
        std::fs::write(
            cache.join("objects.json"),
            serde_json::to_vec(&objects).unwrap(),
        )
        .expect("object bindings");
        let programs: [(&str, &[&str]); 3] = [
            ("transfer", &["backend", "ggml", "quants"]),
            (
                "allocation",
                &["meta", "alloc", "backend", "ggml", "quants"],
            ),
            ("storage", &["ggml", "quants"]),
        ];
        for (name, shared) in programs {
            let destination = cache.join(format!("{name}-oracle"));
            let candidate = destination.with_extension("new");
            let objects = std::iter::once(name).chain(shared.iter().copied());
            run(Command::new("c++")
                .arg("-Wl,--gc-sections")
                .args(objects.map(|object| cache.join(format!("{object}.o"))))
                .args(["-lm", "-o"])
                .arg(&candidate));
            let stamp = match name {
                "storage" => "executable",
                other => other,
            };
            std::fs::write(
                cache.join(format!("{stamp}.sha256")),
                file_digest(&candidate),
            )
            .expect("cache digest");
            std::fs::rename(candidate, destination).expect("publish built oracle");
        }
        executable
    }

    /// Send a batch of tensor descriptions to the actual upstream `ggml_nbytes`.
    pub(super) fn batch(command: &mut Command, input: impl AsRef<[u8]>) -> String {
        let input = input.as_ref();
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
            let writer = scope.spawn(move || stdin.write_all(input));
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
    pub fn spans<const N: usize>(executable: &Path, input: &str) -> Vec<[u64; N]> {
        batch(&mut Command::new(executable), input)
            .lines()
            .map(|line| {
                let values = line
                    .split_whitespace()
                    .map(|v| v.parse::<u64>().expect("oracle integer"))
                    .collect::<Vec<_>>();
                values.try_into().expect("oracle column count")
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
        spans(&executable, input)
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
                symbol.starts_with(prefix)
                    && symbol
                        .bytes()
                        .all(|c| c.is_ascii_alphanumeric() || c == b'_'),
                "unexpected declaration: {symbol}"
            );
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
        for (key, value) in [
            ("spec", "vtensor/ggml-declarations/1"),
            ("authority", "LLAMA-CPP-GGML-BACKEND"),
        ] {
            assert_eq!(inventory[key].as_str(), Some(value));
        }
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

    use super::{ggml_oracle::file_digest, root};
    use std::collections::BTreeSet;

    use repo_model::Model;

    /// `VT-01`: The imported GGML backend contract, implementation interface, and operator oracle match their pinned upstream source
    #[test]
    fn oracle_import_vt_01() {
        let root = root();
        let artifact = root.join("vendor/llama.cpp-oracle");
        let lock = super::ggml_oracle::read_toml(artifact.join("upstream.toml"));

        for (key, value) in [
            ("repository", "https://github.com/ggml-org/llama.cpp"),
            ("commit", "9a080f44b503c64f66c84a6274b9d9ee5d5b09d5"),
        ] {
            assert_eq!(lock[key].as_str(), Some(value));
        }

        let files = lock["file"]
            .as_array()
            .expect("the lock records every imported source file");
        let mut paths = BTreeSet::new();
        for entry in files {
            let path = entry["path"].as_str().expect("source path");
            let expected = entry["sha256"].as_str().expect("source SHA-256");
            let observed = file_digest(artifact.join(path));
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
            file_digest(artifact.join("source.tar.gz")),
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
            "layout" | "native" | "matrix" | "index" | "pointwise" | "fixtures" | "arithmetic"
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
        let checked = ggml_oracle::run(
            std::process::Command::new("lexlean")
                .current_dir(root)
                .args([
                    "--project",
                    &project,
                    "--diagnostic-format",
                    "json",
                    "check",
                ]),
        );
        let current: serde_json::Value =
            serde_json::from_slice(&checked).expect("model identities");
        assert_eq!(
            current["source_id"], result["source"]["source_id"],
            "regenerate layout bundle after model changes"
        );
        assert_eq!(current["semantic_id"], result["source"]["semantic_id"]);
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
        assert_eq!(
            kernel_lock["output"]["sha256"],
            ggml_oracle::file_digest(
                root.join(kernel_lock["output"]["path"].as_str().expect("output path"))
            )
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
        ggml_oracle::run(
            std::process::Command::new("lexlean")
                .current_dir(root)
                .args(["production", "verify-artifact", &bundle]),
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
            for [multiples, rows, planes, batches, padding] in
                ggml_oracle::product([&[1, 2, 4], &[1, 3], &[1, 2], &[1, 2], &[0, 7]])
            {
                let ne = [block * multiples, rows, planes, batches];
                let nb0 = bytes + padding;
                let nb1 = multiples * nb0 + 11;
                let nb2 = rows * nb1 + 13;
                let nb = [nb0, nb1, nb2, planes * nb2 + 17];
                request(&mut inputs, kind, ne, nb);
                layouts.push(layout(ne, nb, block, bytes));
            }
            for axis in 0..4 {
                let mut ne = [block, 2, 2, 2];
                ne[axis] = 0;
                let nb = [bytes, bytes * 2, bytes * 4, bytes * 8];
                request(&mut inputs, kind, ne, nb);
                layouts.push(layout(ne, nb, block, bytes));
            }
        }
        let expected = ggml_oracle::spans::<3>(executable, &inputs);
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

    const VERDANT_ENGINE_REVISION: &str = "34c827db624f951609e49f57bbf324671876eeff";

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
    for [mode, buffer, data, capacity] in
        ggml_oracle::product([&[0, 1, 2, 3], &[0, 1], &[0, 1], &[0, 1, 8, 64]])
    {
        let bounds = [0, 1, capacity, capacity + 1];
        for [offset, size] in ggml_oracle::product([&bounds, &bounds]) {
            writeln!(input, "{mode} {buffer} {data} {capacity} {offset} {size}").unwrap();
            cases.push((buffer != 0, data != 0, capacity, offset, size));
        }
    }
    assert_eq!(cases.len(), 1024);
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
    for (buffer, data, capacity, offset, size, expected) in [
        (true, true, 64, u64::MAX, 2, 12),
        (true, true, u64::MAX, u64::MAX, 1, 12),
        (true, false, 0, u64::MAX, 0, 0),
        (false, false, 0, 0, 0, 10),
    ] {
        assert_eq!(
            outcome(vtensor_transfer::run(buffer, data, capacity, offset, size)),
            expected
        );
    }
}

mod native_backend_oracle {
    use super::ggml_oracle::{compile, digest, file_digest, run, storage_oracle, verify_file};
    use sha2::{Digest, Sha256};
    use std::fmt::Write;
    use std::path::Path;
    use std::process::Command;

    fn verify_fixture_records(family: u64, bytes: &[u8]) {
        let (name, digest) = match family {
            0 => (
                "matrix",
                "8519e3873aab928840517505a2a14de2769ec7140d69e85f41588fe988a02ec0",
            ),
            1 => (
                "arithmetic",
                "6c505c8357480864f8b35e91ba4287b36601f6b4a7cbe06853208ffbf5fb3bc4",
            ),
            _ => panic!("unknown fixture family"),
        };
        static BASELINES: [std::sync::OnceLock<serde_json::Value>; 2] =
            [const { std::sync::OnceLock::new() }; 2];
        let baseline = BASELINES[family as usize].get_or_init(|| {
            let path = super::root().join(format!("verification/{name}-fixture-baseline.json"));
            let bytes = std::fs::read(path).unwrap();
            assert_eq!(
                repo_model::artifact::sha256_hex(&bytes),
                digest,
                "the frozen prior corpus changed"
            );
            serde_json::from_slice(&bytes).unwrap()
        });
        let mut cursor = 0;
        for record in baseline["record"].as_array().unwrap() {
            let start = cursor;
            cursor += 24 * 8;
            for _ in 0..2 {
                let count =
                    u64::from_le_bytes(bytes[cursor..cursor + 8].try_into().unwrap()) as usize;
                cursor += 16 + count * 8;
            }
            assert_eq!(
                repo_model::artifact::sha256_hex(&bytes[start..cursor]),
                record["sha256"].as_str().unwrap(),
                "{name} fixture case {} differs from the frozen corpus",
                record["case"]
            );
        }
        assert_eq!(
            cursor,
            bytes.len(),
            "fixture transcript omitted or added records"
        );
    }

    pub(super) fn fixture_input(family: u64) -> &'static [u8] {
        static INPUT: [std::sync::OnceLock<Vec<u8>>; 2] = [const { std::sync::OnceLock::new() }; 2];
        INPUT[family as usize].get_or_init(|| {
            use vtensor_fixtures::Adt0 as Request;
            super::tensor_layout::certified_current_model(
                &super::root(),
                if family == 0 {
                    "fixtures"
                } else {
                    "arithmetic"
                },
            );
            let query = |request| {
                let result = if family == 0 {
                    vtensor_fixtures::run(0, request).unwrap()
                } else {
                    let (kind, case, source, item) = match request {
                        Request::C0(item) => (0, 0, 0, item),
                        Request::C1(case, item) => (1, case, 0, item),
                        Request::C2(case, source, item) => (2, case, source, item),
                        Request::C3(case, source, item) => (3, case, source, item),
                    };
                    vtensor_arithmetic::run(kind, case, source, item).unwrap()
                };
                result.expect("admitted fixture query")
            };
            let (cases, fields, sources, padding) = (
                query(Request::C0(0)),
                query(Request::C0(1)),
                query(Request::C0(2)),
                query(Request::C0(3)),
            );
            assert_eq!(
                (cases, fields, sources, padding),
                (if family == 0 { 1280 } else { 120 }, 24, 2, 165)
            );
            let mut input: Vec<u8> = [cases, fields, sources, padding]
                .into_iter()
                .flat_map(u64::to_le_bytes)
                .collect();
            for case in 0..cases {
                for field in 0..fields {
                    let value = query(Request::C1(case, field));
                    input.extend_from_slice(&value.to_le_bytes());
                }
                for source in 0..sources {
                    let (count, width) = (
                        query(Request::C2(case, source, 0)),
                        query(Request::C2(case, source, 1)),
                    );
                    for value in [count, width] {
                        input.extend_from_slice(&value.to_le_bytes());
                    }
                    for index in 0..count {
                        let value = query(Request::C3(case, source, index));
                        input.extend_from_slice(&value.to_le_bytes());
                    }
                }
            }
            verify_fixture_records(family, &input[32..]);
            input
        })
    }

    fn compiled_object(common: &Path, source: &str, object: &Path) {
        let pending = object.with_extension("new");
        let stamp = object.with_extension("sha256");
        if !object.exists() {
            compile(common, common.join("ggml/src").join(source), &pending);
            std::fs::write(&stamp, file_digest(&pending)).unwrap();
            std::fs::rename(pending, object).unwrap();
        }
        verify_file(object, stamp);
    }

    pub(super) fn verify(root: &Path, mode: &str, expected_cases: usize) {
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
        let mut driver = String::from(HEADER);
        for (key, value) in vtensor_backend::abi_layout() {
            let (ty, field) = key.rsplit_once('.').unwrap();
            let expression = match field {
                "$size" => format!("sizeof({ty})"),
                "$align" => format!("alignof({ty})"),
                _ => format!("offsetof({ty}, {field})"),
            };
            writeln!(
                driver,
                "static_assert((size_t)({expression}) == {value}, \"ABI {key}\");"
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
                compiled_object(common, source, &frontend.join(object));
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
                compiled_object(common, &source, &object);
                cpu_objects.push(object);
            }
            let mut linker = Command::new("c++");
            linker.args(["-Wl,--gc-sections", "-Wl,--export-dynamic"]);
            linker.arg(cache.join("driver.o"));
            linker.args(["reg.o", "dl.o", "threading.o"].map(|name| frontend.join(name)));
            linker.args(
                ["ggml.o", "quants.o", "alloc.o", "backend.o", "meta.o"]
                    .map(|name| common.join(name)),
            );
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
        let input = if matches!(mode, "matrix" | "elementwise") {
            fixture_input(u64::from(mode == "elementwise")).to_owned()
        } else if mode == "update" {
            Vec::new()
        } else {
            super::ggml_oracle::read_toml(root.join("model/ggml-declarations.toml"))["tensor_type"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|row| row["symbol"].as_str() != Some("GGML_TYPE_COUNT"))
                .map(|row| format!("{}\n", row["value"].as_integer().unwrap()))
                .collect::<String>()
                .into_bytes()
        };
        let mut expected = Vec::new();
        for file in vtensor_backend::SOURCE_FILES {
            expected.extend_from_slice(file.as_bytes());
            expected.push(0);
            expected.extend_from_slice(&std::fs::read(root.join(file)).unwrap());
            expected.push(0);
        }
        assert!(
            vtensor_backend::source_snapshot() == expected,
            "Rust test library is stale"
        );
        let expected_snapshot = cache.join("expected-source.snapshot");
        std::fs::write(&expected_snapshot, &expected).unwrap();
        let text = super::ggml_oracle::batch(
            Command::new(&program)
                .arg(&library)
                .arg(&expected_snapshot)
                .arg(mode),
            &input,
        );
        assert_eq!(text, format!("RESULT cases {expected_cases}\n"));
        if matches!(mode, "matrix" | "elementwise") {
            verify_fixture_records(
                u64::from(mode == "elementwise"),
                &std::fs::read(format!("{}.{mode}-fixtures", expected_snapshot.display())).unwrap(),
            );
        }
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
static size_t refusal_probes = 0;
static void unexpected_abort(const char *message) { fprintf(stderr, "upstream abort at type %d layout %d: %s\n", current_kind, current_layout, message); _exit(99); }
template<class T, class U>
static bool refuses_change(ggml_backend_t backend, ggml_backend_dev_t dev, ggml_cgraph *graph, ggml_tensor *result, T &field, U bad, bool capability=true) {
    ++refusal_probes;
    std::vector<unsigned char> before(ggml_nbytes(result)),after(before.size());
    ggml_backend_tensor_get(result,before.data(),0,before.size());
    const T saved=field;field=bad;
    const bool refused=(!capability || !ggml_backend_dev_supports_op(dev,result)) && ggml_backend_graph_compute(backend,graph)==GGML_STATUS_FAILED;
    field=saved;ggml_backend_tensor_get(result,after.data(),0,after.size());
    return refused && before==after;
}
static ggml_tensor * canonical_copy(ggml_context *ctx, ggml_tensor *input) {
    const int64_t block=ggml_blck_size(input->type), count=ggml_nelements(input)/block;
    ggml_tensor *target=ggml_new_tensor_2d(ctx,input->type,block,count); target->nb[1]*=2; target->nb[2]=target->nb[1]*count; target->nb[3]=target->nb[2];
    return ggml_reshape_4d(ctx,ggml_cont(ctx,ggml_cpy(ctx,input,target)),input->ne[0],input->ne[1],input->ne[2],input->ne[3]);
}
static size_t block_address(const ggml_tensor *tensor, size_t ordinal) {
    int64_t i0,i1,i2,i3;const int64_t block=ggml_blck_size(tensor->type);
    ggml_unravel_index(tensor,ordinal*block,&i0,&i1,&i2,&i3);
    return (i0/block)*tensor->nb[0]+i1*tensor->nb[1]+i2*tensor->nb[2]+i3*tensor->nb[3];
}
static bool read_fixture_word(uint64_t &word) {
    uint8_t bytes[8];if(fread(bytes,1,8,stdin)!=8)return false;
    word=0;for(int i=0;i<8;i++)word|=uint64_t(bytes[i])<<(8*i);
    return true;
}
static int bind_fixture_words(ggml_tensor *left, ggml_tensor *right, size_t padding) {
    for (ggml_tensor *operand : {left,right}) {
      uint64_t count,width;CHECK(read_fixture_word(count) && read_fixture_word(width));
      CHECK(count==uint64_t(operand?ggml_nelements(operand):0) && width==4);
      std::vector<unsigned char> bytes(operand?ggml_nbytes(operand):0,padding);
      for(uint64_t i=0;i<count;i++) {uint64_t value;CHECK(read_fixture_word(value) && value<=UINT32_MAX);const uint32_t word=value;memcpy(bytes.data()+block_address(operand,i),&word,4);}
      if(!bytes.empty())ggml_backend_tensor_set(operand,bytes.data(),0,bytes.size());
    }
    return 0;
}
static void observe_fixture(std::ofstream &trace, const uint64_t (&fields)[24], ggml_tensor *left, ggml_tensor *right) {
    auto emit=[&](uint64_t word) {uint8_t bytes[8];for(int i=0;i<8;i++)bytes[i]=uint8_t(word>>(8*i));trace.write((const char *)bytes,8);};
    for(auto value:fields)emit(value);
    for(auto operand:{left,right}) {emit(operand?ggml_nelements(operand):0);emit(4);if(operand) for(int64_t i=0;i<ggml_nelements(operand);i++) {uint32_t word;memcpy(&word,(char *)operand->data+block_address(operand,i),4);emit(word);}}
}
static bool compare_result(ggml_tensor *result, ggml_tensor *reference, std::vector<unsigned char> expected, int nan_mode) {
    std::vector<unsigned char> actual(ggml_nbytes(result)), raw(reference?ggml_nbytes(reference):0);
    if(!actual.empty()) ggml_backend_tensor_get(result,actual.data(),0,actual.size());
    if(!raw.empty()) ggml_backend_tensor_get(reference,raw.data(),0,raw.size());
    const size_t bytes=ggml_type_size(result->type), block=ggml_blck_size(result->type);
    auto nan=[&](const unsigned char *p) {float value;if(result->type==GGML_TYPE_F32)memcpy(&value,p,4);else {uint16_t word;memcpy(&word,p,2);value=result->type==GGML_TYPE_F16?ggml_fp16_to_fp32(word):ggml_bf16_to_fp32(ggml_bf16_t{word});}return std::isnan(value);};
    for(size_t i=0;i<(reference?size_t(ggml_nelements(reference))/block:0);i++) {
      const size_t at=block_address(result,i);
      if(nan_mode && (nan_mode!=2 || i%(result->ne[0]/block)) && nan(actual.data()+at) && nan(raw.data()+i*bytes)) memcpy(actual.data()+at,raw.data()+i*bytes,bytes);
      memcpy(expected.data()+at,raw.data()+i*bytes,bytes);
    }
    if(actual==expected) return true;
    const size_t at=std::mismatch(actual.begin(),actual.end(),expected.begin()).first-actual.begin();
    fprintf(stderr,"oracle mismatch type=%d width=%lld rows=%lld byte=%zu actual=%u expected=%u\n",result->type,(long long)result->ne[0],(long long)result->ne[1],at,actual[at],expected[at]);return false;
}
int main(int argc, char **argv) {
    CHECK(argc == 4);
    Dl_info loaded = {};
    CHECK(dladdr((void *)&ggml_backend_vtensor_reg, &loaded) != 0);
    CHECK(std::filesystem::canonical(loaded.dli_fname) == std::filesystem::canonical(argv[1]));
    std::ifstream expected_file(argv[2], std::ios::binary);
    CHECK(expected_file.good());
    const std::vector<unsigned char> expected_source((std::istreambuf_iterator<char>(expected_file)), std::istreambuf_iterator<char>());
    CHECK(expected_source.size() == vtensor_build_sources_size());
    CHECK(!memcmp(expected_source.data(), vtensor_build_sources(), expected_source.size()));
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
    ggml_backend_t reference=ggml_backend_cpu_init(); CHECK(reference);
    ggml_backend_cpu_set_n_threads(reference,1);
    if (!strcmp(argv[3], "update")) {
      size_t cases=0;
      for(ggml_type type : {GGML_TYPE_F32,GGML_TYPE_I32}) for(ggml_type patch_type : {GGML_TYPE_F32,GGML_TYPE_I32}) for(int add : {0,1})
      for(int inplace : {0,1}) for(int alias : {0,1}) for(int layout=0;layout<4;layout++) for(size_t offset : {size_t(0),size_t(4),size_t(20)})
      for(int width : {0,1,3}) for(int rows : {1,2}) for(int planes : {1,2}) for(int batches : {1,2}) {
        if((add && (type!=GGML_TYPE_F32 || patch_type!=GGML_TYPE_F32)) || (alias && type!=patch_type)) continue;
        ggml_init_params params={8*1024*1024,nullptr,true}; ggml_context *ctx=ggml_init(params); CHECK(ctx);
        ggml_tensor *left=ggml_new_tensor_4d(ctx,type,8,4,3,2);
        const size_t s1=layout==2?16:width*4+(layout?4:0),s2=layout==2?64:s1*rows+(layout?4:0),s3=layout==2?128:s2*planes+(layout?4:0);
        ggml_tensor *right=alias?ggml_view_4d(ctx,left,width,rows,planes,batches,s1,s2,s3,16):ggml_new_tensor_4d(ctx,patch_type,width,rows,planes,batches);
        right->nb[1]=s1;right->nb[2]=s2;right->nb[3]=s3;
        const size_t n1=layout==3?size_t(-4):layout==2?0:layout?16:width*4,n2=layout==3?size_t(-16):layout==2?4:layout?64:n1*rows,n3=layout==3?size_t(-64):layout==2?8:layout?256:n2*planes;
        const size_t base=offset+(layout==3?100:0);
        ggml_tensor *result=add?(inplace?ggml_acc_inplace(ctx,left,right,n1,n2,n3,base):ggml_acc(ctx,left,right,n1,n2,n3,base)):(inplace?ggml_set_inplace(ctx,left,right,n1,n2,n3,base):ggml_set(ctx,left,right,n1,n2,n3,base));
        ggml_backend_buffer_t buffer=ggml_backend_alloc_ctx_tensors(ctx,backend); CHECK(buffer);
        std::vector<uint32_t> initial(ggml_nelements(left)), expected(initial.size()),actual(initial.size());
        for(ggml_tensor *input : {left,right}) {
          std::vector<uint32_t> data(ggml_nbytes(input)/4);
          for(size_t i=0;i<data.size();i++) {const int value=int((i*11+(input==right?7:0))%47)-23; const float f=float(value)/8;
            if(input->type==GGML_TYPE_F32) memcpy(&data[i],&f,4); else data[i]=uint32_t(value);}
          if(!data.empty()) ggml_backend_tensor_set(input,data.data(),0,data.size()*4);
        }
        ggml_backend_tensor_get(left,initial.data(),0,initial.size()*4);
        ggml_cgraph *graph=ggml_new_graph(ctx); ggml_build_forward_expand(graph,result);
        CHECK(ggml_backend_graph_compute(reference,graph)==GGML_STATUS_SUCCESS); ggml_backend_tensor_get(result,expected.data(),0,expected.size()*4);
        ggml_backend_tensor_set(left,initial.data(),0,initial.size()*4); if(!inplace) ggml_backend_tensor_memset(result,0xA5,0,ggml_nbytes(result));
        if(layout==0 && offset==0 && width==3 && rows==2 && planes==2 && batches==2) {
          CHECK(refuses_change(backend,dev,graph,result,result->src[1],nullptr));
          CHECK(refuses_change(backend,dev,graph,result,result->op_params[3],int32_t(ggml_nbytes(result))));
          CHECK(refuses_change(backend,dev,graph,result,result->op_params[0],-INT32_MAX));
          if(!inplace) CHECK(refuses_change(backend,dev,graph,result,result->op_params[4],int32_t(1)));
        }
        CHECK(ggml_backend_dev_supports_op(dev,result)); CHECK(ggml_backend_graph_compute(backend,graph)==GGML_STATUS_SUCCESS);
        ggml_backend_tensor_get(result,actual.data(),0,actual.size()*4);
        if(actual!=expected) {fprintf(stderr,"update mismatch add=%d type=%d patch=%d inplace=%d alias=%d layout=%d offset=%zu shape=%d,%d,%d,%d\n",add,type,patch_type,inplace,alias,layout,base,width,rows,planes,batches);return 2;}
        ggml_backend_buffer_free(buffer); ggml_free(ctx); cases++;
      }
      ggml_backend_free(reference); ggml_backend_free(backend); printf("RESULT cases %zu\n",cases); return 0;
    }
    const bool indexed=!strcmp(argv[3], "indexed-add");
    if (!strcmp(argv[3], "row-gather") || indexed) {
      size_t cases=0, types=0, decoders=0;
      bool complete=true;
      for (int kind=0; kind<GGML_TYPE_COUNT; ++kind) {
        const auto type=ggml_type(kind);
        if (!ggml_get_type_traits(type)->type_size) continue;
        const bool numerical=indexed?type==GGML_TYPE_F32:type==GGML_TYPE_F32 || type==GGML_TYPE_I32 || ggml_get_type_traits(type)->to_float;
        types++; decoders+=numerical;
        for (int empty_axis : {-1,0,1,2,3}) {
          if (empty_axis<0 && !numerical) continue;
          for (int layout=0; layout<(empty_axis<0?4:1); ++layout) for (int pattern=0; pattern<(empty_axis<0?(indexed?5:2):1); ++pattern) {
          current_kind=type; current_layout=layout;
          int64_t source_shape[]={2*ggml_blck_size(type),3,2,2}, index_shape[]={3,2,2,1};
          if(indexed) {const int widths[]={1,3,17,65};if(empty_axis<0)source_shape[0]=widths[layout];index_shape[3]=2;if(pattern==3 && empty_axis<0)source_shape[1]=0;}
          if (layout>1) {source_shape[3]=3;index_shape[2]=3;}
          if (empty_axis==0) source_shape[0]=0;
          if (empty_axis>0) index_shape[empty_axis-1]=0;
          if (empty_axis>1) source_shape[empty_axis]=0;
          ggml_init_params params={8*1024*1024,nullptr,true};
          ggml_context *ctx=ggml_init(params); CHECK(ctx);
          const int64_t physical_shape[]={source_shape[0]+layout*ggml_blck_size(type),4,3,source_shape[3]};
          ggml_tensor *physical=ggml_new_tensor(ctx,type,4,layout?physical_shape:source_shape);
          ggml_tensor *input=layout?ggml_view_4d(ctx,physical,source_shape[0],source_shape[1],source_shape[2],source_shape[3],physical->nb[1],physical->nb[2],physical->nb[3],physical->nb[1]+ggml_type_size(type)):physical;
          ggml_tensor *indices=ggml_new_tensor(ctx,GGML_TYPE_I32,4,index_shape);
          if (layout) {indices->nb[0]=4*(layout+1);for(int axis=1;axis<4;axis++)indices->nb[axis]=indices->nb[axis-1]*indices->ne[axis-1]+4*layout;}
          ggml_tensor *left=indexed?ggml_new_tensor_4d(ctx,GGML_TYPE_F32,source_shape[0],index_shape[0],index_shape[1],index_shape[2]):nullptr;
          if(left && layout) {left->nb[0]=4*(layout+2);for(int axis=1;axis<4;axis++)left->nb[axis]=left->nb[axis-1]*left->ne[axis-1]+4*(layout+1);}
          ggml_tensor *result=indexed?ggml_add_id(ctx,left,input,indices):ggml_get_rows(ctx,input,indices);
          ggml_tensor *oracle=indexed?ggml_add_id(ctx,empty_axis<0?canonical_copy(ctx,left):left,empty_axis<0?canonical_copy(ctx,input):input,indices):ggml_get_rows(ctx,input,layout?canonical_copy(ctx,indices):indices);
          if (layout) {result->nb[0]=4*(layout+1);for(int axis=1;axis<4;axis++)result->nb[axis]=result->nb[axis-1]*result->ne[axis-1]+4*layout;}
          ggml_backend_buffer_t buffer=nullptr;
          if (empty_axis<0) {
            buffer=ggml_backend_alloc_ctx_tensors(ctx,backend); CHECK(buffer);
            std::vector<unsigned char> source(ggml_nbytes(physical));
            for (int64_t row=0; row<ggml_nrows(physical); ++row) memset(source.data()+row*physical->nb[1],0xA0+row,physical->nb[1]);
            if(indexed) for(size_t i=0;i<source.size()/4;i++) {const float value=float(int((i*13+7)%47)-23)/8;memcpy(source.data()+i*4,&value,4);}
            std::vector<unsigned char> selected(ggml_nbytes(indices),0xA5);
            for (int64_t i=0; i<ggml_nelements(indices); ++i) {
              const int32_t gathered=((pattern?2-i%3:i%3==1?0:2)+i/3)%3;
              const int32_t value=indexed?(i>=6?-2:pattern==3 || (pattern==2 && i%2)?-1:pattern==1?2-i%3:i%3):gathered;
              memcpy(selected.data()+block_address(indices,i),&value,4);
            }
            if(left) {
              static const uint32_t edges[]={0,0x80000000,0x7f800000,0xff800000,0x7fc00001,0x7f800001,0xff800001,1,0x80000001,0x7f7fffff,0xff7fffff,0x00800000};
              std::vector<unsigned char> values(ggml_nbytes(left),0xA5);
              for(int64_t i=0;i<ggml_nelements(left);i++) {const int64_t row=i/left->ne[0];const bool copied=pattern==3 || (pattern==2 && row%6%2);const float value=float(int((i*7+3)%47)-23)/8;uint32_t word;memcpy(&word,&value,4);if(copied || pattern==4)word=edges[i%12];memcpy(values.data()+block_address(left,i),&word,4);}
              ggml_backend_tensor_set(left,values.data(),0,values.size());
            }
            ggml_backend_tensor_set(physical,source.data(),0,source.size());
            ggml_backend_tensor_set(indices,selected.data(),0,selected.size());
          }
          ggml_cgraph *graph=ggml_new_graph(ctx); ggml_build_forward_expand(graph,result);
          ggml_cgraph *reference_graph=ggml_new_graph(ctx); ggml_build_forward_expand(reference_graph,oracle);
          CHECK(ggml_backend_graph_compute(reference,reference_graph)==GGML_STATUS_SUCCESS);
          const std::vector<unsigned char> expected(ggml_nbytes(result),0xA5);
          if (!expected.empty()) ggml_backend_tensor_memset(result,0xA5,0,expected.size());
          const bool supported=ggml_backend_dev_supports_op(dev,result);
          const bool computed=ggml_backend_graph_compute(backend,graph)==GGML_STATUS_SUCCESS;
          const bool equal=computed && compare_result(result,oracle,expected,indexed && pattern==4?1:0);
          complete &= supported && computed && equal;
          if (!supported || !computed || !equal) fprintf(stderr,"row-gather missing kind=%d empty_axis=%d supported=%d computed=%d equal=%d\n",kind,empty_axis,supported,computed,equal);
          if (empty_axis<0 && pattern==0) {
            auto refusal_check=[&](bool ok,const char *label) {if(!ok){fprintf(stderr,"row-gather refusal kind=%d %s\n",kind,label);complete=false;}};
            if (layout==0) {
            refusal_check(refuses_change(backend,dev,graph,result,indices->type,GGML_TYPE_I64),"index-type");
            if(!indexed) refusal_check(refuses_change(backend,dev,graph,result,indices->ne[3],int64_t(2)),"index-batch");
            refusal_check(refuses_change(backend,dev,graph,result,result->type,GGML_TYPE_F16),"output-type");
            refusal_check(refuses_change(backend,dev,graph,result,result->data,input->data),"source-alias");
            refusal_check(refuses_change(backend,dev,graph,result,result->data,indices->data),"index-alias");
            if(indexed) {
              refusal_check(refuses_change(backend,dev,graph,result,result->src[0],nullptr),"missing-values");
              refusal_check(refuses_change(backend,dev,graph,result,result->src[1],nullptr),"missing-table");
              refusal_check(refuses_change(backend,dev,graph,result,result->src[2],nullptr),"missing-third-source");
              refusal_check(refuses_change(backend,dev,graph,result,left->nb[0],size_t(1)),"short-value-word");
              refusal_check(refuses_change(backend,dev,graph,result,input->nb[0],size_t(1)),"short-table-word");
              refusal_check(refuses_change(backend,dev,graph,result,indices->nb[0],size_t(1)),"short-index-word");
              refusal_check(refuses_change(backend,dev,graph,result,result->nb[0],size_t(1)),"short-output-word");
              refusal_check(refuses_change(backend,dev,graph,result,left->type,GGML_TYPE_F16),"value-type");
              refusal_check(refuses_change(backend,dev,graph,result,input->type,GGML_TYPE_I32),"table-type");
              refusal_check(refuses_change(backend,dev,graph,result,left->ne[1],left->ne[1]+1),"value-rows");
              refusal_check(refuses_change(backend,dev,graph,result,result->data,left->data),"value-alias");
              refusal_check(refuses_change(backend,dev,graph,result,indices->ne[1],indices->ne[1]+1),"index-planes");
            }
            if (ggml_blck_size(type)>1) refusal_check(refuses_change(backend,dev,graph,result,input->nb[0],ggml_type_size(type)-1),"short-block-stride");
            }
            const std::vector<unsigned char> untouched(expected.size(),0xA5);
            for (int32_t bad : {indexed?-2:-1,3}) {
              ggml_backend_tensor_memset(result,0xA5,0,expected.size());
              ggml_backend_tensor_set(indices,&bad,indexed?2*indices->nb[0]+indices->nb[1]:ggml_nbytes(indices)-4,4);
              refusal_check(ggml_backend_graph_compute(backend,graph)==GGML_STATUS_FAILED,"late-invalid-index");
              refusal_check(compare_result(result,nullptr,untouched,0),"atomic-index-refusal");
            }
          }
          ggml_backend_buffer_free(buffer); ggml_free(ctx); cases++;
          }
        }
      }
      CHECK(types==36 && decoders==(indexed?1:30));
      CHECK(complete);
      ggml_backend_free(reference); ggml_backend_free(backend);
      printf("RESULT cases %zu\n",cases); return 0;
    }
    if (!strcmp(argv[3], "row-gradient")) {
      const uint32_t edges[]={0,0x80000000,0x3f800000,0xbf800000,0x4b800000,0xcb800000,0x7f800000,0xff800000,0x7fc00001,0x7f800001,1,0x80000001,0x7f7fffff,0x00800000,0x3f000000,0xbf000000};
      size_t cases=0;
      for(auto type : {GGML_TYPE_F32,GGML_TYPE_F16}) for(int width : {0,1,3,17,65}) for(int rows : {0,1,3,9}) for(int output_rows : {0,1,4,7}) for(int layout=0;layout<3;layout++) for(int pattern=0;pattern<7;pattern++) {
        ggml_init_params params={8*1024*1024,nullptr,true};ggml_context *ctx=ggml_init(params);CHECK(ctx);
        ggml_tensor *input=ggml_new_tensor_2d(ctx,type,width,rows+(pattern==1?2:pattern==2?1:0)), *indices=ggml_new_tensor_1d(ctx,GGML_TYPE_I32,rows), *shape=ggml_new_tensor_2d(ctx,GGML_TYPE_F32,width,output_rows);
        const size_t bytes=ggml_type_size(type);
        input->nb[0]=bytes*(layout==2?2:1);input->nb[1]=input->nb[0]*width+bytes*layout;input->nb[2]=input->nb[3]=input->nb[1]*input->ne[1];
        indices->nb[0]=4*(layout+1);indices->nb[1]=indices->nb[2]=indices->nb[3]=indices->nb[0]*rows;
        ggml_tensor *result=ggml_get_rows_back(ctx,input,indices,shape), *oracle=ggml_get_rows_back(ctx,input->nb[0]==bytes?input:canonical_copy(ctx,input),indices->nb[0]==4?indices:canonical_copy(ctx,indices),shape);
        result->nb[0]=4*(layout+1);result->nb[1]=result->nb[0]*width+4*layout;result->nb[2]=result->nb[3]=result->nb[1]*output_rows;
        ggml_backend_buffer_t buffer=ggml_backend_alloc_ctx_tensors(ctx,backend);CHECK(buffer || (!ggml_nbytes(input)&&!ggml_nbytes(indices)&&!ggml_nbytes(result)));
        std::vector<unsigned char> data(ggml_nbytes(input),0xA5), index_data(ggml_nbytes(indices),0xA5), initial(ggml_nbytes(result),0xA5);
        for(int row=0;row<rows;row++) {
          int32_t index=output_rows?pattern==0?0:pattern==2?output_rows-1-row%output_rows:pattern==3?(row%2?0:output_rows-1):pattern==4?output_rows-1:row%output_rows:0;
          if(row==rows-1 && pattern>=5) index=pattern==5?-1:output_rows;
          memcpy(index_data.data()+row*indices->nb[0],&index,4);
          for(int column=0;column<width;column++) {
            const uint32_t bits=edges[(row*width+column+pattern*7)%16];float value;memcpy(&value,&bits,4);const uint16_t half=ggml_fp32_to_fp16(value);
            memcpy(data.data()+row*input->nb[1]+column*input->nb[0],type==GGML_TYPE_F32?(const void *)&bits:(const void *)&half,bytes);
          }
        }
        if(!data.empty()) ggml_backend_tensor_set(input,data.data(),0,data.size());
        if(!index_data.empty()) ggml_backend_tensor_set(indices,index_data.data(),0,index_data.size());
        if(!initial.empty()) ggml_backend_tensor_set(result,initial.data(),0,initial.size());
        ggml_cgraph *graph=ggml_new_graph(ctx);ggml_build_forward_expand(graph,result);
        if(type==GGML_TYPE_F32 && width==1 && rows==1 && output_rows==1 && layout==0 && pattern==0) {
          CHECK(refuses_change(backend,dev,graph,result,indices->type,GGML_TYPE_I64));
          CHECK(refuses_change(backend,dev,graph,result,input->ne[1],int64_t(0)) && refuses_change(backend,dev,graph,result,result->type,GGML_TYPE_F16) && refuses_change(backend,dev,graph,result,result->data,input->data));
        }
        const bool invalid=width && output_rows && rows && pattern>=5;
        if(!invalid) {
          ggml_cgraph *reference_graph=ggml_new_graph(ctx);ggml_build_forward_expand(reference_graph,oracle);
          CHECK(ggml_backend_graph_compute(reference,reference_graph)==GGML_STATUS_SUCCESS);
        }
        if(!rows) {input->data=nullptr;input->buffer=nullptr;}
        CHECK(ggml_backend_dev_supports_op(dev,result));
        CHECK(ggml_backend_graph_compute(backend,graph)==(invalid?GGML_STATUS_FAILED:GGML_STATUS_SUCCESS));
        CHECK(compare_result(result,invalid?nullptr:oracle,initial,invalid?0:1));
        ggml_backend_buffer_free(buffer);ggml_free(ctx);cases++;
      }
      ggml_backend_free(reference);ggml_backend_free(backend);printf("RESULT cases %zu\n",cases);return 0;
    }
    const bool repeating=!strcmp(argv[3], "repeat"), concatenating=!strcmp(argv[3], "concat");
    const bool spatial=!strcmp(argv[3], "spatial"), folding=!strcmp(argv[3], "fold"), diagonal=!strcmp(argv[3], "diagonal"), triangular=!strcmp(argv[3], "triangular"), filling=!strcmp(argv[3], "fill"), casting=!strcmp(argv[3], "cast"), windowing=!strcmp(argv[3], "window"), transforming=!strcmp(argv[3], "transform"), unarying=!strcmp(argv[3], "unary"), scanning=!strcmp(argv[3], "scan"), reducing=!strcmp(argv[3], "reduction"), selecting=!strcmp(argv[3], "rows");
    const bool relative=!strcmp(argv[3], "relative-position");
    const bool wordwise=transforming || unarying || scanning || reducing || selecting;
    if (!strcmp(argv[3], "copy") || repeating || concatenating || spatial || folding || diagonal || triangular || filling || casting || windowing || wordwise || relative) {
      static const int window_sizes[]={1,2,4};
      static const ggml_type float_types[]={GGML_TYPE_F32,GGML_TYPE_F16,GGML_TYPE_BF16};
      static const uint32_t fill_words[]={0,0x80000000,0x3f800000,0xbf800000,0x7f800000,0xff800000,
        0x7fc00001,0x7f800001,0xff800001,1,0x80000001,0x00800000,0x7f7fffff,0xff7fffff,
        0x33800000,0x33000000,0x33000001,0x32ffffff,0x3f801000,0x3f803000,0x477fe000,0x477ff000,0x477fefff,0x38800000};
      size_t cases=0, refused=0; int kind;
      while (scanf("%d",&kind)==1) for(int operation=0;operation<(relative?1:selecting?2:reducing?3:scanning?1:unarying?39:transforming?8:windowing?2:casting?6:filling?2:(concatenating || triangular)?4:(repeating || folding || diagonal || triangular || filling || casting || windowing)?1:3);operation++) {
      const std::vector<int> extents=relative?std::vector<int>{1,3,17}:(concatenating || (spatial && operation==0))?std::vector<int>{0,1,3}:windowing?std::vector<int>{1,3,5}:spatial?std::vector<int>{2,3}:(reducing || selecting)?std::vector<int>{1,2,3,17,65}:scanning?std::vector<int>{1,3,17,65}:std::vector<int>{1,3};
      const auto row_extents=relative?std::vector<int>{1,3,7}:(scanning || reducing || selecting)?std::vector<int>{1,3}:extents;
      const std::vector<int> batches_list=(spatial && operation==2)?std::vector<int>{2,3}:std::vector<int>{1,2};
      for(int pattern=0;pattern<(relative?2:selecting?18:reducing?14:scanning?12:unarying?5:transforming?28:windowing?(operation?30:12):casting?5:filling?120:triangular?5:diagonal?4:folding?21:spatial?(operation==0?22:operation==1?4:17):1);pattern++) for(int layout=0;layout<4;layout++)
      for(int units : extents) for(int rows : row_extents) for(int batches : batches_list) {
        if(relative && ((kind!=GGML_TYPE_F16 && kind!=GGML_TYPE_BF16) || (pattern && units!=1))) continue;
        if((casting && kind!=GGML_TYPE_F32 && kind!=GGML_TYPE_F16 && kind!=GGML_TYPE_BF16) || (filling && kind!=GGML_TYPE_F32 && kind!=GGML_TYPE_F16) || ((diagonal || triangular || wordwise) && !selecting && kind!=GGML_TYPE_F32 && kind!=GGML_TYPE_F16 && kind!=GGML_TYPE_BF16) || ((spatial || folding || windowing || scanning) && kind!=GGML_TYPE_F32) || (repeating && kind!=GGML_TYPE_F32 && kind!=GGML_TYPE_F16 && kind!=GGML_TYPE_BF16 && kind!=GGML_TYPE_I16 && kind!=GGML_TYPE_I32)) continue;
        if(reducing && operation && kind!=GGML_TYPE_F32) continue;
        if(selecting && (kind!=(operation?GGML_TYPE_I32:GGML_TYPE_F32) || (!operation && pattern>=16))) continue;
        if(((selecting && pattern>=14) || (reducing && pattern>=10) || (scanning && pattern>=8) || (transforming && pattern>=24) || (unarying && pattern)) && (units!=1 || rows!=1 || batches!=1)) continue;
        ggml_init_params params={8*1024*1024,nullptr,true}; ggml_context *ctx=ggml_init(params); CHECK(ctx);
        const auto type=(ggml_type)kind; const int64_t block=ggml_blck_size(type); const size_t bytes=ggml_type_size(type);
        const ggml_type output_type=relative?GGML_TYPE_F16:selecting?(operation?GGML_TYPE_I64:GGML_TYPE_I32):casting?float_types[operation%3]:(unarying && operation%3==2)?GGML_TYPE_F32:type;
        const size_t output_bytes=ggml_type_size(output_type);
        if((casting && type==output_type) || (unarying && operation%3==2 && type==GGML_TYPE_F32)) continue;
        const int window_case=windowing?pattern%(operation?10:4):0;
        const int window_size=windowing?window_sizes[pattern/(operation?10:4)]:1;
        const int image_width=window_case==2?0:rows, image_height=window_case==3?0:batches;
        int64_t shape[]={block*units,rows,2,batches};
        if(relative) {shape[0]=pattern?0:units;shape[1]=2*rows-1;}
        if(selecting && !operation) {shape[1]*=batches;shape[2]=shape[3]=1;}
        if(selecting && operation && pattern>=10 && pattern<14) {shape[1]*=batches;shape[2]=shape[3]=1;}
        if(selecting && operation && pattern==9) {const int side=rows+batches-1;shape[1]=shape[2]=side;shape[3]=side*side;}
        if(windowing) {
          shape[0]=window_case==1?0:units;shape[1]=image_width;shape[2]=image_height;shape[3]=1;
          if(operation) {shape[1]=window_size;shape[2]=window_size;shape[3]=((image_width+window_size-1)/window_size)*((image_height+window_size-1)/window_size);}
          if(operation && window_case>=4 && window_case<=6) shape[window_case-3]++;
          if(operation && window_case>=7) {
            shape[3]*=window_size*window_size;shape[1]=shape[2]=1;
            if(window_case>=8) {int last=0;for(int y=0;y<image_height;y++) for(int x=0;x<image_width;x++) last=std::max(last,(y/window_size)*((image_width+window_size-1)/window_size)+x/window_size+y%window_size+x%window_size);shape[3]=last+1-(window_case==9);}
          }
        }
        if(filling && pattern%5) shape[pattern%5-1]=0;
        if((casting || unarying) && pattern) shape[pattern-1]=0;
        if(transforming && pattern>=24) shape[pattern-24]=0;
        if(scanning && pattern>=8) shape[pattern-8]=0;
        if(reducing && pattern>=10) shape[pattern-10]=0;
        if(selecting && pattern>=14) shape[pattern-14]=0;
        if(triangular) {shape[1]=units;shape[2]=rows;if(pattern==1)shape[0]=shape[1]=0;else if(pattern==2 || pattern==3)shape[pattern]=0;}
        if(diagonal) {shape[1]=1;shape[2]=rows;if(pattern)shape[pattern==1?0:pattern]=0;}
        if(folding) for(int axis=0;axis<4;axis++) shape[axis]=(pattern==16 && axis==0) || (pattern>=17 && axis==pattern-17)?0:shape[axis]*((pattern&(1<<axis))?2:1);
        ggml_tensor *input=ggml_new_tensor(ctx,type,4,shape);
        const uint32_t word=(filling || transforming)?fill_words[transforming?std::min(pattern,23):pattern/5]:0; float constant; memcpy(&constant,&word,4);
        const uint32_t bias_word=fill_words[((transforming?std::min(pattern,23):pattern/5)*7+3)%24];float bias;memcpy(&bias,&bias_word,4);
        const bool inplace=(filling && operation) || (transforming && operation%2) || (unarying && operation%3==1);
        auto word_operation = [&](ggml_tensor *a, bool inplace) {
          if(scanning) return ggml_cumsum(ctx,a);
          if(reducing) return operation==0?ggml_sum(ctx,a):operation==1?ggml_sum_rows(ctx,a):ggml_mean(ctx,a);
          if(unarying) {
            static const ggml_unary_op variants[]={GGML_UNARY_OP_ABS,GGML_UNARY_OP_SGN,GGML_UNARY_OP_NEG,GGML_UNARY_OP_STEP,GGML_UNARY_OP_RELU,GGML_UNARY_OP_COUNT,GGML_UNARY_OP_COUNT,GGML_UNARY_OP_HARDSIGMOID,GGML_UNARY_OP_HARDSWISH,GGML_UNARY_OP_FLOOR,GGML_UNARY_OP_CEIL,GGML_UNARY_OP_ROUND,GGML_UNARY_OP_TRUNC};
            const int which=operation/3;ggml_tensor *node=which==5?(inplace?ggml_sqr_inplace(ctx,a):ggml_sqr(ctx,a)):which==6?(inplace?ggml_sqrt_inplace(ctx,a):ggml_sqrt(ctx,a)):inplace?ggml_unary_inplace(ctx,a,variants[which]):ggml_unary(ctx,a,variants[which]);
            if(node->type!=output_type) {node->type=output_type;node->nb[0]=output_bytes;for(int axis=1;axis<4;axis++)node->nb[axis]=node->nb[axis-1]*node->ne[axis-1];}return node;
          }
          return operation<2?(inplace?ggml_scale_inplace(ctx,a,constant):ggml_scale(ctx,a,constant)):operation<4?(inplace?ggml_scale_bias_inplace(ctx,a,constant,bias):ggml_scale_bias(ctx,a,constant,bias)):operation<6?(inplace?ggml_clamp_inplace(ctx,a,constant,bias):ggml_clamp(ctx,a,constant,bias)):ggml_leaky_relu(ctx,a,constant,inplace);};
        ggml_tensor *filled=filling?(operation?ggml_fill_inplace(ctx,input,constant):ggml_fill(ctx,input,constant)):wordwise && !selecting?word_operation(input,inplace):nullptr;
        int64_t right_ne[]={input->ne[0],input->ne[1],input->ne[2],input->ne[3]}; if(concatenating) right_ne[operation%4]=layout==3?0:right_ne[operation%4]+(operation==0?block:1);
        ggml_tensor *right=(concatenating || (selecting && operation))?ggml_new_tensor(ctx,type,4,right_ne):nullptr;
        for(ggml_tensor *operand : {input,right}) if(operand && (layout==1 || layout==2)) {
          operand->nb[0]=bytes*(layout==2?2:1)*(selecting && operand==right?2:1); operand->nb[1]=operand->nb[0]*(operand->ne[0]/block)+bytes;
          operand->nb[2]=operand->nb[1]*operand->ne[1]+bytes; operand->nb[3]=operand->nb[2]*operand->ne[2]+bytes;
        }
        const int64_t blocks=ggml_nelements(input)/block;
        ggml_tensor *target=layout==3?ggml_new_tensor_4d(ctx,output_type,block,blocks,1,1):ggml_new_tensor(ctx,output_type,4,input->ne);
        target->nb[1]+=output_bytes; target->nb[2]=target->nb[1]*target->ne[1]+output_bytes; target->nb[3]=target->nb[2]*target->ne[2]+output_bytes;
        if(folding) {int64_t output_shape[]={units,rows,2,batches};if(pattern>=17) output_shape[pattern-17]=0;target=ggml_new_tensor(ctx,type,4,output_shape);}
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
        ggml_tensor *result=relative?ggml_get_rel_pos(ctx,input,rows,rows):selecting?(operation?ggml_count_equal(ctx,input,right):ggml_argmax(ctx,input)):wordwise?filled:windowing?(operation?ggml_win_unpart(ctx,input,image_width,image_height,window_size):ggml_win_part(ctx,input,window_size)):casting?(operation<3?ggml_cast(ctx,input,output_type):ggml_cpy(ctx,input,target)):filling?filled:triangular?ggml_tri(ctx,ggml_cont(ctx,input),(ggml_tri_type)operation):diagonal?ggml_diag(ctx,input):folding?ggml_repeat_back(ctx,input,target):spatial?spatial_op(operation?ggml_cont(ctx,input):input,false):concatenating?ggml_concat(ctx,input,right,operation):repeating?ggml_repeat(ctx,input,target):operation==0?ggml_dup(ctx,input):(operation==1?(layout==3?ggml_cont_1d(ctx,input,ggml_nelements(input)):ggml_cont(ctx,input)):ggml_cpy(ctx,input,target));
        if(triangular) result->src[0]=input;
        if(inplace) memcpy(result->nb,input->nb,sizeof input->nb);
        if((scanning || reducing || selecting) && layout==3) for(int axis=0;axis<4;axis++) result->nb[axis]*=2;
        ggml_tensor *expected_tensor=relative?ggml_get_rel_pos(ctx,canonical_copy(ctx,input),rows,rows):selecting?(operation?ggml_count_equal(ctx,(ggml_is_matrix(input) || pattern==9)?input:ggml_cont_2d(ctx,input,input->ne[0],ggml_nrows(input)),(ggml_is_matrix(input) || pattern==9)?right:ggml_cont_2d(ctx,right,right->ne[0],ggml_nrows(right))):ggml_argmax(ctx,input->nb[0]==sizeof(float)?input:canonical_copy(ctx,input))):wordwise?word_operation(reducing && !operation && type==GGML_TYPE_F32?input:canonical_copy(ctx,input),false):windowing?(operation?ggml_win_unpart(ctx,canonical_copy(ctx,input),image_width,image_height,window_size):ggml_win_part(ctx,canonical_copy(ctx,input),window_size)):casting?ggml_cast(ctx,canonical_copy(ctx,input),output_type):filling?ggml_fill(ctx,canonical_copy(ctx,input),constant):triangular?ggml_tri(ctx,canonical_copy(ctx,input),(ggml_tri_type)operation):diagonal?ggml_diag(ctx,canonical_copy(ctx,input)):folding?ggml_repeat_back(ctx,canonical_copy(ctx,input),target):spatial?spatial_op(canonical_copy(ctx,input),true):concatenating?ggml_concat(ctx,canonical_copy(ctx,input),canonical_copy(ctx,right),operation):repeating?ggml_repeat(ctx,canonical_copy(ctx,input),target):canonical_copy(ctx,input);
        if(relative || repeating || concatenating || spatial || folding || diagonal || triangular || (filling && !operation) || (casting && operation<3) || windowing || (wordwise && !inplace)) {result->nb[1]+=output_bytes;result->nb[2]=result->nb[1]*result->ne[1]+output_bytes;result->nb[3]=result->nb[2]*result->ne[2]+output_bytes;}
        ggml_backend_buffer_t buffer=ggml_backend_alloc_ctx_tensors(ctx,backend); CHECK(buffer || (!ggml_nbytes(input)&&(!right||!ggml_nbytes(right))&&!ggml_nbytes(result)));
        std::vector<unsigned char> output(ggml_nbytes(result),0xA5);
        for(ggml_tensor *operand : {input,right}) if(operand) {
          std::vector<unsigned char> initial(ggml_nbytes(operand));
          if(casting || wordwise || relative) for(size_t i=0;i<((reducing || selecting)?size_t(ggml_nelements(operand)):initial.size()/bytes);i++) {
            size_t ordinal=i, at=i*bytes;
            if(reducing || selecting) {at=0;for(int axis=0;axis<4;axis++){at+=(ordinal%operand->ne[axis])*operand->nb[axis];ordinal/=operand->ne[axis];}}
            static const uint16_t small[]={0,0x8000,0x3c00,0x3f80,0xbc00,0xbf80,0x7c00,0x7f80,0xfc00,0xff80,0x7e00,0x7fc0,0x7c01,0x7f81,0xfe01,0xff81,1,0x8001,0x03ff,0x007f,0x0400,0x0080,0x7bff,0x7f7f};
            if(scanning || reducing || selecting) {
              static const uint32_t scan_words[][4]={{0x3f800000,0x40000000,0xbf800000,0},{0x4b800000,0x3f800000,0xcb800000,0xbf800000},{0x80000000,0x80000000,0,0x80000000},{0x7f800000,0xff800000,0x3f800000,0},{0x7f800001,0xffc00001,0x7fc00123,0xff800001},{0x7f7fffff,0x7f7fffff,0xff7fffff,0x00800000},{1,0x80000001,0x007fffff,0x00800000}};
              const uint32_t grouped[]={0x5a800000,0,0xda800000,0x3f800000};
              const uint32_t row_words[][4]={{0xff800000,0xff800000,0xff800000,0xff800000},{0x7f800001,0x3f800000,0,0xbf800000},{0x3f800000,0x7fc00001,0xbf800000,0},{0xc0400000,0xc0000000,0xbf800000,0xc0000000}};
              uint32_t word=selecting && pattern>=10 && pattern<14?row_words[pattern-10][i%4]:pattern<7?scan_words[pattern][i%4]:pattern==8?grouped[i%4]:pattern==9?0x7f7fffff:uint32_t(i*2654435761U+0x9e3779b9U);
              if(selecting && operand==right && ((pattern%4==1 && pattern!=9) || ((pattern%4==2 || pattern==9) && i%2) || (pattern%4==3 && i%7==3))) word^=0x80000000;
              float value;memcpy(&value,&word,4);const uint16_t small=type==GGML_TYPE_F16?ggml_fp32_to_fp16(value):ggml_fp32_to_bf16(value).bits;
              if(type==GGML_TYPE_F32 || selecting)memcpy(initial.data()+at,&word,4);else memcpy(initial.data()+at,&small,2);
            } else if(unarying) {
              static const uint32_t unary_words[]={0,0x80000000,0x3f000000,0xbf000000,0x3fc00000,0xbfc00000,0x40200000,0xc0200000,0x40400000,0xc0400000,0x403fffff,0x40400001,0xc03fffff,0xc0400001,0x3f800000,0xbf800000,0x7f800000,0xff800000,0x7fc00001,0x7f800001,0xff800001,1,0x80000001,0x00800000,0x7f7fffff,0xff7fffff,0x4affffff,0xcaffffff,0x4b000000,0xcb000000,0x3f7fffff,0xbf7fffff};
              const uint32_t bits=unary_words[i%32];float value;memcpy(&value,&bits,4);const uint16_t half=type==GGML_TYPE_F16?ggml_fp32_to_fp16(value):ggml_fp32_to_bf16(value).bits;
              if(type==GGML_TYPE_F32)memcpy(initial.data()+at,&bits,4);else memcpy(initial.data()+at,&half,2);
            } else if(type==GGML_TYPE_F32) memcpy(initial.data()+at,&fill_words[i%24],4);else memcpy(initial.data()+at,&small[i%24],2);
          }
          else if(folding) for(size_t i=0;i<initial.size()/4;i++) {const float value=i%4==0?16777216.0f:i%4==2?-16777216.0f:1.0f;memcpy(initial.data()+i*4,&value,4);}
          else for(size_t i=0;i<initial.size();i++) initial[i]=(i*37+kind*13+layout+(operand==right?71:0))%256;
          if(!initial.empty()) ggml_backend_tensor_set(operand,initial.data(),0,initial.size());
        }
        if(!output.empty()) {if(wordwise && inplace) ggml_backend_tensor_get(result,output.data(),0,output.size());else ggml_backend_tensor_set(result,output.data(),0,output.size());}
        ggml_cgraph *graph=ggml_new_graph(ctx); ggml_build_forward_expand(graph,result);
        if(triangular && pattern==4) {input->ne[1]++;result->ne[1]++;}
        if((spatial && operation==0 && pattern%2 && ggml_is_empty(input) && !ggml_is_empty(result)) || (triangular && pattern==4) || (windowing && operation==1 && window_case==9)) {
          CHECK(!ggml_backend_dev_supports_op(dev,result)); CHECK(ggml_backend_graph_compute(backend,graph)==GGML_STATUS_FAILED);
          if(triangular) {input->ne[1]--;result->ne[1]--;}
          std::vector<unsigned char> unchanged(output.size()); ggml_backend_tensor_get(result,unchanged.data(),0,unchanged.size()); CHECK(unchanged==output);
          ggml_backend_buffer_free(buffer); ggml_free(ctx); cases++;refused++;continue;
        }
        ggml_cgraph *oracle_graph=ggml_new_graph(ctx); ggml_build_forward_expand(oracle_graph,expected_tensor);
        CHECK(ggml_backend_graph_compute(reference,oracle_graph)==GGML_STATUS_SUCCESS);
        if((kind==0 || unarying || selecting || relative) && pattern==0 && units==(spatial && operation?2:repeating?3:1) && rows==(spatial && operation?2:1) && batches==(spatial && operation==2?2:1) && layout==0) {
          CHECK(refuses_change(backend,dev,graph,result,result->ne[0],result->ne[0]+(repeating?1:block)));
          CHECK(refuses_change(backend,dev,graph,result,result->src[concatenating || (!spatial && !triangular && !casting && !wordwise && operation==2)?1:0],nullptr));
          if(concatenating) {
            CHECK(refuses_change(backend,dev,graph,result,result->op_params[0],int32_t(4)));
            CHECK(refuses_change(backend,dev,graph,result,right->type,GGML_TYPE_F16));
            CHECK(refuses_change(backend,dev,graph,result,right->ne[(operation+1)%4],right->ne[(operation+1)%4]+1));
          }
          if(relative) {
            CHECK(refuses_change(backend,dev,graph,result,input->type,GGML_TYPE_I32));
            CHECK(refuses_change(backend,dev,graph,result,result->type,GGML_TYPE_BF16));
            CHECK(refuses_change(backend,dev,graph,result,input->ne[1],input->ne[1]+1));
            CHECK(refuses_change(backend,dev,graph,result,result->ne[2],result->ne[2]+1));
            CHECK(refuses_change(backend,dev,graph,result,result->ne[3],int64_t(2)));
            CHECK(refuses_change(backend,dev,graph,result,result->data,input->data));
            CHECK(refuses_change(backend,dev,graph,result,input->ne[2],int64_t(0)));
            CHECK(refuses_change(backend,dev,graph,result,input->nb[0],size_t(1)));
            CHECK(refuses_change(backend,dev,graph,result,result->nb[0],size_t(1)));
            CHECK(refuses_change(backend,dev,graph,result,input->data,nullptr,false));
          }
          if(windowing) {CHECK(refuses_change(backend,dev,graph,result,result->op_params[operation?0:2],int32_t(0)));CHECK(refuses_change(backend,dev,graph,result,result->op_params[operation?0:2],int32_t(-1)));CHECK(refuses_change(backend,dev,graph,result,input->type,GGML_TYPE_F16));if(!operation)CHECK(refuses_change(backend,dev,graph,result,result->op_params[0],int32_t(0)));}
          if(unarying) {CHECK(refuses_change(backend,dev,graph,result,result->type,type==GGML_TYPE_F32?GGML_TYPE_F16:type==GGML_TYPE_F16?GGML_TYPE_BF16:GGML_TYPE_F16));if(result->op==GGML_OP_UNARY) {CHECK(refuses_change(backend,dev,graph,result,result->op_params[0],int32_t(GGML_UNARY_OP_COUNT)));CHECK(refuses_change(backend,dev,graph,result,result->op_params[0],int32_t(-1)));}}
          if(wordwise) {CHECK(refuses_change(backend,dev,graph,result,input->type,selecting && operation?GGML_TYPE_F32:GGML_TYPE_I32));CHECK(refuses_change(backend,dev,graph,result,result->type,selecting?GGML_TYPE_F32:GGML_TYPE_I32));CHECK(refuses_change(backend,dev,graph,result,input->data,nullptr,false));}
          if(selecting) {CHECK(refuses_change(backend,dev,graph,result,input->ne[2],int64_t(3)));if(operation) {CHECK(refuses_change(backend,dev,graph,result,result->src[1],nullptr));CHECK(refuses_change(backend,dev,graph,result,right->type,GGML_TYPE_F32));}else {CHECK(refuses_change(backend,dev,graph,result,input->ne[3],int64_t(2)));CHECK(refuses_change(backend,dev,graph,result,input->ne[0],int64_t(INT32_MAX)+1));}}
          if(casting) {CHECK(refuses_change(backend,dev,graph,result,result->src[1],nullptr));CHECK(refuses_change(backend,dev,graph,result,input->type,GGML_TYPE_I32));CHECK(refuses_change(backend,dev,graph,result,result->type,GGML_TYPE_I32));}
          if(filling) {
            auto saved=input->type; input->type=GGML_TYPE_BF16;
            CHECK(refuses_change(backend,dev,graph,result,result->type,GGML_TYPE_BF16));
            input->type=saved;
          }
          if(triangular) {CHECK(refuses_change(backend,dev,graph,result,result->op_params[0],int32_t(-1)));CHECK(refuses_change(backend,dev,graph,result,result->op_params[0],int32_t(4)));CHECK(refuses_change(backend,dev,graph,result,input->type,GGML_TYPE_I32));}
          if(diagonal) {CHECK(refuses_change(backend,dev,graph,result,input->ne[1],int64_t(2)));CHECK(refuses_change(backend,dev,graph,result,result->ne[1],result->ne[1]+1));CHECK(refuses_change(backend,dev,graph,result,input->type,GGML_TYPE_I32));}
          if(spatial) CHECK(refuses_change(backend,dev,graph,result,result->op_params[0],int32_t(operation==2?input->ne[0]:-1)));
        }
        auto saved_data=input->data;
        auto saved_buffer=input->buffer;
        if(filling && !operation && layout==3) {input->data=nullptr;input->buffer=nullptr;}
        CHECK(ggml_backend_dev_supports_op(dev,result)); CHECK(ggml_backend_graph_compute(backend,graph)==GGML_STATUS_SUCCESS);
        input->data=saved_data; input->buffer=saved_buffer;
        CHECK(compare_result(result,expected_tensor,output,wordwise && !selecting?(scanning?2:1):0));
        ggml_backend_buffer_free(buffer); ggml_free(ctx); cases++;
      }
      }
      CHECK(refused==(spatial?200:triangular?384:windowing?216:0));
      ggml_backend_free(reference); ggml_backend_free(backend); printf("RESULT cases %zu\n",cases);return 0;
    }
    if (!strcmp(argv[3], "empty")) {
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
    if (!strcmp(argv[3], "elementwise")) {
      size_t cases = 0;
      std::ofstream trace(std::string(argv[2])+".elementwise-fixtures",std::ios::binary);CHECK(trace.good());
      uint64_t header[4];for(auto &value:header)CHECK(read_fixture_word(value));
      CHECK(header[0]==120 && header[1]==24 && header[2]==2 && header[3]==165);
      for(size_t ordinal=0;ordinal<header[0];ordinal++) {
        uint64_t fixture[24];for(auto &value:fixture)CHECK(read_fixture_word(value));
        CHECK(fixture[23]==ordinal && fixture[0]<8);
        const int kind=fixture[0];
        ggml_init_params params = {8*1024*1024, nullptr, true};
        ggml_context *ctx = ggml_init(params); CHECK(ctx);
        ggml_tensor *left=ggml_new_tensor_4d(ctx,GGML_TYPE_F32,fixture[4],fixture[5],fixture[6],fixture[7]);
        ggml_tensor *right=kind<4?ggml_new_tensor_4d(ctx,GGML_TYPE_F32,fixture[8],fixture[9],fixture[10],fixture[11]):nullptr;
        for(int source=0;source<2;source++) {auto operand=source?right:left;if(operand)for(int axis=0;axis<4;axis++)operand->nb[axis]=fixture[12+source*4+axis];}
        ggml_tensor *(*binary[])(ggml_context *,ggml_tensor *,ggml_tensor *)={ggml_add,ggml_sub,ggml_mul,ggml_div};
        ggml_tensor *(*unary[])(ggml_context *,ggml_tensor *)={ggml_sqrt,ggml_sqr,ggml_neg,ggml_abs};
        auto operation = [&](ggml_tensor *a, ggml_tensor *b) { return kind<4?binary[kind](ctx,a,b):unary[kind-4](ctx,a); };
        ggml_tensor *result=operation(left,right);
        ggml_tensor *reference_result=operation(ggml_cont(ctx,left),right?ggml_cont(ctx,right):nullptr);
        CHECK(result && uint64_t(result->op)==fixture[1]);
        if (kind>=6) CHECK(uint64_t(ggml_get_unary_op(result))==fixture[2]);
        ggml_backend_buffer_t buffer=ggml_backend_alloc_ctx_tensors(ctx,backend); CHECK(buffer);
        CHECK(bind_fixture_words(left,right,header[3])==0);
        const uint64_t fields[]={uint64_t(kind),uint64_t(result->op),uint64_t(kind>=6?ggml_get_unary_op(result):0),fixture[3],
          uint64_t(left->ne[0]),uint64_t(left->ne[1]),uint64_t(left->ne[2]),uint64_t(left->ne[3]),
          uint64_t(right?right->ne[0]:0),uint64_t(right?right->ne[1]:0),uint64_t(right?right->ne[2]:0),uint64_t(right?right->ne[3]:0),
          left->nb[0],left->nb[1],left->nb[2],left->nb[3],right?right->nb[0]:0,right?right->nb[1]:0,right?right->nb[2]:0,right?right->nb[3]:0,
          uint64_t(ggml_nelements(left)),uint64_t(right?ggml_nelements(right):0),fixture[22],cases};
        observe_fixture(trace,fields,left,right);
        ggml_cgraph *graph=ggml_new_graph(ctx); ggml_build_forward_expand(graph,result);
        CHECK(ggml_backend_dev_supports_op(dev,result));
        ggml_cgraph *reference_graph=ggml_new_graph(ctx); ggml_build_forward_expand(reference_graph,reference_result);
        CHECK(ggml_backend_dev_supports_op(ggml_backend_get_device(reference),reference_result));
        CHECK(ggml_backend_graph_compute(reference,reference_graph)==GGML_STATUS_SUCCESS);
        ggml_backend_tensor_memset(result,0xA5,0,ggml_nbytes(result));
        CHECK(ggml_backend_graph_compute(backend,graph)==GGML_STATUS_SUCCESS);
        CHECK(compare_result(result,reference_result,std::vector<unsigned char>(ggml_nbytes(result),0xA5),1));
        if (fixture[22]) {
          CHECK((fixture[22]==1 && kind==0) || (fixture[22]==2 && kind==6));
          if (kind==0) {
            CHECK(refuses_change(backend,dev,graph,result,result->src[1],nullptr));
            CHECK(refuses_change(backend,dev,graph,result,right->ne[1],int64_t(2)));
            CHECK(refuses_change(backend,dev,graph,result,result->ne[1],int64_t(2)));
            CHECK(refuses_change(backend,dev,graph,result,left->type,GGML_TYPE_COUNT));
          } else CHECK(refuses_change(backend,dev,graph,result,result->op_params[0],int32_t(GGML_UNARY_OP_COUNT)));
        }
        ggml_backend_buffer_free(buffer); ggml_free(ctx); cases++;
      }
      CHECK(refusal_probes==5 && fgetc(stdin)==EOF);
      CHECK(ggml_backend_reg_count()==1 && ggml_backend_dev_count()==1);
      ggml_backend_free(reference); ggml_backend_free(backend);
      printf("RESULT cases %zu\n",cases); return 0;
    }
    if (!strcmp(argv[3], "matrix")) {
      size_t matrix_cases = 0; bool negatives_checked = false;
      std::ofstream trace(std::string(argv[2])+".matrix-fixtures",std::ios::binary);CHECK(trace.good());
      uint64_t header[4];for(auto &value:header)CHECK(read_fixture_word(value));
      CHECK(header[0]==1280 && header[1]==24 && header[2]==2 && header[3]==165);
      for(size_t ordinal=0;ordinal<header[0];ordinal++) {
        uint64_t fixture[24];for(auto &value:fixture)CHECK(read_fixture_word(value));
        CHECK(fixture[23]==ordinal);
        ggml_init_params params = {8*1024*1024, nullptr, true};
        ggml_context *ctx = ggml_init(params); CHECK(ctx);
        ggml_tensor *left = ggml_new_tensor_4d(ctx, GGML_TYPE_F32, fixture[2], fixture[0], fixture[4], fixture[5]);
        ggml_tensor *right = ggml_new_tensor_4d(ctx, GGML_TYPE_F32, fixture[2], fixture[1], fixture[6], fixture[7]);
        for(int source=0;source<2;source++) for(int axis=0;axis<4;axis++) (source?right:left)->nb[axis]=fixture[8+source*4+axis];
        ggml_tensor *product = ggml_mul_mat(ctx, left, right); CHECK(product);
        ggml_tensor *oracle = ggml_mul_mat(ctx, left, right); CHECK(oracle);
        for(int axis=0;axis<4;axis++)product->nb[axis]=fixture[16+axis];
        ggml_backend_buffer_t buffer = ggml_backend_alloc_ctx_tensors(ctx, backend);
        CHECK(buffer || (!ggml_nbytes(left) && !ggml_nbytes(right) && !ggml_nbytes(product)));
        CHECK(bind_fixture_words(left,right,header[3])==0);
        const uint64_t fields[]={uint64_t(left->ne[1]),uint64_t(right->ne[1]),uint64_t(left->ne[0]),fixture[3],uint64_t(left->ne[2]),uint64_t(left->ne[3]),uint64_t(right->ne[2]),uint64_t(right->ne[3]),
            left->nb[0],left->nb[1],left->nb[2],left->nb[3],right->nb[0],right->nb[1],right->nb[2],right->nb[3],product->nb[0],product->nb[1],product->nb[2],product->nb[3],
            uint64_t(ggml_nelements(left)),uint64_t(ggml_nelements(right)),uint64_t(fixture[22]!=0),matrix_cases};
        observe_fixture(trace,fields,left,right);
        ggml_cgraph *graph = ggml_new_graph(ctx); ggml_build_forward_expand(graph, product);
        CHECK(ggml_backend_dev_supports_op(dev, product));
        ggml_cgraph *reference_graph=ggml_new_graph(ctx); ggml_build_forward_expand(reference_graph,oracle);
        CHECK(ggml_backend_graph_compute(reference, reference_graph) == GGML_STATUS_SUCCESS);
        const std::vector<unsigned char> expected(ggml_nbytes(product),0xA5);
        if(!expected.empty()) ggml_backend_tensor_memset(product,0xA5,0,expected.size());
        if (fixture[22]) {
          CHECK(!negatives_checked && ggml_nbytes(product) && ggml_nbytes(left));
          negatives_checked = true;
          CHECK(refuses_change(backend,dev,graph,product,product->ne[0],product->ne[0]+1));
          CHECK(refuses_change(backend,dev,graph,product,product->src[0],nullptr));
          CHECK(refuses_change(backend,dev,graph,product,product->buffer,nullptr,false));
          ggml_backend_buffer_t foreign=ggml_backend_cpu_buffer_from_ptr(left->data,ggml_nbytes(left)); CHECK(foreign);
          CHECK(refuses_change(backend,dev,graph,product,left->buffer,foreign,false)); ggml_backend_buffer_free(foreign);
          ggml_tensor *unsupported=ggml_dup(ctx,product);
          ggml_cgraph *bad_graph=ggml_new_graph(ctx); ggml_build_forward_expand(bad_graph,unsupported);
          CHECK(refuses_change(backend,dev,bad_graph,product,unsupported->op,GGML_OP_COUNT,false));
        }
        CHECK(ggml_backend_graph_compute(backend, graph) == GGML_STATUS_SUCCESS);
        CHECK(compare_result(product,oracle,expected,1));
        ggml_backend_buffer_free(buffer); ggml_free(ctx); matrix_cases++;
      }
      CHECK(negatives_checked && refusal_probes==5 && fgetc(stdin)==EOF);
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
    for(ggml_tensor *node : {input,reshape,ggml_view_1d(ctx,input,2,0),ggml_permute(ctx,reshape,1,0,2,3),ggml_transpose(ctx,reshape)}) {
      if(node!=input && node!=reshape) CHECK(ggml_backend_view_init(node)==GGML_STATUS_SUCCESS);
      ggml_cgraph *graph = ggml_new_graph(ctx); ggml_build_forward_expand(graph, node);
      CHECK(ggml_backend_dev_supports_op(dev, node));
      CHECK(ggml_backend_graph_compute(backend, graph) == GGML_STATUS_SUCCESS);
    }
    ggml_tensor *invalid_op = ggml_dup(ctx, input);
    ggml_cgraph *unsupported = ggml_new_graph(ctx); ggml_build_forward_expand(unsupported, invalid_op);
    invalid_op->op = GGML_OP_COUNT;
    CHECK(!ggml_backend_dev_supports_op(dev, invalid_op));
    CHECK(ggml_backend_graph_compute(backend, unsupported) == GGML_STATUS_FAILED);
    ggml_backend_buffer_free(graph_buffer); ggml_free(ctx); ggml_backend_free(reference); ggml_backend_free(backend);
    printf("RESULT cases %zu\n", cases); return 0;
}
"##;
}
