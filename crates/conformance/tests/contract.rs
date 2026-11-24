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

// Keep each imported batch individually selectable with Cargo. The same entries
// bind the frozen input counts to the test that calls the native evaluator.
macro_rules! ggml_batches {
    ($( $(#[$doc:meta])* fn $name:ident() { $mode:literal, $cases:literal, [$($model:literal),*] $(=> $probe:expr)? } )+) => {
        const GGML_TEST_NAMES: &[&str] = &[$(stringify!($name)),+];
        const GGML_BATCHES: &[(&str, usize)] = &[$(($mode, $cases)),+];
        $(
            $(#[$doc])*
            #[test]
            fn $name() {
                let root = root();
                $(tensor_layout::certified_current_model($model);)*
                $($probe;)?
                native_backend_oracle::verify(&root, $mode, $cases);
            }
        )+
    };
}

ggml_batches! {
    /// Verify GGML device discovery, transfers and typed buffer refusals.
    fn native_backend_vt_11() { "buffers", 108, ["layout", "native"] }
    fn custom_callbacks_vt_136() { "custom-callbacks", 624, ["native"] }
    /// Exercise the CPU threadpool registry functions consumed by the upstream llama host.
    fn cpu_threadpool_vt_59() { "cpu-threadpool", 4, ["native"] }
    /// `VT-12`: GGML F32 matrix-product graphs execute through Verdant across rectangular shapes, padded storage and batch broadcasting and match the imported GGML CPU reference
    fn matrix_graphs_vt_12() { "matrix", 1280, ["matrix", "index"] => assert!(vtensor_index::run(u64::MAX, 0, 0, 0, 2, 0, 0, 0).is_err()) }
    /// `VT-13`: F32 arithmetic and unary graphs execute through Verdant with Prism-defined scalar semantics and broadcasting and match the imported GGML CPU reference
    fn elementwise_graphs_vt_13() { "elementwise", 120, ["typed", "pointwise"] }
    /// `VT-14`: Prism empty-node admission matches the imported GGML CPU no-op behavior for empty copy graphs across every live storage type without accessing operand buffers
    fn empty_graphs_vt_14() { "empty", 864, [] }
    /// `VT-15`: Prism plans same-type DUP, CONT and CPY across every live GGML storage type and Verdant moves the admitted blocks with upstream-equivalent logical order and preserved padding
    fn copy_graphs_vt_15() { "copy", 3456, [] }
    /// Compare the IQ2_XXS identity copy with the actual GGML CPU copy kernel.
    fn iq2_identity_copy_vt_15() { "iq2-identity-copy", 1, [] }
    /// `VT-16`: Prism models four-axis REPEAT addressing and Verdant preserves encoded values for every type admitted by the imported GGML CPU repeat oracle
    fn repeat_graphs_vt_16() { "repeat", 160, [] }
    fn storage_repeats_vt_155() { "storage-repeat", 180, ["pointwise"] }
    fn shared_host_storage_vt_156() { "shared-host-storage", 1, ["native"] }
    /// `VT-17`: Prism selects and addresses CONCAT source blocks on every axis and Verdant execution matches the imported GGML CPU oracle across all live storage types
    fn concat_graphs_vt_17() { "concat", 10368, [] }
    /// `VT-18`: Prism defines F32 zero and circular padding, reflected padding and signed four-axis roll addressing and Verdant execution matches the imported GGML CPU oracle
    fn spatial_graphs_vt_18() { "spatial", 2256, [] }
    /// `VT-19`: Prism plans SET and ACC initialization, bounded update views and aliases and Verdant execution matches the imported GGML CPU oracle for supported float and integer storage
    fn update_graphs_vt_19() { "update", 4608, [] }
    /// `VT-20`: Prism models four-axis REPEAT_BACK initialization and accumulation order and Verdant execution matches the imported GGML F32 CPU oracle across strided and empty tensors
    fn fold_graphs_vt_20() { "fold", 672, [] }
    /// `VT-21`: Prism defines batched DIAG placement and Verdant preserves diagonal encodings and positive-zero off-diagonal values for every type supported by the imported GGML CPU oracle
    fn diagonal_graphs_vt_21() { "diagonal", 384, [] }
    fn bfloat_products_vt_172() { "bfloat-products", 8, ["matrix"] }
    fn prepacked_precision_vt_171() { "prepacked-precision", 1150, ["matrix"] }
    fn prepacked_products_vt_170() { "prepacked-products", 50, ["matrix"] }
    fn flash_accumulators_vt_178() { "flash-precision", 4205, ["attention"] }
    fn flash_storage_pairs_vt_177() { "flash-storage", 697, ["attention"] }
    fn numeric_row_results_vt_184() { "numeric-results", 3906, ["typed", "pointwise"] }
    fn packed_binary_views_vt_183() { "row-repeat", 232, ["typed"] }
    fn scalar_binary_rows_vt_182() { "scalar-binary", 778, ["typed"] }
    fn mixed_row_casts_vt_181() { "mixed-cast", 22846, ["typed"] }
    /// `VT-180`: packed copied and in-place binary graphs match the original CPU decode, arithmetic and encode.
    fn packed_binary_rows_vt_180() { "packed-binary", 6450, ["typed"] }
    fn half_add_casts_vt_179() { "half-add", 54, ["typed"] }
    fn codebook_addition_vt_176() { "codebook-add", 189, ["typed"] }
    fn e4m3_add_repeats_vt_175() { "e4m3-repeat", 12, ["typed"] }
    fn packed_lane_repeats_vt_174() { "packed-lanes", 72, ["typed"] }
    fn broadcast_packed_addition_vt_173() { "packed-broadcast", 661, ["typed"] }
    fn packed_addition_vt_169() { "packed-add", 661, ["typed"] }
    fn signed_indexed_rows_vt_168() { "integer-rows", 8, ["typed"] }
    fn activation_q8_stores_vt_167() { "activation-q8", 20, ["typed"] }
    fn narrow_wide_casts_vt_166() { "wide-narrow", 944, ["pointwise"] }
    fn staged_wide_casts_vt_165() { "wide-staged", 2740, ["pointwise"] }
    fn signed64_floats_vt_164() { "signed64-floats", 396, ["pointwise"] }
    fn signed64_widths_vt_163() { "signed64-widths", 768, ["pointwise"] }
    fn octet_integer_casts_vt_162() { "octet-integers", 12, ["pointwise"] }
    fn binary_widths_vt_161() { "binary-widths", 128, ["pointwise"] }
    fn signed_float_storage_vt_160() { "integer-floats", 36, ["pointwise"] }
    fn signed_integer_widths_vt_159() { "integer-widths", 12, ["pointwise"] }
    fn scalar_diagonals_vt_157() { "scalar-diagonal", 36, ["pointwise"] }
    /// `VT-22`: Prism defines all four TRI variants and Verdant preserves retained encodings and zeroes masked elements for every type supported by the imported GGML CPU oracle
    fn triangular_graphs_vt_22() { "triangular", 1920, [] }
    /// `VT-23`: Prism plans copied and in-place FILL and Verdant converts and transfers F32 and F16 constants with upstream-equivalent encodings and preserved padding
    fn fill_graphs_vt_23() { "fill", 15360, [] }
    /// `VT-24`: Prism plans mixed F32 F16 and BF16 casts and copies by logical element order and Verdant conversion matches the imported GGML CPU oracle across reshaped strided and empty layouts
    fn cast_graphs_vt_24() { "cast", 1920, [] }
    /// `VT-25`: Prism models window partition padding and window join logical indexing and Verdant transfers match the imported GGML CPU oracle for independent strided empty and bounded window inputs
    fn window_graphs_vt_25() { "window", 3024, [] }
    /// `VT-26`: Prism plans scale bias clamp and leaky ReLU graphs and Verdant matches the imported GGML CPU kernels for every supported float storage type across copied in-place strided and empty tensors
    fn transform_graphs_vt_26() { "transform", 18816, [] }
    /// `VT-27`: Prism plans thirteen unary operations for all five supported GGML float source destination pairs and Verdant matches copied in-place strided and empty CPU graphs with exact finite outputs and preserved padding
    fn unary_graphs_vt_27() { "unary", 4992, [] }
    /// `VT-28`: Prism models row-local F32 cumulative sums with exact first-word preservation and Verdant matches the imported GGML CPU scan across strided and empty graphs
    fn scan_graphs_vt_28() { "scan", 528, [] }
    /// `VT-29`: Prism models SUM SUM_ROWS and MEAN accumulation precision rounding and empty reductions and Verdant matches every supported float type against the imported GGML CPU oracle
    fn reduction_graphs_vt_29() { "reduction", 4080, [] }
    /// `VT-30`: Prism models ARGMAX last-tie and NaN state and logical COUNT_EQUAL across all four tensor axes and Verdant matches imported GGML CPU row operations with typed refusals and exact integer outputs
    fn row_graphs_vt_30() { "rows", 2264, ["pointwise"] => verify_row_graphs() }
    /// `VT-31`: Prism models indexed F16 and F32 row-gradient accumulation into F32 and Verdant matches imported GGML GET_ROWS_BACK graphs with duplicate indices empty tensors and atomic invalid-index refusals
    fn row_gradient_graphs_vt_31() { "row-gradient", 3360, [] }
    /// `VT-32`: Prism models four-dimensional indexed row gathering and every executable GGML CPU source format and Verdant decodes the packed blocks with imported authoritative codebooks while preserving integer words padding empty tensors and atomic invalid-index refusals
    fn row_gather_graphs_vt_32() { "row-gather", 384, [] }
    /// `VT-33`: Prism models GGML relative-position word selection and Verdant preserves F16 and BF16 source encodings in F16 outputs across strided empty and higher-dimensional source layouts
    fn relative_position_graphs_vt_33() { "relative-position", 192, [] }
    /// `VT-34`: Prism models three-source indexed row addition and Verdant matches GGML ADD_ID arithmetic and exact minus-one row copies across strides empty tensors and atomic invalid-index refusals
    fn indexed_add_graphs_vt_34() { "indexed-add", 164, [] }
    /// `VT-37`: Prism models zero-source GGML ARANGE planning and Verdant executes every required imported upstream ARANGE case with atomic invalid-parameter refusals
    fn generator_graphs_vt_37() { "upstream-ARANGE", 13, ["pointwise"] }
    /// `VT-38`: Prism models GGML L2 normalization and Verdant matches every imported ordinary and batched upstream case with atomic invalid-parameter refusals
    fn normalization_graphs_vt_38() { "upstream-L2_NORM,L2_NORM_BATCH", 129, ["pointwise", "norm"] }
    /// `VT-39`: Prism models copied and in-place GGML ADD_REL_POS event ordering and Verdant matches the imported CPU oracle with atomic descriptor refusals
    fn relative_position_add_vt_39() { "upstream-ADD_REL_POS", 1315, ["pointwise"] => native_backend_oracle::verify(&root(), "upstream-ORACLE_CALIBRATION", 2) }
    /// `VT-46`: Prism models copied and in-place F32 and BF16 RMS normalization and Verdant matches every imported GGML CPU case with F64 mean ordering and atomic numeric refusals
    fn rms_normalization_vt_46() { "upstream-RMS_NORM", 121, ["pointwise"] }
    /// `VT-46`: Supplemental RMS normalization outputs and atomic refusals match Prism fixtures and the imported CPU oracle.
    fn rms_runtime_edges_vt_46() { "rms-norm", 256, ["pointwise"] }
    /// `VT-48`: NORM preserves stride-selected mean and variance rounding through Prism and Verdant.
    fn normalization_graphs_vt_48() { "upstream-NORM", 61, ["pointwise", "norm"] }
    /// `VT-48`: Prism NORM fixtures match exact CPU words and preserve atomic refusals and padding.
    fn norm_runtime_edges_vt_48() { "norm", 184, ["pointwise", "norm"] }
    /// `VT-49`: GROUP_NORM preserves nested row subtotals and matches ordinary and fused imported graphs.
    fn group_normalization_vt_49() { "upstream-GROUP_NORM,GROUP_NORM_MUL_ADD", 5, ["pointwise", "norm"] }
    /// `VT-49`: GROUP_NORM fixtures preserve nested rounding, physical spacing and atomic refusals.
    fn group_runtime_edges_vt_49() { "group-norm", 224, ["pointwise", "norm"] }
    /// `VT-50`: RMS_NORM_BACK preserves its two-source reductions and narrowing order.
    fn backward_normalization_vt_50() { "upstream-RMS_NORM_BACK", 10, ["pointwise", "norm"] }
    /// `VT-50`: Backward normalization fixtures preserve both sources, narrowing and atomic refusals.
    fn backward_runtime_edges_vt_50() { "rms-backward", 353, ["pointwise", "norm"] }
    /// `VT-51`: Full composed graphs preserve mixed storage, broadcasting and aliases.
    fn composed_normalization_vt_51() { "upstream-NORM_SCALE,RMS_NORM_SCALE,NORM_MUL_ADD,RMS_NORM_MUL_ADD,ADD_RMS_NORM", 187, ["typed", "pointwise", "norm"] }
    /// `VT-51`: Exact CPU words and atomic refusals cover every scalar storage tuple.
    fn typed_runtime_edges_vt_51() { "typed-binary", 977, ["pointwise", "typed"] }
    /// `VT-52`: Scheduled node selection and reuse match the imported CPU.
    fn synchronous_delay_vt_123() { "upstream-SLEEP", 3, ["native"] }

    fn scheduled_compute_flags_vt_52() { "scheduled-flags", 3, ["native"] }
    fn reusable_graph_plans_vt_158() { "graph-plans", 3, ["native"] }
    /// `VT-53`: The upstream factory evaluates all Q8_0 by F32 matrix cases.
    fn symmetric_matrix_graphs_vt_53() { "upstream-MUL_MAT_Q8_0_F32", 122, ["matrix"] }
    /// `VT-54`: The upstream factory evaluates all Q4_0 by F32 matrix cases.
    fn packed_matrix_graphs_vt_54() { "upstream-MUL_MAT_Q4_0_F32", 122, ["matrix"] }
    /// The original GGML graphs exercise grouped Q2_K products with contiguous and viewed weights.
    fn grouped_matrix_graphs_vt_65() { "upstream-MUL_MAT_Q2_K_F32", 56, ["matrix"] }
    /// `VT-55`: Indexed scalar row writes match the independent CPU.
    fn scalar_set_rows_vt_55() { "upstream-SET_ROWS_SCALAR", 126, ["typed"] }
    /// `VT-56`: The imported CPU checks all split, combined and swapped SWIGLU rows.
    fn swiglu_rows_vt_56() { "upstream-SWIGLU", 36, ["typed"] }
    /// `VT-57`: Exercise the upstream rotary-position graph factory on the provider.
    fn rotary_graphs_vt_57() { "upstream-ROPE", 510, ["rotary"] }
    /// `VT-57`: Exercise all upstream inverse-rotation graphs.
    fn rotary_backward_graphs_vt_57() { "upstream-ROPE_BACK", 424, ["rotary"] }
    /// Six scalar input pairs in upstream dense and permuted matrix products.
    fn float_matrix_graphs_vt_64() { "upstream-MUL_MAT_FLOAT_STORAGE", 11, ["pointwise", "matrix"] }

    /// The CPU compares both packed row formats in all four mask/sink combinations.
    fn packed_attention_vt_66() { "upstream-FLASH_ATTN_EXT_PACKED", 8, ["attention"] }

    fn hyperbolic_graphs_vt_67() { "upstream-TANH,SOFTCAP", 13, ["pointwise"] }

    fn capped_attention_vt_68() { "upstream-FLASH_ATTN_EXT_SOFTCAP", 4, ["attention"] }

    fn scalar_outer_products_vt_69() { "upstream-OUT_PROD_SCALAR", 87, ["pointwise", "matrix"] }

    fn image_columns_vt_70() { "upstream-IM2COL_3D_VIEW", 6, ["pointwise"] }

    fn packed_outer_products_vt_71() { "upstream-OUT_PROD_PACKED", 144, ["matrix", "pointwise"] }

    fn planar_columns_vt_72() { "upstream-IM2COL_PLANAR", 4, ["pointwise"] => native_backend_oracle::verify(&root(), "upstream-IM2COL_HALF_INPUT", 2) }

    fn outer_reference_vt_73() { "upstream-OUT_PROD_DECODED", 23, ["matrix", "pointwise"] }

    fn direct_convolution_vt_74() { "upstream-CONV_2D_DIRECT", 5, ["pointwise"] }

    fn antialiased_resampling_vt_147() { "upstream-UPSCALE_ANTIALIAS", 2, ["pointwise"] }
    fn bicubic_resampling_vt_146() { "upstream-UPSCALE_BICUBIC", 5, ["pointwise"] }
    fn bilinear_resampling_vt_145() { "upstream-UPSCALE_BILINEAR", 5, ["pointwise"] }
    fn nearest_resampling_vt_144() { "upstream-UPSCALE_NEAREST", 2, ["pointwise"] }
    fn recurrent_weighted_keys_vt_143() { "upstream-RWKV_BOUNDARIES", 2, ["pointwise"] }
    fn state_space_scan_vt_150() { "upstream-SSM_SCAN_BOUNDARIES", 4, ["attention"] }
    fn gated_delta_state_vt_149() { "upstream-GATED_DELTA_BOUNDARIES", 4, ["attention"] }
    fn gated_linear_attention_vt_142() { "upstream-GATED_LINEAR_ATTN_BOUNDARIES", 1, ["pointwise"] }
    fn timestep_embeddings_vt_141() { "upstream-TIMESTEP_EMBEDDING_BOUNDARIES", 5, ["pointwise"] }
    fn pool_gradients_vt_140() { "pool-gradients", 37, ["pointwise"] }
    fn column_gradients_vt_139() { "column-gradients", 64, ["pointwise"] }
    fn exponential_units_vt_138() { "upstream-EXPONENTIAL_UNITS", 18, ["pointwise"] }
    fn gaussian_error_graphs_vt_137() { "upstream-GAUSSIAN_ERROR", 48, ["typed", "pointwise"] }
    fn gaussian_graphs_vt_75() { "upstream-GELU_FAMILIES", 64, ["pointwise"] }

    fn contiguous_convolution_vt_76() { "upstream-CONV_2D_CONTIGUOUS", 3, ["pointwise"] }

    fn gated_activations_vt_77() { "upstream-GATED_ACTIVATIONS", 108, ["typed", "pointwise"] }

    fn biased_attention_vt_78() { "upstream-BIASED_ATTENTION", 7, ["attention"] }

    fn pooling_vt_79() { "upstream-POOLING", 8, ["pointwise"] => native_backend_oracle::verify(&root(), "upstream-POOL_HALF_INPUT", 4) }

    fn volume_convolution_vt_80() { "upstream-CONV_3D_DIRECT", 6, ["pointwise"] }

    fn top_k_mask_vt_92() { "upstream-TOP_K_MASK_VARIANTS", 6, ["attention"] }

    fn small_format_attention_vt_96() { "upstream-SMALL_FORMAT_ATTENTION", 19, ["attention"] }

    fn indexed_cache_attention_vt_97() { "upstream-INDEXED_KV_ATTENTION", 4, ["attention"] }

    fn e4m3_casts_vt_98() { "upstream-E4M3_CASTS", 29, ["pointwise"] }

    fn scaled_fp4_products_vt_99() { "upstream-SCALED_FP4_PRODUCTS", 4, ["matrix"] }

    fn e4m3_row_stores_vt_100() { "upstream-E4M3_ROW_STORES", 40, ["typed", "pointwise"] }

    fn q8_storage_writes_vt_101() { "upstream-Q8_STORAGE_WRITES", 31, ["typed"] }

    fn affine_weight_rows_vt_95() { "upstream-AFFINE_WEIGHT_ROWS", 8, ["attention", "matrix"] }

    fn compact_weight_rows_vt_94() { "upstream-COMPACT_WEIGHT_ROWS", 8, ["attention", "matrix"] }

    fn clamped_swiglu_vt_93() { "upstream-SWIGLU_CLAMP_VARIANTS", 12, ["typed"] }

    fn row_softmax_vt_87() { "upstream-ROW_SOFTMAX", 6, ["attention"] }

    fn packed_half_activations_vt_90() { "upstream-MUL_MAT_HALF_ACTIVATIONS", 3, ["matrix"] }

    fn grouped_affine_products_vt_88() { "upstream-GROUPED_AFFINE_PRODUCTS", 6, ["matrix"] }

    fn softplus_activation_vt_89() { "upstream-SOFTPLUS_ACTIVATION", 3, ["pointwise"] }

    fn bitpacked_products_vt_91() { "upstream-BITPACKED_PRODUCTS", 5, ["matrix"] }

    fn row_ranking_vt_86() { "upstream-ROW_RANKING", 6, ["typed"] }

    fn e4m3_products_vt_85() { "upstream-E4M3_PRODUCTS", 5, ["matrix", "pointwise"] }

    fn scalar_expert_matrix_vt_84() { "upstream-SCALAR_MUL_MAT_ID", 6, ["matrix"] }

    fn packed_expert_matrix_vt_83() { "upstream-PACKED_MUL_MAT_ID", 9, ["matrix"] }

    fn column_overlap_vt_82() { "upstream-COL2IM_1D", 33, ["pointwise"] }

    fn state_convolution_vt_81() { "upstream-SSM_WINDOWS", 6, ["pointwise"] }

    /// Exercise all upstream alpha, clipping, view and storage variants.
    fn parameterized_swiglu_vt_111() { "upstream-SWIGLU_OAI_VARIANTS", 24, ["typed", "pointwise"] }
    /// Compare each signed codebook layout through both ordinary and expert matrix paths.
    fn codebook_products_vt_103() { "upstream-CODEBOOK_PRODUCTS", 6, ["matrix"] }
    fn four_lane_codebooks_vt_115() { "upstream-FOUR_LANE_CODEBOOKS", 4, ["matrix"] }
    fn shifted_codebooks_vt_116() { "upstream-SHIFTED_CODEBOOKS", 4, ["matrix"] }
    fn ternary_row_products_vt_114() { "upstream-TERNARY_ROW_PRODUCTS", 4, ["matrix"] }
    fn grouped_nonlinear_product_vt_112() { "upstream-GROUPED_NONLINEAR_PRODUCT", 2, ["matrix"] }
    /// Compare grouped signed weights through ordinary and expert paths.
    fn grouped_signed_products_vt_110() { "upstream-GROUPED_SIGNED_PRODUCTS", 4, ["matrix"] }
    /// Decode contiguous and permuted source blocks with all original upstream copy inputs.
    fn decoded_copies_vt_104() { "upstream-DECODED_COPIES", 53, ["pointwise"] }
    /// Compare the original indexed and copied low-bit block stores.
    fn low_bit_writes_vt_105() { "upstream-LOW_BIT_WRITES", 121, ["typed"] }
    /// Contract and overlap all original one-dimensional transposed convolutions.
    fn transpose_convolution_vt_106() { "upstream-TRANSPOSE_CONV_1D", 116, ["pointwise"] }
    /// Compare masked and weighted per-head scores with the imported CPU evaluator.
    fn lightning_indexer_vt_108() { "upstream-LIGHTNING_INDEXER_BOUNDARIES", 10, ["attention"] }
    /// Contract and overlap all original two-dimensional transposed convolutions.
    fn transpose_plane_vt_107() { "upstream-TRANSPOSE_CONV_2D", 8, ["pointwise"] }
    fn grouped_affine_writes_vt_122() { "upstream-GROUPED_AFFINE_WRITES", 6, ["typed"] }
    fn importance_storage_vt_148() { "upstream-IMPORTANCE_THREE_STORAGE", 4, ["typed"] }
    fn importance_two_storage_vt_151() { "upstream-IMPORTANCE_TWO_STORAGE", 2, ["typed"] }
    fn parity_storage_vt_152() { "upstream-PARITY_GRID_STORAGE", 2, ["typed"] }
    fn grouped_parity_storage_vt_153() { "upstream-GROUPED_PARITY_STORAGE", 2, ["typed"] }
    fn shifted_block_storage_vt_154() { "upstream-SHIFTED_BLOCK_STORAGE", 4, ["typed"] }
    fn nonlinear_storage_vt_135() { "upstream-NONLINEAR_STORAGE", 4, ["typed"] }
    fn triangular_solve_vt_134() { "upstream-SOLVE_TRI_BOUNDARIES", 3, ["pointwise"] }
    fn grouped_three_writes_vt_133() { "upstream-GROUPED_THREE_WRITES", 2, ["typed"] }
    fn grouped_six_writes_vt_119() { "upstream-GROUPED_SIX_WRITES", 2, ["typed"] }

    fn compact_float_writes_vt_117() { "upstream-COMPACT_FLOAT_WRITES", 4, ["typed"] }
    fn ternary_weight_writes_vt_113() { "upstream-TERNARY_WEIGHT_WRITES", 60, ["typed"] }
    /// Replay all original copied and indexed sign and two-bit destinations.
    fn sign_weight_writes_vt_109() { "upstream-SIGN_WEIGHT_WRITES", 60, ["typed"] }
    /// Compare all remaining source formats against authoritative GGML decoding and CPU scatter.
    fn decoded_row_gradients_vt_102() { "upstream-GET_ROWS_BACK_DECODED", 54, ["pointwise"] }
    /// Four imported BF16 attention cases vary mask and sink presence independently.
    fn bf16_attention_vt_63() { "upstream-FLASH_ATTN_EXT_BF16", 4, ["attention"] }

    /// The first dense attention graph from the imported GGML factory.
    fn bf16_row_gradient_vt_62() { "row-gradient-bf16", 1680, ["pointwise"] }

    fn logistic_graphs_vt_61() { "upstream-SIGMOID,SILU,SIGMOID_MUL,SILU_MUL,ADD_SIGMOID_MUL,ADD_SILU_MUL", 122, ["pointwise"] }

    fn softmax_gradient_vt_121() { "upstream-SOFTMAX_GRADIENT", 2, ["pointwise"] }
    fn depthwise_spatial_vt_125() { "upstream-DEPTHWISE_SPATIAL", 8, ["pointwise"] }
    fn projection_precision_vt_128() { "upstream-PROJECTION_PRECISION", 36, ["matrix", "pointwise"] }
    fn signed_copy_vt_126() { "upstream-SIGNED_COPY", 2, ["pointwise"] }
    fn hyperconnection_post_vt_129() { "upstream-HYPERCONNECTION_POST", 11, ["pointwise"] }
    fn hyperconnection_combination_vt_130() { "upstream-HYPERCONNECTION_COMB", 11, ["pointwise"] }
    fn cross_entropy_vt_132() { "upstream-CROSS_ENTROPY_TRAINING", 4, ["pointwise"] }
    fn optimizer_updates_vt_131() { "upstream-OPTIMIZER_UPDATES", 2, ["pointwise"] }
    fn hyperconnection_pre_vt_127() { "upstream-HYPERCONNECTION_PRE", 18, ["pointwise"] }
    fn silu_gradient_vt_124() { "upstream-SILU_GRADIENT", 1, ["pointwise"] }
    fn small_float_add_vt_120() { "upstream-SMALL_FLOAT_ADD", 6, ["typed", "pointwise"] }
    fn exponential_difference_vt_118() { "upstream-EXPM1", 12, ["pointwise"] }

    fn elementary_graphs_vt_60() { "upstream-LOG,SIN,COS,EXP", 39, ["pointwise"] }

    fn extended_attention_vt_58() { "upstream-FLASH_ATTN_EXT_FIRST", 1, ["attention"] }
    /// Two short attention graphs cover a populated sink and its absence.
    fn small_attention_vt_58() { "upstream-FLASH_ATTN_EXT_SMALL", 2, ["attention"] }
}

/// `TC-01`: The checker answers conformant exactly when every universal policy file's observed digest equals its locked digest
#[test]
fn contract_tc_01() {
    let model = std::fs::read_to_string(root().join("app/src/Contract.lex.tex"))
        .expect("the contract model");

    let closed = model.rsplit_once("\n}\n").expect("closed declarations").0;
    let declarations: std::collections::BTreeMap<_, _> = closed
        .split("\n}\n")
        .filter_map(|block| block.split_once("\n  name: ")?.1.split_once(",\n"))
        .collect();
    let requires = |name: &str, term: &str| {
        assert!(declarations[name].contains(term), "{name} omitted {term}");
    };
    let obligations = native_backend_oracle::frozen_json(
        "template-contract-obligations",
        "4e9cc146865e995570b59117fe3ff34e0cc9e53fcfce4874c6cc6e627df6f033",
    );
    for (name, clauses) in obligations["declarations"].as_object().unwrap() {
        for clause in clauses.as_array().unwrap() {
            requires(name, clause.as_str().unwrap());
        }
    }
    let contract = ggml_oracle::read_json(root().join("template-contract.json"));
    let paths = contract["universal_policy_paths"].as_array().unwrap();
    let bound: Vec<_> = paths.iter().filter(|p| *p != "template.lock").collect();
    assert_eq!(serde_json::json!(bound), obligations["paths"]);
    for path in bound {
        requires("drifted", &format!("\" {}\"", path.as_str().unwrap()));
    }
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
    use vtensor_pointwise::{run, Adt0, Adt10, Adt9};
    for probe in probes(30, 2, 0, 0) {
        let vtensor_probes::Adt0::C7(op, kind, dest, extent) = probe else {
            panic!("missing empty row")
        };
        assert!(matches!(
            run(Adt0::C7(op, kind, dest, extent)),
            Ok(Adt9::C0(Adt10::C4(2, ..)))
        ));
    }
}

/// `VT-35`: Prism generates all matrix fixture dimensions layouts and input words matching the complete frozen prior oracle corpus while GGML independently computes every expected result
#[test]
fn fixture_generation_vt_35() {
    native_backend_oracle::fixture_input(0);
    assert!(probes(35, 6, 0, 0).all(|probe| matches!(probe, vtensor_probes::Adt0::C6(true))));
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
    // Verify that each declaration is an independently runnable, enabled libtest case.
    let listed = |extra: &[&str]| {
        let mut cmd = std::process::Command::new(std::env::current_exe().unwrap());
        String::from_utf8(ggml_oracle::run(cmd.arg("--list").args(extra)))
            .unwrap()
            .lines()
            .filter_map(|line| line.strip_suffix(": test").map(str::to_owned))
            .collect::<std::collections::BTreeSet<_>>()
    };
    let active = listed(&[]);
    let ignored = listed(&["--ignored"]);
    for &name in GGML_TEST_NAMES.iter().chain(FIXTURE_TEST_NAMES) {
        assert!(
            active.contains(name) && !ignored.contains(name),
            "unregistered or ignored batch {name}"
        );
    }
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
    tensor_layout::certified_current_model("probes");
    let query = move |index| vtensor_probes::run(suite, index, block, bytes).unwrap();
    assert!(matches!(query(count), vtensor_probes::Adt0::C0));
    (0..count).map(query)
}

fn fixture_queries(suite: u64, count: u64) {
    let spatial = suite == 47;
    tensor_layout::certified_current_model(if spatial { "spatial" } else { "arithmetic" });
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
    tensor_layout::certified_current_model("layout");
    let rows: Vec<_> = probes(suite, count, 0, 0).map(unpack).collect();
    let mut input = String::new();
    for row in &rows {
        writeln!(input, "{}", row.map(|word| word.to_string()).join(" ")).unwrap();
    }
    let reference = ggml_oracle::rows(name, &input);
    assert_eq!(rows.len(), reference.len());
    rows.into_iter().zip(reference)
}

macro_rules! fixture_query_tests {
    ($( $(#[$meta:meta])* fn $name:ident() { $suite:literal, $count:literal $(=> $prepare:expr)? } )+) => {
        const FIXTURE_TEST_NAMES: &[&str] = &[$(stringify!($name)),+];
        $( $(#[$meta])* #[test] fn $name() { $($prepare;)? fixture_queries($suite, $count); } )+
    };
}

fixture_query_tests! {
    /// `VT-36`: Prism generates every arithmetic fixture descriptor and input word while preserving the complete frozen broadcasting and exceptional-word corpus and the independent GGML CPU numerical oracle
    fn arithmetic_fixture_generation_vt_36() { 36, 5 => native_backend_oracle::fixture_input(1) }
    /// `VT-41`: Prism generates COPY fixture enumeration source and target descriptors and initial bytes while preserving all 3456 frozen cases and independent GGML CPU expected results
    fn copy_fixture_generation_vt_41() { 41, 11 }
    /// `VT-42`: Prism generates REPEAT fixture enumeration source target and output layouts and initial bytes while preserving all 160 frozen cases and independent GGML CPU expected results
    fn repeat_fixture_generation_vt_42() { 42, 5 }
    /// `VT-43`: Prism generates CONCAT fixture enumeration operand and output layouts and byte recipes while preserving all 10368 frozen cases and independent GGML CPU expected results
    fn concat_fixture_generation_vt_43() { 43, 5 }
    /// `VT-44`: Prism generates FILL and CAST fixture enumeration typed layouts constants and initial bytes while preserving all 17280 frozen cases source release states and independent GGML CPU expected results
    fn typed_fixture_generation_vt_44() { 44, 10 }
    /// `VT-47`: Prism supplies spatial fixture parameters and rejects undefined recipe queries.
    fn fixture_generation_vt_47() { 47, 4 }
}

/// `VT-45`: Verdant supplies certified F64 division and unsigned count to F64 conversion for normalization and preserves the imported GGML accumulator division before narrowing regression
#[test]
fn normalization_arithmetic_vt_45() {
    use verdant_tensor::linear::evaluate_wide;
    verdant_api::verify_quotient();
    let divide = |a, b| evaluate_wide(4, a, b).unwrap().unwrap();
    let widen = |a| evaluate_wide(5, a, 0).unwrap().unwrap();
    assert_eq!(divide(1.0f64.to_bits(), 2.0f64.to_bits()), 0.5f64.to_bits());
    assert_eq!(widen(u64::MAX), 0x43f0000000000000);
    let mean = divide(widen(16777217), widen(5));
    assert_eq!(evaluate_wide(2, mean, 0).unwrap(), Some(0x4a4cccce));
    assert_eq!(evaluate_wide(6, 0, 0).unwrap(), None);
}

fn assert_model_identity(name: &str, model: &serde_json::Value, source: &serde_json::Value) {
    for key in ["source_id", "semantic_id"] {
        assert_eq!(model[key], source[key], "stale {name} model: {key}");
    }
}

// Hash the compiler's own recipe tokens. A change to either the cached
// artifact's inputs or the command that produces it selects a new artifact.
macro_rules! build_recipe {
    ($id:ident; $($item:item)+) => { const $id: &str = stringify!($($item)+); $($item)+ };
}
build_recipe! { STORAGE_RECIPE;
mod ggml_oracle {

    use sha2::{Digest, Sha256};
    use std::io::Write;
    use std::path::{Path, PathBuf};
    use std::process::{Command, Output, Stdio};

    pub(super) fn read_bytes(path: &Path) -> Vec<u8> {
        std::fs::read(path).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
    }

    pub(super) fn read_json(path: impl AsRef<Path>) -> serde_json::Value {
        let path = path.as_ref();
        serde_json::from_slice(&read_bytes(path))
            .unwrap_or_else(|error| panic!("{}: {error}", path.display()))
    }

    pub(super) static UPSTREAM: std::sync::LazyLock<toml::Value> = std::sync::LazyLock::new(|| {
        let artifact = super::root().join("vendor/llama.cpp-oracle");
        // The frozen manifest binds the exact upstream revision, archive selection,
        // order, filenames and digests. Check the bytes of every listed artifact.
        let manifest = read_bytes(&artifact.join("upstream.toml"));
        let pin = &super::native_backend_oracle::ORACLE_BINDINGS["ggml_source_manifest_sha256"];
        let digest = repo_model::artifact::sha256_hex(&manifest);
        assert_eq!(pin, &digest, "GGML source manifest changed");
        let lock: toml::Value = toml::from_slice(&manifest).expect("GGML source manifest");
        let files = lock["file"].as_array().expect("pinned GGML files");
        let archives = lock["archive"].as_array().expect("pinned GGML archive");
        for entry in files.iter().chain(archives) {
            let path = entry["path"].as_str().expect("artifact path");
            let expected = entry["sha256"].as_str().expect("artifact digest");
            assert_eq!(file_digest(artifact.join(path)), expected, "{path}");
        }
        lock
    });

    pub(super) fn publish_file(candidate: &Path, destination: &Path) {
        std::fs::write(destination.with_extension("sha256"), file_digest(candidate)).expect("cache digest");
        std::fs::rename(candidate, destination).expect("publish built oracle");
    }

    pub(super) fn verify_file(path: &Path) {
        assert_eq!(file_digest(path), std::fs::read_to_string(path.with_extension("sha256")).unwrap());
    }

    pub(super) fn file_digest(path: impl AsRef<Path>) -> String {
        repo_model::artifact::file_sha256(path.as_ref()).unwrap()
    }

    pub(super) fn artifact_cache(common: &Path, family: &str, inputs: &[&[u8]]) -> PathBuf {
        let mut identity = Sha256::new();
        for input in inputs {
            identity.update((input.len() as u64).to_le_bytes());
            identity.update(input);
        }
        common.join(family).join(repo_model::artifact::digest_hex(&identity.finalize()))
    }

    // Build only the requested oracle component. A changed recipe or compiler selects
    // a fresh cache, while all hosts reuse the same verified CPU and core objects.
    const MAKEFILE: &str = r#"
CPPFLAGS = -Iggml/include -Iggml/src -Iggml/src/ggml-cpu -I. -D_GNU_SOURCE -DGGML_CPU_GENERIC -U__ARM_FP16_FORMAT_IEEE
flags = -O2 -ffp-contract=off -ffunction-sections -fdata-sections
core = ggml.c ggml-quants.c ggml-alloc.c ggml-backend.cpp ggml-backend-meta.cpp
cpu = ggml-backend-reg.cpp ggml-backend-dl.cpp ggml-threading.cpp $(shell awk '/list .APPEND GGML_CPU_SOURCES/ {listing=1; next} listing && NF==1 && $$1 !~ /[.]/ {exit} listing && $$1 ~ /[.]c(pp)?$$/ {print $$1}' ggml/src/ggml-cpu/CMakeLists.txt)
cxx = c++ $(CPPFLAGS) $(flags) -std=c++17
objects/ggml-cpu/ops.cpp.o: CPPFLAGS += -U__ARM_NEON
objects/%.c.o: ggml/src/%.c
	@mkdir -p "$(@D)"
	cc $(CPPFLAGS) $(flags) -std=c11 -c "$<" -o "$@"
objects/%.cpp.o: ggml/src/%.cpp
	@mkdir -p "$(@D)"
	$(cxx) -c "$<" -o "$@"
core: $(addprefix objects/,$(addsuffix .o,$(core)))
	c++ -r $^ -o "$$VT_OUTPUT"
cpu: $(addprefix objects/,$(addsuffix .o,$(cpu)))
	c++ -r $^ -o "$$VT_OUTPUT"
storage:
	$(cxx) boundary.cpp ggml-oracle-core.o -Wl,--gc-sections -lm -o "$$VT_OUTPUT"
suite:
	$(cxx) -c "$$VT_INPUT" -o "$$VT_OUTPUT"
host:
	$(cxx) "$$VT_INPUT" $${VT_SUITE:+"$$VT_SUITE"} ggml-oracle-core.o ggml-oracle-cpu.o "$$VT_LIBRARY" -Wl,-rpath,"$$VT_LIBDIR" -Wl,--gc-sections -Wl,--export-dynamic -lm -ldl -pthread -o "$$VT_OUTPUT"
"#;

    pub(super) fn build(common: &Path, kind: &str, out: &Path, variables: &[(&str, &Path)]) {
        let mut make = Command::new("make");
        make.current_dir(common).args(["-rB", "-j4", "-f", "-", kind]).env("VT_OUTPUT", out);
        make.env_remove("MAKEFLAGS").env_remove("VT_SUITE").envs(variables.iter().copied());
        let result = pipe_input(&mut make, MAKEFILE.as_bytes());
        checked_output(&make, result);
    }

    pub(super) fn run(command: &mut Command) -> Vec<u8> {
        let output = command.output().expect("oracle build tool available in SDK");
        checked_output(command, output)
    }

    pub(super) fn tool(root: &Path, name: &str, args: &[&str]) -> Vec<u8> {
        run(Command::new(name).current_dir(root).args(args))
    }

    pub(super) fn tool_json(root: &Path, name: &str, args: &[&str]) -> serde_json::Value {
        serde_json::from_slice(&tool(root, name, args)).expect("tool JSON response")
    }

    fn checked_output(command: &Command, output: Output) -> Vec<u8> {
        assert!(output.status.success(), "{command:?}: {output:?}");
        output.stdout
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
        let expected_status = Some(i32::from(!differences.is_empty()));
        assert!(result.stderr.is_empty() && result.status.code() == expected_status, "{result:?}");
        for line in differences.lines() {
            let reason = line.rsplit_once(": ").map(|(_, reason)| reason);
            let metadata = matches!(reason, Some("Uid differs" | "Gid differs" | "Mode differs" | "Mod time differs"));
            assert!(metadata, "cached GGML reference differs from its pinned archive: {line}");
        }
    }

    /// Build the oracle once per source, compiler, SDK and build-recipe identity.
    /// A cache hit still verifies the executable digest and executes the oracle.
    pub(super) static STORAGE_ORACLE: std::sync::LazyLock<PathBuf> =
        std::sync::LazyLock::new(|| build_storage_oracle(&super::root()));


    fn build_storage_oracle(root: &Path) -> PathBuf {
        let imported = root.join("vendor/llama.cpp-oracle");
        let lock = &*UPSTREAM;
        let archive_digest = lock["archive"][0]["sha256"].as_str().expect("archive pin");
        let sdk = std::env::var_os("PRISMPM_SDK_INVENTORY")
            .map(|path| std::fs::read(path).expect("running SDK inventory"))
            .unwrap_or_else(|| b"host-development-only".to_vec());
        let target = std::env::var_os("CARGO_TARGET_DIR").map(PathBuf::from)
            .unwrap_or_else(|| std::env::temp_dir().join("vtensor-native"));
        let cache = artifact_cache(&target, "ggml-oracle", &[
            archive_digest.as_bytes(), super::STORAGE_RECIPE.as_bytes(), &sdk, std::env::consts::ARCH.as_bytes(),
            &run(Command::new("make").arg("--version")), &run(Command::new("cc").arg("--version")),
            &run(Command::new("c++").arg("--version")),
        ]);
        let executable = cache.join("storage-oracle");
        if executable.exists() {
            verify_file(&executable);
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
        let header = std::fs::read_to_string(cache.join("ggml/include/ggml.h")).unwrap();
        let mut declarations = String::new();
        for (table, name) in [("operation","ggml_op"), ("tensor_type","ggml_type"), ("unary","ggml_unary_op"), ("glu","ggml_glu_op")] {
            let (_, tail) = header.split_once(&format!("enum {name} {{")).expect("upstream enum");
            let (body, _) = tail.split_once("};").expect("complete enum");
            for line in body.lines().map(|s| s.split("//").next().unwrap().trim()).filter(|s| !s.is_empty()) {
                let item = line.strip_suffix(',').expect("one enumerator per line");
                let symbol = item.split('=').next().unwrap().trim();
                assert!(symbol.bytes().all(|b| b.is_ascii_alphanumeric() || b==b'_'));
                declarations.push_str(&format!("EMIT({table}, {symbol});\n"));
            }
        }
        let boundary = STORAGE_ORACLE_SOURCE.replace("/* pinned enumerators */", &declarations);
        std::fs::write(cache.join("boundary.cpp"), boundary).expect("oracle transport");
        for (kind, file) in [("core", cache.join("ggml-oracle-core.o")), ("storage", executable.clone())] {
            let candidate = file.with_extension("new");
            build(&cache, kind, &candidate, &[]);
            publish_file(&candidate, &file);
        }
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
            let written = writer.join().expect("input writer");
            if output.status.success() { written.expect("complete oracle input"); }
            output
        })
    }

    pub(super) fn batch(command: &mut Command, input: impl AsRef<[u8]>) -> String {
        let output = pipe_input(command, input.as_ref());
        String::from_utf8(checked_output(command, output)).expect("oracle text")
    }

    pub(super) fn query(mode: &str, input: &str) -> String {
        let executable = &*STORAGE_ORACLE;
        verify_file(executable);
        batch(Command::new(executable).arg(mode), input)
    }

    pub(super) fn rows<const N: usize>(mode: &str, input: &str) -> Vec<[u64; N]> {
        query(mode, input)
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

    const STORAGE_ORACLE_SOURCE: &str = r##"// Test transport for three independent upstream GGML interfaces.
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
// Isolate GGML abort callbacks from the other oracle requests in this stream.
template <class F> static int observe_child(F invoke) {
    fflush(stdout);
    const pid_t child = fork();
    if (child < 0) return -3;
    if (child == 0) { const int code = invoke(); fflush(stdout); _exit(code); }
    int status;
    if (waitpid(child, &status, 0) != child) return -4;
    return WIFEXITED(status) ? WEXITSTATUS(status) : 99;
}
static int allocation() {
    size_t capacity, offset, size;
    for (;;) {
        const int fields = scanf("%zu %zu %zu %zu", &capacity, &offset, &size, &alignment);
        if (fields == EOF) return 0;
        if (fields != 4 || !alignment || (alignment & (alignment - 1)) || alignment > 4096 || capacity > 4096 || size > 4096 || offset > 8192 || offset % alignment) return 2;
        const int code = observe_child([&] {
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
            return 0;
        });
        if (code < 0) return -code;
        if (code == 99) return 5;
        if (code == 10) printf("1 0 0 0\n");
        else if (code != 0) return 6;
    }
}

static int storage() {
    for (;;) {
        int type;
        ggml_tensor tensor = {};
        auto &ne=tensor.ne;
        auto &nb=tensor.nb;
        const int fields = scanf("%d %" SCNd64 " %" SCNd64 " %" SCNd64 " %" SCNd64
                                " %zu %zu %zu %zu",
                                &type, &ne[0], &ne[1], &ne[2], &ne[3],
                                &nb[0], &nb[1], &nb[2], &nb[3]);
        if (fields == EOF) { return 0; }
        if (fields != 9 || type < 0 || type >= GGML_TYPE_COUNT) { return 2; }
        tensor.type = static_cast<ggml_type>(type);
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
        const int code = observe_child([&] {
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
            return calls ? 1 : 0;
        });
        if (code < 0) return -code;
        printf("%d\n", code);
    }
}

int main(int argc, char **argv) {
    if (argc==1 || (argc==2 && !strcmp(argv[1],"storage"))) return storage();
    if (argc==2 && !strcmp(argv[1],"declarations")) {
#define EMIT(table, symbol) printf("[[" #table "]]\nsymbol=\"" #symbol "\"\nvalue=%d\n", int(symbol))
        /* pinned enumerators */
#undef EMIT
        return 0;
    }
    if (argc==2 && !strcmp(argv[1],"allocation")) return allocation();
    if (argc==2 && !strcmp(argv[1],"transfer")) return transfer();
    return 2;
}
"##;
}

}

mod ggml_inventory {

    use std::sync::LazyLock;

    static INVENTORY: LazyLock<toml::Value> = LazyLock::new(|| {
        let file = super::root().join("model/ggml-declarations.toml");
        toml::from_slice(&super::ggml_oracle::read_bytes(&file))
            .expect("GGML declaration inventory")
    });
    pub(super) static STORAGE_TYPES: LazyLock<Vec<u64>> = LazyLock::new(|| {
        let types = INVENTORY["tensor_type"].as_array().expect("types");
        types
            .iter()
            .filter(|row| row["symbol"].as_str() != Some("GGML_TYPE_COUNT"))
            .map(|row| {
                u64::try_from(row["value"].as_integer().expect("type identifier"))
                    .expect("nonnegative GGML type")
            })
            .collect()
    });

    /// `VT-04`: The imported GGML operation and tensor type declarations are inventoried exactly without implementation claims
    #[test]
    fn ggml_inventory_vt_04() {
        let compiled: toml::Value =
            toml::from_str(&super::ggml_oracle::query("declarations", "")).unwrap();
        let inventory = &*INVENTORY;
        for (key, expected) in [
            ("spec", "vtensor/ggml-declarations/1"),
            ("authority", "LLAMA-CPP-GGML-BACKEND"),
            ("source", "ggml/include/ggml.h"),
        ] {
            assert_eq!(inventory[key].as_str(), Some(expected), "{key}");
        }
        assert_eq!(
            inventory["revision"],
            super::ggml_oracle::UPSTREAM["commit"]
        );
        for key in ["operation", "tensor_type", "unary", "glu"] {
            let expected = compiled[key].as_array().expect("upstream C enumerators");
            let original = inventory[key].as_array().expect("declarations");
            let matches = |rows: &[toml::Value]| rows == expected;
            assert!(matches(original), "changed GGML {key} declarations");
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
        let executable = &*ggml_oracle::STORAGE_ORACLE;
        let fixture = &super::native_backend_oracle::ORACLE_BINDINGS["storage_probe"];
        let expected: [[u64; 3]; 6] =
            serde_json::from_value(fixture["span_block_bytes"].clone()).unwrap();
        let modified = || std::fs::metadata(executable).unwrap().modified().unwrap();
        let built_at = modified();
        // Send the fixed probes and a duplex batch larger than the pipe capacity through one oracle process.
        let mut request = fixture["request"].as_str().unwrap().to_owned();
        request.push_str(&"0 1 1 1 1 4 4 4 4\n".repeat(8192));
        let rows = ggml_oracle::rows("storage", &request);
        let reference = expected
            .into_iter()
            .chain(std::iter::repeat_n([4, 1, 4], 8192));
        assert!(rows.into_iter().eq(reference));
        let malformed = ggml_oracle::pipe_input(
            &mut std::process::Command::new(executable),
            &b"malformed\n".repeat(131072),
        );
        assert_eq!(malformed.status.code(), Some(2));

        assert_eq!(modified(), built_at, "warm storage oracle was rebuilt");
    }
}

mod oracle_import {

    use super::root;

    use repo_model::Model;

    /// `VT-01`: The imported GGML backend contract, implementation interface, and operator oracle match their pinned upstream source
    #[test]
    fn oracle_import_vt_01() {
        let root = root();
        let directory = "vendor/llama.cpp-oracle";
        let lock = &*super::ggml_oracle::UPSTREAM;
        let files = lock["file"].as_array().expect("pinned GGML files");
        let exemption = super::ggml_oracle::read_json(root.join("oracle-import.lock"));
        let expected_files: Vec<_> = files.iter().map(|file| serde_json::json!({"path": format!("{directory}/{}", file["path"].as_str().expect("path")), "sha256": file["sha256"].as_str().expect("digest")})).collect();
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
        assert_eq!(authority.artifact.as_deref(), Some(directory));
    }
}

mod tensor_layout {

    use super::ggml_oracle;
    use std::{
        collections::BTreeMap,
        sync::{Arc, Mutex, OnceLock},
    };

    use vtensor_layout::{run, Adt0, Adt1, Overflow};

    pub(super) fn certified_current_model(name: &'static str) {
        static SCOPES: Mutex<BTreeMap<&str, Arc<OnceLock<()>>>> = Mutex::new(BTreeMap::new());
        let scope = SCOPES.lock().unwrap().entry(name).or_default().clone();
        scope.get_or_init(|| verify_scope(&super::root(), name));
    }

    fn verify_scope(root: &std::path::Path, name: &str) {
        let project = match name {
            "layout" => "app/lexlean.toml".to_owned(),
            _ => format!("app/{name}.lexlean.toml"),
        };
        let bundle = format!("generated/{name}");
        let result = ggml_oracle::read_json(root.join(format!("{bundle}/result.json")));
        let flags = ["--project", &project, "--diagnostic-format=json", "check"];
        let current = ggml_oracle::tool_json(root, "lexlean", &flags);
        super::assert_model_identity(name, &current, &result["source"]);
        let lock_digest = ggml_oracle::file_digest(root.join(project).with_extension("lock"));
        assert_eq!(result["source"]["lock_sha256"], lock_digest);
        let lock_args = ["--quiet", "certified-kernel-lock", name, &bundle];
        let expected = ggml_oracle::tool_json(root, "just", &lock_args);
        let lock = ggml_oracle::read_json(root.join(format!("{name}-kernel.lock")));
        assert_eq!(lock, expected, "stale {name} kernel lock");
        ggml_oracle::tool(root, "lexlean", &["production", "verify-artifact", &bundle]);
    }

    static STORAGE_METADATA: std::sync::LazyLock<Vec<[u64; 3]>> = std::sync::LazyLock::new(|| {
        certified_current_model("layout");
        let kinds = &*super::ggml_inventory::STORAGE_TYPES;
        let input: String = kinds
            .iter()
            .map(|kind| format!("{kind} 0 0 0 0 0 0 0 0\n"))
            .collect();
        let metadata = ggml_oracle::rows("storage", &input);
        assert_eq!(metadata.len(), kinds.len(), "oracle omitted a type");
        metadata
    });

    /// `VT-06`: The Prism-generated four-dimensional byte-span function matches the pinned GGML storage oracle and rejects invalid blocks and arithmetic overflow
    #[test]
    fn tensor_layout_vt_06() {
        let kinds = &*super::ggml_inventory::STORAGE_TYPES;
        let mut inputs = String::new();
        let mut layouts = Vec::new();
        for (&kind, &[_, block, bytes]) in kinds.iter().zip(STORAGE_METADATA.iter()) {
            assert!(
                block > 0 && bytes > 0,
                "invalid upstream descriptor for type {kind}"
            );
            for probe in super::probes(6, 52, block, bytes) {
                let vtensor_probes::Adt0::C4(n0, n1, n2, n3, b0, b1, b2, b3) = probe else {
                    panic!("missing layout probe for type {kind}");
                };
                use std::fmt::Write;
                writeln!(inputs, "{kind} {n0} {n1} {n2} {n3} {b0} {b1} {b2} {b3}").unwrap();
                layouts.push(Adt0::C0(n0, n1, n2, n3, b0, b1, b2, b3, block, bytes));
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
        for probe in super::probes(106, 5, 0, 0) {
            let vtensor_probes::Adt0::C5(a, b, c, d, e, f, g, h, block, width, expected) = probe
            else {
                panic!("missing layout boundary");
            };
            let result = run(Adt0::C0(a, b, c, d, e, f, g, h, block, width));
            assert!(matches!(
                (expected, result),
                (0, Ok(Adt1::C0)) | (1, Err(Overflow)) | (2, Ok(Adt1::C1(0)))
            ));
        }
    }

    /// `VT-07`: The Prism-generated type catalog matches every declared GGML storage type and refuses removed and out-of-range identifiers
    #[test]
    fn tensor_layout_vt_07() {
        let kinds = &*super::ggml_inventory::STORAGE_TYPES;
        for (&kind, &[_, block, bytes]) in kinds.iter().zip(STORAGE_METADATA.iter()) {
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

    use super::ggml_oracle::tool_json;
    use prism_kernel::supportsF64Matmul4;
    use verdant_tensor::linear::multiply_mat4_cols_exact;
    use verdant_tensor::{verified_unified_tensor_kernels, TensorComputeError};

    use std::sync::LazyLock;

    static SOURCE: LazyLock<serde_json::Value> =
        LazyLock::new(|| tool_json(&super::root(), "just", &["verdant-source-check"]));

    pub(super) fn verify_quotient() {
        let binding = &super::native_backend_oracle::ORACLE_BINDINGS["verdant_quotient"];
        let image = binding["sdk"].as_str().expect("pinned Verdant compiler");
        let check = tool_json(&super::root(), "just", &["verdant-quotient-check", image]);
        super::assert_model_identity("Verdant quotient", &check, &SOURCE);
    }

    /// `VT-02`: The backend resolves the pinned Verdant API with fifteen ordered kernel descriptors and imports only GGML host services
    #[test]
    fn verdant_api_vt_02() {
        LazyLock::force(&SOURCE);
        let kernels = verified_unified_tensor_kernels().expect("Verdant verified kernel inventory");
        assert!(kernels.iter().map(|kernel| kernel.index).eq(0..15));

        let listing = super::ggml_oracle::run(
            std::process::Command::new("nm")
                .args(["-D", "--undefined-only", "--format=just-symbols"])
                .arg(super::provider_library()),
        );
        let allowed = super::native_backend_oracle::ORACLE_BINDINGS["allowed_ggml_host_imports"]
            .as_array()
            .expect("GGML host imports");
        let symbols = std::str::from_utf8(&listing).unwrap().lines();
        for symbol in symbols.filter(|s| s.starts_with("ggml_") || s.starts_with("gguf_")) {
            assert!(
                allowed.iter().any(|s| s == symbol),
                "unexpected provider import {symbol}"
            );
        }
    }

    /// `VT-03`: vTensor delegates fixed 4x4 F64 matrix multiplication to Verdant Tensor and preserves column-major results
    #[test]
    fn verdant_api_vt_03() {
        for bits in 0..8 {
            let [f64_type, left_4x4, right_4x4] = [1, 2, 4].map(|mask| bits & mask != 0);
            assert_eq!(supportsF64Matmul4(f64_type, left_4x4, right_4x4), bits == 7);
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
    use super::ggml_oracle::{artifact_cache, build, run, verify_file, STORAGE_ORACLE};
    use repo_model::artifact::sha256_hex;
    use std::fmt::Write;
    use std::path::Path;
    use std::process::Command;

    pub(super) fn frozen_json(name: &str, expected: &str) -> serde_json::Value {
        let bytes = std::fs::read(super::root().join(format!("verification/{name}.json"))).unwrap();
        assert_eq!(sha256_hex(&bytes), expected, "frozen {name} changed");
        serde_json::from_slice(&bytes).unwrap()
    }

    use std::sync::LazyLock;

    pub(super) static ORACLE_BINDINGS: LazyLock<serde_json::Value> = LazyLock::new(|| {
        frozen_json(
            "native-oracle-bindings",
            "29ca7e8a52848098a861fe9562b52dd9bba073a850446b6fd87b0d52697fea24",
        )
    });

    fn fixture_record(name: &str) -> serde_json::Value {
        let digest = ORACLE_BINDINGS["fixture_digests"][name]
            .as_str()
            .expect("fixture digest");
        frozen_json(name, digest)
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
            use vtensor_fixture_stream::{Adt0 as Cursor, Adt1 as Step};
            let record =
                ["matrix-fixture-baseline", "arithmetic-fixture-baseline"][family as usize];
            super::tensor_layout::certified_current_model("fixtures");
            let baseline = fixture_record(record);
            let mut input = Vec::new();
            let mut cursor = Cursor::C0;
            loop {
                match vtensor_fixture_stream::run(family, cursor).expect("bounded fixture stream") {
                    Step::C0 => break,
                    Step::C1 => panic!("certified fixture stream refused a registered case"),
                    Step::C2(word, next) => {
                        input.extend(word.to_le_bytes());
                        cursor = next;
                    }
                }
            }
            let header = ["cases", "metadata_fields", "sources", "padding_byte"]
                .map(|key| baseline[key].as_u64().unwrap().to_le_bytes());
            assert_eq!(input.get(..32), Some(header.as_flattened()));
            let records = &input[32..];
            assert_eq!(baseline["transcript_bytes"], records.len());
            assert_eq!(baseline["transcript_sha256"], sha256_hex(records));
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

    build_recipe! { FIXTURE_RECIPE;
    fn fixture_library(common: &Path, model: &'static str) -> std::path::PathBuf {
        use std::{collections::BTreeMap, path::PathBuf, sync::Mutex};
        static LIBRARIES: Mutex<BTreeMap<&str, PathBuf>> = Mutex::new(BTreeMap::new());
        let mut libraries = LIBRARIES.lock().expect("fixture library cache");
        let library = libraries.entry(model).or_insert_with(|| {
            super::tensor_layout::certified_current_model(model);
            let source = super::root().join(format!(
                "generated/{model}/packages/rust-core/vtensor_{model}/src/lib.rs"
            ));
            static COMPILER: LazyLock<Vec<u8>> = LazyLock::new(|| run(Command::new("rustc").args(["--version", "--verbose"])));
            let cache = artifact_cache(common, "fixture-model", &[&std::fs::read(&source).unwrap(), FIXTURE_BRIDGE.as_bytes(), FIXTURE_RECIPE.as_bytes(), OBJECT_RECIPE.as_bytes(), &COMPILER]);
            let library = cache.join("libfixtures.so");
            checked_build(&library, |pending| {
                let input = cache.join("bridge.rs");
                let bridge = format!("#[path = {source:?}] mod vtensor_fixture_model;\n{FIXTURE_BRIDGE}");
                std::fs::write(&input, bridge).unwrap();
                let mut rustc = Command::new("rustc");
                rustc.args(["--edition=2021", "--crate-type=cdylib", "-Copt-level=2"]);
                rustc.args(["-Cdebug-assertions=on", "-Coverflow-checks=on"]);
                run(rustc.arg(input).arg("-o").arg(pending));
            });
            library
        });
        verify_file(library);
        library.clone()
    }

    }

    build_recipe! { OBJECT_RECIPE;
    fn checked_build(output: &Path, build: impl FnOnce(&Path)) {
        if !output.exists() {
            std::fs::create_dir_all(output.parent().unwrap()).unwrap();
            let pending = output.with_extension("new");
            build(&pending);
            super::ggml_oracle::publish_file(&pending, output);
        }
        verify_file(output);
    }

    }

    build_recipe! { HOST_RECIPE;
    fn build_host(common: &Path, library: &Path, imported: bool) -> std::path::PathBuf {
        let mut driver = String::from(HEADER);
        if imported {
            driver.insert_str(0, "#define VTENSOR_IMPORTED_SUITE\n");
        }
        for (key, value) in vtensor_backend::abi_layout() {
            let (ty, field) = key.rsplit_once('.').unwrap();
            let expression = match field {
                "$size" => format!("sizeof({ty})"),
                "$align" => format!("alignof({ty})"),
                _ => format!("offsetof({ty}, {field})"),
            };
            writeln!(driver, "static_assert({expression} == {value});").unwrap();
        }
        let mut stores = Vec::new();
        for (batch, types) in ORACLE_BINDINGS["atomic_store_types"].as_object().unwrap() {
            let ids = types.as_array().unwrap().iter().map(|id| id.as_u64().unwrap().to_string()).collect::<Vec<_>>();
            stores.push(format!("{{{}, {{{}}}}}", serde_json::json!(batch), ids.join(",")));
        }
        writeln!(driver, "#define VTENSOR_STORE_TYPES {}", stores.join(",")).unwrap();
        driver.push_str(BODY);
        // The host dynamically loads the plugin. Its snapshot binds the bytes at the library path.
        let suite = if imported { UPSTREAM_SUITE.as_bytes() } else { &[] };
        let cache = artifact_cache(common, "native", &[library.as_os_str().as_encoded_bytes(), driver.as_bytes(),
            common.as_os_str().as_encoded_bytes(), HOST_RECIPE.as_bytes(), OBJECT_RECIPE.as_bytes(), suite]);
        let program = cache.join("native-oracle");
        checked_build(&program, |pending| {
            let input = cache.join("driver.cpp");
            std::fs::write(&input, &driver).unwrap();
            let cpu = common.join("ggml-oracle-cpu.o");
            checked_build(&cpu, |out| build(common, "cpu", out, &[]));
            let core = common.join("ggml-oracle-core.o");
            verify_file(&core);
            let suite = common.join("imported-suite").join(sha256_hex(UPSTREAM_SUITE.as_bytes())).join("suite.o");
            let mut variables = vec![("VT_INPUT", input.as_path()), ("VT_LIBRARY", library), ("VT_LIBDIR", library.parent().unwrap())];
            if imported {
                checked_build(&suite, |out| {
                    let source = suite.with_extension("cpp");
                    std::fs::write(&source, UPSTREAM_SUITE).unwrap();
                    build(common, "suite", out, &[("VT_INPUT", &source)]);
                });
                variables.push(("VT_SUITE", &suite));
            }
            build(common, "host", pending, &variables);
        });
        program
    }
    }

    pub(super) fn verify(root: &Path, mode: &str, expected_cases: usize) {
        static ACCESS: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let build_guard = ACCESS.lock().unwrap_or_else(|error| error.into_inner());
        let common = STORAGE_ORACLE.parent().unwrap();
        let library = super::provider_library();
        static HOSTS: [std::sync::OnceLock<std::path::PathBuf>; 2] =
            [const { std::sync::OnceLock::new() }; 2];
        let imported = mode.starts_with("upstream-");
        let program =
            HOSTS[usize::from(imported)].get_or_init(|| build_host(common, &library, imported));
        let cache = program.parent().unwrap();
        verify_file(program);
        static STORAGE_TYPES: LazyLock<String> = LazyLock::new(|| {
            super::ggml_inventory::STORAGE_TYPES
                .iter()
                .map(|kind| format!("{kind}\n"))
                .collect()
        });
        let words;
        let input: &[u8] = match (mode, ORACLE_BINDINGS["cast_word_tables"][mode].as_str()) {
            ("matrix" | "elementwise", _) => fixture_input(u64::from(mode == "elementwise")),
            (_, Some(table)) => {
                let path = root.join(format!("verification/{table}-words.tsv"));
                let oracle = fixture_record(&format!("{table}-oracle"));
                words = if oracle["compression"] == "gzip" {
                    run(Command::new("gzip")
                        .arg("-dc")
                        .arg(path.with_extension("tsv.gz")))
                } else {
                    std::fs::read(path).unwrap()
                };
                assert_eq!(sha256_hex(&words), oracle["words_sha256"]);
                &words
            }
            ("update", _) => &[],
            _ if imported => ORACLE_BINDINGS["factory_inputs"][mode]
                .as_str()
                .unwrap_or("")
                .as_bytes(),
            _ => STORAGE_TYPES.as_bytes(),
        };
        static SOURCES: std::sync::Once = std::sync::Once::new();
        SOURCES.call_once(|| {
            for &(name, compiled) in vtensor_backend::SOURCE_INPUTS {
                let current = std::fs::read(root.join(name)).unwrap();
                assert!(current == compiled, "Rust test library has stale {name}");
            }
        });
        let unique = super::ggml_oracle::tool(cache, "mktemp", &["-d", "-p", "."]);
        let scratch = cache.join(std::str::from_utf8(&unique).unwrap().trim_end());
        let expected_snapshot = scratch.join("inputs.snapshot");
        std::fs::write(&expected_snapshot, vtensor_backend::source_snapshot()).unwrap();
        let recipe = if imported {
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
        let trace = |suffix: &str| expected_snapshot.with_extension(format!("snapshot.{suffix}"));
        let mut recipes = Vec::new();
        if let Some(recipe) = recipe {
            assert_eq!(recipe["cases"].as_u64().unwrap() as usize, expected_cases);
            recipes.push((trace(&format!("{mode}-recipes")), recipe));
        }
        if mode == "fill" {
            recipes.push((trace("fill-source-release"), &*FILL_RELEASE));
        }
        let mut command = Command::new(program);
        command.arg(&library).arg(&expected_snapshot).arg(mode);
        if let Some(model) = ORACLE_BINDINGS["fixture_libraries"][mode].as_str() {
            command.arg(fixture_library(common, model));
        }
        // The verified binaries are immutable. Each concurrent evaluator owns its trace paths.
        drop(build_guard);
        let text = super::ggml_oracle::batch(&mut command, input);
        let footer = format!("RESULT cases {expected_cases}\n");
        if let Some(digest) = ORACLE_BINDINGS["factory_transcripts"][mode].as_str() {
            let rows = text.strip_suffix(&footer).expect("complete factory count");
            assert_eq!(
                sha256_hex(rows.as_bytes()),
                digest,
                "factory ordinals or parameters differ from the pinned transcript"
            );
        } else {
            assert_eq!(text, footer);
        }
        for (path, recipe) in recipes {
            let bytes =
                std::fs::read(&path).expect("this execution must emit its fixture transcript");
            assert_eq!(recipe["bytes"], bytes.len(), "{mode} fixture length");
            let hash = sha256_hex(&bytes);
            assert_eq!(recipe["sha256"], hash, "{mode} fixture digest");
        }
        if matches!(mode, "matrix" | "elementwise") {
            let path = trace(&format!("{mode}-fixtures"));
            let bound = std::fs::read(&path).expect("native execution emitted its bound fixtures");
            assert!(
                bound == input[32..],
                "{mode} tensor bindings differ from the frozen fixture input"
            );
        }
        std::fs::remove_dir_all(scratch).unwrap();
    }

    macro_rules! graph_fixture_source { () => { r##"
static std::vector<unsigned char> tensor_bytes(const ggml_tensor *tensor) {
    std::vector<unsigned char> bytes(tensor ? ggml_nbytes(tensor) : 0);
    if (!bytes.empty()) ggml_backend_tensor_get(tensor,bytes.data(),0,bytes.size());
    return bytes;
}
struct graph_fixture {
    ggml_context_ptr arena;
    ggml_backend_buffer_ptr storage;
    explicit graph_fixture(size_t tensors, size_t graphs=1) : arena(ggml_init({graphs*ggml_graph_overhead()+tensors*ggml_tensor_overhead(),nullptr,true})) {}
    operator ggml_context *() const { return arena.get(); }
    ggml_cgraph *build(ggml_tensor *out) {
        auto graph=ggml_new_graph(arena.get());
        ggml_build_forward_expand(graph,out);
        return graph;
    }
    bool placed(ggml_cgraph *graph, ggml_backend_t backend) {
        ggml_backend_sched_ptr sched(ggml_backend_sched_new(&backend,nullptr,1,GGML_DEFAULT_GRAPH_SIZE,false,true));
        if (!sched) return false;
        ggml_backend_sched_split_graph(sched.get(),graph);
        for (int n=0;n<ggml_graph_n_nodes(graph);++n)
            if (ggml_backend_sched_get_tensor_backend(sched.get(),ggml_graph_node(graph,n))!=backend) return false;
        return true;
    }
    ggml_cgraph *allocate(ggml_tensor *out, ggml_backend_t backend, bool check_placement=false) {
        auto graph=build(out);
        if (check_placement && !(ggml_backend_supports_op(backend,out) && placed(graph,backend))) return nullptr;
        storage.reset(ggml_backend_alloc_ctx_tensors(arena.get(),backend));
        return storage ? graph : nullptr;
    }
};
"## }; }

    const UPSTREAM_SUITE: &str = concat!(
        r##"#define main vtensor_unused_upstream_main
#include "ggml-cpu.h"
#include "tests/test-backend-ops.cpp"
#undef main
"##,
        graph_fixture_source!(),
        r##"
size_t block_address(const ggml_tensor *, size_t);
bool compare_result(ggml_tensor *, ggml_tensor *, std::vector<unsigned char>, int);
struct vtensor_observation : printer {
  size_t records=0; bool bad=false; std::string label;
  void print_test_result(const test_result &result) override {
    ++records; label=result.op_name;
    if(!result.supported || !result.passed || result.test_mode!="test") {
      bad=true;fprintf(stderr,"imported case rejected: %s(%s): %s [backend: %s]\n",result.op_name.c_str(),result.op_params.c_str(),result.error_message.c_str(),result.backend_name.c_str());
    }
  }
};
// Both references run the original graph on the provider. The CPU either decodes
// the operand types or independently materializes their strided views.
static bool reference_graph_case(test_case & test, ggml_backend_t backend, ggml_backend_t cpu, ggml_op &op) {
    graph_fixture target(128), reference(128);
    if (!target || !reference) return false;
    auto out=test.build_graph(target);
    if (!out) return false;
    if (op==GGML_OP_NONE && (out->op==GGML_OP_CPY || out->op==GGML_OP_SET_ROWS)) op=out->op;
    if (out->op!=op || ggml_backend_supports_op(cpu,out) || !ggml_backend_supports_op(backend,out)) return false;
    auto original_graph=target.allocate(out,backend);
    if (!original_graph || !ggml_backend_supports_op(backend,out)) return false;
    test.initialize_tensors(target);
    auto original=op==GGML_OP_OUT_PROD || op==GGML_OP_CPY ? nullptr : test.build_graph(reference);
    if (op==GGML_OP_CPY || op==GGML_OP_SET_ROWS) {
        if (out->type!=GGML_TYPE_IQ3_XXS && out->type!=GGML_TYPE_IQ3_S && out->type!=GGML_TYPE_IQ2_S && out->type!=GGML_TYPE_IQ2_XS && out->type!=GGML_TYPE_IQ2_XXS && out->type!=GGML_TYPE_IQ1_S && out->type!=GGML_TYPE_IQ1_M) return false;
        int64_t width=out->ne[0], rows=ggml_nrows(out);
        if (width<=0 || width%ggml_blck_size(out->type) || !ggml_is_contiguous_rows(out)) return false;
        std::vector<float> importance(ggml_quantize_requires_imatrix(out->type) ? width : 0, 1.0f);
        auto values=tensor_to_float(out->src[0]);
        std::vector<float> mapping(rows,-1.f);
        if (op==GGML_OP_CPY) {
            if (uint64_t(rows)*width!=values.size()) return false;
            std::iota(mapping.begin(),mapping.end(),0.f);
        } else {
            auto dest=ggml_new_tensor_4d(reference,GGML_TYPE_F32,1,out->ne[1],out->ne[2],out->ne[3]);
            auto ids=ggml_new_tensor_4d(reference,GGML_TYPE_F32,1,out->src[0]->ne[1],out->src[0]->ne[2],out->src[0]->ne[3]);
            if (out->src[0]->ne[0]!=width || ggml_nelements(ids)>=1<<24) return false;
            auto mapped=ggml_set_rows(reference,dest,ids,original->src[1]);
            auto scatter=reference.allocate(mapped,cpu);
            if (!scatter || !ggml_backend_supports_op(cpu,mapped)) return false;
            ggml_backend_tensor_copy(out->src[1],original->src[1]);
            if (auto count=ggml_nelements(ids)) std::iota((float *)ids->data,(float *)ids->data+count,0.f);
            ggml_set_f32(dest,-1.f);
            if (ggml_backend_graph_compute(cpu,scatter)!=GGML_STATUS_SUCCESS) return false;
            mapping=tensor_to_float(mapped);
        }
        auto expected=tensor_bytes(out);
        size_t bytes=ggml_row_size(out->type,width);
        for (int64_t row=0;row<rows;++row) {
            if (mapping[row]==-1.f) continue;
            int64_t source=int64_t(mapping[row]);
            if (source<0 || float(source)!=mapping[row] || uint64_t(source+1)>values.size()/width) return false;
            size_t offset=block_address(out,row*(width/ggml_blck_size(out->type)));
            if (offset>expected.size() || bytes>expected.size()-offset ||
                ggml_quantize_chunk(out->type,values.data()+source*width,expected.data()+offset,0,1,width,importance.empty()?nullptr:importance.data())!=bytes) return false;
        }
        if (ggml_backend_graph_compute(backend,original_graph)!=GGML_STATUS_SUCCESS) return false;
        return compare_result(out,nullptr,std::move(expected),0);
    }
    ggml_tensor * expected;
    auto decoded=[&](int slot){return ggml_new_tensor(reference,GGML_TYPE_F32,4,out->src[slot]->ne);};
    if (op==GGML_OP_OUT_PROD) {
        expected=ggml_out_prod(reference,decoded(0),decoded(1));
    } else if (op==GGML_OP_GET_ROWS_BACK) {
        expected=ggml_get_rows_back(reference,decoded(0),original->src[1],original);
    } else if (op==GGML_OP_MUL_MAT) {
        if (original->src[1]->type!=GGML_TYPE_F16) return false;
        auto activation=ggml_cast(reference,original->src[1],GGML_TYPE_F32);
        if (!ggml_backend_supports_op(cpu,activation)) return false;
        expected=ggml_mul_mat(reference,original->src[0],activation);
        memcpy(expected->op_params,original->op_params,sizeof(expected->op_params));
    } else {
        auto kernel=ggml_cont(reference,original->src[0]);
        auto image=ggml_cont(reference,original->src[1]);
        auto p=original->op_params;
        expected=ggml_conv_2d_direct(reference,kernel,image,p[0],p[1],p[2],p[3],p[4],p[5]);
    }
    if (!ggml_are_same_shape(out,expected) || !ggml_backend_supports_op(cpu,expected)) return false;
    auto reference_graph=reference.allocate(expected,cpu);
    if (!reference_graph) return false;
    const int decoded_sources=op==GGML_OP_OUT_PROD ? 2 : (op==GGML_OP_GET_ROWS_BACK ? 1 : 0);
    for (int k=0;k<decoded_sources;++k) {
        auto values=tensor_to_float(out->src[k]);
        ggml_backend_tensor_set(expected->src[k],values.data(),0,ggml_nbytes(expected->src[k]));
    }
    if (op!=GGML_OP_OUT_PROD) {
        auto copy=ggml_get_first_tensor(reference);
        for (auto src=ggml_get_first_tensor(target);src;src=ggml_get_next_tensor(target,src)) {
            if (!copy || src->op!=copy->op || src->type!=copy->type || !ggml_are_same_shape(src,copy) || !ggml_are_same_stride(src,copy)) return false;
            if (src->op==GGML_OP_NONE && !src->view_src) ggml_backend_tensor_copy(src,copy);
            copy=ggml_get_next_tensor(reference,copy);
        }
    }
    if (ggml_backend_graph_compute(backend,original_graph)!=GGML_STATUS_SUCCESS ||
        ggml_backend_graph_compute(cpu,reference_graph)!=GGML_STATUS_SUCCESS) return false;
    auto actual=tensor_to_float(out),wanted=tensor_to_float(expected);
    if (actual.size()!=wanted.size()) return false;
    // An exactly matching all-padding output has zero reference energy and undefined NMSE.
    if (actual==wanted) return true;
    const double error=nmse(wanted.data(),actual.data(),wanted.size()), limit=test.max_nmse_err(backend);
    if (error<=limit) return true;
    fprintf(stderr,"[%s] ERR = %.9f > %.9f\n",ggml_op_name(op),error,limit);
    return false;
}
size_t vtensor_imported_suite(ggml_backend_t backend, ggml_backend_t reference, const char *op) {
    vtensor_observation observed;size_t cases=0;
    decltype(make_test_cases_eval()) tests;
    if (!strcmp(op,"POOL_HALF_INPUT")) {
      for (ggml_op_pool kind : {GGML_OP_POOL_AVG, GGML_OP_POOL_MAX}) {
        tests.emplace_back(new test_pool1d(kind, GGML_TYPE_F16, {7,3,2,2}, 3,2,3));
        tests.emplace_back(new test_pool2d(kind, GGML_TYPE_F16, {5,3,2,2}, 3,2,2,1,2,2));
      }
    } else if (!strcmp(op,"IM2COL_HALF_INPUT")) {
      for (bool planar : {false,true}) {
        const std::array<int64_t,4> input = planar ? std::array<int64_t,4>{7,5,2,3} : std::array<int64_t,4>{7,2,3,1};
        tests.emplace_back(new test_im2col(GGML_TYPE_F16,GGML_TYPE_F16,GGML_TYPE_F16,input,{3,2,2,1},2,1,1,0,1,1,planar));
      }
    }
    const bool supplemental=!tests.empty();
    std::vector<size_t> selected;
    if (!supplemental) {
      tests=make_test_cases_eval();
      size_t id;
      while (std::scanf("%zu", &id)==1) selected.push_back(id);
      if (!std::feof(stdin)) return SIZE_MAX;
    }
    std::vector<bool> seen(selected.empty()?0:tests.size());
    for (size_t i:selected) {
      if (i>=tests.size() || seen[i]) return SIZE_MAX;
      seen[i]=true;
    }
    if (!strncmp(op,"UPSCALE_",8)) {
      auto first=selected.empty() ? nullptr : dynamic_cast<test_interpolate *>(tests[selected.front()].get());
      if (!first) return SIZE_MAX;
      test_interpolate rounding(GGML_TYPE_F32,{14,5,3,2},{2,7,2,3},first->mode);
      vtensor_observation control;
      if (rounding.eval(backend,reference,nullptr,&control)!=test_status_t::OK || control.bad || control.records!=1) return SIZE_MAX;
    }
    for (size_t pos=0;pos<(selected.empty()?tests.size():selected.size());++pos) {
      const size_t i=selected.empty()?pos:selected[pos];
      auto &test=tests[i];
      const bool importance=!strcmp(op,"IMPORTANCE_THREE_STORAGE") || !strcmp(op,"IMPORTANCE_TWO_STORAGE") || !strcmp(op,"PARITY_GRID_STORAGE") || !strcmp(op,"GROUPED_PARITY_STORAGE") || !strcmp(op,"SHIFTED_BLOCK_STORAGE");
      auto operation= !strcmp(op,"GET_ROWS_BACK_DECODED") ? GGML_OP_GET_ROWS_BACK : !strcmp(op,"OUT_PROD_DECODED") ? GGML_OP_OUT_PROD : !strcmp(op,"MUL_MAT_HALF_ACTIVATIONS") ? GGML_OP_MUL_MAT : !strcmp(op,"CONV_2D_CONTIGUOUS") ? GGML_OP_CONV_2D : GGML_OP_NONE;
      if (importance || operation!=GGML_OP_NONE) {
        if (selected.empty() || !reference_graph_case(*test,backend,reference,operation)) {
          fprintf(stderr,"CPU reference rejected ordinal %zu: %s\n",i,test->vars().c_str());
          return SIZE_MAX;
        }
        observed.label=ggml_op_name(operation);
        ++observed.records;
      } else {
        const auto status=test->eval(backend,reference,(selected.empty() && !supplemental)?op:nullptr,&observed);
        if(status==test_status_t::SKIPPED && !supplemental) continue;
        if(status!=test_status_t::OK || observed.bad) return SIZE_MAX;
      }
      if(!selected.empty()) std::printf("CASE\t%zu\t%s\t%s\n",i,observed.label.c_str(),test->vars().c_str());
      ++cases;
    }
    if (!strcmp(op,"TRANSPOSE_CONV_1D")) {
      struct half_convolution : test_conv_transpose_1d {
        bool exact=false;
        ggml_tensor *build_graph(ggml_context *ctx) override {
          auto kernel=ggml_new_tensor_4d(ctx,GGML_TYPE_F16,3,2,4,1);
          auto image=ggml_new_tensor_4d(ctx,GGML_TYPE_F32,5,4,1,1);
          return ggml_conv_transpose_1d(ctx,kernel,image,2,0,1);
        }
        void initialize_tensors(ggml_context *ctx) override {
          test_case::initialize_tensors(ctx);
          if (exact) for (auto t=ggml_get_first_tensor(ctx);t;t=ggml_get_next_tensor(ctx,t)) {
            if (t->op!=GGML_OP_NONE || t->view_src) continue;
            for (int i=0;i<ggml_nelements(t);++i) ggml_set_f32_1d(t,i,float((i*5)%13-6)*0.25f);
          }
        }
        double max_nmse_err() override {return exact?0.0:test_conv_transpose_1d::max_nmse_err();}
      } half;
      vtensor_observation checked;
      for (bool exact : {false,true}) {
        half.exact=exact;
        if (half.eval(backend,reference,"CONV_TRANSPOSE_1D",&checked)!=test_status_t::OK || checked.bad) return SIZE_MAX;
      }
      if (checked.records!=2) return SIZE_MAX;
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
    vtensor_observation observed;
    return test.eval(backend,reference,op,&observed)==test_status_t::OK && !observed.bad && observed.records==1;
}
"##
    );

    const HEADER: &str = concat!(
        r##"#include "ggml-cpp.h"
#include <cinttypes>
#include "ggml-backend.h"
#include "ggml-backend-impl.h"
#include "ggml-alloc.h"
#include "ggml-cpu.h"
#include <cmath>
#include <algorithm>
#include <array>
#include <atomic>
#include <tuple>
#include <cstddef>
#include <cstdio>
#include <cstring>
#include <cstdint>
#include <fstream>
#include <iterator>
#include <filesystem>
#include <limits>
#include <numeric>
#include <map>
#include <set>
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
#define main vtensor_sleep_main
#define printf(...) std::fprintf(stderr,__VA_ARGS__)
"##,
        include_str!("../../../vendor/llama.cpp-oracle/tests/test-sleep.cpp"),
        r##"
#undef printf
#undef main
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
"##,
        graph_fixture_source!(),
        r##"
template<class T, class U>
static bool refuses_change(ggml_backend_t backend, ggml_backend_dev_t dev, ggml_cgraph *graph, ggml_tensor *result, T &field, U bad, bool capability=true) {
    ++refusal_probes;
    auto before=tensor_bytes(result);
    const T saved=field;field=bad;recipe_state(1);
    const bool refused=(!capability || !ggml_backend_dev_supports_op(dev,result)) && ggml_backend_graph_compute(backend,graph)==GGML_STATUS_FAILED;
    field=saved;
    return refused && before==tensor_bytes(result);
}
static bool refuses_poisoned_output(ggml_backend_t backend, ggml_cgraph *graph, ggml_tensor *storage) {
    const auto size=ggml_nbytes(storage);
    ggml_backend_tensor_memset(storage,0x33,0,size);
    return ggml_backend_graph_compute(backend,graph)==GGML_STATUS_FAILED && tensor_bytes(storage)==std::vector<uint8_t>(size,0x33);
}
static const decltype(&ggml_add) binary_nodes[][2]={{ggml_add,ggml_add_inplace},{ggml_sub,ggml_sub_inplace},{ggml_mul,ggml_mul_inplace},{ggml_div,ggml_div_inplace}};
static std::array<ggml_tensor *,3> scalar_cast_graph(ggml_context *ctx, ggml_type from, ggml_type to, int rows, bool padded) {
    auto backing=ggml_new_tensor_2d(ctx,from,padded?10:8,rows);
    auto input=padded?ggml_view_2d(ctx,backing,8,rows,backing->nb[1],ggml_type_size(from)):backing;
    auto storage=padded?ggml_new_tensor_2d(ctx,to,6,2*rows):ggml_cast(ctx,input,to);
    if (!padded) return {input,storage,storage};
    auto view=ggml_view_2d(ctx,storage,4,2*rows,storage->nb[1],ggml_type_size(to));
    return {input,storage,ggml_cpy(ctx,input,view)};
}
static ggml_tensor * canonical_copy(ggml_context *ctx, ggml_tensor *input) {
    const int64_t block=ggml_blck_size(input->type), count=ggml_nelements(input)/block;
    ggml_tensor *target=ggml_new_tensor_2d(ctx,input->type,block,count); target->nb[1]*=2; target->nb[2]=target->nb[1]*count; target->nb[3]=target->nb[2];
    return ggml_reshape_4d(ctx,ggml_cont(ctx,ggml_cpy(ctx,input,target)),input->ne[0],input->ne[1],input->ne[2],input->ne[3]);
}
size_t block_address(const ggml_tensor *tensor, size_t ordinal) {
    int64_t i0,i1,i2,i3;const int64_t block=ggml_blck_size(tensor->type);
    ggml_unravel_index(tensor,ordinal*block,&i0,&i1,&i2,&i3);
    return (i0/block)*tensor->nb[0]+i1*tensor->nb[1]+i2*tensor->nb[2]+i3*tensor->nb[3];
}
static std::vector<unsigned char> read_hex_bytes(size_t count) {
    std::vector<unsigned char> bytes(count);
    for (auto &byte:bytes) {
        unsigned value;
        if (scanf("%2x",&value)!=1) return {};
        byte=static_cast<unsigned char>(value);
    }
    return bytes;
}
static bool finite_f32_matches(const std::vector<unsigned char> &actual, const std::vector<unsigned char> &reference, bool exact, bool require_energy) {
    if (actual.size()!=reference.size() || actual.size()%sizeof(float)) return false;
    double error=0,energy=0;
    for(size_t i=0;i<actual.size();i+=sizeof(float)) {
        float value,ref; memcpy(&value,actual.data()+i,sizeof(value)); memcpy(&ref,reference.data()+i,sizeof(ref));
        if (!std::isfinite(value) || !std::isfinite(ref)) return false;
        double delta=double(value)-ref; error+=delta*delta; energy+=double(ref)*ref;
    }
    return (!exact || actual==reference) && (!require_energy || energy>0) && error<=5e-4*energy;
}
static void set_scalar_word(ggml_tensor *tensor, size_t element, uint64_t word) {
    std::array<uint8_t,8> bytes{};
    for (size_t j=0;j<tensor->nb[0];++j) bytes.at(j)=uint8_t(word>>(8*j));
    ggml_backend_tensor_set(tensor,bytes.data(),block_address(tensor,element),tensor->nb[0]);
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
bool compare_result(ggml_tensor *result, ggml_tensor *reference, std::vector<unsigned char> expected, int nan_mode) {
    auto actual=tensor_bytes(result), raw=tensor_bytes(reference);
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
"##
    );

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
    auto finish = [&](size_t cases) {
        ggml_backend_free(reference);
        ggml_backend_free(backend);
        printf("RESULT cases %zu\n", cases);
        return 0;
    };
    const bool pooling_gradient=!strcmp(argv[3],"pool-gradients");
    if (pooling_gradient || !strcmp(argv[3],"column-gradients")) {
      size_t cases=0;
      for (bool second : {false,true}) for (int v=0;v<(pooling_gradient?(second?20:17):32);++v) {
        bool plane=pooling_gradient || second;
        auto mode=second?GGML_OP_POOL_MAX:GGML_OP_POOL_AVG;
        int sx=1+(v&1), sy=plane?3-sx:0, px=(v>>1)&1, py=plane?px:0;
        int dx=1+((v>>2)&1), dy=plane?1:0, batches=1+((v>>(pooling_gradient?2:3))&1);
        std::vector<float> expected;
        for (auto selected : {reference,backend}) {
          graph_fixture ctx(6); CHECK(ctx);
          auto input_type=pooling_gradient && !second && v==16 ? GGML_TYPE_F16 : GGML_TYPE_F32;
          auto x=ggml_new_tensor_4d(ctx,input_type,7,plane?5:3,plane?3:batches,plane?batches:1);
          auto k=ggml_new_tensor_4d(ctx,(v&16)?GGML_TYPE_F16:GGML_TYPE_F32,3,plane?2:3,plane?3:2,plane?2:1);
          auto forward=pooling_gradient ? ggml_pool_2d(ctx,x,mode,2,2,sx,sy,px,py) : ggml_im2col(ctx,k,x,sx,sy,px,py,dx,dy,plane,GGML_TYPE_F32);
          auto grad=ggml_dup_tensor(ctx,forward);
          auto out=pooling_gradient ? ggml_pool_2d_back(ctx,grad,x,mode,2,2,sx,sy,px,py) : ggml_im2col_back(ctx,grad,k,x->ne,sx,sy,px,py,dx,dy,plane);
          CHECK(ggml_backend_supports_op(selected,out));
          auto graph=ctx.allocate(out,selected); CHECK(graph);
          if(pooling_gradient) for(int i=0;i<ggml_nelements(x);++i) {
            float value=(v&8)?1.f:float((i*7)%23-11)/4;
            const float edge[]={-std::numeric_limits<float>::max(), i%3==1?NAN:value, i%3?INFINITY:-INFINITY, i%2?-0.f:0.f};
            ggml_set_f32_1d(x,i,second && v>=16 ? edge[v-16] : value);
          }
          for(int i=0;i<ggml_nelements(grad);++i) ggml_set_f32_1d(grad,i,float((i*5)%17-8)/8);
          std::vector<float> actual(ggml_nelements(out),13.f);
          ggml_backend_tensor_set(out,actual.data(),0,ggml_nbytes(out));
          CHECK(ggml_backend_graph_compute(selected,graph)==GGML_STATUS_SUCCESS);
          ggml_backend_tensor_get(out,actual.data(),0,ggml_nbytes(out));
          if(selected==reference) expected=actual;
          else {
            CHECK(!std::memcmp(expected.data(),actual.data(),ggml_nbytes(out)));
            CHECK(refuses_change(backend,dev,graph,out,out->src[2],x));
            CHECK(refuses_change(backend,dev,graph,out,out->op_params[pooling_gradient?3:0],int32_t(0)));
          }
        }
        ++cases;
      }
      return finish(cases);
    }
    if (!strcmp(argv[3],"custom-callbacks")) {
        static_assert(GGML_DEFAULT_N_THREADS==4);
        struct observation {
            ggml_tensor *out=nullptr;
            std::array<ggml_tensor *,GGML_MAX_SRC> args{};
            std::atomic<unsigned> mask{0};
            std::atomic<int> width{0};
            static void apply(ggml_tensor *dst,int ith,int nth,void *user) {
                auto &s=*static_cast<observation *>(user);
                GGML_ASSERT(dst==s.out && ith>=0 && ith<nth && nth<=GGML_DEFAULT_N_THREADS);
                GGML_ASSERT(!(s.mask.fetch_or(1U<<ith)&(1U<<ith)));
                int prior=0; GGML_ASSERT(s.width.compare_exchange_strong(prior,nth) || prior==nth);
                GGML_ASSERT(std::equal(s.args.begin(),s.args.end(),dst->src));
                for(int i=ith;i<ggml_nelements(dst);i+=nth) {
                    float value=11.0f;
                    for(auto src:s.args) if(src) value+=ggml_get_f32_1d(src,i%ggml_nelements(src));
                    ggml_set_f32_1d(dst,i,value);
                }
            }
        };
        auto one=[](ggml_tensor *d,const ggml_tensor *a,int i,int n,void *u){GGML_ASSERT(a==d->src[0]);observation::apply(d,i,n,u);};
        auto two=[](ggml_tensor *d,const ggml_tensor *a,const ggml_tensor *b,int i,int n,void *u){GGML_ASSERT(a==d->src[0] && b==d->src[1]);observation::apply(d,i,n,u);};
        auto three=[](ggml_tensor *d,const ggml_tensor *a,const ggml_tensor *b,const ggml_tensor *c,int i,int n,void *u){GGML_ASSERT(a==d->src[0] && b==d->src[1] && c==d->src[2]);observation::apply(d,i,n,u);};
        using result = std::tuple<unsigned,int,std::array<float,17>>;
        std::vector<result> expected;
        for (auto selected : {reference,backend}) {
            auto registry=ggml_backend_dev_backend_reg(ggml_backend_get_device(selected));
            auto set_threads=reinterpret_cast<ggml_backend_set_n_threads_t>(ggml_backend_reg_get_proc_address(registry,"ggml_backend_set_n_threads")); CHECK(set_threads);
            size_t count=0;
            for(int limit : {0,1,2,4}) for(int request : {1,2,5,GGML_N_TASKS_MAX}) for(int mixed : {0,1,2})
            for(int op : {1,2,3,4}) for(int arity : (op==4?std::vector<int>{0,1,5,9}:std::vector<int>{op})) for(bool alias : {false,true}) {
                if(!arity && alias) continue;
                if(limit) set_threads(selected,limit);
                graph_fixture ctx(16); CHECK(ctx); observation s;
                for(int j=0;j<arity;++j) s.args[j]=ggml_new_tensor_1d(ctx,GGML_TYPE_F32,j?3+j:17);
                auto a=s.args[0], b=s.args[1], c=s.args[2];
                if(op==1) s.out=(alias?ggml_map_custom1_inplace:ggml_map_custom1)(ctx,a,one,request,&s);
                if(op==2) s.out=(alias?ggml_map_custom2_inplace:ggml_map_custom2)(ctx,a,b,two,request,&s);
                if(op==3) s.out=(alias?ggml_map_custom3_inplace:ggml_map_custom3)(ctx,a,b,c,three,request,&s);
                if(op==4) s.out=alias?ggml_custom_inplace(ctx,a,s.args.data()+1,arity-1,observation::apply,request,&s):ggml_custom_4d(ctx,GGML_TYPE_F32,17,1,1,1,s.args.data(),arity,observation::apply,request,&s);
                CHECK(ggml_backend_supports_op(selected,s.out));
                auto graph=ggml_new_graph(ctx);
                if(mixed==1) {auto x=ggml_new_tensor_1d(ctx,GGML_TYPE_F32,17);ggml_build_forward_expand(graph,ggml_add(ctx,x,x));}
                if(mixed==2) {
                    auto empty=[](ggml_tensor *,int,int,void *u){GGML_ASSERT(!u);};
                    ggml_build_forward_expand(graph,ggml_custom_4d(ctx,GGML_TYPE_F32,1,1,1,1,nullptr,0,empty,2,nullptr));
                }
                ggml_build_forward_expand(graph,s.out);
                ctx.storage.reset(ggml_backend_alloc_ctx_tensors(ctx,selected)); CHECK(ctx.storage);
                ggml_backend_buffer_clear(ctx.storage.get(),0); CHECK(!alias || s.out->data==a->data);
                for(int j=0;j<arity;++j) for(int i=0;i<ggml_nelements(s.args[j]);++i) ggml_set_f32_1d(s.args[j],i,float(i+j));
                if(selected==backend && !limit && request==1 && !mixed && !alias) {
                    CHECK(refuses_change(selected,dev,graph,s.out,s.out->src[GGML_MAX_SRC-1],s.out));
                    const auto params=s.out->op_params;
                    std::array<int32_t,GGML_MAX_OP_PARAMS/sizeof(int32_t)> saved; std::copy_n(params,saved.size(),saved.begin());
                    std::fill_n(s.out->op_params,sizeof(void*)/sizeof(int32_t),0);
                    CHECK(!ggml_backend_supports_op(selected,s.out) && ggml_backend_graph_compute(selected,graph)==GGML_STATUS_FAILED && !s.mask);
                    std::copy(saved.begin(),saved.end(),s.out->op_params);
                    for(int bad : {-2,0}) CHECK(refuses_change(selected,dev,graph,s.out,s.out->op_params[sizeof(void*)/sizeof(int32_t)],bad));
                }
                CHECK(ggml_backend_graph_compute(selected,graph)==GGML_STATUS_SUCCESS);
                std::array<float,17> words;
                ggml_backend_tensor_get(s.out,words.data(),0,sizeof(words));
                CHECK(s.width>0 && s.mask==(1U<<s.width)-1);
                result actual{s.mask.load(),s.width.load(),words};
                if(selected==reference) expected.push_back(actual); else CHECK(expected.at(count)==actual);
                ++count;
            }
            CHECK(count==expected.size());
        }
        return finish(expected.size());
    }
    ggml_backend_cpu_set_n_threads(reference,1);
    if (!strcmp(argv[3],"cpu-threadpool")) {
        std::array<std::array<float,17>,4> expected{};
        for(auto selected : {reference,backend}) {
            auto registry=ggml_backend_dev_backend_reg(ggml_backend_get_device(selected));
            auto lookup=[&](const char *name){return ggml_backend_reg_get_proc_address(registry,name);};
            auto create=reinterpret_cast<decltype(&ggml_threadpool_new)>(lookup("ggml_threadpool_new"));
            auto release=reinterpret_cast<decltype(&ggml_threadpool_free)>(lookup("ggml_threadpool_free"));
            auto attach=reinterpret_cast<decltype(&ggml_backend_cpu_set_threadpool)>(lookup("ggml_backend_cpu_set_threadpool"));
            auto set_threads=reinterpret_cast<ggml_backend_set_n_threads_t>(lookup("ggml_backend_set_n_threads"));
            CHECK(create && release && attach && set_threads);
            CHECK(!lookup("vtensor_unknown_threadpool_proc"));
            auto one=ggml_threadpool_params_default(1), two=ggml_threadpool_params_default(2);
            two.paused=true;
            if(selected==backend) {
                CHECK(!create(nullptr));
                for(int n : {-1,0,GGML_MAX_N_THREADS+1}) {
                    auto invalid=one;invalid.n_threads=n;CHECK(!create(&invalid));
                }
                auto invalid=one;invalid.poll=101;CHECK(!create(&invalid));
            }
            auto first=create(&one), second=create(&two); CHECK(first && second && first!=second);
            graph_fixture ctx(8); CHECK(ctx);
            auto input=ggml_new_tensor_1d(ctx,GGML_TYPE_F32,32);
            auto weights=ggml_new_tensor_2d(ctx,GGML_TYPE_Q8_0,32,17);
            auto output=ggml_mul_mat(ctx,weights,input); auto graph=ctx.allocate(output,selected); CHECK(graph);
            std::array<float,32*17> original{};
            for(size_t i=0;i<original.size();i++) original[i]=float(int(i/32)+1)*float(int(i%7)-3);
            std::vector<uint8_t> encoded(ggml_nbytes(weights));
            CHECK(ggml_quantize_chunk(GGML_TYPE_Q8_0,original.data(),encoded.data(),0,17,32,nullptr)==encoded.size());
            ggml_backend_tensor_set(weights,encoded.data(),0,encoded.size());
            const std::array<ggml_threadpool_t,4> bindings={first,second,first,nullptr};
            for(size_t step=0;step<bindings.size();step++) {
                attach(selected,bindings[step]);
                set_threads(selected,step==0 || step==2 ? 1 : 2);
                std::array<float,32> values{};std::array<float,17> result{};
                for(size_t i=0;i<values.size();i++) values[i]=float(int(i%5)-2)*float(step+1);
                ggml_backend_tensor_set(input,values.data(),0,sizeof(values));
                CHECK(ggml_backend_graph_compute(selected,graph)==GGML_STATUS_SUCCESS);
                ggml_backend_tensor_get(output,result.data(),0,sizeof(result));
                if(selected==reference) expected[step]=result;
                else CHECK(result==expected[step]);
            }
            release(first);release(second);release(nullptr);

        }
        return finish(expected.size());
    }
    const bool bfloat_products=!strcmp(argv[3],"bfloat-products");
    const bool explicit_precision=!strcmp(argv[3],"prepacked-precision");
    if (explicit_precision || bfloat_products || !strcmp(argv[3],"prepacked-products")) {
        int cases; CHECK(scanf("%d",&cases)==1 && cases==(bfloat_products?8:50));
        size_t compared=0;
        for (int i=0;i<cases;++i) {
            int exact=0,expert,type,activation,width;CHECK(scanf("%d%d%d%d",&expert,&type,&activation,&width)==4);
            if (bfloat_products) CHECK(scanf("%d",&exact)==1 && (exact==0 || exact==1));
            graph_fixture ctx(8);CHECK(ctx);
            auto x=ggml_new_tensor_3d(ctx,ggml_type(type),width,3,expert?2:1);
            auto y=ggml_new_tensor_3d(ctx,ggml_type(activation),width,expert?1:2,expert?2:1);
            auto ids=ggml_new_tensor_2d(ctx,GGML_TYPE_I32,1,2);
            auto out=expert?ggml_mul_mat_id(ctx,x,y,ids):ggml_mul_mat(ctx,x,y);
            auto graph=ctx.build(out);
            auto weights=read_hex_bytes(ggml_nbytes(x)),inputs=read_hex_bytes(ggml_nbytes(y)),expected=read_hex_bytes(ggml_nbytes(out));
            CHECK(!weights.empty() && !inputs.empty() && expected.size()==6*sizeof(float));
            for (bool execute : {false,true}) {
                if (execute) {
                    ctx.storage.reset(ggml_backend_alloc_ctx_tensors(ctx,backend));CHECK(ctx.storage);
                    ggml_backend_tensor_set(x,weights.data(),0,weights.size());ggml_backend_tensor_set(y,inputs.data(),0,inputs.size());
                    set_scalar_word(ids,0,1);set_scalar_word(ids,1,0);
                }
                for (auto acc : {GGML_PREC_UNDEFINED,GGML_PREC_F32,GGML_PREC_BF16,GGML_PREC_F16}) {
                    for (auto src : {GGML_PREC_UNDEFINED,GGML_PREC_F32,GGML_PREC_BF16,GGML_PREC_F16,GGML_PREC_Q8,GGML_PREC_Q4}) {
                        if ((acc!=GGML_PREC_UNDEFINED || src!=GGML_PREC_UNDEFINED)!=explicit_precision) continue;
                        CHECK(ggml_prec_set_acc(out,acc) && ggml_prec_set_src(out,src,1));
                        if (!execute) {CHECK(ggml_backend_supports_op(backend,out) && ctx.placed(graph,backend));continue;}
                        ggml_backend_tensor_memset(out,0xff,0,ggml_nbytes(out));
                        CHECK(ggml_backend_graph_compute(backend,graph)==GGML_STATUS_SUCCESS);
                        CHECK(finite_f32_matches(tensor_bytes(out),expected,exact,false));
                        ++compared;
                    }
                }
            }
        }
        return finish(compared);
    }
    const bool flash_precision=!strcmp(argv[3],"flash-precision");
    if (flash_precision || !strcmp(argv[3],"flash-storage")) {
        std::map<std::pair<int,int>,std::vector<unsigned char>> samples;
        int count; CHECK(scanf("%d",&count)==1 && count==59);
        for(int i=0;i<count;++i) {
            int slot,kind; size_t size; CHECK(scanf("%d%d%zu",&slot,&kind,&size)==3);
            auto bytes=read_hex_bytes(size); CHECK(!bytes.empty() && samples.emplace(std::make_pair(slot,kind),std::move(bytes)).second);
        }
        size_t cases=0; int key_type,value_type;
        std::set<std::pair<int,int>> pairs;
        while(scanf("%d%d",&key_type,&value_type)==2) CHECK(pairs.emplace(key_type,value_type).second);
        CHECK(feof(stdin) && pairs.size()==697);
        if (flash_precision) {
            for(const auto &k:samples) if(k.first.first==1)
                for(const auto &v:samples) if(v.first.first==2) pairs.emplace(k.first.second,v.first.second);
            CHECK(pairs.size()==841);
        }
        for(auto [key_type,value_type]:pairs) {
            graph_fixture ctx(6); CHECK(ctx);
            auto q=ggml_new_tensor_2d(ctx,GGML_TYPE_F32,256,2);
            auto k=ggml_new_tensor_2d(ctx,ggml_type(key_type),256,3),v=ggml_new_tensor_2d(ctx,ggml_type(value_type),256,3);
            auto out=ggml_flash_attn_ext(ctx,q,k,v,nullptr,0.0625f,0,0);
            auto graph=ctx.build(out);
            std::vector<ggml_prec> modes=flash_precision?std::vector<ggml_prec>{GGML_PREC_F32,GGML_PREC_BF16,GGML_PREC_F16,GGML_PREC_Q8,GGML_PREC_Q4}:std::vector<ggml_prec>{GGML_PREC_UNDEFINED};
            for(auto prec:modes) {
                CHECK(ggml_prec_set_acc(out,prec) && ggml_prec_get_acc(out)==prec);
                const bool legal=prec!=GGML_PREC_Q8 && prec!=GGML_PREC_Q4;
                CHECK(ggml_backend_supports_op(backend,out)==legal);
                if(legal) CHECK(ctx.placed(graph,backend));
            }
            CHECK(ggml_prec_set_acc(out,flash_precision?GGML_PREC_F32:GGML_PREC_UNDEFINED));
            ctx.storage.reset(ggml_backend_alloc_ctx_tensors(ctx,backend)); CHECK(ctx.storage);
            int slot=0;
            for(auto t:{q,k,v}) {
                const auto &bytes=samples.at({slot++,t->type}); CHECK(bytes.size()==ggml_nbytes(t));
                ggml_backend_tensor_set(t,bytes.data(),0,bytes.size());
            }
            CHECK(ggml_backend_graph_compute(reference,graph)==GGML_STATUS_SUCCESS);
            const auto expected=tensor_bytes(out); CHECK(expected.size()==512*sizeof(float));
            for(auto prec:modes) {
                ggml_backend_tensor_memset(out,0xff,0,expected.size());
                CHECK(ggml_prec_set_acc(out,prec));
                if(prec==GGML_PREC_Q8 || prec==GGML_PREC_Q4) {
                    auto before=tensor_bytes(out); CHECK(ggml_backend_graph_compute(backend,graph)==GGML_STATUS_FAILED && before==tensor_bytes(out));
                    ++cases; continue;
                }
                CHECK(ggml_backend_graph_compute(backend,graph)==GGML_STATUS_SUCCESS);
                if(!finite_f32_matches(tensor_bytes(out),expected,false,true)) {fprintf(stderr,"flash attention differs for %d / %d\n",key_type,value_type);return 2;}
                ++cases;
            }
        }
        return finish(cases);
    }
    if (!strcmp(argv[3],"e4m3-repeat")) {
        size_t cases=0;
        for (int period : {1,8,16}) for (int padded : {0,1}) for (int inplace : {0,1}) {
            std::vector<unsigned char> expected;
            for (auto selected : {reference,backend}) {
                graph_fixture ctx(8); CHECK(ctx);
                auto x=ggml_new_tensor_4d(ctx,GGML_TYPE_F8_E4M3,16,4,4,2);
                auto storage=ggml_new_tensor_4d(ctx,GGML_TYPE_F32,period+2*padded,2,2,1);
                auto y=ggml_view_4d(ctx,storage,period,2,2,1,storage->nb[1],storage->nb[2],storage->nb[3],padded*sizeof(float));
                auto rhs=selected==reference?ggml_repeat_4d(ctx,y,16,4,4,2):y;
                auto out=inplace?ggml_add_inplace(ctx,x,rhs):ggml_add(ctx,x,rhs);
                auto graph=ctx.allocate(out,selected,selected==backend); CHECK(graph);
                std::vector<unsigned char> bytes(ggml_nbytes(x));
                for (size_t i=0;i<bytes.size();++i) bytes[i]=uint8_t((i%2?128:0)+(19*i+3)%112);
                ggml_backend_tensor_set(x,bytes.data(),0,bytes.size());
                for (int i=0;i<ggml_nelements(storage);++i) ggml_set_f32_1d(storage,i,float((13*i+7)%29-14)/8.f);
                CHECK(ggml_backend_graph_compute(selected,graph)==GGML_STATUS_SUCCESS);
                if (selected==reference) expected=tensor_bytes(out);
                else CHECK(tensor_bytes(out)==expected);
            }
            ++cases;
        }
        return finish(cases);
    }
    const bool lane_repeat=!strcmp(argv[3],"packed-lanes");
    const bool broadcast_add=lane_repeat || !strcmp(argv[3],"packed-broadcast");
    const bool cast_add=!strcmp(argv[3],"codebook-add") || !strcmp(argv[3],"half-add");
    const bool views=!strcmp(argv[3],"row-repeat");
    const bool row_binary=views || !strcmp(argv[3],"scalar-binary") || !strcmp(argv[3],"packed-binary");
    const bool numeric_results=!strcmp(argv[3],"numeric-results");
    const bool mixed_cast=numeric_results || !strcmp(argv[3],"mixed-cast");
    const bool paired=row_binary || mixed_cast;
    if (paired || cast_add || broadcast_add || !strcmp(argv[3],"packed-add")) {
        std::map<std::pair<int,int>,std::vector<unsigned char>> sources;
        int n=0; CHECK(scanf("%d",&n)==1 && n>0);
        for (int i=0;i<n;++i) {
            int type,width; CHECK(scanf("%d%d",&type,&width)==2 && width>0);
            auto &bytes=sources[{type,width}]; CHECK(bytes.empty());
            bytes=read_hex_bytes(2*ggml_row_size(ggml_type(type),width)); CHECK(!bytes.empty());
        }
        int pairs; CHECK(scanf("%d",&pairs)==1 && pairs>0);
        size_t cases=0;
        for (int p=0;p<pairs;++p) {
            int from,to,width; CHECK(scanf("%d%d%d",&from,&to,&width)==3);
            int right_type=GGML_TYPE_F32,kind=0;
            if (paired) CHECK(scanf("%d%d",&right_type,&kind)==2 && (mixed_cast?kind==0:from==to && kind>=0 && kind<4));
            auto &data=sources.at({from,width});
            const int columns=views?2*width:width;
            auto expected=read_hex_bytes((views?16:2)*ggml_row_size(ggml_type(to),columns)); CHECK(!expected.empty());
            const int rows=views || broadcast_add?4:2, planes=views?2:broadcast_add?4:1, batches=views || broadcast_add?2:1;
            if (lane_repeat && from!=to) continue;
            const int periods[]={lane_repeat?1:width,width/2};
            for (int period_index=0;period_index<(lane_repeat?2:1);++period_index) {
                const int period=periods[period_index];
                const int first=broadcast_add?-1:0;
                for (int inplace=first;inplace<=(from==to);++inplace) {
                    const bool oracle=inplace<0;
                    auto selected=oracle?reference:backend;
                    graph_fixture ctx(12); CHECK(ctx);
                    auto make=[&](ggml_type ty,int n0,int n1,int n2,int n3) {
                        if (!views) return ggml_new_tensor_4d(ctx,ty,n0,n1,n2,n3);
                        auto backing=ggml_new_tensor_4d(ctx,ty,n0+ggml_blck_size(ty),n1+1,n2+1,n3);
                        return ggml_view_4d(ctx,backing,n0,n1,n2,n3,backing->nb[1],backing->nb[2],backing->nb[3],ggml_type_size(ty));
                    };
                    auto x=make(ggml_type(from),columns,rows,planes,batches);
                    auto y=make(ggml_type(right_type),period,2,broadcast_add?2:1,1);
                    auto rhs=oracle?ggml_repeat_4d(ctx,y,width,rows,planes,batches):y;
                    auto out=row_binary || inplace>0 || lane_repeat?binary_nodes[kind][inplace>0](ctx,x,rhs):ggml_add_cast(ctx,x,rhs,ggml_type(to));
                    if (!inplace && !kind && from==to && !lane_repeat && !views && (ggml_is_quantized(ggml_type(from)) || from==GGML_TYPE_F16 || from==GGML_TYPE_BF16)) {
                        auto ordinary=row_binary?ggml_add_cast(ctx,x,y,ggml_type(to)):ggml_add(ctx,x,y);
                        CHECK(ordinary->op==out->op && ordinary->type==out->type && ggml_are_same_shape(ordinary,out) && ggml_are_same_stride(ordinary,out));
                        CHECK(std::equal(std::begin(out->src),std::end(out->src),ordinary->src) && !memcmp(ordinary->op_params,out->op_params,sizeof(out->op_params)) && ordinary->flags==out->flags && !ordinary->view_src && !out->view_src);
                        ++cases;
                    }
                    if (oracle) CHECK(ggml_backend_supports_op(selected,out));
                    auto graph=ctx.allocate(out,selected,!oracle); CHECK(graph);
                    if (views) ggml_backend_buffer_clear(ctx.storage.get(),0xa5);
                    const size_t row_bytes=data.size()/2;
                    auto row_at=[](ggml_tensor *t,size_t row){ return block_address(t,row*(t->ne[0]/ggml_blck_size(t->type))); };
                    for (int row=0;row<ggml_nrows(x);++row) for (int half=0;half<columns/width;++half) {
                        int sample=(row+row/rows+row/(rows*planes)+half)%2;
                        ggml_backend_tensor_set(x,data.data()+sample*row_bytes,row_at(x,row)+half*row_bytes,row_bytes);
                    }
                    if (paired) {
                        const auto &right=sources.at({right_type,width});
                        auto stride=right.size()/2;
                        for (size_t row=0;row<2;++row) ggml_backend_tensor_set(y,right.data()+(1-row)*stride,row_at(y,row),stride);
                    } else for (int i=0;i<ggml_nelements(y);++i) ggml_set_f32_1d(y,i,(float((13*i+7)%29)-14.f)/32.f);
                    std::vector<unsigned char> left_backing,right_backing;
                    if (views) {
                        left_backing=tensor_bytes(x->view_src); right_backing=tensor_bytes(y->view_src);
                        if (inplace) for (int row=0;row<ggml_nrows(x);++row)
                            memcpy(left_backing.data()+x->view_offs+row_at(x,row),expected.data()+2*row_bytes*row,2*row_bytes);
                    }
                    CHECK(ggml_backend_graph_compute(selected,graph)==GGML_STATUS_SUCCESS);
                    if (oracle) {expected=tensor_bytes(out);continue;}
                    if (views) CHECK(tensor_bytes(x->view_src)==left_backing && tensor_bytes(y->view_src)==right_backing);
                    if (!(views && inplace) && tensor_bytes(out)!=expected) {fprintf(stderr,"packed addition differs for %d -> %d inplace %d right %d operation %d\n",from,to,inplace,right_type,kind);return 2;}
                    if (numeric_results && from==GGML_TYPE_F16 && right_type==GGML_TYPE_F32 && to>=GGML_TYPE_I8 && to<=GGML_TYPE_I64) {
                        for (float invalid : {NAN,INFINITY,-INFINITY,to==GGML_TYPE_I64?0x1p63f:0x1p31f}) {
                            ggml_set_f32_1d(y,ggml_nelements(y)-1,invalid);
                            CHECK(refuses_poisoned_output(backend,graph,out)); ++cases;
                        }
                    }
                    ++cases;
                }
            }
        }
        return finish(cases);
    }
    const bool integer_rows=!strcmp(argv[3],"integer-rows");
    if (integer_rows || !strcmp(argv[3],"activation-q8")) {
        size_t cases=0;
        for (auto to : {GGML_TYPE_Q8_1,GGML_TYPE_Q8_K,GGML_TYPE_I32}) for (auto from : {GGML_TYPE_F32,GGML_TYPE_F16,GGML_TYPE_BF16}) {
            if (integer_rows!=(to==GGML_TYPE_I32) || (integer_rows && from==GGML_TYPE_BF16)) continue;
            const auto width=integer_rows?8:ggml_blck_size(to);
            for (int variant=integer_rows?2:0;variant<(integer_rows?6:from==GGML_TYPE_BF16?2:4);++variant) {
                const bool padded=variant==1 || variant>=4;
                std::vector<unsigned char> expected;
                for (auto selected : {reference,backend}) {
                    graph_fixture ctx(12); CHECK(ctx);
                    auto src=ggml_new_tensor_2d(ctx,from,width,2);
                    auto dst=ggml_new_tensor_2d(ctx,to,padded?2*width:width,3);
                    auto view=ggml_view_2d(ctx,dst,width,variant>=4?3:2,dst->nb[1],padded?ggml_type_size(to):0);
                    auto idx=ggml_new_tensor_1d(ctx,variant%2?GGML_TYPE_I64:GGML_TYPE_I32,2);
                    auto out=variant>=2?ggml_set_rows(ctx,variant>=4?view:dst,src,idx):variant==1 && selected==backend?ggml_cpy(ctx,src,view):ggml_cast(ctx,src,to);
                    auto graph=ctx.allocate(out,selected,selected==backend); CHECK(graph);
                    ggml_backend_buffer_clear(ctx.storage.get(),0xa5);
                    for (int i=0;i<2*width;++i) ggml_set_f32_1d(src,i,float((i*13)%31-15)/8);
                    if (integer_rows) {
                        ggml_set_f32_1d(src,0,from==GGML_TYPE_F16?65504.f:2147483520.f);
                        ggml_set_f32_1d(src,1,from==GGML_TYPE_F16?-65504.f:-2147483648.f);
                    }
                    set_scalar_word(idx,0,2); set_scalar_word(idx,1,0);
                    CHECK(ggml_backend_graph_compute(selected,graph)==GGML_STATUS_SUCCESS);
                    auto storage=variant==0?out:dst;
                    if (selected==reference) {
                        expected=tensor_bytes(storage);
                        if (variant==1) for (int row=0;row<2;++row)
                            ggml_backend_tensor_get(out,expected.data()+view->view_offs+row*view->nb[1],row*out->nb[1],out->nb[1]);
                    }
                    else {
                        CHECK(tensor_bytes(storage)==expected);
                        if (integer_rows) {
                            set_scalar_word(idx,1,3);
                            CHECK(ggml_backend_graph_compute(backend,graph)==GGML_STATUS_FAILED && tensor_bytes(storage)==expected);
                            set_scalar_word(idx,1,0);
                        }
                        ggml_set_f32_1d(src,2*width-1,NAN);
                        CHECK(refuses_poisoned_output(backend,graph,storage));
                    }
                }
                ++cases;
            }
        }
        return finish(cases);
    }
    const bool binary_widths=!strcmp(argv[3],"binary-widths");
    const bool signed64_floats=!strcmp(argv[3],"signed64-floats");
    const bool narrow=!strcmp(argv[3],"wide-narrow");
    const bool staged=narrow || !strcmp(argv[3],"wide-staged");
    const bool integer_mixed=!strcmp(argv[3],"integer-floats");
    const bool small=integer_mixed || !strcmp(argv[3],"integer-widths") || !strcmp(argv[3],"octet-integers");
    if (small || staged || binary_widths || signed64_floats || !strcmp(argv[3],"signed64-widths")) {
        size_t compared=0;
        for (int pair=0;pair<(integer_mixed?18:small?6:narrow?7:staged?21:binary_widths?2:signed64_floats?3:6);++pair) {
            int from=pair==0?GGML_TYPE_F32:GGML_TYPE_F64, to=pair==0?GGML_TYPE_F64:GGML_TYPE_F32;
            if (!binary_widths) CHECK(scanf("%d %d",&from,&to)==2);
            int invalid_count=(from==GGML_TYPE_F64 && to==GGML_TYPE_I64)?6:0;
            if (staged || small) CHECK(scanf("%d",&invalid_count)==1 && invalid_count>=0 && invalid_count<=64);
            std::vector<uint64_t> inputs(small?16:64), expected(inputs.size());
            for (size_t i=0;i<inputs.size();++i) CHECK(scanf("%" SCNx64 " %" SCNx64,&inputs[i],&expected[i])==2);
            std::vector<uint64_t> invalid(invalid_count);
            for (auto &word:invalid) CHECK(scanf("%" SCNx64,&word)==1);
            for (bool padded : {false,true}) {
                if (binary_widths && padded) continue;
                graph_fixture target(8); CHECK(target);
                auto [input,out,result]=scalar_cast_graph(target,ggml_type(from),ggml_type(to),inputs.size()/8,padded);
                auto graph=target.allocate(result,backend,true);
                CHECK(graph && ggml_backend_supports_op(backend,result));
                ggml_backend_buffer_clear(target.storage.get(),0xa5);
                for (size_t i=0;i<inputs.size();++i) set_scalar_word(input,i,inputs[i]);
                CHECK(ggml_backend_graph_compute(backend,graph)==GGML_STATUS_SUCCESS);
                auto actual=tensor_bytes(out);
                std::vector<uint8_t> oracle(actual.size(),0xa5);
                for (size_t i=0;i<expected.size();++i) {
                    size_t offset=(padded?ggml_type_size(ggml_type(to)):0)+block_address(result,i);
                    uint64_t word=0;
                    for (size_t j=0;j<result->nb[0];++j) word|=uint64_t(actual.at(offset+j))<<(8*j);
                    const auto nan=[&](uint64_t x) {
                        switch(to) {
                            case GGML_TYPE_F64: return (x&0x7fffffffffffffffULL)>0x7ff0000000000000ULL;
                            case GGML_TYPE_F32: return (x&0x7fffffff)>0x7f800000;
                            case GGML_TYPE_F16: return (x&0x7fff)>0x7c00;
                            case GGML_TYPE_BF16: return (x&0x7fff)>0x7f80;
                            default: return false;
                        }
                    };
                    CHECK(word==expected[i] || ((binary_widths || staged) && nan(word) && nan(expected[i])));
                    for (size_t j=0;j<result->nb[0];++j) oracle.at(offset+j)=actual.at(offset+j);
                    if (!small) ++compared;
                }
                CHECK(oracle==actual);
                for (uint64_t word:invalid) {
                    set_scalar_word(input,inputs.size()-1,word);
                    CHECK(refuses_poisoned_output(backend,graph,out));
                    if (!small) ++compared;
                }
                if (small) ++compared;
            }
        }
        return finish(compared);
    }
    if (!strcmp(argv[3],"scalar-diagonal")) {
        size_t cases=0;
        int kind;
        while (scanf("%d",&kind)==1) {
            const auto type=ggml_type(kind);
            if (ggml_blck_size(type)!=1 || type==GGML_TYPE_F32 || type==GGML_TYPE_F16 || type==GGML_TYPE_BF16) continue;
            const auto bytes=ggml_type_size(type);
            for (int width : {1,4,7}) for (int pad : {0,2}) {
                graph_fixture target(8), cpu(4); CHECK(target && cpu);
                auto backing=ggml_new_tensor_4d(target,type,width+pad,1,2,3);
                auto src=pad ? ggml_view_4d(target,backing,width,1,2,3,backing->nb[1],backing->nb[2],backing->nb[3],bytes) : backing;
                auto out=ggml_diag(target,src);
                auto lanes=ggml_new_tensor_4d(cpu,GGML_TYPE_F16,width,1,2,3*bytes);
                auto oracle=ggml_diag(cpu,lanes);
                CHECK(ggml_backend_supports_op(backend,out));
                auto graph=target.allocate(out,backend), reference_graph=cpu.allocate(oracle,reference);
                CHECK(graph && reference_graph && ggml_backend_supports_op(backend,out));
                const size_t inputs=ggml_nelements(src), outputs=ggml_nelements(out);
                std::vector<uint16_t> planes(inputs*bytes);
                std::vector<uint8_t> input(inputs*bytes);
                for (size_t i=0;i<inputs;++i) for (size_t b=0;b<bytes;++b)
                    planes[b*inputs+i]=input[i*bytes+b]=uint8_t(1+((i*37+b*71)%255));
                for (int row=0;row<6;++row) ggml_backend_tensor_set(src,input.data()+row*width*bytes,row*src->nb[2],width*bytes);
                ggml_backend_tensor_set(lanes,planes.data(),0,ggml_nbytes(lanes));
                CHECK(ggml_backend_graph_compute(reference,reference_graph)==GGML_STATUS_SUCCESS);
                CHECK(ggml_backend_graph_compute(backend,graph)==GGML_STATUS_SUCCESS);
                planes.resize(outputs*bytes); ggml_backend_tensor_get(oracle,planes.data(),0,ggml_nbytes(oracle));
                const auto actual=tensor_bytes(out);
                for (size_t i=0;i<outputs;++i) for (size_t b=0;b<bytes;++b) CHECK(actual[i*bytes+b]==planes[b*outputs+i]);
                ++cases;
            }
        }
        return finish(cases);
    }
    const bool shared_host=!strcmp(argv[3],"shared-host-storage");
    const bool identity_copy=shared_host || !strcmp(argv[3],"iq2-identity-copy");
    if (identity_copy || !strcmp(argv[3],"storage-repeat")) {
      if (shared_host) CHECK(ggml_backend_supports_buft(backend,ggml_backend_get_default_buffer_type(reference)));
      size_t cases=0;
      int kind;
      while (scanf("%d",&kind)==1) {
        const auto type=ggml_type(kind);
        if (identity_copy && type!=GGML_TYPE_IQ2_XXS) continue;
        for (int axis=0;axis<(identity_copy?1:5);++axis) {
          std::vector<uint8_t> expected;
          for (auto selected : {reference,backend}) {
            graph_fixture ctx(12); CHECK(ctx);
            auto src=ggml_new_tensor_4d(ctx,type,identity_copy?256:2*ggml_blck_size(type),2,3,4);
            ggml_set_param(src);
            auto out=src;
            if (identity_copy) out=ggml_cpy(ctx,src,ggml_dup_tensor(ctx,src));
            else if (selected==reference) {
              for (int dim=0;dim<4;++dim) if (axis==dim || axis==4) out=ggml_concat(ctx,out,out,dim);
            } else {
              std::array<int64_t,4> shape;
              for (int dim=0;dim<4;++dim) shape[dim]=src->ne[dim]*(1+(axis==dim || axis==4));
              out=ggml_repeat_4d(ctx,src,shape[0],shape[1],shape[2],shape[3]);
            }
            const bool cpu_identity=identity_copy && selected==reference; // GGML executes this copy despite its device query.
            CHECK(cpu_identity || ggml_backend_supports_op(selected,out));
            auto graph=ctx.allocate(out,shared_host ? reference : selected); CHECK(graph && (cpu_identity || ggml_backend_supports_op(selected,out)));
            std::vector<uint8_t> data(ggml_nbytes(src));
            for(size_t i=0;i<data.size();++i) data[i]=uint8_t(i*197+19);
            ggml_backend_tensor_set(src,data.data(),0,data.size());
            ggml_backend_tensor_memset(out,0,0,ggml_nbytes(out));
            CHECK(ggml_backend_graph_compute(selected,graph)==GGML_STATUS_SUCCESS);
            if (selected==reference) expected=tensor_bytes(out);
            else CHECK(tensor_bytes(out)==expected);
            if (identity_copy) CHECK(expected==data);
          }
          ++cases;
        }
      }
      return finish(cases);
    }
    const bool planned=!strcmp(argv[3],"graph-plans");
    if (planned || !strcmp(argv[3],"scheduled-flags")) {
        std::array<std::array<float,2>,3> expected{};
        for (auto selected : {reference, backend}) {
            graph_fixture ctx(3); CHECK(ctx);
            auto input = ggml_new_tensor_1d(ctx, GGML_TYPE_F32, 1);
            auto intermediate = ggml_scale(ctx, input, 2.0f);
            auto result = ggml_add(ctx, intermediate, input);
            ggml_set_input(input); ggml_set_output(intermediate); ggml_set_output(result);
            auto graph = ggml_new_graph(ctx); ggml_build_forward_expand(graph, result);
            CHECK(!input->data && ggml_backend_supports_op(selected,intermediate) && ggml_backend_supports_op(selected,result));
            ggml_backend_sched_ptr sched(ggml_backend_sched_new(&selected, nullptr, 1, GGML_DEFAULT_GRAPH_SIZE, false, true)); CHECK(sched);
            CHECK(ggml_backend_sched_alloc_graph(sched.get(), graph));
            for (auto node : {intermediate,result}) CHECK(ggml_backend_sched_get_tensor_backend(sched.get(),node)==selected);
            ggml_backend_graph_plan_t plan=nullptr;
            if (planned) {
                CHECK(selected->iface.graph_plan_create && selected->iface.graph_plan_compute && selected->iface.graph_plan_free);
                plan=ggml_backend_graph_plan_create(selected,graph); CHECK(plan);
                ggml_graph_clear(graph); // Both plans retain the submitted topology.
            }
            for (size_t pass=0; pass<expected.size(); ++pass) {
                const float value = float(pass+3), canary = -17.0f;
                if (pass==1) intermediate->flags &= ~GGML_TENSOR_FLAG_COMPUTE;
                else intermediate->flags |= GGML_TENSOR_FLAG_COMPUTE;
                ggml_backend_tensor_set(input,&value,0,sizeof(value));
                ggml_backend_tensor_set(intermediate,&canary,0,sizeof(canary));
                CHECK((planned ? ggml_backend_graph_plan_compute(selected,plan) : ggml_backend_sched_graph_compute(sched.get(),graph))==GGML_STATUS_SUCCESS);
                ggml_backend_sched_synchronize(sched.get());
                std::array<float,2> observed;
                ggml_backend_tensor_get(intermediate,&observed[0],0,sizeof(float));
                ggml_backend_tensor_get(result,&observed[1],0,sizeof(float));
                if (selected==reference) {
                    CHECK((observed[0]==canary)==(pass==1)); expected[pass]=observed;
                } else CHECK(observed==expected[pass]);
            }
            if (planned) {
                if (selected==backend) {
                    ggml_backend_tensor_memset(intermediate,0x75,0,ggml_nbytes(intermediate));
                    const auto initial=tensor_bytes(intermediate);
                    auto saved=result->op; result->op=GGML_OP_COUNT;
                    CHECK(ggml_backend_graph_plan_compute(selected,plan)==GGML_STATUS_FAILED && tensor_bytes(intermediate)==initial);
                    result->op=saved;
                    CHECK(ggml_backend_graph_plan_compute(selected,nullptr)==GGML_STATUS_FAILED);
                    ggml_backend_graph_plan_free(selected,nullptr);
                }
                ggml_backend_graph_plan_free(selected,plan);
                plan=ggml_backend_graph_plan_create(selected,graph);
                CHECK(plan && ggml_backend_graph_plan_compute(selected,plan)==GGML_STATUS_SUCCESS);
                ggml_backend_graph_plan_free(selected,plan);
            }
        }
        return finish(expected.size());
    }
#ifdef VTENSOR_IMPORTED_SUITE
    if (!strncmp(argv[3], "upstream-", 9)) {
      ggml_backend_cpu_set_use_ref(reference,true);
      if (!strcmp(argv[3], "upstream-SLEEP")) {
        CHECK(test_sleep(backend,GGML_TYPE_F32,4096,5000,4)==TEST_OK);
        graph_fixture ctx(6); CHECK(ctx);
        auto storage=ggml_new_tensor_1d(ctx,GGML_TYPE_I32,5);
        auto view=ggml_view_1d(ctx,storage,3,sizeof(int32_t));
        auto out=ggml_sleep(ctx,view,0);
        auto graph=ctx.allocate(out,backend); CHECK(graph && out->data==view->data);
        CHECK(ggml_backend_supports_op(backend,out));
        CHECK(refuses_change(backend,dev,graph,out,out->op_params[0],int32_t(-1)));
        CHECK(refuses_change(backend,dev,graph,out,out->view_offs,size_t(0)));
      }
      size_t cases = vtensor_imported_suite(backend,reference,argv[3]+9);
      CHECK(cases != SIZE_MAX);
      if (!strcmp(argv[3]+9,"TOP_K_MASK_VARIANTS")) {
        std::array<uint32_t,28> expected{};
        for(auto selected : {reference,backend}) {
          graph_fixture ctx(12); CHECK(ctx);
          auto mask=ggml_new_tensor_1d(ctx,GGML_TYPE_F32,4);
          auto top=ggml_new_tensor_3d(ctx,GGML_TYPE_I32,7,2,2);
          auto scores=ggml_new_tensor_3d(ctx,GGML_TYPE_F32,4,2,2);
          auto extra=ggml_new_tensor_3d(ctx,GGML_TYPE_I32,3,2,2);
          auto out=ggml_top_k_mask(ctx,mask,top,scores,nullptr,extra);
          out->nb[2]=8*sizeof(uint32_t); out->nb[1]=2*out->nb[2]; out->nb[3]=2*out->nb[1];
          auto graph=ctx.allocate(out,selected); CHECK(graph);
          const uint32_t words[]={0x3f800000,0x80000000,0x7fc01234,1};
          const uint32_t gates[]={0xff800000,0x7f800000,0x7fc01234,0x80000000,0,0xff800000,0xff800000,0xff800000};
          const int32_t picks[]={0,1,2,3,-1,99,1,0,1,2,3,-1,99,1}, extras[]={-1,99,2,-1,99,3};
          ggml_backend_tensor_set(mask,words,0,sizeof(words)); for(size_t h=0;h<2;++h) ggml_backend_tensor_set(scores,gates,h*scores->nb[2],sizeof(gates));
          for(size_t h=0;h<2;++h) { ggml_backend_tensor_set(top,picks,h*top->nb[2],sizeof(picks)); ggml_backend_tensor_set(extra,extras,h*extra->nb[2],sizeof(extras)); }
          std::array<uint32_t,28> initial{};initial.fill(0xdeadbeef);ggml_backend_tensor_set(out,initial.data(),0,sizeof(initial));
          CHECK(ggml_backend_supports_op(selected,out) && ggml_backend_graph_compute(selected,graph)==GGML_STATUS_SUCCESS);
          std::array<uint32_t,28> actual{}; ggml_backend_tensor_get(out,actual.data(),0,sizeof(actual));
          if(selected==reference) expected=actual;
          else {
            CHECK(actual==expected && actual[4]==0xdeadbeef && actual[7]==0xdeadbeef && actual[20]==0xdeadbeef);
            CHECK(refuses_change(backend,dev,graph,out,out->nb[1],size_t(4)));
            CHECK(refuses_change(backend,dev,graph,out,top->type,GGML_TYPE_F32));
            CHECK(refuses_change(backend,dev,graph,out,scores->ne[1],int64_t(3)));
            CHECK(refuses_change(backend,dev,graph,out,out->src[5],extra));
          }

        }
      }
      const bool units = !strcmp(argv[3]+9,"EXPONENTIAL_UNITS");
      const bool parameterized = !strcmp(argv[3]+9,"SWIGLU_OAI_VARIANTS");
      const bool erf = !strcmp(argv[3]+9,"GAUSSIAN_ERROR");
      if (units || erf || parameterized || !strcmp(argv[3]+9,"SWIGLU_CLAMP_VARIANTS")) {
        const std::array<float,12> inputs=units
          ? std::array<float,12>{-0.f,0.f,-0x1p-149f,-0x1p-28f,-0x1p-25f,-0.25f,-0.5f,-1.25f,0.25f,1.f,NAN,-8.f}
          : std::array<float,12>{erf?1.25f:8.f,-8,0,-0.f,2,-2,erf?3.f:-8.f,8,-3,3,0,-0.f};
        for (int variant=0;variant<(units?4:2);++variant) {
          std::vector<float> expected;
          for (auto selected : {reference,backend}) {
            graph_fixture ctx(4); CHECK(ctx);
            auto x=ggml_new_tensor_1d(ctx,erf?GGML_TYPE_F16:GGML_TYPE_F32,inputs.size());
            ggml_tensor *out;
            if (units) out=variant==0 ? ggml_elu(ctx,x) : ggml_xielu(ctx,x,-0.75f,1.25f,0.5f,variant==1?-0.125f:variant==2?-0.f:NAN);
            else {
              out=erf ? ggml_geglu_erf(ctx,x) : parameterized ? ggml_swiglu_oai(ctx,x,nullptr,-0.5f,2.f) : ggml_swiglu_clamp(ctx,x,nullptr,2.f);
              out->op_params[1]=variant;
            }
            auto graph=ctx.allocate(out,selected); CHECK(graph);
            for (size_t i=0;i<inputs.size();++i) ggml_set_f32_1d(x,i,inputs[i]);
            CHECK(ggml_backend_supports_op(selected,out) && ggml_backend_graph_compute(selected,graph)==GGML_STATUS_SUCCESS);
            for (int64_t i=0;i<ggml_nelements(out);++i) {
              float actual=ggml_get_f32_1d(out,i);
              if (selected==reference) { expected.push_back(actual); continue; }
              float want=expected.at(i);
              if (std::isnan(want)) { CHECK(std::isnan(actual)); continue; }
              CHECK(std::signbit(actual)==std::signbit(want));
              if (units) {
                uint32_t a,b; std::memcpy(&a,&actual,4); std::memcpy(&b,&want,4);
                CHECK(std::max(a,b)-std::min(a,b)<=(i<5 && variant==0 ? 0u : 3u));
              } else CHECK(std::fabs(actual-want)<=1e-6f);
            }
            if (!units && selected==backend) CHECK(refuses_change(backend,dev,graph,out,out->op_params[1],int32_t(2)));
          }
        }
      }
      if (!strcmp(argv[3]+9,"ROW_SOFTMAX")) {
        std::vector<float> expected;
        for (auto selected : {reference,backend}) {
          graph_fixture ctx(12); CHECK(ctx);
          auto scores=ggml_new_tensor_2d(ctx,GGML_TYPE_F32,4,2);
          auto mask=ggml_new_tensor_2d(ctx,GGML_TYPE_F32,4,3);
          auto sink=ggml_new_tensor_1d(ctx,GGML_TYPE_F32,1);
          scores->nb[2]+=4; mask->nb[3]+=8; sink->nb[0]=16;
          auto out=ggml_soft_max_ext_inplace(ctx,scores,mask,0.5f,2.0f); ggml_soft_max_add_sinks(out,sink);
          auto graph=ctx.allocate(out,selected); CHECK(graph);
          const float inputs[]={1,2,3,-4,9,3,-2,1}, weights[]={0,-INFINITY,1,-2,1,2,-INFINITY,0,91,92,93,94}, extra[]={1};
          ggml_backend_tensor_set(scores,inputs,0,sizeof(inputs)); ggml_backend_tensor_set(mask,weights,0,sizeof(weights)); ggml_backend_tensor_set(sink,extra,0,sizeof(extra));
          CHECK(ggml_backend_supports_op(selected,out) && ggml_backend_graph_compute(selected,graph)==GGML_STATUS_SUCCESS);
          std::vector<float> actual(8); ggml_backend_tensor_get(out,actual.data(),0,ggml_nbytes(out));
          if(selected==reference) expected=actual;
          else {
            for(size_t i=0;i<actual.size();++i) CHECK(std::abs(actual[i]-expected[i])<1e-6f);
            CHECK(refuses_change(backend,dev,graph,out,out->src[1],static_cast<ggml_tensor*>(nullptr)));
            CHECK(refuses_change(backend,dev,graph,out,mask->type,GGML_TYPE_I32));
            CHECK(refuses_change(backend,dev,graph,out,sink->ne[0],int64_t(2)));
            CHECK(refuses_change(backend,dev,graph,out,out->src[3],mask));
          }

        }
      }
      if (!strcmp(argv[3]+9,"ROW_RANKING")) {
        for (bool partial : {false,true}) {
          std::vector<int32_t> expected;
          for (auto selected : {reference,backend}) {
            graph_fixture ctx(12); CHECK(ctx);
            auto base=ggml_new_tensor_2d(ctx,GGML_TYPE_F32,8,2);
            auto source=ggml_view_2d(ctx,base,4,2,base->nb[1],sizeof(float));
            auto out=partial?ggml_top_k(ctx,source,2):ggml_argsort(ctx,source,GGML_SORT_ORDER_DESC);
            auto graph=ctx.allocate(out,selected); CHECK(graph);
            const float values[]={99,1,3,4,2,99,99,99,99,-2,-5,0,1,99,99,99};
            ggml_backend_tensor_set(base,values,0,sizeof(values));
            CHECK(ggml_backend_supports_op(selected,out) && ggml_backend_graph_compute(selected,graph)==GGML_STATUS_SUCCESS);
            std::vector<int32_t> actual(ggml_nelements(out));
            ggml_backend_tensor_get(out,actual.data(),0,ggml_nbytes(out));
            if(partial) for(size_t start=0;start<actual.size();start+=2) std::sort(actual.begin()+start,actual.begin()+start+2);
            if(selected==reference) expected=actual;
            else {
              CHECK(actual==expected);
              CHECK(refuses_change(backend,dev,graph,out,source->nb[0],size_t(8)));
              CHECK(refuses_change(backend,dev,graph,out,out->type,GGML_TYPE_F32));
              if(!partial) CHECK(refuses_change(backend,dev,graph,out,out->op_params[0],int32_t(2)));
            }

          }
        }
      }
      if (!strcmp(argv[3]+9,"SCALAR_MUL_MAT_ID")) {
        for (auto kind : {GGML_TYPE_F32, GGML_TYPE_Q4_0}) {
          graph_fixture ctx(12); CHECK(ctx);
          auto weights=ggml_new_tensor_3d(ctx,kind,32,2,2);
          auto inputs=ggml_new_tensor_3d(ctx,GGML_TYPE_F32,32,1,2);
          auto storage=ggml_new_tensor_2d(ctx,GGML_TYPE_I32,3,2);
          auto ids=ggml_view_2d(ctx,storage,1,2,storage->nb[1],sizeof(int32_t));
          auto output=ggml_mul_mat_id(ctx,weights,inputs,ids);
          auto graph=ctx.allocate(output,backend); CHECK(graph);
          ggml_backend_buffer_clear(ctx.storage.get(),0);
          CHECK(ggml_backend_dev_supports_op(dev,output));
          const std::array<float,4> marker={11,12,13,14}, zeros={};
          for (int32_t last : {-1, -2, 2}) {
            const int32_t words[]={9,0,9,9,last,9};
            ggml_backend_tensor_set(storage,words,0,sizeof(words));
            ggml_backend_tensor_set(output,marker.data(),0,sizeof(marker));
            const auto status=ggml_backend_graph_compute(backend,graph);
            CHECK(status==(last==-1?GGML_STATUS_SUCCESS:GGML_STATUS_FAILED));
            std::array<float,4> actual; ggml_backend_tensor_get(output,actual.data(),0,sizeof(actual));
            CHECK(actual==(last==-1?zeros:marker));
          }

        }
      }
      static const struct {const char *batch; std::initializer_list<int> types;} stores[] = { VTENSOR_STORE_TYPES };
      for (const auto &probe : stores) if (!strcmp(argv[3]+9,probe.batch)) {
        for (int storage : probe.types) for (auto type : {GGML_TYPE_I32, GGML_TYPE_I64}) {
          const auto kind=static_cast<ggml_type>(storage);
          const bool packed=ggml_is_quantized(kind);
          const int width=packed ? ggml_blck_size(kind) : 2;
          graph_fixture ctx(12); CHECK(ctx);
          auto dst=ggml_new_tensor_2d(ctx,kind,width,3);
          auto src=ggml_new_tensor_2d(ctx,GGML_TYPE_F32,width,2);
          auto idx=ggml_new_tensor_1d(ctx,type,2);
          auto out=ggml_set_rows(ctx,dst,src,idx);
          auto graph=ctx.allocate(out,backend); CHECK(graph);
          CHECK(ggml_backend_dev_supports_op(dev,out));
          auto *original=static_cast<uint8_t*>(dst->data);
          std::iota(original,original+ggml_nbytes(dst),uint8_t(1));
          auto *values=static_cast<float*>(src->data);
          std::iota(values,values+ggml_nelements(src),1.0f);
          const int64_t wide[]={0,1}; const int32_t narrow[]={0,1};
          ggml_backend_tensor_set(idx,type==GGML_TYPE_I32?(const void*)narrow:(const void*)wide,0,ggml_nbytes(idx));
          if (packed) CHECK(refuses_change(backend,dev,graph,out,values[width],NAN,false));
          if (type==GGML_TYPE_I32) CHECK(refuses_change(backend,dev,graph,out,static_cast<int32_t*>(idx->data)[1],-1,false));
          else CHECK(refuses_change(backend,dev,graph,out,static_cast<int64_t*>(idx->data)[1],INT64_C(4294967296),false));
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
          graph_fixture ctx(12,2); CHECK(ctx);
          auto a=ggml_new_tensor_4d(ctx,GGML_TYPE_F32,9,1,1,2);auto pw=ggml_new_tensor_1d(ctx,GGML_TYPE_F32,3);auto ph=ggml_new_tensor_1d(ctx,GGML_TYPE_F32,3);
          auto ar=ggml_new_tensor_4d(ctx,GGML_TYPE_F32,9,1,1,2);auto pwr=ggml_new_tensor_1d(ctx,GGML_TYPE_F32,3);auto phr=ggml_new_tensor_1d(ctx,GGML_TYPE_F32,3);
          auto result=inplace?ggml_add_rel_pos_inplace(ctx,a,pw,ph):ggml_add_rel_pos(ctx,a,pw,ph);
          auto control=inplace?ggml_add_rel_pos_inplace(ctx,ar,pwr,phr):ggml_add_rel_pos(ctx,ar,pwr,phr);
          auto graph=ctx.allocate(result,backend); CHECK(graph);
          auto expected=ggml_new_graph(ctx);ggml_build_forward_expand(expected,control);
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
          result->src[2]=saved_height;
        }
        CHECK(refusal_probes==42 && tail_probes==2);
        for(int axis=0;axis<5;++axis) for(bool inplace:{false,true}) {
          graph_fixture ctx(8); CHECK(ctx);
          int64_t d[]={3,2,3,2,2};d[axis]=0;
          auto a=ggml_new_tensor_4d(ctx,GGML_TYPE_F32,d[0]*d[0],d[1]*d[2],d[3],d[4]);
          auto pw=ggml_new_tensor_4d(ctx,GGML_TYPE_F32,d[0],d[1],d[2],d[3]);auto ph=ggml_dup_tensor(ctx,pw);
          auto result=inplace?ggml_add_rel_pos_inplace(ctx,a,pw,ph):ggml_add_rel_pos(ctx,a,pw,ph);
          auto graph=ggml_new_graph(ctx);ggml_build_forward_expand(graph,result);
          CHECK(ggml_backend_dev_supports_op(dev,result));CHECK(ggml_backend_graph_compute(reference,graph)==GGML_STATUS_SUCCESS);
          CHECK(ggml_backend_graph_compute(backend,graph)==GGML_STATUS_SUCCESS);++cases;
        }
      }
      if (!strcmp(argv[3]+9,"ARANGE")) {
        CHECK(cases==10);
        graph_fixture ctx(2); CHECK(ctx);
        auto result=ggml_arange(ctx,0.0f,10.0f,1.0f);
        auto graph=ctx.allocate(result,backend); CHECK(graph);
        ggml_backend_tensor_memset(result,0xA5,0,ggml_nbytes(result));
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
        for(int axis:{1,2}) {
          graph_fixture strided(2); CHECK(strided); auto result=ggml_arange(strided,0.0f,10.0f,1.0f);
          result->ne[0]=2;result->ne[axis]=5;result->nb[1]=16;result->nb[2]=axis==2?20:80;result->nb[3]=100;
          auto graph=strided.allocate(result,backend); CHECK(graph);
          std::vector<unsigned char> expected(ggml_nbytes(result));ggml_backend_tensor_memset(result,0xA5,0,expected.size());
          CHECK(ggml_backend_graph_compute(reference,graph)==GGML_STATUS_SUCCESS);ggml_backend_tensor_get(result,expected.data(),0,expected.size());
          ggml_backend_tensor_memset(result,0xA5,0,expected.size());CHECK(ggml_backend_graph_compute(backend,graph)==GGML_STATUS_SUCCESS);
          CHECK(compare_result(result,nullptr,expected,0));++cases;
        }
      }
      if (!strcmp(argv[3]+9,"L2_NORM,L2_NORM_BATCH")) {
        CHECK(cases==61);
        for(int layout:{0,1}) {
          graph_fixture ctx(8,4); CHECK(ctx);
          auto left=ggml_new_tensor_4d(ctx,GGML_TYPE_F32,3,2,2,2);
          auto reference_left=ggml_new_tensor_4d(ctx,GGML_TYPE_F32,3,2,2,2);
          if(layout) for(auto input:{left,reference_left}) {input->nb[0]=8;input->nb[1]=32;input->nb[2]=80;input->nb[3]=192;}
          auto result=ggml_l2_norm(ctx,left,0.0f);auto control=ggml_l2_norm(ctx,left,0.0f);
          auto inplace=ggml_l2_norm_inplace(ctx,left,0.0f);auto reference_inplace=ggml_l2_norm_inplace(ctx,reference_left,0.0f);
          result->nb[0]=8;result->nb[1]=32;result->nb[2]=80;result->nb[3]=192;
          auto graph=ctx.allocate(result,backend); CHECK(graph);
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
            auto before=tensor_bytes(node);
            node->ne[0]=0;node->op_params[0]=int32_t(0x7fc00001u);auto active=node==result?graph:inplace_graph;
            CHECK(ggml_backend_graph_compute(reference,active)==GGML_STATUS_SUCCESS);
            CHECK(ggml_backend_graph_compute(backend,active)==GGML_STATUS_SUCCESS);
            node->ne[0]=3;CHECK(compare_result(node,nullptr,before,0));++cases;
          }
        }
        CHECK(refusal_probes==28);
      }
      return finish(cases);
    }
#endif
    if (!strcmp(argv[3], "update")) {
      size_t cases=0;
      for(ggml_type type : {GGML_TYPE_F32,GGML_TYPE_I32}) for(ggml_type patch_type : {GGML_TYPE_F32,GGML_TYPE_I32}) for(int add : {0,1})
      for(int inplace : {0,1}) for(int alias : {0,1}) for(int layout=0;layout<4;layout++) for(size_t offset : {size_t(0),size_t(4),size_t(20)})
      for(int width : {0,1,3}) for(int rows : {1,2}) for(int planes : {1,2}) for(int batches : {1,2}) {
        if((add && (type!=GGML_TYPE_F32 || patch_type!=GGML_TYPE_F32)) || (alias && type!=patch_type)) continue;
        graph_fixture ctx(6); CHECK(ctx);
        ggml_tensor *left=ggml_new_tensor_4d(ctx,type,8,4,3,2);
        const size_t s1=layout==2?16:width*4+(layout?4:0),s2=layout==2?64:s1*rows+(layout?4:0),s3=layout==2?128:s2*planes+(layout?4:0);
        ggml_tensor *right=alias?ggml_view_4d(ctx,left,width,rows,planes,batches,s1,s2,s3,16):ggml_new_tensor_4d(ctx,patch_type,width,rows,planes,batches);
        right->nb[1]=s1;right->nb[2]=s2;right->nb[3]=s3;
        const size_t n1=layout==3?size_t(-4):layout==2?0:layout?16:width*4,n2=layout==3?size_t(-16):layout==2?4:layout?64:n1*rows,n3=layout==3?size_t(-64):layout==2?8:layout?256:n2*planes;
        const size_t base=offset+(layout==3?100:0);
        ggml_tensor *result=add?(inplace?ggml_acc_inplace(ctx,left,right,n1,n2,n3,base):ggml_acc(ctx,left,right,n1,n2,n3,base)):(inplace?ggml_set_inplace(ctx,left,right,n1,n2,n3,base):ggml_set(ctx,left,right,n1,n2,n3,base));
        auto graph=ctx.allocate(result,backend); CHECK(graph);
        std::vector<uint32_t> initial(ggml_nelements(left)), expected(initial.size()),actual(initial.size());
        for(ggml_tensor *input : {left,right}) {
          std::vector<uint32_t> data(ggml_nbytes(input)/4);
          for(size_t i=0;i<data.size();i++) {const int value=int((i*11+(input==right?7:0))%47)-23; const float f=float(value)/8;
            if(input->type==GGML_TYPE_F32) memcpy(&data[i],&f,4); else data[i]=uint32_t(value);}
          if(!data.empty()) ggml_backend_tensor_set(input,data.data(),0,data.size()*4);
        }
        ggml_backend_tensor_get(left,initial.data(),0,initial.size()*4);
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
        cases++;
      }
      return finish(cases);
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
          graph_fixture ctx(32,2); CHECK(ctx);
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
          if (empty_axis<0) {
            ctx.storage.reset(ggml_backend_alloc_ctx_tensors(ctx,backend)); CHECK(ctx.storage);
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
          cases++;
          }
        }
      }
      CHECK(types==36 && decoders==(indexed?1:30));
      CHECK(complete);
      return finish(cases);
    }
    const bool bf16_gradient = !strcmp(argv[3], "row-gradient-bf16");
    if (bf16_gradient || !strcmp(argv[3], "row-gradient")) {
      const uint32_t edges[]={0,0x80000000,0x3f800000,0xbf800000,0x4b800000,0xcb800000,0x7f800000,0xff800000,0x7fc00001,0x7f800001,1,0x80000001,0x7f7fffff,0x00800000,0x3f000000,0xbf000000};
      size_t cases=0;
      for(auto type : {GGML_TYPE_F32,GGML_TYPE_F16,GGML_TYPE_BF16}) if ((type==GGML_TYPE_BF16)==bf16_gradient) for(int width : {0,1,3,17,65}) for(int rows : {0,1,3,9}) for(int output_rows : {0,1,4,7}) for(int layout=0;layout<3;layout++) for(int pattern=0;pattern<7;pattern++) {
        graph_fixture ctx(16,2); CHECK(ctx);
        ggml_tensor *input=ggml_new_tensor_2d(ctx,type,width,rows+(pattern==1?2:pattern==2?1:0)), *indices=ggml_new_tensor_1d(ctx,GGML_TYPE_I32,rows), *shape=ggml_new_tensor_2d(ctx,GGML_TYPE_F32,width,output_rows);
        const size_t bytes=ggml_type_size(type);
        input->nb[0]=bytes*(layout==2?2:1);input->nb[1]=input->nb[0]*width+bytes*layout;input->nb[2]=input->nb[3]=input->nb[1]*input->ne[1];
        indices->nb[0]=4*(layout+1);indices->nb[1]=indices->nb[2]=indices->nb[3]=indices->nb[0]*rows;
        ggml_tensor *result=ggml_get_rows_back(ctx,input,indices,shape);
        auto source=input->nb[0]==bytes?input:canonical_copy(ctx,input);
        if(bf16_gradient) source=ggml_cast(ctx,source,GGML_TYPE_F32);
        auto oracle=ggml_get_rows_back(ctx,source,indices->nb[0]==4?indices:canonical_copy(ctx,indices),shape);
        result->nb[0]=4*(layout+1);result->nb[1]=result->nb[0]*width+4*layout;result->nb[2]=result->nb[3]=result->nb[1]*output_rows;
        ctx.storage.reset(ggml_backend_alloc_ctx_tensors(ctx,backend));CHECK(ctx.storage || (!ggml_nbytes(input)&&!ggml_nbytes(indices)&&!ggml_nbytes(result)));
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
        cases++;
      }
      return finish(cases);
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

      size_t cases=0, refused=0, source_releases=0, converted_descriptors=0; int kind;
      const bool copying=!strcmp(argv[3],"copy");
      const bool modeled=copying || repeating || concatenating || filling || casting || normalizing;
      const uint64_t fixture_base=copying?5:repeating?11:concatenating?17:filling?24:casting?31:(physical_norm || typing)?0:38;
      struct FixtureAnswer {uint64_t status,value;};
      static_assert(sizeof(FixtureAnswer)==16 && offsetof(FixtureAnswer,value)==8);
      using FixtureQuery=FixtureAnswer(*)(uint64_t,uint64_t,uint64_t,uint64_t);
      FixtureQuery query=nullptr;
      {CHECK(argc==5);auto model=dlopen(argv[4],RTLD_NOW|RTLD_LOCAL);CHECK(model);query=(FixtureQuery)dlsym(model,"fixture_query");CHECK(query);}
      auto query_value=[&](uint64_t q,uint64_t c,uint64_t source,uint64_t item,uint64_t limit=UINT64_MAX) {
        const auto answer=query(q,c,source,item);
        if(answer.status!=1 || answer.value>limit) unexpected_abort("invalid Prism fixture response");
        return answer.value;
      };
      auto fixture=[&](uint64_t q,uint64_t c,uint64_t source,uint64_t item) {return query_value(fixture_base+q,c,source,item);};
      auto pattern_word=[&](int mode,int pattern,size_t index,unsigned item=0) {return uint32_t(query_value(mode,pattern,index,item,UINT32_MAX));};
      uint64_t config[5]={};
      std::vector<uint64_t> fixture_plan;
      std::vector<std::vector<unsigned char>> patterns;
      if(modeled) {for(uint64_t i=0;i<5;++i)config[i]=fixture(0,0,0,i);CHECK(query(10,0,0,0).status==0 && query(fixture_base+1,config[0],0,0).status==0);fixture_plan.resize(config[1]);CHECK(fixture_plan.size()==size_t(typing?13:backward?12:grouping?11:filling || casting || normalizing?10:9));patterns.resize(config[2]);}
      const unsigned family=folding?1:diagonal?2:triangular?3:windowing?4:transforming?5:unarying?6:scanning?7:reducing?8:selecting?9:relative?10:0;
      auto exercise=[&](int kind,int operation,int pattern,int layout,int units,int rows,int batches)->int {
        const int geometry=units+128*(rows+128*batches), case_id=32*operation+pattern;
        if(!modeled && !pattern_word(150+family,case_id,geometry,kind)) return 0;
        ggml_init_params params={8*1024*1024,nullptr,true}; ggml_context *ctx=ggml_init(params); CHECK(ctx);
        const auto type=(ggml_type)kind; const int64_t block=ggml_blck_size(type); const size_t bytes=ggml_type_size(type);
        const ggml_type output_type=typing?(ggml_type)fixture_plan[12]:relative?GGML_TYPE_F16:selecting?(operation?GGML_TYPE_I64:GGML_TYPE_I32):casting?(ggml_type)fixture_plan[9]:(unarying && operation%3==2)?GGML_TYPE_F32:type;
        const size_t output_bytes=ggml_type_size(output_type);

        auto shape_field=[&](int axis) {return int64_t(query_value(100+family,case_id,geometry,axis));};
        const int window_case=windowing?shape_field(4):0, window_size=windowing?shape_field(5):1;
        const int image_width=windowing?shape_field(6):rows, image_height=windowing?shape_field(7):batches;
        int64_t shape[4];
        for(int axis=0;axis<4;++axis) shape[axis]=modeled?fixture(2,cases,0,axis):shape_field(axis);
        ggml_tensor *input=ggml_new_tensor(ctx,type,4,shape);
        const uint32_t word=filling || normalizing?fixture_plan[9]:transforming?pattern_word(120,0,std::min(pattern,23)):0; float constant; memcpy(&constant,&word,4);
        const uint32_t bias_word=transforming?pattern_word(120,0,std::min(pattern,23)*7+3):0;float bias;memcpy(&bias,&bias_word,4);
        const bool inplace=((filling || normalizing) && operation) || (transforming && operation%2) || (unarying && operation%3==1);
        ggml_tensor *backward_input=nullptr;
        if(two_source) {int64_t d[4];for(int axis=0;axis<4;++axis)d[axis]=fixture(2,cases,1,axis);backward_input=operation==3?ggml_view_tensor(ctx,input):ggml_new_tensor(ctx,(ggml_type)fixture_plan[11],4,d);}
        auto output_storage = [&](ggml_tensor *node) {if(node->type!=output_type) {node->type=output_type;node->nb[0]=output_bytes;for(int axis=1;axis<4;axis++)node->nb[axis]=node->nb[axis-1]*node->ne[axis-1];}return node;};
        auto word_operation = [&](ggml_tensor *a, bool inplace) {
          if(typing) {
            const auto code=fixture_plan[9];if(code!=2 && code!=5 && code!=6 && code!=7)unexpected_abort("invalid typed binary constructor");
            auto node=binary_nodes[code==2?0:code==5?1:code==6?2:3][inplace](ctx,a,backward_input);
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
            const auto word=pattern_word(operation==0 && reference?3:operation,pattern,a->ne[axis],operation==2?axis:side);
            memcpy(&args[operation==0?axis*2+side:operation==1?side:axis],&word,sizeof word);
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
        // Allocate the right reference first to preserve the frozen graph's tensor IDs on every ABI.
        auto reference_right=concatenating ? canonical_copy(ctx,right) : right;
        if(selecting && operation && !ggml_is_matrix(input) && pattern!=9) reference_right=ggml_cont_2d(ctx,right,right->ne[0],ggml_nrows(right));
        ggml_tensor *expected_tensor=relative?ggml_get_rel_pos(ctx,canonical_copy(ctx,input),rows,rows):selecting?(operation?ggml_count_equal(ctx,(ggml_is_matrix(input) || pattern==9)?input:ggml_cont_2d(ctx,input,input->ne[0],ggml_nrows(input)),reference_right):ggml_argmax(ctx,input->nb[0]==sizeof(float)?input:canonical_copy(ctx,input))):(physical_norm || typing)?word_operation(input,false):wordwise?word_operation(reducing && !operation && type==GGML_TYPE_F32?input:canonical_copy(ctx,input),false):windowing?(operation?ggml_win_unpart(ctx,canonical_copy(ctx,input),image_width,image_height,window_size):ggml_win_part(ctx,canonical_copy(ctx,input),window_size)):casting?ggml_cast(ctx,canonical_copy(ctx,input),output_type):filling?ggml_fill(ctx,canonical_copy(ctx,input),constant):triangular?ggml_tri(ctx,canonical_copy(ctx,input),(ggml_tri_type)operation):diagonal?ggml_diag(ctx,canonical_copy(ctx,input)):folding?ggml_repeat_back(ctx,canonical_copy(ctx,input),target):spatial?spatial_op(canonical_copy(ctx,input),true):concatenating?ggml_concat(ctx,canonical_copy(ctx,input),reference_right,operation):repeating?ggml_repeat(ctx,canonical_copy(ctx,input),target):canonical_copy(ctx,input);
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
            const size_t at=(reducing || selecting)?block_address(operand,i):i*bytes;
            const bool convert=unarying || scanning || reducing || selecting;
            uint32_t word=pattern_word(selecting?124:scanning || reducing?123:unarying?122:type==GGML_TYPE_F32?120:121,pattern,i,operand==right);
            if(bytes==4 || selecting) memcpy(initial.data()+at,&word,4);
            else {
              float f;memcpy(&f,&word,4);
              uint16_t small=convert?(type==GGML_TYPE_F16?ggml_fp32_to_fp16(f):ggml_fp32_to_bf16(f).bits):uint16_t(word);
              memcpy(initial.data()+at,&small,2);
            }
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
          CHECK(tensor_bytes(result)==output);
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
          if (casting) {
            CHECK(refuses_change(backend,dev,graph,result,result->src[1],nullptr));
            auto before=tensor_bytes(result);
            CHECK(input->type==GGML_TYPE_F32);
            input->type=GGML_TYPE_I32; recipe_state(1);
            auto expected=before; auto oracle=*result; oracle.data=expected.data();
            for (int i=0;i<ggml_nelements(input);++i) ggml_set_i32_1d(&oracle,i,ggml_get_i32_1d(input,i));
            CHECK(ggml_backend_dev_supports_op(dev,result) && ggml_backend_graph_compute(backend,graph)==GGML_STATUS_SUCCESS);
            CHECK(tensor_bytes(result)==expected); ++converted_descriptors;
            input->type=GGML_TYPE_F32; ggml_backend_tensor_set(result,before.data(),0,before.size());
            CHECK(refuses_change(backend,dev,graph,result,result->type,GGML_TYPE_I32));
          }
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
        CHECK(compare_result(result,expected_tensor,std::move(output),(grouping || backward || typing)?4:wordwise && !selecting?(scanning?2:1):0));
        recipe_word(5);recipe_word(1);recipe_context=nullptr;ggml_backend_buffer_free(buffer); ggml_free(ctx); cases++;
        return 0;
      };
      if(modeled) {
        const auto count=config[0];
        for(uint64_t ordinal=0;ordinal<count;++ordinal) {CHECK(cases==ordinal);for(size_t field=0;field<fixture_plan.size();++field) {fixture_plan[field]=fixture(1,ordinal,0,field);CHECK(fixture_plan[field]<=uint64_t(field<7?std::numeric_limits<int>::max():UINT32_MAX));}CHECK(exercise(fixture_plan[0],fixture_plan[1],fixture_plan[2],fixture_plan[3],fixture_plan[4],fixture_plan[5],fixture_plan[6])==0);}
      } else {
        auto field=[&](int op,size_t ordinal,int index){return pattern_word(130+family,op,ordinal,index);};
        const int ops=field(0,0,0);
        while(scanf("%d",&kind)==1) for(int op=0;op<ops;++op) {
          const auto count=field(op,0,1);
          for(size_t i=0;i<count;++i) CHECK(exercise(kind,op,field(op,i,2),field(op,i,3),field(op,i,4),field(op,i,5),field(op,i,6))==0);
        }
      }
      CHECK(refused==((normalizing)?fixture(0,0,0,5):spatial?200:triangular?384:windowing?216:0));
      if (casting) CHECK(converted_descriptors==4 && refusal_probes==16);
      recipe_word(4);recipe_word(cases);recipe_word(refusal_probes+converted_descriptors);recipe_word(refused);
      if(filling) {recipe_trace=&released;recipe_word(UINT64_MAX);recipe_word(source_releases);recipe_trace=&trace;}
      return finish(cases);
    }
    if (!strcmp(argv[3], "empty")) {
      size_t cases=0; int kind;
      while (scanf("%d",&kind)==1) for (int axis=0;axis<4;axis++) for (int64_t scale : {int64_t(2),int64_t(INT32_MAX)}) {
        graph_fixture ctx(8,3); CHECK(ctx);
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
      }
      return finish(cases);
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
        graph_fixture ctx(24,3); CHECK(ctx);
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
        ctx.storage.reset(ggml_backend_alloc_ctx_tensors(ctx,backend));
        CHECK(ctx.storage || (matrix && !ggml_nbytes(left) && !ggml_nbytes(right) && !ggml_nbytes(result)));
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
          ggml_backend_buffer_t foreign=ggml_backend_cpu_buffer_from_ptr(left->data,ggml_nbytes(left)-1); CHECK(foreign);
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
            for (auto operand : {right,result}) CHECK(refuses_change(backend,dev,graph,result,operand->ne[1],int64_t(2)));
            CHECK(refuses_change(backend,dev,graph,result,left->type,GGML_TYPE_COUNT));
          } else CHECK(refuses_change(backend,dev,graph,result,result->op_params[0],int32_t(GGML_UNARY_OP_COUNT)));
        }
      }
      CHECK((!matrix || matrix_refusals_checked) && refusal_probes==5 && fgetc(stdin)==EOF);
      CHECK(ggml_backend_reg_count()==1 && ggml_backend_dev_count()==1);
      return finish(size_t(header[0]));
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
          auto viewed = tensor_bytes(&view);
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
    graph_fixture ctx(6,6); CHECK(ctx);
    ggml_tensor *input = ggml_new_tensor_1d(ctx, GGML_TYPE_F32, 4);
    ctx.storage.reset(ggml_backend_alloc_buffer(backend,128)); CHECK(ctx.storage);
    ggml_tallocr graph_allocator = ggml_tallocr_new(ctx.storage.get());
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
    return finish(cases);
}
"##;
}
