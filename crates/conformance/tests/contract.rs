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

fn provider_library() -> std::path::PathBuf {
    std::env::current_exe()
        .unwrap()
        .with_file_name("libvtensor_backend.so")
        .canonicalize()
        .expect("Cargo-built vTensor backend")
}

/// The body of the top-level declaration named `name`, up to its closing brace.
fn declaration<'a>(model: &'a str, name: &str) -> &'a str {
    let (_, body) = model
        .split_once(&format!("\n  name: {name},\n"))
        .unwrap_or_else(|| panic!("{name} is not declared"));
    body.split_once("\n}\n")
        .unwrap_or_else(|| panic!("{name} is not closed"))
        .0
}

// Keep each imported batch individually selectable with Cargo. The same entries
// bind the frozen input counts to the test that calls the native evaluator.
macro_rules! ggml_batches {
    ($( $(#[$doc:meta])* fn $name:ident() { $mode:literal, $cases:literal, [$($model:literal),*] $(=> $probe:expr)? } )+) => {
        const GGML_BATCHES: &[(&str, usize)] = &[$(($mode, $cases)),+];
        $(
            $(#[$doc])*
            fn $name() {
                let root = root();
                $(tensor_layout::certified_current_model(&root, $model);)*
                $($probe;)?
                native_backend_oracle::verify(&root, $mode, $cases);
            }
        )+
    };
}

ggml_batches! {
    /// `VT-11`: GGML discovers the Prism-defined vTensor device and its owned buffers preserve upstream allocation and byte-transfer behavior
    #[test]
    fn native_backend_vt_11() { "buffers", 108, ["layout", "native"] }
    /// `VT-12`: GGML F32 matrix-product graphs execute through Verdant across rectangular shapes, padded storage and batch broadcasting and match the imported GGML CPU reference
    #[test]
    fn matrix_graphs_vt_12() { "matrix", 1280, ["matrix", "index"] => assert!(vtensor_index::run(u64::MAX, 0, 0, 0, 2, 0, 0, 0).is_err()) }
    /// `VT-13`: F32 arithmetic and unary graphs execute through Verdant with Prism-defined scalar semantics and broadcasting and match the imported GGML CPU reference
    #[test]
    fn elementwise_graphs_vt_13() { "elementwise", 120, ["typed", "pointwise"] }
    /// `VT-14`: Prism empty-node admission matches the imported GGML CPU no-op behavior for empty copy graphs across every live storage type without accessing operand buffers
    #[test]
    fn empty_graphs_vt_14() { "empty", 864, [] }
    /// `VT-15`: Prism plans same-type DUP, CONT and CPY across every live GGML storage type and Verdant moves the admitted blocks with upstream-equivalent logical order and preserved padding
    #[test]
    fn copy_graphs_vt_15() { "copy", 3456, [] }
    /// Compare the IQ2_XXS identity copy with the actual GGML CPU copy kernel.
    #[test]
    fn iq2_identity_copy_vt_15() { "iq2-identity-copy", 1, [] }
    /// `VT-16`: Prism models four-axis REPEAT addressing and Verdant preserves encoded values for every type admitted by the imported GGML CPU repeat oracle
    #[test]
    fn repeat_graphs_vt_16() { "repeat", 160, [] }
    /// `VT-17`: Prism selects and addresses CONCAT source blocks on every axis and Verdant execution matches the imported GGML CPU oracle across all live storage types
    #[test]
    fn concat_graphs_vt_17() { "concat", 10368, [] }
    /// `VT-18`: Prism defines F32 zero and circular padding, reflected padding and signed four-axis roll addressing and Verdant execution matches the imported GGML CPU oracle
    #[test]
    fn spatial_graphs_vt_18() { "spatial", 2256, [] }
    /// `VT-19`: Prism plans SET and ACC initialization, bounded update views and aliases and Verdant execution matches the imported GGML CPU oracle for supported float and integer storage
    #[test]
    fn update_graphs_vt_19() { "update", 4608, [] }
    /// `VT-20`: Prism models four-axis REPEAT_BACK initialization and accumulation order and Verdant execution matches the imported GGML F32 CPU oracle across strided and empty tensors
    #[test]
    fn fold_graphs_vt_20() { "fold", 672, [] }
    /// `VT-21`: Prism defines batched DIAG placement and Verdant preserves diagonal encodings and positive-zero off-diagonal values for every type supported by the imported GGML CPU oracle
    #[test]
    fn diagonal_graphs_vt_21() { "diagonal", 384, [] }
    /// `VT-22`: Prism defines all four TRI variants and Verdant preserves retained encodings and zeroes masked elements for every type supported by the imported GGML CPU oracle
    #[test]
    fn triangular_graphs_vt_22() { "triangular", 1920, [] }
    /// `VT-23`: Prism plans copied and in-place FILL and Verdant converts and transfers F32 and F16 constants with upstream-equivalent encodings and preserved padding
    #[test]
    fn fill_graphs_vt_23() { "fill", 15360, [] }
    /// `VT-24`: Prism plans mixed F32 F16 and BF16 casts and copies by logical element order and Verdant conversion matches the imported GGML CPU oracle across reshaped strided and empty layouts
    #[test]
    fn cast_graphs_vt_24() { "cast", 1920, [] }
    /// `VT-25`: Prism models window partition padding and window join logical indexing and Verdant transfers match the imported GGML CPU oracle for independent strided empty and bounded window inputs
    #[test]
    fn window_graphs_vt_25() { "window", 3024, [] }
    /// `VT-26`: Prism plans scale bias clamp and leaky ReLU graphs and Verdant matches the imported GGML CPU kernels for every supported float storage type across copied in-place strided and empty tensors
    #[test]
    fn transform_graphs_vt_26() { "transform", 18816, [] }
    /// `VT-27`: Prism plans thirteen unary operations for all five supported GGML float source destination pairs and Verdant matches copied in-place strided and empty CPU graphs with exact finite outputs and preserved padding
    #[test]
    fn unary_graphs_vt_27() { "unary", 4992, [] }
    /// `VT-28`: Prism models row-local F32 cumulative sums with exact first-word preservation and Verdant matches the imported GGML CPU scan across strided and empty graphs
    #[test]
    fn scan_graphs_vt_28() { "scan", 528, [] }
    /// `VT-29`: Prism models SUM SUM_ROWS and MEAN accumulation precision rounding and empty reductions and Verdant matches every supported float type against the imported GGML CPU oracle
    #[test]
    fn reduction_graphs_vt_29() { "reduction", 4080, [] }
    /// `VT-30`: Prism models ARGMAX last-tie and NaN state and logical COUNT_EQUAL across all four tensor axes and Verdant matches imported GGML CPU row operations with typed refusals and exact integer outputs
    #[test]
    fn row_graphs_vt_30() { "rows", 2264, [] => verify_row_graphs() }
    /// `VT-31`: Prism models indexed F16 and F32 row-gradient accumulation into F32 and Verdant matches imported GGML GET_ROWS_BACK graphs with duplicate indices empty tensors and atomic invalid-index refusals
    #[test]
    fn row_gradient_graphs_vt_31() { "row-gradient", 3360, [] }
    /// `VT-32`: Prism models four-dimensional indexed row gathering and every executable GGML CPU source format and Verdant decodes the packed blocks with imported authoritative codebooks while preserving integer words padding empty tensors and atomic invalid-index refusals
    #[test]
    fn row_gather_graphs_vt_32() { "row-gather", 384, [] }
    /// `VT-33`: Prism models GGML relative-position word selection and Verdant preserves F16 and BF16 source encodings in F16 outputs across strided empty and higher-dimensional source layouts
    #[test]
    fn relative_position_graphs_vt_33() { "relative-position", 192, [] }
    /// `VT-34`: Prism models three-source indexed row addition and Verdant matches GGML ADD_ID arithmetic and exact minus-one row copies across strides empty tensors and atomic invalid-index refusals
    #[test]
    fn indexed_add_graphs_vt_34() { "indexed-add", 164, [] }
    /// `VT-37`: Prism models zero-source GGML ARANGE planning and Verdant executes every required imported upstream ARANGE case with atomic invalid-parameter refusals
    #[test]
    fn generator_graphs_vt_37() { "upstream-ARANGE", 13, ["pointwise"] }
    /// `VT-38`: Prism models GGML L2 normalization and Verdant matches every imported ordinary and batched upstream case with atomic invalid-parameter refusals
    #[test]
    fn normalization_graphs_vt_38() { "upstream-L2_NORM,L2_NORM_BATCH", 129, ["pointwise", "norm"] }
    /// `VT-39`: Prism models copied and in-place GGML ADD_REL_POS event ordering and Verdant matches the imported CPU oracle with atomic descriptor refusals
    #[test]
    fn relative_position_add_vt_39() { "upstream-ADD_REL_POS", 1315, ["pointwise"] => native_backend_oracle::verify(&root(), "upstream-ORACLE_CALIBRATION", 2) }
    /// `VT-46`: Prism models copied and in-place F32 and BF16 RMS normalization and Verdant matches every imported GGML CPU case with F64 mean ordering and atomic numeric refusals
    #[test]
    fn rms_normalization_vt_46() { "upstream-RMS_NORM", 121, ["pointwise"] }
    /// `VT-46`: Supplemental RMS normalization outputs and atomic refusals match Prism fixtures and the imported CPU oracle.
    #[test]
    fn rms_runtime_edges_vt_46() { "rms-norm", 256, ["pointwise"] }
    /// `VT-48`: NORM preserves stride-selected mean and variance rounding through Prism and Verdant.
    #[test]
    fn normalization_graphs_vt_48() { "upstream-NORM", 61, ["pointwise", "norm"] }
    /// `VT-48`: Prism NORM fixtures match exact CPU words and preserve atomic refusals and padding.
    #[test]
    fn norm_runtime_edges_vt_48() { "norm", 184, ["pointwise", "norm"] }
    /// `VT-49`: GROUP_NORM preserves nested row subtotals and matches ordinary and fused imported graphs.
    #[test]
    fn group_normalization_vt_49() { "upstream-GROUP_NORM,GROUP_NORM_MUL_ADD", 5, ["pointwise", "norm"] }
    /// `VT-49`: GROUP_NORM fixtures preserve nested rounding, physical spacing and atomic refusals.
    #[test]
    fn group_runtime_edges_vt_49() { "group-norm", 224, ["pointwise", "norm"] }
    /// `VT-50`: RMS_NORM_BACK preserves its two-source reductions and narrowing order.
    #[test]
    fn backward_normalization_vt_50() { "upstream-RMS_NORM_BACK", 10, ["pointwise", "norm"] }
    /// `VT-50`: Backward normalization fixtures preserve both sources, narrowing and atomic refusals.
    #[test]
    fn backward_runtime_edges_vt_50() { "rms-backward", 353, ["pointwise", "norm"] }
    /// `VT-51`: Full composed graphs preserve mixed storage, broadcasting and aliases.
    #[test]
    fn composed_normalization_vt_51() { "upstream-NORM_SCALE,RMS_NORM_SCALE,NORM_MUL_ADD,RMS_NORM_MUL_ADD,ADD_RMS_NORM", 187, ["typed", "pointwise", "norm"] }
    /// `VT-51`: Exact CPU words and atomic refusals cover every scalar storage tuple.
    #[test]
    fn typed_runtime_edges_vt_51() { "typed-binary", 977, ["pointwise", "typed"] }
    /// `VT-52`: Scheduled node selection and reuse match the imported CPU.
    #[test]
    fn scheduled_compute_flags_vt_52() { "scheduled-flags", 3, ["native"] }
    /// `VT-53`: The upstream factory evaluates all Q8_0 by F32 matrix cases.
    #[test]
    fn symmetric_matrix_graphs_vt_53() { "upstream-MUL_MAT_Q8_0_F32", 122, ["matrix"] }
    /// `VT-54`: The upstream factory evaluates all Q4_0 by F32 matrix cases.
    #[test]
    fn packed_matrix_graphs_vt_54() { "upstream-MUL_MAT_Q4_0_F32", 122, ["matrix"] }
    /// The original GGML graphs exercise grouped Q2_K products with contiguous and viewed weights.
    #[test]
    fn grouped_matrix_graphs_vt_65() { "upstream-MUL_MAT_Q2_K_F32", 56, ["matrix"] }
    /// `VT-55`: Indexed scalar row writes match the independent CPU.
    #[test]
    fn scalar_set_rows_vt_55() { "upstream-SET_ROWS_SCALAR", 126, ["typed"] }
    /// `VT-56`: The imported CPU checks all split, combined and swapped SWIGLU rows.
    #[test]
    fn swiglu_rows_vt_56() { "upstream-SWIGLU", 36, ["typed"] }
    /// `VT-57`: Exercise the upstream rotary-position graph factory on the provider.
    #[test]
    fn rotary_graphs_vt_57() { "upstream-ROPE", 510, ["rotary"] }
    /// `VT-57`: Exercise all upstream inverse-rotation graphs.
    #[test]
    fn rotary_backward_graphs_vt_57() { "upstream-ROPE_BACK", 424, ["rotary"] }
    /// Six scalar input pairs in upstream dense and permuted matrix products.
    #[test]
    fn float_matrix_graphs_vt_64() { "upstream-MUL_MAT_FLOAT_STORAGE", 11, ["pointwise", "matrix"] }

    /// The CPU compares both packed row formats in all four mask/sink combinations.
    #[test]
    fn packed_attention_vt_66() { "upstream-FLASH_ATTN_EXT_PACKED", 8, ["attention"] }

    #[test]
    fn hyperbolic_graphs_vt_67() { "upstream-TANH,SOFTCAP", 13, ["pointwise"] }

    #[test]
    fn capped_attention_vt_68() { "upstream-FLASH_ATTN_EXT_SOFTCAP", 4, ["attention"] }

    #[test]
    fn scalar_outer_products_vt_69() { "upstream-OUT_PROD_SCALAR", 87, ["pointwise", "matrix"] }

    #[test]
    fn image_columns_vt_70() { "upstream-IM2COL_3D_VIEW", 6, ["pointwise"] }

    #[test]
    fn packed_outer_products_vt_71() { "upstream-OUT_PROD_PACKED", 144, ["matrix", "pointwise"] }

    #[test]
    fn planar_columns_vt_72() { "upstream-IM2COL_PLANAR", 4, ["pointwise"] => native_backend_oracle::verify(&root(), "upstream-IM2COL_HALF_INPUT", 2) }

    #[test]
    fn outer_reference_vt_73() { "upstream-OUT_PROD_DECODED", 23, ["matrix", "pointwise"] }

    #[test]
    fn direct_convolution_vt_74() { "upstream-CONV_2D_DIRECT", 5, ["pointwise"] }

    #[test]
    fn gaussian_graphs_vt_75() { "upstream-GELU_FAMILIES", 64, ["pointwise"] }

    #[test]
    fn contiguous_convolution_vt_76() { "upstream-CONV_2D_CONTIGUOUS", 3, ["pointwise"] }

    #[test]
    fn gated_activations_vt_77() { "upstream-GATED_ACTIVATIONS", 108, ["typed", "pointwise"] }

    #[test]
    fn pooling_vt_79() { "upstream-POOLING", 8, ["pointwise"] }

    /// Four imported BF16 attention cases vary mask and sink presence independently.
    #[test]
    fn bf16_attention_vt_63() { "upstream-FLASH_ATTN_EXT_BF16", 4, ["attention"] }

    /// The first dense attention graph from the imported GGML factory.
    #[test]
    fn bf16_row_gradient_vt_62() { "row-gradient-bf16", 1680, ["pointwise"] }

    #[test]
    fn logistic_graphs_vt_61() { "upstream-SIGMOID,SILU,SIGMOID_MUL,SILU_MUL,ADD_SIGMOID_MUL,ADD_SILU_MUL", 122, ["pointwise"] }

    #[test]
    fn elementary_graphs_vt_60() { "upstream-LOG,SIN,COS,EXP", 39, ["pointwise"] }

    #[test]
    fn extended_attention_vt_58() { "upstream-FLASH_ATTN_EXT_FIRST", 1, ["attention"] }
    /// Two short attention graphs cover a populated sink and its absence.
    #[test]
    fn small_attention_vt_58() { "upstream-FLASH_ATTN_EXT_SMALL", 2, ["attention"] }
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
    let paths = contract["universal_policy_paths"].as_array().unwrap();
    let bound: Vec<_> = paths
        .iter()
        .map(|path| path.as_str().unwrap())
        .filter(|&path| path != "template.lock")
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
    use vtensor_allocation::Adt0::*;
    let rows = boundary_rows("allocation", 10, 864, |probe| match probe {
        vtensor_probes::Adt0::C2(c, o, s, a) => [c, o, s, a],
        _ => panic!("missing allocation case"),
    });
    let edges: [([u64; 4], [u64; 4]); 10] =
        serde_json::from_value(native_backend_oracle::ORACLE_BINDINGS["allocation_edges"].clone())
            .unwrap();
    for ([capacity, offset, size, alignment], expected) in rows.chain(edges) {
        let actual = match vtensor_allocation::run(capacity, offset, size, alignment) {
            Ok(C3(at, padded, next)) => [0, at, padded, next],
            Ok(C2) => [1, 0, 0, 0], // capacity exhausted
            Ok(C0) => [2, 0, 0, 0], // invalid alignment
            Ok(C1) => [3, 0, 0, 0], // misaligned offset
            Err(_) => [4, 0, 0, 0], // overflow
        };
        assert_eq!(actual, expected);
    }
}

fn verify_row_graphs() {
    use vtensor_pointwise::{
        Adt0 as Request, Adt1 as Parameters, Adt2 as Descriptor, Adt8 as Response, Adt9 as Plan,
    };
    for (operation, ty, output_type) in [(13, 0, 0), (18, 26, 27)] {
        let empty = Descriptor::C0(ty, 0, 1 << 40, 1 << 40, 1 << 40, 4, 0, 0, 0, 0, 0);
        let output = Descriptor::C0(output_type, 1, 1, 1, 1, 8, 8, 8, 8, 8, 0);
        let params = Parameters::C0(0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
        let request = Request::C0(
            operation,
            params,
            Some(empty.clone()),
            Some(empty),
            None,
            output,
        );
        assert!(matches!(
            vtensor_pointwise::run(request),
            Ok(Response::C0(Plan::C4(2, ..)))
        ));
    }
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
    fixture_queries(36, 5);
}

/// `VT-40`: Native fixture construction initialized bytes execution descriptors and atomic refusal mutations match the frozen pre-migration corpus while expected numerical results remain computed by the imported GGML CPU oracle
#[test]
fn native_fixture_baselines_vt_40() {
    let baseline = &*native_backend_oracle::NATIVE_FIXTURES;
    let released = &*native_backend_oracle::FILL_RELEASE;
    assert_eq!(released["cases"].as_u64(), Some(1920));
    assert_eq!(released["record"].as_array().unwrap().len(), 1920);
    let source =
        std::fs::read_to_string(root().join("crates/conformance/tests/contract.rs")).unwrap();
    assert_eq!(source, include_str!("contract.rs"), "stale fixture gate");
    assert!(source
        .lines()
        .any(|line| line.trim() == "native_backend_oracle::verify(&root, $mode, $cases);"));
    let modes = baseline["mode"].as_array().unwrap();
    assert_eq!(modes.len(), 16);
    for row in modes {
        let name = row["name"].as_str().unwrap();
        let cases = row["cases"].as_u64().unwrap();
        let mut registered = GGML_BATCHES.iter().filter(|(mode, _)| *mode == name);
        assert_eq!(registered.next(), Some(&(name, cases as usize)));
        assert_eq!(registered.next(), None, "duplicate frozen batch {name}");
        assert!(row["bytes"].as_u64().unwrap() > 0);
        assert_eq!(row["sha256"].as_str().unwrap().len(), 64);
        assert_eq!(row["record"].as_array().unwrap().len() as u64, cases);
        assert_eq!(
            row["positive_cases"].as_u64().unwrap() + row["constructed_refusals"].as_u64().unwrap(),
            cases
        );
    }
}

fn probes(
    suite: u64,
    count: u64,
    block: u64,
    bytes: u64,
) -> impl Iterator<Item = vtensor_probes::Adt0> {
    tensor_layout::certified_current_model(&root(), "probes");
    assert!(matches!(
        vtensor_probes::run(suite, count, block, bytes).unwrap(),
        vtensor_probes::Adt0::C0
    ));
    (0..count).map(move |index| vtensor_probes::run(suite, index, block, bytes).unwrap())
}

fn fixture_queries(suite: u64, count: u64) {
    let spatial = suite == 47;
    tensor_layout::certified_current_model(&root(), if spatial { "spatial" } else { "arithmetic" });
    for probe in probes(suite, count, 0, 0) {
        let vtensor_probes::Adt0::C1(query, case, source, item, expected) = probe else {
            panic!("missing fixture probe");
        };
        let actual = if spatial {
            vtensor_spatial::run(query, case, source, item).unwrap()
        } else {
            vtensor_arithmetic::run(query, case, source, item).unwrap()
        };
        assert_eq!(actual, expected);
    }
}

fn boundary_rows<const N: usize, const M: usize>(
    name: &str,
    suite: u64,
    count: u64,
    unpack: impl Fn(vtensor_probes::Adt0) -> [u64; N],
) -> impl Iterator<Item = ([u64; N], [u64; M])> {
    use std::fmt::Write;
    tensor_layout::certified_current_model(&root(), "layout");
    let rows: Vec<_> = probes(suite, count, 0, 0).map(unpack).collect();
    let mut input = String::new();
    for row in &rows {
        writeln!(input, "{}", row.map(|word| word.to_string()).join(" ")).unwrap();
    }
    let reference = ggml_oracle::rows(name, &input);
    assert_eq!(rows.len(), reference.len());
    rows.into_iter().zip(reference)
}

/// `VT-41`: Prism generates COPY fixture enumeration source and target descriptors and initial bytes while preserving all 3456 frozen cases and independent GGML CPU expected results
#[test]
fn copy_fixture_generation_vt_41() {
    fixture_queries(41, 11);
}

/// `VT-42`: Prism generates REPEAT fixture enumeration source target and output layouts and initial bytes while preserving all 160 frozen cases and independent GGML CPU expected results
#[test]
fn repeat_fixture_generation_vt_42() {
    fixture_queries(42, 5);
}

/// `VT-43`: Prism generates CONCAT fixture enumeration operand and output layouts and byte recipes while preserving all 10368 frozen cases and independent GGML CPU expected results
#[test]
fn concat_fixture_generation_vt_43() {
    fixture_queries(43, 5);
}

/// `VT-44`: Prism generates FILL and CAST fixture enumeration typed layouts constants and initial bytes while preserving all 17280 frozen cases source release states and independent GGML CPU expected results
#[test]
fn typed_fixture_generation_vt_44() {
    fixture_queries(44, 10);
}

/// `VT-45`: Verdant supplies certified F64 division and unsigned count to F64 conversion for normalization and preserves the imported GGML accumulator division before narrowing regression
#[test]
fn normalization_arithmetic_vt_45() {
    use verdant_tensor::linear::evaluate_wide;
    tensor_layout::certified_current_model(&verdant_api::ENGINE_ROOT, "quotient");
    assert_eq!(
        evaluate_wide(4, 0x3ff0000000000000, 0x4000000000000000).unwrap(),
        Some(0x3fe0000000000000)
    );
    assert_eq!(
        evaluate_wide(5, u64::MAX, 0).unwrap(),
        Some(0x43f0000000000000)
    );
    let count = evaluate_wide(5, 5, 0).unwrap().unwrap();
    let sum = evaluate_wide(5, 16777217, 0).unwrap().unwrap();
    let mean = evaluate_wide(4, sum, count).unwrap().unwrap();
    assert_eq!(evaluate_wide(2, mean, 0).unwrap(), Some(0x4a4cccce));
    assert_eq!(evaluate_wide(6, 0, 0).unwrap(), None);
}

/// `VT-47`: Prism supplies spatial fixture parameters and rejects undefined recipe queries.
#[test]
fn fixture_generation_vt_47() {
    fixture_queries(47, 4);
}

mod ggml_oracle {

    use sha2::{Digest, Sha256};
    use std::io::Write;
    use std::path::{Path, PathBuf};
    use std::process::{Command, Output, Stdio};

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

    pub(super) fn publish_file(candidate: &Path, destination: &Path, stamp: impl AsRef<Path>) {
        std::fs::write(stamp, file_digest(candidate)).expect("cache digest");
        std::fs::rename(candidate, destination).expect("publish built oracle");
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
        let output = command
            .output()
            .expect("oracle build tool available in SDK");
        checked_output(command, output)
    }

    pub(super) fn tool_json(root: &Path, tool: &str, args: &[&str]) -> serde_json::Value {
        let output = run(Command::new(tool).current_dir(root).args(args));
        serde_json::from_slice(&output).expect("tool JSON response")
    }

    fn checked_output(command: &Command, output: Output) -> Vec<u8> {
        assert!(
            output.status.success(),
            "{command:?}: {:?}\n{}\n{}",
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        output.stdout
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
        // GNU tar checks content, size and file type against the pinned archive in one pass.
        // Extraction by the unprivileged SDK user may change ownership, modes or timestamps.
        let result = Command::new("tar")
            .env("LC_ALL", "C")
            .args(["--compare", "--gzip", "--file"])
            .arg(root.join("vendor/llama.cpp-oracle/source.tar.gz"))
            .arg("--directory")
            .arg(cache)
            .output()
            .expect("compare imported oracle");
        let differences = std::str::from_utf8(&result.stdout).unwrap();
        assert!(
            result.stderr.is_empty(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(
            result.status.code(),
            Some(i32::from(!differences.is_empty()))
        );
        for line in differences.lines() {
            assert!(
                matches!(
                    line.rsplit_once(": ").map(|(_, reason)| reason),
                    Some("Uid differs" | "Gid differs" | "Mode differs" | "Mod time differs")
                ),
                "cached GGML reference differs from its pinned archive: {line}"
            );
        }
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
        let archive_digest = file_digest(imported.join("source.tar.gz"));
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
            verify_file(&executable, executable.with_extension("sha256"));
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
        std::fs::write(cache.join("boundary.cpp"), STORAGE_ORACLE_SOURCE)
            .expect("oracle transport");
        for (input, object) in [
            (source.join("ggml.c"), "ggml.o"),
            (source.join("ggml-quants.c"), "quants.o"),
            (source.join("ggml-alloc.c"), "alloc.o"),
            (cache.join("boundary.cpp"), "boundary.o"),
            (source.join("ggml-backend.cpp"), "backend.o"),
            (source.join("ggml-backend-meta.cpp"), "meta.o"),
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
        let candidate = executable.with_extension("new");
        let shared = [
            "boundary.o",
            "meta.o",
            "alloc.o",
            "backend.o",
            "ggml.o",
            "quants.o",
        ];
        run(Command::new("c++")
            .arg("-Wl,--gc-sections")
            .args(shared.map(|name| cache.join(name)))
            .args(["-lm", "-o"])
            .arg(&candidate));
        publish_file(&candidate, &executable, executable.with_extension("sha256"));
        executable
    }

    pub(super) fn pipe_input(command: &mut Command, input: &[u8]) -> Output {
        let mut child = command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("native GGML oracle");
        let mut stdin = child.stdin.take().expect("oracle input");
        // Drain output while writing so large differential batches cannot fill
        // both pipe buffers and block each other.
        std::thread::scope(|scope| {
            let writer = scope.spawn(move || stdin.write_all(input));
            let output = child.wait_with_output().expect("oracle result");
            writer.join().expect("input writer").expect("write input");
            output
        })
    }

    pub(super) fn batch(command: &mut Command, input: impl AsRef<[u8]>) -> String {
        let output = pipe_input(command, input.as_ref());
        String::from_utf8(checked_output(command, output)).expect("oracle text")
    }

    /// Query the selected imported interface using its verified executable.
    pub(super) fn rows<const N: usize>(mode: &str, input: &str) -> Vec<[u64; N]> {
        let executable = storage_oracle(&super::root());
        verify_file(&executable, executable.with_extension("sha256"));
        batch(Command::new(executable).arg(mode), input)
            .lines()
            .map(|line| {
                let mut values = line
                    .split_whitespace()
                    .map(|v| v.parse::<u64>().expect("oracle integer"));
                let row = std::array::from_fn(|_| values.next().expect("oracle column count"));
                assert!(values.next().is_none(), "oracle column count");
                row
            })
            .collect()
    }

    const STORAGE_ORACLE_SOURCE: &[u8] = br##"// Test transport for three independent upstream GGML interfaces.
#include "ggml.h"
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
#include <inttypes.h>
#include <stdio.h>
#include <climits>

static size_t alignment;
static size_t get_alignment(ggml_backend_buffer_type_t) { return alignment; }
static void *get_base(ggml_backend_buffer_t buffer) { return buffer->context; }
static void allocation_refusal(const char *message) { _exit(strstr(message, "not enough space in the buffer") ? 10 : 99); }
static void quiet(enum ggml_log_level, const char *, void *) {}
static int allocation() {
    size_t capacity, offset, size;
    for (;;) {
        const int fields = scanf("%zu %zu %zu %zu", &capacity, &offset, &size, &alignment);
        if (fields == EOF) return 0;
        if (fields != 4 || !alignment || (alignment & (alignment - 1)) || alignment > 4096 || capacity > 4096 || size > 4096 || offset > 8192 || offset % alignment) return 2;
        fflush(stdout);
        pid_t child = fork();
        if (child < 0) return 3;
        if (!child) {
            ggml_set_abort_callback(allocation_refusal); ggml_log_set(quiet, nullptr);
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

static int storage() {
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
        ggml_tensor tensor = {};
        tensor.type = (enum ggml_type) type;
        for (int i = 0; i < GGML_MAX_DIMS; ++i) {
            tensor.ne[i] = ne[i];
            tensor.nb[i] = nb[i];
        }
        printf("%zu %" PRId64 " %zu\n", ggml_nbytes(&tensor),
               ggml_blck_size(tensor.type), ggml_type_size(tensor.type));
    }
}

static int calls;
static void set(ggml_backend_buffer_t, ggml_tensor *, const void *, size_t, size_t) { calls++; }
static void get(ggml_backend_buffer_t, const ggml_tensor *, void *, size_t, size_t) { calls++; }
static void transfer_refusal(const char *message) {
    if (strstr(message, "tensor buffer not set")) _exit(10);
    if (strstr(message, "tensor not allocated")) _exit(11);
    if (strstr(message, "out of bounds")) _exit(12);
    _exit(99);
}
static int transfer() {
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
            ggml_set_abort_callback(transfer_refusal);
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

int main(int argc, char **argv) {
    if (argc==1 || (argc==2 && !strcmp(argv[1],"storage"))) return storage();
    if (argc==2 && !strcmp(argv[1],"allocation")) return allocation();
    if (argc==2 && !strcmp(argv[1],"transfer")) return transfer();
    return 2;
}
"##;
}

mod ggml_inventory {

    use std::collections::BTreeSet;

    pub(super) static STORAGE_TYPES: std::sync::LazyLock<Vec<u64>> =
        std::sync::LazyLock::new(|| {
            let inventory =
                super::ggml_oracle::read_toml(super::root().join("model/ggml-declarations.toml"));
            inventory["tensor_type"]
                .as_array()
                .expect("types")
                .iter()
                .filter(|row| row["symbol"].as_str() != Some("GGML_TYPE_COUNT"))
                .map(|row| {
                    u64::try_from(row["value"].as_integer().expect("numeric type"))
                        .expect("positive type")
                })
                .collect()
        });

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
            ("glu", "ggml_glu_op", "GGML_GLU_OP_"),
        ] {
            let expected: Vec<_> = declarations(&header, name, prefix)
                .into_iter()
                .map(|(symbol, value)| {
                    toml::Value::Table(toml::toml! { symbol = (symbol) value = (value) })
                })
                .collect();
            let original = inventory[key].as_array().expect("declarations");
            let matches = |rows: &[toml::Value]| rows == expected;
            assert!(
                matches(original),
                "{key} differs from the pinned declarations"
            );
            // This gate must reject omissions, additions, reordering, numeric drift,
            // and invented implementation claims, including changes to its own comparator.
            for mutation in 0..5 {
                let mut rows = original.clone();
                match mutation {
                    0 => {
                        rows.remove(0);
                    }
                    1 => rows.push(rows[0].clone()),
                    2 => rows.swap(0, 1),
                    3 => rows[0]["value"] = toml::Value::Integer(-1),
                    _ => {
                        let table = rows[0].as_table_mut().expect("table");
                        table.insert("implemented".into(), true.into());
                    }
                }
                assert!(!matches(&rows), "mutation {mutation} escaped");
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
        let fixture = &super::native_backend_oracle::ORACLE_BINDINGS["storage_probe"];
        let expected: [[u64; 3]; 6] =
            serde_json::from_value(fixture["span_block_bytes"].clone()).unwrap();
        assert_eq!(
            ggml_oracle::rows("storage", fixture["request"].as_str().unwrap()),
            expected
        );
        let malformed =
            ggml_oracle::pipe_input(&mut std::process::Command::new(&executable), b"malformed\n");
        assert_eq!(malformed.status.code(), Some(2));

        // Exercise enough input and output to exceed pipe capacity in both
        // directions without starting a separate oracle process for every case.
        let batch = ggml_oracle::rows("storage", &"0 1 1 1 1 4 4 4 4\n".repeat(8192));
        assert_eq!(batch.len(), 8192);
        assert!(batch.iter().all(|row| *row == [4, 1, 4]));
        let modified = || std::fs::metadata(&executable).unwrap().modified().unwrap();
        let built_at = modified();
        assert_eq!(ggml_oracle::storage_oracle(&root), executable);
        assert_eq!(
            modified(),
            built_at,
            "unchanged oracle was unnecessarily rebuilt"
        );
    }
}

mod oracle_import {

    use super::{ggml_oracle::file_digest, root};

    use repo_model::Model;

    /// `VT-01`: The imported GGML backend contract, implementation interface, and operator oracle match their pinned upstream source
    #[test]
    fn oracle_import_vt_01() {
        let root = root();
        let artifact = root.join("vendor/llama.cpp-oracle");
        // The frozen manifest binds the exact upstream revision, archive selection,
        // order, filenames and digests. Check the bytes of every listed artifact.
        let manifest = artifact.join("upstream.toml");
        let pin = &super::native_backend_oracle::ORACLE_BINDINGS["ggml_source_manifest_sha256"];
        assert_eq!(pin, &file_digest(&manifest), "GGML source manifest changed");
        let lock = super::ggml_oracle::read_toml(manifest);
        let files = lock["file"].as_array().expect("pinned GGML files");
        for entry in files
            .iter()
            .chain(lock["archive"].as_array().expect("pinned GGML archive"))
        {
            let path = entry["path"].as_str().expect("artifact path");
            assert_eq!(
                file_digest(artifact.join(path)),
                entry["sha256"].as_str().unwrap(),
                "{path}"
            );
        }

        let exemption = super::ggml_oracle::read_json(root.join("oracle-import.lock"));
        let expected_files: Vec<_> = files.iter().map(|file| serde_json::json!({"path": format!("vendor/llama.cpp-oracle/{}", file["path"].as_str().expect("path")), "sha256": file["sha256"].as_str().expect("digest")})).collect();
        let origin = &exemption["source"];
        assert_eq!(origin["repository"], "ggml-org/llama.cpp");
        assert_eq!(origin["commit"].as_str(), lock["commit"].as_str());
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
    use std::{collections::BTreeSet, path::PathBuf, sync::Mutex};

    use vtensor_layout::{run, Adt0, Adt1, Overflow};

    pub(super) fn certified_current_model(root: &std::path::Path, name: &str) {
        static VERIFIED: Mutex<BTreeSet<(PathBuf, String)>> = Mutex::new(BTreeSet::new());
        let mut verified = VERIFIED
            .lock()
            .expect("a failed certificate poisons the cache");
        if !verified.insert((root.canonicalize().unwrap(), name.to_owned())) {
            return;
        }
        let (primary, directory) = match name {
            "quotient" => ("verdant_quotient_f64".to_owned(), "tensor-quotient"),
            "native" => ("vtensor_native_config".to_owned(), "native"),
            _ => (format!("vtensor_{name}"), name),
        };
        let project = if name == "layout" {
            "app/lexlean.toml".to_owned()
        } else {
            format!("app/{name}.lexlean.toml")
        };
        let bundle = format!("generated/{directory}");
        let result = ggml_oracle::read_json(root.join(format!("{bundle}/result.json")));
        let args = [
            "--project",
            &project,
            "--diagnostic-format",
            "json",
            "check",
        ];
        let current = ggml_oracle::tool_json(root, "lexlean", &args);
        for key in ["source_id", "semantic_id"] {
            assert_eq!(
                current[key], result["source"][key],
                "stale {name} model: {key}"
            );
        }
        let lock_digest = ggml_oracle::file_digest(root.join(project).with_extension("lock"));
        assert_eq!(result["source"]["lock_sha256"], lock_digest);
        let request = ggml_oracle::read_json(root.join(format!("app/{name}-production.json")));
        let wanted = request["roots"].as_array().expect("requested roots");
        let exported = result["roots"].as_array().expect("exported roots");
        assert_eq!(exported.len(), wanted.len());
        for (actual, expected) in exported.iter().zip(wanted) {
            assert_eq!(actual["root"], expected["root"]);
            assert_eq!(actual["outputs"].as_array().expect("outputs").len(), 1);
            let output = &actual["outputs"][0];
            assert_eq!(output["target"], expected["targets"][0]);
            assert_eq!(output["package"]["name"], expected["package"]["name"]);
            assert_eq!(output["package"]["version"], expected["package"]["version"]);
            assert_eq!(output["certification"]["status"], "kernel-checked");
        }
        let kernel_lock = ggml_oracle::read_json(root.join(format!("{name}-kernel.lock")));
        assert_eq!(kernel_lock["source"]["result"], result["result_id"]);
        assert_eq!(
            kernel_lock["source"]["attestation"],
            result["verification"]["attestation_id"]
        );
        let output = format!("{bundle}/packages/rust-core/{primary}/src/lib.rs");
        let certified_files: Vec<_> = exported.iter()
        .flat_map(|root| root["outputs"].as_array().expect("outputs"))
        .flat_map(|output| output["package"]["files"].as_array().expect("files").iter().filter(|file| file["path"].as_str().expect("path").ends_with(".rs")).chain(std::iter::once(&output["crate"])))
        .map(|file| serde_json::json!({"path": format!("{bundle}/{}", file["path"].as_str().expect("path")), "sha256": file["sha256"]}))
        .collect();
        assert_eq!(kernel_lock["output"]["path"], output);
        assert!(certified_files.contains(&kernel_lock["output"]));
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

    fn storage_metadata() -> &'static [[u64; 3]] {
        static METADATA: std::sync::LazyLock<Vec<[u64; 3]>> = std::sync::LazyLock::new(|| {
            let root = super::root();
            certified_current_model(&root, "layout");
            let kinds = &*super::ggml_inventory::STORAGE_TYPES;
            let mut input = String::new();
            for &kind in kinds {
                request(&mut input, kind, [0; 4], [0; 4]);
            }
            let metadata = ggml_oracle::rows("storage", &input);
            assert_eq!(metadata.len(), kinds.len(), "oracle omitted a type");
            metadata
        });
        &METADATA
    }

    /// `VT-06`: The Prism-generated four-dimensional byte-span function matches the pinned GGML storage oracle and rejects invalid blocks and arithmetic overflow
    #[test]
    fn tensor_layout_vt_06() {
        let kinds = &*super::ggml_inventory::STORAGE_TYPES;
        let mut inputs = String::new();
        let mut layouts = Vec::new();
        for (&kind, &[_, block, bytes]) in kinds.iter().zip(storage_metadata()) {
            assert!(
                block > 0 && bytes > 0,
                "invalid upstream descriptor for type {kind}"
            );
            for probe in super::probes(6, 52, block, bytes) {
                let vtensor_probes::Adt0::C4(n0, n1, n2, n3, b0, b1, b2, b3) = probe else {
                    panic!("missing layout probe for type {kind}");
                };
                let ne = [n0, n1, n2, n3];
                let nb = [b0, b1, b2, b3];
                request(&mut inputs, kind, ne, nb);
                layouts.push(layout(ne, nb, block, bytes));
            }
        }
        assert_eq!(
            repo_model::artifact::sha256_hex(inputs.as_bytes()),
            "c16f7845ee80d5bcc0cdba789e92eff7395c84b28a95fd06407bef0e14172570"
        );
        let expected = ggml_oracle::rows::<3>("storage", &inputs);
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
        let kinds = &*super::ggml_inventory::STORAGE_TYPES;
        for (&kind, &[_, block, bytes]) in kinds.iter().zip(storage_metadata()) {
            let vtensor_types::Adt0::C1(vtensor_types::Adt1::C0(id, lanes, width)) =
                vtensor_types::run(kind)
            else {
                panic!("declared GGML type {kind} was refused");
            };
            assert_eq!([id, lanes, width], [kind, block, bytes], "GGML type {kind}");
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

    use std::{path::PathBuf, sync::LazyLock};

    pub(super) static ENGINE_ROOT: LazyLock<PathBuf> = LazyLock::new(|| {
        let root = super::root();
        let args = ["metadata", "--locked", "--offline", "--format-version=1"];
        let metadata = super::ggml_oracle::tool_json(&root, "cargo", &args);
        let packages = metadata["packages"].as_array().expect("resolved packages");
        let engine = packages
            .iter()
            .find(|p| p["name"] == "verdant-tensor")
            .expect("Verdant package");
        let pin = super::ggml_oracle::read_json(root.join("model/verdant-development.json"));
        let revision = pin["commit"].as_str().expect("Verdant patch revision");
        let expected =
            format!("git+https://github.com/afflom/verdant-engine?rev={revision}#{revision}");
        assert_eq!(engine["source"], expected, "wrong Verdant source");
        let manifest =
            std::path::Path::new(engine["manifest_path"].as_str().expect("Cargo manifest"));
        manifest
            .ancestors()
            .nth(3)
            .expect("Engine workspace")
            .to_owned()
    });

    /// `VT-02`: The backend resolves the pinned Verdant API with fifteen ordered kernel descriptors and imports only GGML host services
    #[test]
    fn verdant_api_vt_02() {
        LazyLock::force(&ENGINE_ROOT);
        let kernels = verified_unified_tensor_kernels().expect("Verdant verified kernel inventory");
        assert!(kernels.iter().map(|kernel| kernel.index).eq(0..15));

        let library = super::provider_library();
        let listing = super::ggml_oracle::run(
            std::process::Command::new("nm")
                .args(["-D", "--undefined-only", "--format=just-symbols"])
                .arg(library),
        );
        let allowed = super::native_backend_oracle::ORACLE_BINDINGS["allowed_ggml_host_imports"]
            .as_array()
            .expect("GGML host imports");
        for symbol in std::str::from_utf8(&listing).unwrap().lines() {
            if symbol.starts_with("ggml_") || symbol.starts_with("gguf_") {
                assert!(
                    allowed.iter().any(|entry| entry.as_str() == Some(symbol)),
                    "unexpected provider import {symbol}"
                );
            }
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

        let [lhs, rhs, expected]: [[[f64; 4]; 4]; 3] = serde_json::from_value(
            super::native_backend_oracle::ORACLE_BINDINGS["fixed_matrix"]["columns"].clone(),
        )
        .expect("pinned column-major GGML CPU fixture");
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
    fn outcome(decision: vtensor_transfer::Adt0) -> u64 {
        match decision {
            vtensor_transfer::Adt0::C0 => 10,
            vtensor_transfer::Adt0::C1 => 0,
            vtensor_transfer::Adt0::C2 => 11,
            vtensor_transfer::Adt0::C3 => 12,
            vtensor_transfer::Adt0::C4 => 1,
        }
    }
    let rows = boundary_rows("transfer", 9, 1024, |probe| match probe {
        vtensor_probes::Adt0::C3(mode, buffer, data, capacity, offset, size) => {
            [mode, buffer, data, capacity, offset, size]
        }
        _ => panic!("missing transfer case"),
    });
    // The imported unsigned-add guard can wrap. Preserve typed refusals at those boundaries.
    let extremes: [([u64; 6], [u64; 1]); 4] =
        serde_json::from_value(native_backend_oracle::ORACLE_BINDINGS["transfer_edges"].clone())
            .unwrap();
    for ([_, buffer, data, capacity, offset, size], [expected]) in rows.chain(extremes) {
        let decision = vtensor_transfer::run(buffer != 0, data != 0, capacity, offset, size);
        assert_eq!(outcome(decision), expected);
    }
}

mod native_backend_oracle {
    use super::ggml_oracle::{
        compile, digest, file_digest, publish_file, run, storage_oracle, verify_file,
    };
    use sha2::{Digest, Sha256};
    use std::fmt::Write;
    use std::path::Path;
    use std::process::Command;

    fn frozen_json(name: &str, expected: &str) -> serde_json::Value {
        let bytes = std::fs::read(super::root().join(format!("verification/{name}.json"))).unwrap();
        assert_eq!(
            repo_model::artifact::sha256_hex(&bytes),
            expected,
            "frozen {name} changed"
        );
        serde_json::from_slice(&bytes).unwrap()
    }

    use std::sync::LazyLock;

    pub(super) static ORACLE_BINDINGS: LazyLock<serde_json::Value> = LazyLock::new(|| {
        frozen_json(
            "native-oracle-bindings",
            "fff1c7b7288e1af75fb1a9d5b7b1219d085c7966812629991e78c53ab02b79a6",
        )
    });

    fn fixture_record(name: &str) -> serde_json::Value {
        frozen_json(
            name,
            ORACLE_BINDINGS["fixture_digests"][name]
                .as_str()
                .expect("fixture digest"),
        )
    }
    pub(super) static NATIVE_FIXTURES: LazyLock<serde_json::Value> =
        LazyLock::new(|| fixture_record("native-fixture-baseline"));
    pub(super) static FILL_RELEASE: LazyLock<serde_json::Value> =
        LazyLock::new(|| fixture_record("native-fixture-source-release"));
    static TYPED_BINARY: LazyLock<serde_json::Value> =
        LazyLock::new(|| fixture_record("typed-binary-fixture-baseline"));

    pub(super) fn fixture_input(family: u64) -> &'static [u8] {
        static INPUT: [std::sync::OnceLock<Vec<u8>>; 2] = [const { std::sync::OnceLock::new() }; 2];
        INPUT[family as usize].get_or_init(|| {
            use vtensor_fixtures::Adt0 as Request;
            let (model, record) = [
                ("fixtures", "matrix-fixture-baseline"),
                ("arithmetic", "arithmetic-fixture-baseline"),
            ][family as usize];
            super::tensor_layout::certified_current_model(&super::root(), model);
            let baseline = fixture_record(record);
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
            let [cases, fields, sources, padding] =
                [0, 1, 2, 3].map(|item| query(Request::C0(item)));
            let header = ["cases", "metadata_fields", "sources", "padding_byte"]
                .map(|key| baseline[key].as_u64().expect("fixture header"));
            assert_eq!([cases, fields, sources, padding], header);
            let mut input: Vec<u8> = [cases, fields, sources, padding]
                .into_iter()
                .flat_map(u64::to_le_bytes)
                .collect();
            for case in 0..cases {
                for field in 0..fields {
                    input.extend(query(Request::C1(case, field)).to_le_bytes());
                }
                for source in 0..sources {
                    let [count, width] = [0, 1].map(|item| query(Request::C2(case, source, item)));
                    input.extend([count, width].into_iter().flat_map(u64::to_le_bytes));
                    for index in 0..count {
                        input.extend(query(Request::C3(case, source, index)).to_le_bytes());
                    }
                }
            }
            let records = &input[32..];
            assert_eq!(baseline["transcript_bytes"], records.len());
            assert_eq!(
                baseline["transcript_sha256"],
                repo_model::artifact::sha256_hex(records)
            );
            input
        })
    }

    const FIXTURE_BRIDGE: &str = r#"
#[repr(C)]
pub struct Answer { pub status: u64, pub value: u64 }
#[no_mangle]
pub extern "C" fn fixture_query(query:u64, case:u64, source:u64, item:u64)->Answer {
    match vtensor_fixture_model::run(query,case,source,item) {
        Ok(Some(value))=>Answer{status:1,value},
        Ok(None)=>Answer{status:0,value:0},
        Err(_)=>Answer{status:2,value:0},
    }
}
"#;

    fn recipe_tokens(start: &str, end: &str, closing: &str) -> String {
        let (_, tail) = include_str!("contract.rs")
            .split_once(start)
            .expect("recipe start");
        let (body, _) = tail.split_once(end).expect("recipe end");
        format!("{start}{body}{closing}")
            .parse::<proc_macro2::TokenStream>()
            .unwrap()
            .to_string()
    }

    fn fixture_library(common: &Path, model: &'static str) -> std::path::PathBuf {
        use std::{collections::BTreeMap, path::PathBuf, sync::Mutex};
        static LIBRARIES: Mutex<BTreeMap<&str, PathBuf>> = Mutex::new(BTreeMap::new());
        let mut libraries = LIBRARIES.lock().expect("fixture library cache");
        let library = libraries.entry(model).or_insert_with(|| {
            super::tensor_layout::certified_current_model(&super::root(), model);
            let source = super::root().join(format!(
                "generated/{model}/packages/rust-core/vtensor_{model}/src/lib.rs"
            ));
            let mut identity = Sha256::new();
            identity.update(std::fs::read(&source).unwrap());
            identity.update(FIXTURE_BRIDGE);
            identity.update(recipe_tokens(
                "\n    fn fixture_library(",
                "\n    fn compiled_object(",
                "",
            ));
            identity.update(run(Command::new("rustc").args(["--version", "--verbose"])));
            let cache = common
                .join("fixture-model")
                .join(digest(&identity.finalize()));
            std::fs::create_dir_all(&cache).unwrap();
            let library = cache.join("libfixtures.so");
            if !library.exists() {
                let input = cache.join("bridge.rs");
                let bridge =
                    format!("#[path = {source:?}] mod vtensor_fixture_model;\n{FIXTURE_BRIDGE}");
                std::fs::write(&input, bridge).unwrap();
                let pending = library.with_extension("new");
                run(Command::new("rustc")
                    .args([
                        "--edition=2021",
                        "-Copt-level=2",
                        "-Cdebug-assertions=on",
                        "-Coverflow-checks=on",
                        "--crate-type=cdylib",
                    ])
                    .arg(input)
                    .arg("-o")
                    .arg(&pending));
                publish_file(&pending, &library, cache.join("library.sha256"));
            }
            library
        });
        verify_file(&*library, library.parent().unwrap().join("library.sha256"));
        library.clone()
    }

    fn compiled_object(common: &Path, source: &str, object: &Path) {
        let pending = object.with_extension("new");
        let stamp = object.with_extension("sha256");
        if !object.exists() {
            compile(common, common.join("ggml/src").join(source), &pending);
            publish_file(&pending, object, &stamp);
        }
        verify_file(object, stamp);
    }

    // The parent cache already binds the source archive, compiler, flags and SDK.
    fn reference_objects(common: &Path, sources: &[impl AsRef<str>]) -> Vec<std::path::PathBuf> {
        sources
            .iter()
            .map(|source| {
                let source = source.as_ref();
                let object = common.join("reference-object").join(format!("{source}.o"));
                std::fs::create_dir_all(object.parent().unwrap()).unwrap();
                compiled_object(common, source, &object);
                object
            })
            .collect()
    }

    pub(super) fn verify(root: &Path, mode: &str, expected_cases: usize) {
        static ACCESS: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let build_guard = ACCESS.lock().unwrap_or_else(|error| error.into_inner());
        let oracle = storage_oracle(root);
        let common = oracle.parent().unwrap();
        let objects = super::ggml_oracle::read_json(common.join("objects.json"));
        for name in ["ggml.o", "quants.o", "alloc.o", "backend.o", "meta.o"] {
            assert_eq!(objects[name], file_digest(common.join(name)));
        }
        let library = super::provider_library();
        let mut driver = String::from(HEADER);
        if mode.starts_with("upstream-") {
            driver.insert_str(0, "#define VTENSOR_IMPORTED_SUITE\n");
        }
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
        // Bind the compiler/linker recipe, not fixture generation or runtime orchestration.
        // The synthetic closing brace completes `verify` for lossless Rust tokenization.
        identity.update(recipe_tokens(
            "\n    fn compiled_object(",
            "\n        verify_file(&program,",
            "\n}",
        ));
        if mode.starts_with("upstream-") {
            identity.update(UPSTREAM_SUITE.as_bytes());
        }
        let cache = common.join("native").join(digest(&identity.finalize()));
        std::fs::create_dir_all(&cache).unwrap();
        let program = cache.join("native-oracle");
        if !program.exists() {
            std::fs::write(cache.join("driver.cpp"), &driver).unwrap();
            let mut linker = Command::new("c++");
            linker.args(["-Wl,--gc-sections", "-Wl,--export-dynamic"]);
            let frontend = [
                "ggml-backend-reg.cpp",
                "ggml-backend-dl.cpp",
                "ggml-threading.cpp",
            ];
            linker.args(reference_objects(common, &frontend));
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
            let cpu_objects = reference_objects(common, &cpu_sources);
            if mode.starts_with("upstream-") {
                let suite = common
                    .join("imported-suite")
                    .join(digest(&Sha256::digest(UPSTREAM_SUITE)));
                std::fs::create_dir_all(&suite).unwrap();
                let input = suite.join("suite.cpp");
                std::fs::write(&input, UPSTREAM_SUITE).unwrap();
                let object = suite.join("suite.o");
                compiled_object(common, input.to_str().unwrap(), &object);
                linker.arg(object);
            }
            linker.arg(cache.join("driver.o"));
            linker.args(
                ["ggml.o", "quants.o", "alloc.o", "backend.o", "meta.o"]
                    .map(|name| common.join(name)),
            );
            let pending = program.with_extension("new");
            linker.args(cpu_objects);
            linker
                .arg(&library)
                .arg(format!(
                    "-Wl,-rpath,{}",
                    library.parent().unwrap().display()
                ))
                .args(["-lm", "-ldl", "-pthread", "-o"])
                .arg(&pending);
            run(&mut linker);
            publish_file(&pending, &program, cache.join("program.sha256"));
        }
        verify_file(&program, cache.join("program.sha256"));
        static STORAGE_TYPES: LazyLock<Vec<u8>> = LazyLock::new(|| {
            super::ggml_inventory::STORAGE_TYPES
                .iter()
                .map(|kind| format!("{kind}\n"))
                .collect::<String>()
                .into_bytes()
        });
        let input: &[u8] = match mode {
            "matrix" | "elementwise" => fixture_input(u64::from(mode == "elementwise")),
            "update" => &[],
            mode if mode.starts_with("upstream-") => ORACLE_BINDINGS["factory_inputs"][mode]
                .as_str()
                .unwrap_or("")
                .as_bytes(),
            _ => &STORAGE_TYPES,
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
        let expected_snapshot = cache.join(format!(
            "expected-source-{}-{:?}.snapshot",
            std::process::id(),
            std::thread::current().id()
        ));
        std::fs::write(&expected_snapshot, &expected).unwrap();
        let recipe = if mode.starts_with("upstream-") {
            None
        } else if mode == "typed-binary" {
            Some(&*TYPED_BINARY)
        } else {
            NATIVE_FIXTURES["mode"]
                .as_array()
                .unwrap()
                .iter()
                .find(|row| row["name"] == mode)
        };
        let trace = |suffix: &str| {
            std::path::PathBuf::from(format!("{}.{suffix}", expected_snapshot.display()))
        };
        let mut recipes = Vec::new();
        if let Some(recipe) = recipe {
            assert_eq!(recipe["cases"].as_u64().unwrap() as usize, expected_cases);
            recipes.push((trace(&format!("{mode}-recipes")), recipe));
        }
        if mode == "fill" {
            recipes.push((trace("fill-source-release"), &*FILL_RELEASE));
        }
        for (path, _) in &recipes {
            if path.exists() {
                std::fs::remove_file(path).unwrap();
            }
        }
        let mut command = Command::new(&program);
        command.arg(&library).arg(&expected_snapshot).arg(mode);
        if let Some(model) = ORACLE_BINDINGS["fixture_libraries"][mode].as_str() {
            command.arg(fixture_library(common, model));
        }
        // The verified binaries are immutable. Each concurrent evaluator owns its trace paths.
        drop(build_guard);
        let text = super::ggml_oracle::batch(&mut command, input);
        let footer = format!("RESULT cases {expected_cases}\n");
        let transcript = ORACLE_BINDINGS["factory_transcripts"][mode].as_str();
        if let Some(digest) = transcript {
            let rows = text.strip_suffix(&footer).expect("complete factory count");
            assert_eq!(
                repo_model::artifact::sha256_hex(rows.as_bytes()),
                digest,
                "factory ordinals or parameters differ from the pinned transcript"
            );
        } else {
            assert_eq!(text, footer);
        }
        for (path, recipe) in recipes {
            let bytes =
                std::fs::read(&path).expect("this execution must emit its fixture transcript");
            let hash = repo_model::artifact::sha256_hex(&bytes);
            assert_eq!(recipe["bytes"], bytes.len(), "{mode} fixture length");
            assert_eq!(recipe["sha256"], hash, "{mode} fixture digest");
            std::fs::remove_file(path).unwrap();
        }
        if matches!(mode, "matrix" | "elementwise") {
            let path = trace(&format!("{mode}-fixtures"));
            let bound = std::fs::read(&path).expect("native execution emitted its bound fixtures");
            assert!(
                bound == input[32..],
                "{mode} tensor bindings differ from the frozen fixture input"
            );
            std::fs::remove_file(path).unwrap();
        }
        std::fs::remove_file(expected_snapshot).unwrap();
    }

    const UPSTREAM_SUITE: &str = r##"#define main vtensor_unused_upstream_main
#include "tests/test-backend-ops.cpp"
#undef main
struct vtensor_observation : printer {
  size_t records=0; bool bad=false; std::string label;
  void print_test_result(const test_result &result) override {
    ++records; label=result.op_name;
    if(!result.supported || !result.passed || result.test_mode!="test") {
      bad=true;fprintf(stderr,"imported case rejected: %s(%s): %s [backend: %s]\n",result.op_name.c_str(),result.op_params.c_str(),result.error_message.c_str(),result.backend_name.c_str());
    }
  }
};
static bool vtensor_use_reference(ggml_backend_t reference) {
    using set_reference = void (*)(ggml_backend_t,bool);
    auto reg=ggml_backend_dev_backend_reg(ggml_backend_get_device(reference));
    auto use_reference=(set_reference)ggml_backend_reg_get_proc_address(reg,"ggml_backend_cpu_set_use_ref");
    if(!use_reference) return false;
    use_reference(reference,true);return true;
}
// Both references run the original graph on the provider. The CPU either decodes
// the operand types or independently materializes their strided views.
static bool reference_graph_case(test_case & test, ggml_backend_t backend, ggml_backend_t cpu, ggml_op op) {
    const ggml_init_params init{ggml_tensor_overhead()*128+ggml_graph_overhead(), nullptr, true};
    ggml_context_ptr target(ggml_init(init)), reference(ggml_init(init));
    if (!target || !reference) return false;
    ggml_tensor * out=test.build_graph(target.get());
    if (out->op!=op || ggml_backend_dev_supports_op(ggml_backend_get_device(cpu),out)) return false;
    ggml_tensor * expected;
    if (op==GGML_OP_OUT_PROD) {
        auto a=ggml_new_tensor(reference.get(),GGML_TYPE_F32,4,out->src[0]->ne);
        auto b=ggml_new_tensor(reference.get(),GGML_TYPE_F32,4,out->src[1]->ne);
        expected=ggml_out_prod(reference.get(),a,b);
    } else {
        auto original=test.build_graph(reference.get());
        auto kernel=ggml_cont(reference.get(),original->src[0]);
        auto image=ggml_cont(reference.get(),original->src[1]);
        auto p=original->op_params;
        expected=ggml_conv_2d_direct(reference.get(),kernel,image,p[0],p[1],p[2],p[3],p[4],p[5]);
    }
    if (!ggml_are_same_shape(out,expected) || !ggml_backend_supports_op(cpu,expected)) return false;
    ggml_cgraph * original_graph=ggml_new_graph(target.get()), * reference_graph=ggml_new_graph(reference.get());
    ggml_build_forward_expand(original_graph,out);
    ggml_build_forward_expand(reference_graph,expected);
    ggml_backend_buffer_ptr actual_buffer(ggml_backend_alloc_ctx_tensors(target.get(),backend));
    ggml_backend_buffer_ptr reference_buffer(ggml_backend_alloc_ctx_tensors(reference.get(),cpu));
    if (!actual_buffer || !reference_buffer || !ggml_backend_dev_supports_op(ggml_backend_get_device(backend),out)) return false;
    test.initialize_tensors(target.get());
    if (op==GGML_OP_OUT_PROD) {
        for (int k=0;k<2;++k) {
            auto values=tensor_to_float(out->src[k]);
            ggml_backend_tensor_set(expected->src[k],values.data(),0,ggml_nbytes(expected->src[k]));
        }
    } else {
        auto copy=ggml_get_first_tensor(reference.get());
        for (auto src=ggml_get_first_tensor(target.get());src;src=ggml_get_next_tensor(target.get(),src)) {
            if (!copy || src->op!=copy->op || src->type!=copy->type || !ggml_are_same_shape(src,copy) || !ggml_are_same_stride(src,copy)) return false;
            if (src->op==GGML_OP_NONE && !src->view_src) ggml_backend_tensor_copy(src,copy);
            copy=ggml_get_next_tensor(reference.get(),copy);
        }
    }
    if (ggml_backend_graph_compute(backend,original_graph)!=GGML_STATUS_SUCCESS ||
        ggml_backend_graph_compute(cpu,reference_graph)!=GGML_STATUS_SUCCESS) return false;
    auto actual=tensor_to_float(out),wanted=tensor_to_float(expected);
    if (actual.size()!=wanted.size()) return false;
    // An exactly matching all-padding output has zero reference energy and undefined NMSE.
    if (actual==wanted) return true;
    const double error=nmse(wanted.data(),actual.data(),wanted.size());
    if (!(error<=test.max_nmse_err(backend))) {
        fprintf(stderr,"[%s] ERR = %.9f > %.9f\n",ggml_op_name(op),error,test.max_nmse_err(backend));
        return false;
    }
    return true;
}
size_t vtensor_imported_suite(ggml_backend_t backend, ggml_backend_t reference, const char *op) {
    if(!vtensor_use_reference(reference)) return SIZE_MAX;
    vtensor_observation observed;size_t cases=0;
    if (!strcmp(op,"IM2COL_HALF_INPUT")) {
      for (bool planar : {false,true}) {
        const std::array<int64_t,4> input = planar ? std::array<int64_t,4>{7,5,2,3} : std::array<int64_t,4>{7,2,3,1};
        test_im2col test(GGML_TYPE_F16,GGML_TYPE_F16,GGML_TYPE_F16,input,{3,2,2,1},2,1,1,0,1,1,planar);
        if (test.eval(backend,reference,nullptr,&observed) != test_status_t::OK || observed.bad) return SIZE_MAX;
      }
      return observed.records;
    }
    std::vector<size_t> selected;
    size_t id;
    while (std::scanf("%zu", &id)==1) selected.push_back(id);
    if (!std::feof(stdin)) return SIZE_MAX;
    size_t ordinal=0;
    for(auto &test:make_test_cases_eval()) {
      const auto i=ordinal++;
      if (!selected.empty() && (cases==selected.size() || selected[cases]!=i)) continue;
      if (!strcmp(op,"OUT_PROD_DECODED") || !strcmp(op,"CONV_2D_CONTIGUOUS")) {
        const ggml_op operation=!strcmp(op,"OUT_PROD_DECODED") ? GGML_OP_OUT_PROD : GGML_OP_CONV_2D;
        if (selected.empty() || !reference_graph_case(*test,backend,reference,operation)) {
          fprintf(stderr,"CPU reference rejected ordinal %zu: %s\n",i,test->vars().c_str());
          return SIZE_MAX;
        }
        observed.label=ggml_op_name(operation);
        ++observed.records;
      } else {
        const auto status=test->eval(backend,reference,selected.empty()?op:nullptr,&observed);
        if(status==test_status_t::SKIPPED) continue;
        if(status!=test_status_t::OK || observed.bad) return SIZE_MAX;
      }
      if(!selected.empty()) std::printf("CASE\t%zu\t%s\t%s\n",i,observed.label.c_str(),test->vars().c_str());
      ++cases;
    }
    return observed.records==cases && (selected.empty() || cases==selected.size()) ? cases : SIZE_MAX;
}
using vtensor_graph_factory = ggml_tensor * (*)(ggml_context *, void *, void *);
using vtensor_initializer = void (*)(ggml_context *, void *);
ggml_tensor *vtensor_oracle_tensor(void *owner, ggml_context *ctx, ggml_type type, int64_t n0, int64_t n1, int64_t n2, int64_t n3) {
    return static_cast<test_case *>(owner)->ggml_new_tensor_4d(ctx,type,n0,n1,n2,n3);
}
bool vtensor_imported_case(ggml_backend_t backend, ggml_backend_t reference, const char *op, const char *label, void *payload, vtensor_graph_factory factory, vtensor_initializer initialize) {
    struct constructed_case : test_case {
      void *payload;vtensor_graph_factory factory;vtensor_initializer initialize;std::string label;
      constructed_case(void *p, vtensor_graph_factory f, vtensor_initializer init, const char *name):payload(p),factory(f),initialize(init),label(name) {}
      ggml_tensor *build_graph(ggml_context *ctx) override {return factory(ctx,payload,static_cast<test_case *>(this));}
      void initialize_tensors(ggml_context *ctx) override {test_case::initialize_tensors(ctx);if(initialize) initialize(ctx,payload);}
      std::string vars() override {return label;}
      double max_nmse_err() override {return 0.0;}
    } test(payload,factory,initialize,label);
    if(!vtensor_use_reference(reference)) return false;
    vtensor_observation observed;
    return test.eval(backend,reference,op,&observed)==test_status_t::OK && !observed.bad && observed.records==1;
}
"##;

    const HEADER: &str = r##"#include "ggml.h"
#include "ggml-backend.h"
#include "ggml-backend-impl.h"
#include "ggml-alloc.h"
#include "ggml-cpu.h"
#include <cmath>
#include <algorithm>
#include <array>
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
#ifdef VTENSOR_IMPORTED_SUITE
extern size_t vtensor_imported_suite(ggml_backend_t, ggml_backend_t, const char *);
extern bool vtensor_imported_case(ggml_backend_t, ggml_backend_t, const char *, const char *, void *, ggml_tensor * (*)(ggml_context *, void *, void *), void (*)(ggml_context *, void *)=nullptr);
extern ggml_tensor *vtensor_oracle_tensor(void *,ggml_context *,ggml_type,int64_t,int64_t,int64_t,int64_t);
#endif
#define CHECK(x) do { if (!(x)) { fprintf(stderr, "native check line %d: %s\n", __LINE__, #x); return 2; } } while (0)
static void refusal(const char *message) { _exit(strstr(message, "out of bounds") ? 12 : 99); }
static int current_kind = -1, current_layout = -1;
static size_t refusal_probes = 0;
static std::ofstream *recipe_trace=nullptr;
static ggml_context *recipe_context=nullptr;
static uintptr_t recipe_base=0;
static std::vector<ggml_tensor *> recipe_tensors;
static uint64_t recipe_id(const ggml_tensor *t) {
    if(!t)return 0;auto at=std::find(recipe_tensors.begin(),recipe_tensors.end(),t);GGML_ASSERT(at!=recipe_tensors.end());return at-recipe_tensors.begin()+1;
}
static void recipe_word(uint64_t word) {
    uint8_t bytes[8];for(int i=0;i<8;++i)bytes[i]=uint8_t(word>>(8*i));recipe_trace->write((const char *)bytes,8);
}
static void recipe_state(uint64_t phase) {
    if(!recipe_context)return;
    recipe_tensors.clear();
    for(auto t=ggml_get_first_tensor(recipe_context);t;t=ggml_get_next_tensor(recipe_context,t))recipe_tensors.push_back(t);
    recipe_word(phase);recipe_word(recipe_tensors.size());
    for(auto t:recipe_tensors) {
      recipe_word(t->type);recipe_word(t->op);for(auto n:t->ne)recipe_word(n);for(auto n:t->nb)recipe_word(n);
      for(auto p:t->op_params)recipe_word(uint32_t(p));for(auto src:t->src)recipe_word(recipe_id(src));
      recipe_word(recipe_id(t->view_src));recipe_word(t->view_offs);recipe_word(t->buffer!=nullptr);recipe_word(t->data?uintptr_t(t->data)-recipe_base+1:0);
    }
}
static void unexpected_abort(const char *message) { fprintf(stderr, "upstream abort at type %d layout %d: %s\n", current_kind, current_layout, message); _exit(99); }
template<class T, class U>
static bool refuses_change(ggml_backend_t backend, ggml_backend_dev_t dev, ggml_cgraph *graph, ggml_tensor *result, T &field, U bad, bool capability=true) {
    ++refusal_probes;
    std::vector<unsigned char> before(ggml_nbytes(result)),after(before.size());
    ggml_backend_tensor_get(result,before.data(),0,before.size());
    const T saved=field;field=bad;recipe_state(1);
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
    ggml_tensor physical=*result, physical_reference=reference?*reference:ggml_tensor{};
    if(nan_mode==4 && reference) {expected=raw;physical.nb[0]=physical_reference.nb[0]=bytes;result=&physical;reference=&physical_reference;}
    auto nan=[&](const unsigned char *p) {float value;if(result->type==GGML_TYPE_F32)memcpy(&value,p,4);else {uint16_t word;memcpy(&word,p,2);value=result->type==GGML_TYPE_F16?ggml_fp16_to_fp32(word):ggml_bf16_to_fp32(ggml_bf16_t{word});}return std::isnan(value);};
    for(size_t i=0;i<(reference?size_t(ggml_nelements(reference))/block:0);i++) {
      const size_t at=block_address(result,i), reference_at=block_address(reference,i);
      if(nan_mode && (nan_mode!=2 || i%(result->ne[0]/block)) && (nan_mode!=3 || i<size_t(ggml_nelements(result)/result->ne[3])) && nan(actual.data()+at) && nan(raw.data()+reference_at)) memcpy(actual.data()+at,raw.data()+reference_at,bytes);
      memcpy(expected.data()+at,raw.data()+reference_at,bytes);
    }
    if(actual==expected) return true;
    const size_t at=std::mismatch(actual.begin(),actual.end(),expected.begin()).first-actual.begin();
    fprintf(stderr,"oracle mismatch type=%d width=%lld rows=%lld byte=%zu actual=%u expected=%u\n",result->type,(long long)result->ne[0],(long long)result->ne[1],at,actual[at],expected[at]);return false;
}
int main(int argc, char **argv) {
    CHECK(argc == 4 || argc == 5);
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
    if (!strcmp(argv[3],"iq2-identity-copy")) {
        for (auto selected : {reference, backend}) {
            auto ctx=ggml_init({ggml_graph_overhead()+8*ggml_tensor_overhead(),nullptr,true});
            CHECK(ctx);
            auto src=ggml_new_tensor_4d(ctx,GGML_TYPE_IQ2_XXS,256,2,3,4);
            ggml_set_param(src);
            auto dst=ggml_dup_tensor(ctx,src);
            auto out=ggml_cpy(ctx,src,dst);
            auto graph=ggml_new_graph(ctx);
            ggml_build_forward_expand(graph,out);
            auto buffer=ggml_backend_alloc_ctx_tensors(ctx,selected);
            CHECK(buffer);
            std::vector<uint8_t> data(ggml_nbytes(src)), observed(data.size(),0);
            for(size_t i=0;i<data.size();++i) data[i]=uint8_t(i*197+19);
            ggml_backend_tensor_set(src,data.data(),0,data.size());
            ggml_backend_tensor_set(dst,observed.data(),0,observed.size());
            CHECK(ggml_backend_graph_compute(selected,graph)==GGML_STATUS_SUCCESS);
            ggml_backend_tensor_get(out,observed.data(),0,observed.size());
            CHECK(observed==data);
            ggml_backend_buffer_free(buffer); ggml_free(ctx);
        }
        ggml_backend_free(reference); ggml_backend_free(backend);
        puts("RESULT cases 1"); return 0;
    }
    if (!strcmp(argv[3],"scheduled-flags")) {
        std::array<std::array<float,2>,3> expected{};
        for (auto selected : {reference, backend}) {
            auto ctx = ggml_init({1u << 20, nullptr, true}); CHECK(ctx);
            auto input = ggml_new_tensor_1d(ctx, GGML_TYPE_F32, 1);
            auto intermediate = ggml_scale(ctx, input, 2.0f);
            auto result = ggml_add(ctx, intermediate, input);
            ggml_set_input(input); ggml_set_output(intermediate); ggml_set_output(result);
            auto graph = ggml_new_graph(ctx); ggml_build_forward_expand(graph, result);
            CHECK(!input->data && ggml_backend_supports_op(selected,intermediate) && ggml_backend_supports_op(selected,result));
            auto sched = ggml_backend_sched_new(&selected, nullptr, 1, GGML_DEFAULT_GRAPH_SIZE, false, true); CHECK(sched);
            CHECK(ggml_backend_sched_alloc_graph(sched, graph));
            for (auto node : {intermediate,result}) CHECK(ggml_backend_sched_get_tensor_backend(sched,node)==selected);
            for (size_t pass=0; pass<expected.size(); ++pass) {
                const float value = float(pass+3), canary = -17.0f;
                if (pass==1) intermediate->flags &= ~GGML_TENSOR_FLAG_COMPUTE;
                else intermediate->flags |= GGML_TENSOR_FLAG_COMPUTE;
                ggml_backend_tensor_set(input,&value,0,sizeof(value));
                ggml_backend_tensor_set(intermediate,&canary,0,sizeof(canary));
                CHECK(ggml_backend_sched_graph_compute(sched,graph)==GGML_STATUS_SUCCESS);
                ggml_backend_sched_synchronize(sched);
                std::array<float,2> observed;
                ggml_backend_tensor_get(intermediate,&observed[0],0,sizeof(float));
                ggml_backend_tensor_get(result,&observed[1],0,sizeof(float));
                if (selected==reference) {
                    CHECK((observed[0]==canary)==(pass==1)); expected[pass]=observed;
                } else CHECK(observed==expected[pass]);
            }
            ggml_backend_sched_free(sched); ggml_free(ctx);
        }
        ggml_backend_free(reference); ggml_backend_free(backend);
        printf("RESULT cases %zu\n",expected.size()); return 0;
    }
#ifdef VTENSOR_IMPORTED_SUITE
    if (!strncmp(argv[3], "upstream-", 9)) {
      size_t cases = vtensor_imported_suite(backend,reference,argv[3]+9);
      CHECK(cases != SIZE_MAX);
      if (!strcmp(argv[3]+9,"SET_ROWS_SCALAR")) {
        for (const auto type : {GGML_TYPE_I32, GGML_TYPE_I64}) {
          auto ctx=ggml_init({ggml_tensor_overhead()*12+ggml_graph_overhead(),nullptr,true});CHECK(ctx);
          auto dst=ggml_new_tensor_2d(ctx,GGML_TYPE_F16,2,3);
          auto src=ggml_new_tensor_2d(ctx,GGML_TYPE_F32,2,2);
          auto idx=ggml_new_tensor_1d(ctx,type,2);
          auto out=ggml_set_rows(ctx,dst,src,idx);
          auto graph=ggml_new_graph(ctx);ggml_build_forward_expand(graph,out);
          auto buffer=ggml_backend_alloc_ctx_tensors(ctx,backend);CHECK(buffer);
          CHECK(ggml_backend_dev_supports_op(dev,out));
          const uint16_t original[]={1,2,3,4,5,6};
          const float values[]={1,2,3,4};
          ggml_backend_tensor_set(src,values,0,sizeof(values));
          ggml_backend_tensor_set(dst,original,0,sizeof(original));
          const int32_t narrow[]={0,-1}; const int64_t wide[]={0,INT64_C(4294967296)};
          if(type==GGML_TYPE_I32) ggml_backend_tensor_set(idx,narrow,0,sizeof(narrow));
          else ggml_backend_tensor_set(idx,wide,0,sizeof(wide));
          CHECK(ggml_backend_graph_compute(backend,graph)==GGML_STATUS_FAILED);
          uint16_t after[6];ggml_backend_tensor_get(dst,after,0,sizeof(after));
          CHECK(!std::memcmp(original,after,sizeof(after)));
          ggml_backend_buffer_free(buffer);ggml_free(ctx);
        }
      }
      if (!strcmp(argv[3]+9,"ORACLE_CALIBRATION")) {
        CHECK(cases==0);
        auto duplicate=[](ggml_context *ctx,void *,void *owner) {return ggml_dup(ctx,vtensor_oracle_tensor(owner,ctx,GGML_TYPE_F32,4,1,1,1));};
        CHECK(vtensor_imported_case(backend,reference,"DUP","oracle calibration",nullptr,duplicate));
        auto alias_add=[](ggml_context *ctx,void *,void *owner) {auto a=vtensor_oracle_tensor(owner,ctx,GGML_TYPE_F32,4,1,1,1);return ggml_add_inplace(ctx,a,a);};
        CHECK(vtensor_imported_case(backend,reference,"ADD","in-place oracle calibration",nullptr,alias_add));
        CHECK(!vtensor_imported_case(backend,reference,"ADD_REL_POS","wrong operator",nullptr,duplicate));
        cases=2;
      }
      if (!strcmp(argv[3]+9,"ADD_REL_POS")) {
        CHECK(cases==0);
        auto build=[](ggml_context *ctx,void *payload,void *owner) {
          auto d=static_cast<int64_t *>(payload);
          auto a=vtensor_oracle_tensor(owner,ctx,GGML_TYPE_F32,d[0]*d[0],d[1]*d[2],d[3],d[4]);
          auto pw=vtensor_oracle_tensor(owner,ctx,GGML_TYPE_F32,d[0],d[1],d[2],d[3]);
          auto ph=vtensor_oracle_tensor(owner,ctx,GGML_TYPE_F32,d[0],d[1],d[2],d[3]);
          auto view=[&]() {return ggml_view_4d(ctx,a,d[0],d[1],d[2],d[3],4*d[0],4*d[0]*d[1],4*d[0]*d[1]*d[2],0);};
          if(d[6]==1)ph=pw;
          if(d[6]==2)pw=view();
          if(d[6]==3)ph=view();
          if(d[6]==4)pw=ph=view();
          if(d[7]) for(auto input:{a,pw,ph}) for(int axis=0;axis<4;++axis) if(input->ne[axis]==1)input->nb[axis]+=8;
          ggml_set_name(a,"base");ggml_set_name(pw,"width");ggml_set_name(ph,"height");
          auto result=d[5]?ggml_add_rel_pos_inplace(ctx,a,pw,ph):ggml_add_rel_pos(ctx,a,pw,ph);
          ggml_set_name(result,"result");return result;
        };
        for(int64_t k:{1,2,3,8}) for(int64_t qh:{1,2}) for(int64_t qw:{1,3})
        for(int64_t heads:{1,2}) for(int64_t batches:{1,2}) for(int64_t inplace:{0,1})
        for(int64_t alias:{0,1,2,3,4}) for(int64_t unit_strides:{0,1}) {
          int64_t dimensions[]={k,qh,qw,heads,batches,inplace,alias,unit_strides};auto label=std::to_string(cases);
          CHECK(vtensor_imported_case(backend,reference,"ADD_REL_POS",label.c_str(),dimensions,build));++cases;
        }
        size_t tail_probes=0;
        for(bool inplace:{false,true}) {
          ggml_init_params params={2*1024*1024,nullptr,true};auto ctx=ggml_init(params);CHECK(ctx);
          auto a=ggml_new_tensor_4d(ctx,GGML_TYPE_F32,9,1,1,2);auto pw=ggml_new_tensor_1d(ctx,GGML_TYPE_F32,3);auto ph=ggml_new_tensor_1d(ctx,GGML_TYPE_F32,3);
          auto ar=ggml_new_tensor_4d(ctx,GGML_TYPE_F32,9,1,1,2);auto pwr=ggml_new_tensor_1d(ctx,GGML_TYPE_F32,3);auto phr=ggml_new_tensor_1d(ctx,GGML_TYPE_F32,3);
          auto result=inplace?ggml_add_rel_pos_inplace(ctx,a,pw,ph):ggml_add_rel_pos(ctx,a,pw,ph);
          auto control=inplace?ggml_add_rel_pos_inplace(ctx,ar,pwr,phr):ggml_add_rel_pos(ctx,ar,pwr,phr);
          auto buffer=ggml_backend_alloc_ctx_tensors(ctx,backend);CHECK(buffer);
          auto graph=ggml_new_graph(ctx);ggml_build_forward_expand(graph,result);auto expected=ggml_new_graph(ctx);ggml_build_forward_expand(expected,control);
          const uint32_t flags[]={inplace?1u:0u,0u,0x80000000u,0xffffffffu};
          const uint32_t words[][3]={{0x4b800000u,0xcb800000u,0x3f800000u},{0x80000000u,0x80000000u,0x80000000u},{0x7f800000u,0xff800000u,0x3f800000u},{0x7fc00123u,0xffc00101u,0x80000000u},{0x3f800000u,0x7f800000u,0xff800000u}};
          for(const auto &values:words) for(int variant=0;variant<(inplace?4:1);++variant) {
            const std::pair<ggml_tensor *,uint32_t> inputs[]={{a,values[0]},{ar,values[0]},{pw,values[1]},{pwr,values[1]},{ph,values[2]},{phr,values[2]}};
            for(auto input:inputs) {std::vector<uint32_t> data(ggml_nelements(input.first),input.second);ggml_backend_tensor_set(input.first,data.data(),0,data.size()*4);}
            result->op_params[0]=control->op_params[0]=int32_t(flags[variant]);
            if(!inplace)ggml_backend_tensor_memset(result,0xA5,0,ggml_nbytes(result));
            CHECK(ggml_backend_graph_compute(reference,expected)==GGML_STATUS_SUCCESS);
            CHECK(ggml_backend_dev_supports_op(dev,result) && ggml_backend_graph_compute(backend,graph)==GGML_STATUS_SUCCESS);
            CHECK(compare_result(result,control,std::vector<unsigned char>(ggml_nbytes(result),0xA5),3));++cases;
            if(values[0]==0x7fc00123u && variant==0) {
              uint32_t saved;ggml_backend_tensor_get(result,&saved,36,4);const uint32_t corrupt=saved^1u;ggml_backend_tensor_set(result,&corrupt,36,4);
              CHECK(!compare_result(result,control,std::vector<unsigned char>(ggml_nbytes(result),0xA5),3));
              ggml_backend_tensor_set(result,&saved,36,4);CHECK(compare_result(result,control,std::vector<unsigned char>(ggml_nbytes(result),0xA5),3));++tail_probes;
            }
          }
          for(int slot:{0,1,2})CHECK(refuses_change(backend,dev,graph,result,result->src[slot],nullptr));
          for(auto t:{result,a,pw,ph})CHECK(refuses_change(backend,dev,graph,result,t->type,GGML_TYPE_I32));
          CHECK(refuses_change(backend,dev,graph,result,result->ne[3],int64_t(1)));
          CHECK(refuses_change(backend,dev,graph,result,ph->ne[0],int64_t(2)));
          for(auto t:{a,pw,ph,result})CHECK(refuses_change(backend,dev,graph,result,t->nb[0],size_t(8)));
          CHECK(refuses_change(backend,dev,graph,result,result->data,static_cast<char *>(a->data)+4));
          if(inplace)CHECK(refuses_change(backend,dev,graph,result,result->data,ar->data));
          else CHECK(refuses_change(backend,dev,graph,result,result->op_params[0],int32_t(1)));
          CHECK(refuses_change(backend,dev,graph,result,a->ne[0],int64_t(-1)));
          for(auto t:{pw,result})CHECK(refuses_change(backend,dev,graph,result,t->nb[0],SIZE_MAX));
          auto saved_height=result->src[2];result->src[2]=pw;
          for(int axis:{0,1,3})CHECK(refuses_change(backend,dev,graph,result,pw->ne[axis],int64_t(2)));
          result->src[2]=saved_height;ggml_backend_buffer_free(buffer);ggml_free(ctx);
        }
        CHECK(refusal_probes==42 && tail_probes==2);
        for(int axis=0;axis<5;++axis) for(bool inplace:{false,true}) {
          ggml_init_params params={2*1024*1024,nullptr,true};auto ctx=ggml_init(params);CHECK(ctx);
          int64_t d[]={3,2,3,2,2};d[axis]=0;
          auto a=ggml_new_tensor_4d(ctx,GGML_TYPE_F32,d[0]*d[0],d[1]*d[2],d[3],d[4]);
          auto pw=ggml_new_tensor_4d(ctx,GGML_TYPE_F32,d[0],d[1],d[2],d[3]);auto ph=ggml_dup_tensor(ctx,pw);
          auto result=inplace?ggml_add_rel_pos_inplace(ctx,a,pw,ph):ggml_add_rel_pos(ctx,a,pw,ph);
          auto graph=ggml_new_graph(ctx);ggml_build_forward_expand(graph,result);
          CHECK(ggml_backend_dev_supports_op(dev,result));CHECK(ggml_backend_graph_compute(reference,graph)==GGML_STATUS_SUCCESS);
          CHECK(ggml_backend_graph_compute(backend,graph)==GGML_STATUS_SUCCESS);++cases;ggml_free(ctx);
        }
      }
      if (!strcmp(argv[3]+9,"ARANGE")) {
        CHECK(cases==10);
        ggml_init_params params={1024*1024,nullptr,true};auto ctx=ggml_init(params);CHECK(ctx);
        auto result=ggml_arange(ctx,0.0f,10.0f,1.0f);auto buffer=ggml_backend_alloc_ctx_tensors(ctx,backend);CHECK(buffer);
        ggml_backend_tensor_memset(result,0xA5,0,ggml_nbytes(result));auto graph=ggml_new_graph(ctx);ggml_build_forward_expand(graph,result);
        for(uint32_t bad:{0u,0x80000000u,0xbf800000u,0x7f800000u,0xff800000u,0x7fc00001u})
          CHECK(refuses_change(backend,dev,graph,result,result->op_params[2],int32_t(bad)));
        for(int slot:{0,1}) for(uint32_t bad:{0x7f800000u,0xff800000u,0x7fc00001u})
          CHECK(refuses_change(backend,dev,graph,result,result->op_params[slot],int32_t(bad)));
        CHECK(refuses_change(backend,dev,graph,result,result->op_params[2],int32_t(1),false));
        for(uint32_t start:{0x41200000u,0x41a00000u})
          CHECK(refuses_change(backend,dev,graph,result,result->op_params[0],int32_t(start),false));
        for(int64_t count:{9,11}) CHECK(refuses_change(backend,dev,graph,result,result->ne[0],count,false));
        CHECK(refuses_change(backend,dev,graph,result,result->nb[0],size_t(8)));
        CHECK(refuses_change(backend,dev,graph,result,result->type,GGML_TYPE_I32));CHECK(refusal_probes==19);
        result->ne[0]=0;result->op_params[2]=int32_t(0x7fc00001u);
        CHECK(ggml_backend_graph_compute(reference,graph)==GGML_STATUS_SUCCESS);
        CHECK(ggml_backend_graph_compute(backend,graph)==GGML_STATUS_SUCCESS);
        result->ne[0]=10;CHECK(compare_result(result,nullptr,std::vector<unsigned char>(40,0xA5),0));++cases;
        ggml_backend_buffer_free(buffer);ggml_free(ctx);
        for(int axis:{1,2}) {
          ctx=ggml_init(params);CHECK(ctx);result=ggml_arange(ctx,0.0f,10.0f,1.0f);
          result->ne[0]=2;result->ne[axis]=5;result->nb[1]=16;result->nb[2]=axis==2?20:80;result->nb[3]=100;
          buffer=ggml_backend_alloc_ctx_tensors(ctx,backend);CHECK(buffer);graph=ggml_new_graph(ctx);ggml_build_forward_expand(graph,result);
          std::vector<unsigned char> expected(ggml_nbytes(result));ggml_backend_tensor_memset(result,0xA5,0,expected.size());
          CHECK(ggml_backend_graph_compute(reference,graph)==GGML_STATUS_SUCCESS);ggml_backend_tensor_get(result,expected.data(),0,expected.size());
          ggml_backend_tensor_memset(result,0xA5,0,expected.size());CHECK(ggml_backend_graph_compute(backend,graph)==GGML_STATUS_SUCCESS);
          CHECK(compare_result(result,nullptr,expected,0));++cases;ggml_backend_buffer_free(buffer);ggml_free(ctx);
        }
      }
      if (!strcmp(argv[3]+9,"L2_NORM,L2_NORM_BATCH")) {
        CHECK(cases==61);
        for(int layout:{0,1}) {
          ggml_init_params params={2*1024*1024,nullptr,true};auto ctx=ggml_init(params);CHECK(ctx);
          auto left=ggml_new_tensor_4d(ctx,GGML_TYPE_F32,3,2,2,2);
          auto reference_left=ggml_new_tensor_4d(ctx,GGML_TYPE_F32,3,2,2,2);
          if(layout) for(auto input:{left,reference_left}) {input->nb[0]=8;input->nb[1]=32;input->nb[2]=80;input->nb[3]=192;}
          auto result=ggml_l2_norm(ctx,left,0.0f);auto control=ggml_l2_norm(ctx,left,0.0f);
          auto inplace=ggml_l2_norm_inplace(ctx,left,0.0f);auto reference_inplace=ggml_l2_norm_inplace(ctx,reference_left,0.0f);
          result->nb[0]=8;result->nb[1]=32;result->nb[2]=80;result->nb[3]=192;
          auto buffer=ggml_backend_alloc_ctx_tensors(ctx,backend);CHECK(buffer);
          auto graph=ggml_new_graph(ctx);ggml_build_forward_expand(graph,result);
          auto expected=ggml_new_graph(ctx);ggml_build_forward_expand(expected,control);
          auto inplace_graph=ggml_new_graph(ctx);ggml_build_forward_expand(inplace_graph,inplace);
          auto reference_graph=ggml_new_graph(ctx);ggml_build_forward_expand(reference_graph,reference_inplace);
          for(uint8_t fill:{0u,0x3fu,0x7fu,0xffu}) for(uint32_t epsilon:{0u,0x80000000u,0x358637bdu,0x7f800000u}) {
            for(auto node:{result,control,inplace,reference_inplace})node->op_params[0]=int32_t(epsilon);
            for(auto input:{left,reference_left})ggml_backend_tensor_memset(input,fill,0,ggml_nbytes(input));
            ggml_backend_tensor_memset(result,0xA5,0,ggml_nbytes(result));
            CHECK(ggml_backend_graph_compute(reference,expected)==GGML_STATUS_SUCCESS);
            CHECK(ggml_backend_dev_supports_op(dev,result) && ggml_backend_graph_compute(backend,graph)==GGML_STATUS_SUCCESS);
            CHECK(compare_result(result,control,std::vector<unsigned char>(ggml_nbytes(result),0xA5),1));++cases;
            CHECK(ggml_backend_graph_compute(reference,reference_graph)==GGML_STATUS_SUCCESS);
            CHECK(ggml_backend_dev_supports_op(dev,inplace) && ggml_backend_graph_compute(backend,inplace_graph)==GGML_STATUS_SUCCESS);
            CHECK(compare_result(inplace,reference_inplace,std::vector<unsigned char>(ggml_nbytes(inplace),fill),1));++cases;
          }
          for(uint32_t bad:{0xbf800000u,0xff800000u,0x7fc00001u,0x7f800001u})
            CHECK(refuses_change(backend,dev,graph,result,result->op_params[0],int32_t(bad)));
          CHECK(refuses_change(backend,dev,graph,result,result->type,GGML_TYPE_I32));
          CHECK(refuses_change(backend,dev,graph,result,left->type,GGML_TYPE_I32));
          CHECK(refuses_change(backend,dev,graph,result,result->src[0],nullptr));
          CHECK(refuses_change(backend,dev,graph,result,result->ne[0],int64_t(2)));
          CHECK(refuses_change(backend,dev,graph,result,result->data,static_cast<char *>(left->data)+4));
          CHECK(refuses_change(backend,dev,graph,result,left->ne[0],int64_t(-1)));
          CHECK(refuses_change(backend,dev,graph,result,result->nb[0],SIZE_MAX));
          CHECK(refuses_change(backend,dev,graph,result,left->nb[0],SIZE_MAX));
          CHECK(refuses_change(backend,dev,inplace_graph,inplace,inplace->data,static_cast<char *>(left->data)+4));
          CHECK(refuses_change(backend,dev,inplace_graph,inplace,inplace->nb[0],inplace->nb[0]+4));
          for(auto node:{result,inplace}) {
            std::vector<unsigned char> before(ggml_nbytes(node));ggml_backend_tensor_get(node,before.data(),0,before.size());
            node->ne[0]=0;node->op_params[0]=int32_t(0x7fc00001u);auto active=node==result?graph:inplace_graph;
            CHECK(ggml_backend_graph_compute(reference,active)==GGML_STATUS_SUCCESS);
            CHECK(ggml_backend_graph_compute(backend,active)==GGML_STATUS_SUCCESS);
            node->ne[0]=3;CHECK(compare_result(node,nullptr,before,0));++cases;
          }
          ggml_backend_buffer_free(buffer);ggml_free(ctx);
        }
        CHECK(refusal_probes==28);
      }
      printf("RESULT cases %zu\n",cases);
      ggml_backend_free(reference);ggml_backend_free(backend);return 0;
    }
#endif
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
    const bool bf16_gradient = !strcmp(argv[3], "row-gradient-bf16");
    if (bf16_gradient || !strcmp(argv[3], "row-gradient")) {
      const uint32_t edges[]={0,0x80000000,0x3f800000,0xbf800000,0x4b800000,0xcb800000,0x7f800000,0xff800000,0x7fc00001,0x7f800001,1,0x80000001,0x7f7fffff,0x00800000,0x3f000000,0xbf000000};
      size_t cases=0;
      for(auto type : {GGML_TYPE_F32,GGML_TYPE_F16,GGML_TYPE_BF16}) if ((type==GGML_TYPE_BF16)==bf16_gradient) for(int width : {0,1,3,17,65}) for(int rows : {0,1,3,9}) for(int output_rows : {0,1,4,7}) for(int layout=0;layout<3;layout++) for(int pattern=0;pattern<7;pattern++) {
        ggml_init_params params={8*1024*1024,nullptr,true};ggml_context *ctx=ggml_init(params);CHECK(ctx);
        ggml_tensor *input=ggml_new_tensor_2d(ctx,type,width,rows+(pattern==1?2:pattern==2?1:0)), *indices=ggml_new_tensor_1d(ctx,GGML_TYPE_I32,rows), *shape=ggml_new_tensor_2d(ctx,GGML_TYPE_F32,width,output_rows);
        const size_t bytes=ggml_type_size(type);
        input->nb[0]=bytes*(layout==2?2:1);input->nb[1]=input->nb[0]*width+bytes*layout;input->nb[2]=input->nb[3]=input->nb[1]*input->ne[1];
        indices->nb[0]=4*(layout+1);indices->nb[1]=indices->nb[2]=indices->nb[3]=indices->nb[0]*rows;
        ggml_tensor *result=ggml_get_rows_back(ctx,input,indices,shape);
        auto gradient_input=[&]() {
          auto source=input->nb[0]==bytes?input:canonical_copy(ctx,input);
          return bf16_gradient?ggml_cast(ctx,source,GGML_TYPE_F32):source;
        };
        auto oracle=ggml_get_rows_back(ctx,gradient_input(),indices->nb[0]==4?indices:canonical_copy(ctx,indices),shape);
        result->nb[0]=4*(layout+1);result->nb[1]=result->nb[0]*width+4*layout;result->nb[2]=result->nb[3]=result->nb[1]*output_rows;
        ggml_backend_buffer_t buffer=ggml_backend_alloc_ctx_tensors(ctx,backend);CHECK(buffer || (!ggml_nbytes(input)&&!ggml_nbytes(indices)&&!ggml_nbytes(result)));
        std::vector<unsigned char> data(ggml_nbytes(input),0xA5), index_data(ggml_nbytes(indices),0xA5), initial(ggml_nbytes(result),0xA5);
        for(int row=0;row<rows;row++) {
          int32_t index=output_rows?pattern==0?0:pattern==2?output_rows-1-row%output_rows:pattern==3?(row%2?0:output_rows-1):pattern==4?output_rows-1:row%output_rows:0;
          if(row==rows-1 && pattern>=5) index=pattern==5?-1:output_rows;
          memcpy(index_data.data()+row*indices->nb[0],&index,4);
          for(int column=0;column<width;column++) {
            const uint32_t bits=edges[(row*width+column+pattern*7)%16];float value;memcpy(&value,&bits,4);const uint16_t half=bf16_gradient?ggml_fp32_to_bf16(value).bits:ggml_fp32_to_fp16(value);
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
    const bool relative=!strcmp(argv[3], "relative-position"), rmsing=!strcmp(argv[3],"rms-norm"), norming=!strcmp(argv[3],"norm"), grouping=!strcmp(argv[3],"group-norm"), backward=!strcmp(argv[3],"rms-backward");
    const bool typing=!strcmp(argv[3],"typed-binary"), two_source=backward || typing;
    const bool normalizing=rmsing || norming || grouping || backward || typing;
    const bool physical_norm=norming || grouping || backward;
    const bool wordwise=transforming || unarying || scanning || reducing || selecting || normalizing;
    if (!strcmp(argv[3], "copy") || repeating || concatenating || spatial || folding || diagonal || triangular || filling || casting || windowing || wordwise || relative) {
      std::ofstream trace(std::string(argv[2])+"."+argv[3]+"-recipes",std::ios::binary);CHECK(trace);recipe_trace=&trace;
      recipe_word(1);recipe_word(GGML_MAX_SRC);recipe_word(GGML_MAX_OP_PARAMS/sizeof(int32_t));
      std::ofstream released;
      if(filling) {released.open(std::string(argv[2])+".fill-source-release",std::ios::binary);CHECK(released);recipe_trace=&released;recipe_word(1);recipe_trace=&trace;}
      static const int window_sizes[]={1,2,4};
      static const uint32_t fill_words[]={0,0x80000000,0x3f800000,0xbf800000,0x7f800000,0xff800000,
        0x7fc00001,0x7f800001,0xff800001,1,0x80000001,0x00800000,0x7f7fffff,0xff7fffff,
        0x33800000,0x33000000,0x33000001,0x32ffffff,0x3f801000,0x3f803000,0x477fe000,0x477ff000,0x477fefff,0x38800000};
      size_t cases=0, refused=0, source_releases=0; int kind;
      const bool copying=!strcmp(argv[3],"copy");
      const bool modeled=copying || repeating || concatenating || filling || casting || normalizing;
      const uint64_t fixture_base=copying?5:repeating?11:concatenating?17:filling?24:casting?31:(physical_norm || typing)?0:38;
      struct FixtureAnswer {uint64_t status,value;};
      static_assert(sizeof(FixtureAnswer)==16 && offsetof(FixtureAnswer,value)==8);
      using FixtureQuery=FixtureAnswer(*)(uint64_t,uint64_t,uint64_t,uint64_t);
      FixtureQuery query=nullptr;
      if(modeled || spatial) {CHECK(argc==5);auto model=dlopen(argv[4],RTLD_NOW|RTLD_LOCAL);CHECK(model);query=(FixtureQuery)dlsym(model,"fixture_query");CHECK(query);}
      auto fixture=[&](uint64_t q,uint64_t c,uint64_t source,uint64_t item) {auto answer=query(fixture_base+q,c,source,item);if(answer.status!=1)unexpected_abort("invalid Prism fixture response");return answer.value;};
      uint64_t config[5]={};
      std::vector<uint64_t> fixture_plan;
      std::vector<std::vector<unsigned char>> patterns;
      if(modeled) {for(uint64_t i=0;i<5;++i)config[i]=fixture(0,0,0,i);CHECK(query(10,0,0,0).status==0 && query(fixture_base+1,config[0],0,0).status==0);fixture_plan.resize(config[1]);CHECK(fixture_plan.size()==size_t(typing?13:backward?12:grouping?11:filling || casting || normalizing?10:9));patterns.resize(config[2]);}
      auto exercise=[&](int kind,int operation,int pattern,int layout,int units,int rows,int batches)->int {
        if(relative && ((kind!=GGML_TYPE_F16 && kind!=GGML_TYPE_BF16) || (pattern && units!=1))) return 0;
        if(((diagonal || triangular || wordwise) && !selecting && kind!=GGML_TYPE_F32 && kind!=GGML_TYPE_F16 && kind!=GGML_TYPE_BF16) || ((spatial || folding || windowing || scanning) && kind!=GGML_TYPE_F32)) return 0;
        if(reducing && operation && kind!=GGML_TYPE_F32) return 0;
        if(selecting && (kind!=(operation?GGML_TYPE_I32:GGML_TYPE_F32) || (!operation && pattern>=16))) return 0;
        if(((selecting && pattern>=14) || (reducing && pattern>=10) || (scanning && pattern>=8) || (transforming && pattern>=24) || (unarying && pattern)) && (units!=1 || rows!=1 || batches!=1)) return 0;
        ggml_init_params params={8*1024*1024,nullptr,true}; ggml_context *ctx=ggml_init(params); CHECK(ctx);
        const auto type=(ggml_type)kind; const int64_t block=ggml_blck_size(type); const size_t bytes=ggml_type_size(type);
        const ggml_type output_type=typing?(ggml_type)fixture_plan[12]:relative?GGML_TYPE_F16:selecting?(operation?GGML_TYPE_I64:GGML_TYPE_I32):casting?(ggml_type)fixture_plan[9]:(unarying && operation%3==2)?GGML_TYPE_F32:type;
        const size_t output_bytes=ggml_type_size(output_type);
        if(unarying && operation%3==2 && type==GGML_TYPE_F32) return 0;
        const int window_case=windowing?pattern%(operation?10:4):0;
        const int window_size=windowing?window_sizes[pattern/(operation?10:4)]:1;
        const int image_width=window_case==2?0:rows, image_height=window_case==3?0:batches;
        int64_t shape[4];
        if(modeled) for(int axis=0;axis<4;++axis)shape[axis]=fixture(2,cases,0,axis);
        else {shape[0]=block*units;shape[1]=rows;shape[2]=2;shape[3]=batches;}
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
        if(unarying && pattern) shape[pattern-1]=0;
        if(transforming && pattern>=24) shape[pattern-24]=0;
        if(scanning && pattern>=8) shape[pattern-8]=0;
        if(reducing && pattern>=10) shape[pattern-10]=0;
        if(selecting && pattern>=14) shape[pattern-14]=0;
        if(triangular) {shape[1]=units;shape[2]=rows;if(pattern==1)shape[0]=shape[1]=0;else if(pattern==2 || pattern==3)shape[pattern]=0;}
        if(diagonal) {shape[1]=1;shape[2]=rows;if(pattern)shape[pattern==1?0:pattern]=0;}
        if(folding) for(int axis=0;axis<4;axis++) shape[axis]=(pattern==16 && axis==0) || (pattern>=17 && axis==pattern-17)?0:shape[axis]*((pattern&(1<<axis))?2:1);
        ggml_tensor *input=ggml_new_tensor(ctx,type,4,shape);
        const uint32_t word=filling || normalizing?fixture_plan[9]:transforming?fill_words[std::min(pattern,23)]:0; float constant; memcpy(&constant,&word,4);
        const uint32_t bias_word=fill_words[((transforming?std::min(pattern,23):pattern/5)*7+3)%24];float bias;memcpy(&bias,&bias_word,4);
        const bool inplace=((filling || normalizing) && operation) || (transforming && operation%2) || (unarying && operation%3==1);
        ggml_tensor *backward_input=nullptr;
        if(two_source) {int64_t d[4];for(int axis=0;axis<4;++axis)d[axis]=fixture(2,cases,1,axis);backward_input=operation==3?ggml_view_tensor(ctx,input):ggml_new_tensor(ctx,(ggml_type)fixture_plan[11],4,d);}
        auto output_storage = [&](ggml_tensor *node) {if(node->type!=output_type) {node->type=output_type;node->nb[0]=output_bytes;for(int axis=1;axis<4;axis++)node->nb[axis]=node->nb[axis-1]*node->ne[axis-1];}return node;};
        auto word_operation = [&](ggml_tensor *a, bool inplace) {
          if(typing) {
            using Binary=ggml_tensor *(*)(ggml_context *,ggml_tensor *,ggml_tensor *);
            static const Binary ops[2][4]={{ggml_add,ggml_sub,ggml_mul,ggml_div},{ggml_add_inplace,ggml_sub_inplace,ggml_mul_inplace,ggml_div_inplace}};
            const auto code=fixture_plan[9];if(code!=2 && code!=5 && code!=6 && code!=7)unexpected_abort("invalid typed binary constructor");
            auto node=ops[inplace][code==2?0:code==5?1:code==6?2:3](ctx,a,backward_input);
            if(inplace && operation==2)node->view_src=backward_input;
            return output_storage(node);
          }
          if(norming) return inplace?ggml_norm_inplace(ctx,a,constant):ggml_norm(ctx,a,constant);
          if(grouping) return inplace?ggml_group_norm_inplace(ctx,a,fixture_plan[10],constant):ggml_group_norm(ctx,a,fixture_plan[10],constant);
          if(rmsing) return inplace?ggml_rms_norm_inplace(ctx,a,constant):ggml_rms_norm(ctx,a,constant);
          if(backward) {auto out=ggml_rms_norm_back(ctx,a,backward_input,constant);if(inplace)out->view_src=operation==2?backward_input:a;return out;}
          if(scanning) return ggml_cumsum(ctx,a);
          if(reducing) return operation==0?ggml_sum(ctx,a):operation==1?ggml_sum_rows(ctx,a):ggml_mean(ctx,a);
          if(unarying) {
            static const ggml_unary_op variants[]={GGML_UNARY_OP_ABS,GGML_UNARY_OP_SGN,GGML_UNARY_OP_NEG,GGML_UNARY_OP_STEP,GGML_UNARY_OP_RELU,GGML_UNARY_OP_COUNT,GGML_UNARY_OP_COUNT,GGML_UNARY_OP_HARDSIGMOID,GGML_UNARY_OP_HARDSWISH,GGML_UNARY_OP_FLOOR,GGML_UNARY_OP_CEIL,GGML_UNARY_OP_ROUND,GGML_UNARY_OP_TRUNC};
            const int which=operation/3;ggml_tensor *node=which==5?(inplace?ggml_sqr_inplace(ctx,a):ggml_sqr(ctx,a)):which==6?(inplace?ggml_sqrt_inplace(ctx,a):ggml_sqrt(ctx,a)):inplace?ggml_unary_inplace(ctx,a,variants[which]):ggml_unary(ctx,a,variants[which]);
            return output_storage(node);
          }
          return operation<2?(inplace?ggml_scale_inplace(ctx,a,constant):ggml_scale(ctx,a,constant)):operation<4?(inplace?ggml_scale_bias_inplace(ctx,a,constant,bias):ggml_scale_bias(ctx,a,constant,bias)):operation<6?(inplace?ggml_clamp_inplace(ctx,a,constant,bias):ggml_clamp(ctx,a,constant,bias)):ggml_leaky_relu(ctx,a,constant,inplace);};
        ggml_tensor *filled=filling?(operation?ggml_fill_inplace(ctx,input,constant):ggml_fill(ctx,input,constant)):wordwise && !selecting?word_operation(input,inplace):nullptr;
        int64_t right_ne[]={input->ne[0],input->ne[1],input->ne[2],input->ne[3]}; if(concatenating) for(int axis=0;axis<4;++axis)right_ne[axis]=fixture(2,cases,2,axis);
        ggml_tensor *right=(concatenating || (selecting && operation))?ggml_new_tensor(ctx,type,4,right_ne):backward_input;
        for(ggml_tensor *operand : {input,right}) if(operand) {
          if(modeled) {for(int axis=0;axis<4;++axis)operand->nb[axis]=fixture(2,cases,operand==right?(two_source?1:2):0,axis+4);}
          else if(layout==1 || layout==2) {
            operand->nb[0]=bytes*(layout==2?2:1)*(selecting && operand==right?2:1); operand->nb[1]=operand->nb[0]*(operand->ne[0]/block)+bytes;
            operand->nb[2]=operand->nb[1]*operand->ne[1]+bytes; operand->nb[3]=operand->nb[2]*operand->ne[2]+bytes;
          }
        }
        ggml_tensor *target;
        if(two_source) target=nullptr;
        else if(modeled) {int64_t d[4];for(int axis=0;axis<4;++axis)d[axis]=fixture(2,cases,1,axis);target=ggml_new_tensor(ctx,output_type,4,d);for(int axis=0;axis<4;++axis)target->nb[axis]=fixture(2,cases,1,axis+4);}
        else {const int64_t blocks=ggml_nelements(input)/block;
          target=layout==3?ggml_new_tensor_4d(ctx,output_type,block,blocks,1,1):ggml_new_tensor(ctx,output_type,4,input->ne);
          target->nb[1]+=output_bytes; target->nb[2]=target->nb[1]*target->ne[1]+output_bytes; target->nb[3]=target->nb[2]*target->ne[2]+output_bytes;
        }
        if(folding) {int64_t output_shape[]={units,rows,2,batches};if(pattern>=17) output_shape[pattern-17]=0;target=ggml_new_tensor(ctx,type,4,output_shape);}
        if(repeating) {int64_t d[4];for(int axis=0;axis<4;++axis)d[axis]=fixture(2,cases,2,axis);target=ggml_new_tensor(ctx,type,4,d);for(int axis=0;axis<4;++axis)target->nb[axis]=fixture(2,cases,2,axis+4);}
        auto spatial_op = [&](ggml_tensor *a, bool reference) -> ggml_tensor * {
          int args[8]={};
          for(int axis=0;axis<(operation==1?1:4);++axis) for(int side=0;side<(operation==2?1:2);++side) {
            auto answer=query(operation==0 && reference?3:operation,pattern,a->ne[axis],operation==2?axis:side);
            if(answer.status!=1 || answer.value>UINT32_MAX)unexpected_abort("invalid Prism spatial fixture response");uint32_t word=answer.value;memcpy(&args[operation==0?axis*2+side:operation==1?side:axis],&word,4);
          }
          if(operation==0) return pattern%2?ggml_pad_ext_circular(ctx,a,args[0],args[1],args[2],args[3],args[4],args[5],args[6],args[7]):ggml_pad_ext(ctx,a,args[0],args[1],args[2],args[3],args[4],args[5],args[6],args[7]);
          if(operation==1) return ggml_pad_reflect_1d(ctx,a,args[0],args[1]);
          return ggml_roll(ctx,a,args[0],args[1],args[2],args[3]);
        };
        ggml_tensor *result=relative?ggml_get_rel_pos(ctx,input,rows,rows):selecting?(operation?ggml_count_equal(ctx,input,right):ggml_argmax(ctx,input)):wordwise?filled:windowing?(operation?ggml_win_unpart(ctx,input,image_width,image_height,window_size):ggml_win_part(ctx,input,window_size)):casting?(operation<3?ggml_cast(ctx,input,output_type):ggml_cpy(ctx,input,target)):filling?filled:triangular?ggml_tri(ctx,ggml_cont(ctx,input),(ggml_tri_type)operation):diagonal?ggml_diag(ctx,input):folding?ggml_repeat_back(ctx,input,target):spatial?spatial_op(operation?ggml_cont(ctx,input):input,false):concatenating?ggml_concat(ctx,input,right,operation):repeating?ggml_repeat(ctx,input,target):operation==0?ggml_dup(ctx,input):(operation==1?(fixture_plan[8]?ggml_cont_1d(ctx,input,fixture(2,cases,0,9)):ggml_cont(ctx,input)):ggml_cpy(ctx,input,target));
        if(physical_norm) {uint32_t parameter;memcpy(&parameter,result->op_params+(grouping?1:0),4);CHECK(parameter==fixture_plan[9] && result->op==(grouping?GGML_OP_GROUP_NORM:backward?GGML_OP_RMS_NORM_BACK:GGML_OP_NORM) && result->src[0]==input && (result->view_src!=nullptr)==inplace);if(grouping)CHECK(uint32_t(result->op_params[0])==fixture_plan[10]);if(backward)CHECK(result->src[1]==right && result->view_src==(operation?(operation==2?right:input):nullptr));}
        if(typing)CHECK(uint64_t(result->op)==fixture_plan[9] && result->type==output_type && result->src[0]==input && result->src[1]==right && result->view_src==(operation?(operation==2?right:input):nullptr));
        if(triangular) result->src[0]=input;
        if(inplace) memcpy(result->nb,input->nb,sizeof input->nb);
        if((scanning || reducing || selecting) && layout==3) for(int axis=0;axis<4;axis++) result->nb[axis]*=2;
        ggml_tensor *expected_tensor=relative?ggml_get_rel_pos(ctx,canonical_copy(ctx,input),rows,rows):selecting?(operation?ggml_count_equal(ctx,(ggml_is_matrix(input) || pattern==9)?input:ggml_cont_2d(ctx,input,input->ne[0],ggml_nrows(input)),(ggml_is_matrix(input) || pattern==9)?right:ggml_cont_2d(ctx,right,right->ne[0],ggml_nrows(right))):ggml_argmax(ctx,input->nb[0]==sizeof(float)?input:canonical_copy(ctx,input))):(physical_norm || typing)?word_operation(input,false):wordwise?word_operation(reducing && !operation && type==GGML_TYPE_F32?input:canonical_copy(ctx,input),false):windowing?(operation?ggml_win_unpart(ctx,canonical_copy(ctx,input),image_width,image_height,window_size):ggml_win_part(ctx,canonical_copy(ctx,input),window_size)):casting?ggml_cast(ctx,canonical_copy(ctx,input),output_type):filling?ggml_fill(ctx,canonical_copy(ctx,input),constant):triangular?ggml_tri(ctx,canonical_copy(ctx,input),(ggml_tri_type)operation):diagonal?ggml_diag(ctx,canonical_copy(ctx,input)):folding?ggml_repeat_back(ctx,canonical_copy(ctx,input),target):spatial?spatial_op(canonical_copy(ctx,input),true):concatenating?ggml_concat(ctx,canonical_copy(ctx,input),canonical_copy(ctx,right),operation):repeating?ggml_repeat(ctx,canonical_copy(ctx,input),target):canonical_copy(ctx,input);
        if(modeled && !copying) {for(int axis=0;axis<4;++axis)result->nb[axis]=fixture(2,cases,3,axis+4);}
        else if(relative || spatial || folding || diagonal || triangular || (filling && !operation) || (casting && operation<3) || windowing || (wordwise && !inplace)) {result->nb[1]+=output_bytes;result->nb[2]=result->nb[1]*result->ne[1]+output_bytes;result->nb[3]=result->nb[2]*result->ne[2]+output_bytes;}
        if(physical_norm || typing)memcpy(expected_tensor->nb,result->nb,sizeof result->nb);
        ggml_backend_buffer_t buffer=ggml_backend_alloc_ctx_tensors(ctx,backend); CHECK(buffer || (!ggml_nbytes(input)&&(!right||!ggml_nbytes(right))&&!ggml_nbytes(result)));
        std::vector<unsigned char> output(ggml_nbytes(result),modeled?config[4]:0xA5);
        for(ggml_tensor *operand : {input,right}) if(operand) {
          std::vector<unsigned char> initial(ggml_nbytes(operand));
          if(modeled) {
            auto &pattern=patterns.at(fixture_plan[operand==right?(two_source?10:8):7]);
            if(pattern.empty())for(uint64_t i=0;i<config[3];++i) {auto value=fixture(3,cases,operand==right?1:0,i);CHECK(value<=255);pattern.push_back(value);}
            const uint64_t chunk_query=operand==right?5:4;
            const auto chunks=fixture(chunk_query,cases,0,0);
            for(uint64_t i=1;i<=chunks;++i) {auto offset=fixture(chunk_query,cases,i,0),length=fixture(chunk_query,cases,i,1);CHECK(length<=pattern.size() && offset<=initial.size() && length<=initial.size()-offset);std::copy_n(pattern.begin(),length,initial.begin()+offset);}
          }
          else if(wordwise || relative) for(size_t i=0;i<((reducing || selecting)?size_t(ggml_nelements(operand)):initial.size()/bytes);i++) {
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
        recipe_context=ctx;recipe_base=uintptr_t(buffer?ggml_backend_buffer_get_base(buffer):nullptr);
        recipe_word(3);
        for(uint64_t value:{uint64_t(cases),uint64_t(kind),uint64_t(operation),uint64_t(pattern),uint64_t(layout),uint64_t(units),uint64_t(rows),uint64_t(batches)})recipe_word(value);
        recipe_state(0);
        for(auto operand:{input,right,result}) {const auto size=operand?ggml_nbytes(operand):0;recipe_word(recipe_id(operand));recipe_word(size);if(size)trace.write((const char *)operand->data,size);}
        ggml_cgraph *graph=ggml_new_graph(ctx); ggml_build_forward_expand(graph,result);
        if(triangular && pattern==4) {input->ne[1]++;result->ne[1]++;recipe_state(2);}
        if(((normalizing) && fixture_plan[8]) || (spatial && operation==0 && pattern%2 && ggml_is_empty(input) && !ggml_is_empty(result)) || (triangular && pattern==4) || (windowing && operation==1 && window_case==9)) {
          CHECK(ggml_backend_dev_supports_op(dev,result)==((normalizing) && fixture_plan[8]==1)); CHECK(ggml_backend_graph_compute(backend,graph)==GGML_STATUS_FAILED);
          if(triangular) {input->ne[1]--;result->ne[1]--;}
          std::vector<unsigned char> unchanged(output.size()); ggml_backend_tensor_get(result,unchanged.data(),0,unchanged.size()); CHECK(unchanged==output);
          recipe_word(5);recipe_word(0);recipe_context=nullptr;ggml_backend_buffer_free(buffer); ggml_free(ctx); cases++;refused++;return 0;
        }
        if((grouping || backward || typing) && !output.empty())ggml_backend_tensor_set(expected_tensor,output.data(),0,output.size());
        ggml_cgraph *oracle_graph=ggml_new_graph(ctx); ggml_build_forward_expand(oracle_graph,expected_tensor);
        CHECK(ggml_backend_graph_compute(reference,oracle_graph)==GGML_STATUS_SUCCESS);
        if(typing && cases==0) {
          CHECK(refuses_change(backend,dev,graph,result,result->src[0],nullptr));CHECK(refuses_change(backend,dev,graph,result,result->src[1],nullptr));
          CHECK(refuses_change(backend,dev,graph,result,input->type,GGML_TYPE_I32));CHECK(refuses_change(backend,dev,graph,result,result->type,GGML_TYPE_I32));CHECK(refuses_change(backend,dev,graph,result,right->type,GGML_TYPE_F16));
          CHECK(refuses_change(backend,dev,graph,result,input->nb[0],2*bytes));CHECK(refuses_change(backend,dev,graph,result,result->nb[0],2*output_bytes));CHECK(refuses_change(backend,dev,graph,result,right->nb[0],size_t(1)));
          CHECK(refuses_change(backend,dev,graph,result,result->ne[1],result->ne[1]+1));CHECK(refuses_change(backend,dev,graph,result,right->ne[0],right->ne[0]+1));
          CHECK(refuses_change(backend,dev,graph,result,right->data,nullptr,false));CHECK(refuses_change(backend,dev,graph,result,result->data,(char *)input->data+bytes));CHECK(refusal_probes==12);
        }
        if(backward && cases==0) {
          CHECK(refuses_change(backend,dev,graph,result,result->src[0],nullptr));CHECK(refuses_change(backend,dev,graph,result,result->src[1],nullptr));
          CHECK(refuses_change(backend,dev,graph,result,input->type,GGML_TYPE_I32));CHECK(refuses_change(backend,dev,graph,result,result->type,GGML_TYPE_I32));
          CHECK(refuses_change(backend,dev,graph,result,input->nb[0],size_t(8)));CHECK(refuses_change(backend,dev,graph,result,right->nb[0],size_t(8)));
          for(int axis=0;axis<4;++axis)CHECK(refuses_change(backend,dev,graph,result,right->ne[axis],right->ne[axis]+1));
          CHECK(refuses_change(backend,dev,graph,result,right->data,nullptr,false));CHECK(refuses_change(backend,dev,graph,result,result->data,(char *)input->data+4));
          CHECK(refusal_probes==12);
        }
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
        if(filling && !operation && layout==3) {input->data=nullptr;input->buffer=nullptr;recipe_trace=&released;recipe_word(cases);recipe_state(6);recipe_trace=&trace;++source_releases;}
        CHECK(ggml_backend_dev_supports_op(dev,result)); CHECK(ggml_backend_graph_compute(backend,graph)==GGML_STATUS_SUCCESS);
        input->data=saved_data; input->buffer=saved_buffer;
        CHECK(compare_result(result,expected_tensor,output,(grouping || backward || typing)?4:wordwise && !selecting?(scanning?2:1):0));
        recipe_word(5);recipe_word(1);recipe_context=nullptr;ggml_backend_buffer_free(buffer); ggml_free(ctx); cases++;
        return 0;
      };
      if(modeled) {
        const auto count=config[0];
        for(uint64_t ordinal=0;ordinal<count;++ordinal) {CHECK(cases==ordinal);for(size_t field=0;field<fixture_plan.size();++field) {fixture_plan[field]=fixture(1,ordinal,0,field);CHECK(fixture_plan[field]<=uint64_t(field<7?std::numeric_limits<int>::max():UINT32_MAX));}CHECK(exercise(fixture_plan[0],fixture_plan[1],fixture_plan[2],fixture_plan[3],fixture_plan[4],fixture_plan[5],fixture_plan[6])==0);}
      } else {
      while (scanf("%d",&kind)==1) for(int operation=0;operation<(relative?1:selecting?2:reducing?3:scanning?1:unarying?39:transforming?8:windowing?2:triangular?4:(folding || diagonal || triangular || windowing)?1:3);operation++) {
      const std::vector<int> extents=relative?std::vector<int>{1,3,17}:(spatial && operation==0)?std::vector<int>{0,1,3}:windowing?std::vector<int>{1,3,5}:spatial?std::vector<int>{2,3}:(reducing || selecting)?std::vector<int>{1,2,3,17,65}:scanning?std::vector<int>{1,3,17,65}:std::vector<int>{1,3};
      const auto row_extents=relative?std::vector<int>{1,3,7}:(scanning || reducing || selecting)?std::vector<int>{1,3}:extents;
      const std::vector<int> batches_list=(spatial && operation==2)?std::vector<int>{2,3}:std::vector<int>{1,2};
      for(int pattern=0;pattern<(relative?2:selecting?18:reducing?14:scanning?12:unarying?5:transforming?28:windowing?(operation?30:12):triangular?5:diagonal?4:folding?21:spatial?(operation==0?22:operation==1?4:17):1);pattern++) for(int layout=0;layout<4;layout++)
      for(int units : extents) for(int rows : row_extents) for(int batches : batches_list) {
        CHECK(exercise(kind,operation,pattern,layout,units,rows,batches)==0);
      }
      }
      }
      CHECK(refused==((normalizing)?fixture(0,0,0,5):spatial?200:triangular?384:windowing?216:0));
      recipe_word(4);recipe_word(cases);recipe_word(refusal_probes);recipe_word(refused);
      if(filling) {recipe_trace=&released;recipe_word(UINT64_MAX);recipe_word(source_releases);recipe_trace=&trace;}
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
    const bool matrix = !strcmp(argv[3], "matrix");
    if (matrix || !strcmp(argv[3], "elementwise")) {
      bool matrix_refusals_checked = false;
      std::ofstream trace(std::string(argv[2])+"."+argv[3]+"-fixtures",std::ios::binary); CHECK(trace.good());
      uint64_t header[4]; for(auto &value:header) CHECK(read_fixture_word(value));
      CHECK(header[0]==(matrix?1280:120) && header[1]==24 && header[2]==2 && header[3]==165);
      for(size_t ordinal=0;ordinal<header[0];ordinal++) {
        uint64_t fixture[24]; for(auto &value:fixture) CHECK(read_fixture_word(value));
        CHECK(fixture[23]==ordinal);
        ggml_init_params params = {8*1024*1024, nullptr, true};
        ggml_context *ctx = ggml_init(params); CHECK(ctx);
        ggml_tensor *left, *right, *result, *oracle;
        const auto kind=fixture[0];
        if (matrix) {
          left = ggml_new_tensor_4d(ctx, GGML_TYPE_F32, fixture[2], fixture[0], fixture[4], fixture[5]);
          right = ggml_new_tensor_4d(ctx, GGML_TYPE_F32, fixture[2], fixture[1], fixture[6], fixture[7]);
          for(int source=0;source<2;source++) for(int axis=0;axis<4;axis++) (source?right:left)->nb[axis]=fixture[8+source*4+axis];
          result = ggml_mul_mat(ctx, left, right); CHECK(result);
          oracle = ggml_mul_mat(ctx, left, right); CHECK(oracle);
          for(int axis=0;axis<4;axis++)result->nb[axis]=fixture[16+axis];
        } else {
          CHECK(kind<8);
          left=ggml_new_tensor_4d(ctx,GGML_TYPE_F32,fixture[4],fixture[5],fixture[6],fixture[7]);
          right=kind<4?ggml_new_tensor_4d(ctx,GGML_TYPE_F32,fixture[8],fixture[9],fixture[10],fixture[11]):nullptr;
          for(int source=0;source<2;source++) {auto operand=source?right:left;if(operand)for(int axis=0;axis<4;axis++)operand->nb[axis]=fixture[12+source*4+axis];}
          ggml_tensor *(*binary[])(ggml_context *,ggml_tensor *,ggml_tensor *)={ggml_add,ggml_sub,ggml_mul,ggml_div};
          ggml_tensor *(*unary[])(ggml_context *,ggml_tensor *)={ggml_sqrt,ggml_sqr,ggml_neg,ggml_abs};
          auto operation = [&](ggml_tensor *a, ggml_tensor *b) { return kind<4?binary[kind](ctx,a,b):unary[kind-4](ctx,a); };
          result=operation(left,right);
          oracle=operation(ggml_cont(ctx,left),right?ggml_cont(ctx,right):nullptr);
          CHECK(result && uint64_t(result->op)==fixture[1]);
          if (kind>=6) CHECK(uint64_t(ggml_get_unary_op(result))==fixture[2]);
        }
        ggml_backend_buffer_t buffer=ggml_backend_alloc_ctx_tensors(ctx,backend);
        CHECK(buffer || (matrix && !ggml_nbytes(left) && !ggml_nbytes(right) && !ggml_nbytes(result)));
        CHECK(bind_fixture_words(left,right,header[3])==0);
        if (matrix) {
          const uint64_t fields[]={uint64_t(left->ne[1]),uint64_t(right->ne[1]),uint64_t(left->ne[0]),fixture[3],uint64_t(left->ne[2]),uint64_t(left->ne[3]),uint64_t(right->ne[2]),uint64_t(right->ne[3]),
              left->nb[0],left->nb[1],left->nb[2],left->nb[3],right->nb[0],right->nb[1],right->nb[2],right->nb[3],result->nb[0],result->nb[1],result->nb[2],result->nb[3],
              uint64_t(ggml_nelements(left)),uint64_t(ggml_nelements(right)),uint64_t(fixture[22]!=0),ordinal};
          observe_fixture(trace,fields,left,right);
        } else {
          const uint64_t fields[]={uint64_t(kind),uint64_t(result->op),uint64_t(kind>=6?ggml_get_unary_op(result):0),fixture[3],
            uint64_t(left->ne[0]),uint64_t(left->ne[1]),uint64_t(left->ne[2]),uint64_t(left->ne[3]),
            uint64_t(right?right->ne[0]:0),uint64_t(right?right->ne[1]:0),uint64_t(right?right->ne[2]:0),uint64_t(right?right->ne[3]:0),
            left->nb[0],left->nb[1],left->nb[2],left->nb[3],right?right->nb[0]:0,right?right->nb[1]:0,right?right->nb[2]:0,right?right->nb[3]:0,
            uint64_t(ggml_nelements(left)),uint64_t(right?ggml_nelements(right):0),fixture[22],ordinal};
          observe_fixture(trace,fields,left,right);
        }
        auto graph=ggml_new_graph(ctx); ggml_build_forward_expand(graph,result);
        auto reference_graph=ggml_new_graph(ctx); ggml_build_forward_expand(reference_graph,oracle);
        CHECK(ggml_backend_dev_supports_op(dev,result));
        CHECK(ggml_backend_dev_supports_op(ggml_backend_get_device(reference),oracle));
        CHECK(ggml_backend_graph_compute(reference,reference_graph)==GGML_STATUS_SUCCESS);
        const std::vector<unsigned char> expected(ggml_nbytes(result),0xA5);
        if (!expected.empty()) ggml_backend_tensor_memset(result,0xA5,0,expected.size());
        if (matrix && fixture[22]) {
          CHECK(!matrix_refusals_checked && ggml_nbytes(result) && ggml_nbytes(left));
          matrix_refusals_checked = true;
          CHECK(refuses_change(backend,dev,graph,result,result->ne[0],result->ne[0]+1));
          CHECK(refuses_change(backend,dev,graph,result,result->src[0],nullptr));
          CHECK(refuses_change(backend,dev,graph,result,result->buffer,nullptr,false));
          ggml_backend_buffer_t foreign=ggml_backend_cpu_buffer_from_ptr(left->data,ggml_nbytes(left)); CHECK(foreign);
          CHECK(refuses_change(backend,dev,graph,result,left->buffer,foreign,false)); ggml_backend_buffer_free(foreign);
          ggml_tensor *unsupported=ggml_dup(ctx,result);
          ggml_cgraph *bad_graph=ggml_new_graph(ctx); ggml_build_forward_expand(bad_graph,unsupported);
          CHECK(refuses_change(backend,dev,bad_graph,result,unsupported->op,GGML_OP_COUNT,false));
        }
        CHECK(ggml_backend_graph_compute(backend,graph)==GGML_STATUS_SUCCESS);
        CHECK(compare_result(result,oracle,expected,1));
        if (!matrix && fixture[22]) {
          CHECK((fixture[22]==1 && kind==0) || (fixture[22]==2 && kind==6));
          if (kind==0) {
            CHECK(refuses_change(backend,dev,graph,result,result->src[1],nullptr));
            CHECK(refuses_change(backend,dev,graph,result,right->ne[1],int64_t(2)));
            CHECK(refuses_change(backend,dev,graph,result,result->ne[1],int64_t(2)));
            CHECK(refuses_change(backend,dev,graph,result,left->type,GGML_TYPE_COUNT));
          } else CHECK(refuses_change(backend,dev,graph,result,result->op_params[0],int32_t(GGML_UNARY_OP_COUNT)));
        }
        ggml_backend_buffer_free(buffer); ggml_free(ctx);
      }
      CHECK((!matrix || matrix_refusals_checked) && refusal_probes==5 && fgetc(stdin)==EOF);
      CHECK(ggml_backend_reg_count()==1 && ggml_backend_dev_count()==1);
      ggml_backend_free(reference); ggml_backend_free(backend);
      printf("RESULT cases %zu\n",size_t(header[0])); return 0;
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
