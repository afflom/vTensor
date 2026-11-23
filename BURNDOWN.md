# vTensor completion checklist

Target the complete backend contract at llama.cpp `9a080f44`. Close each box with a commit and evidence link. Track remaining **contract variants**, **factory cases without a verdict**, and **release blockers** separately. The upstream factory lists 27,755 cases under 154 labels. The direct and supplemental GGML oracle records show **19,889 passed**, **7,766 refused** and **100 untested**. [GGML pin](vendor/llama.cpp-oracle/upstream.toml) · [Factory inventory](verification/ggml-factory-inventory.json) · [Standalone oracle verdicts](verification/factory-standalone-vt59.json) · [Existing component evidence](README.md#current-evidence)

## Establish coverage and unblock release

- [ ] Build the coverage ledger from the imported headers, constructors and backend interfaces. Cross-reference 100 opcodes (including NONE), 22 unary variants, all seven GLU variants, 36 live storage types, legal type tuples, flags, views and in-place forms with existing VT IDs, Prism roots, Verdant entry points and oracle evidence. [Current declaration inventory](model/ggml-declarations.toml)
- [x] Index all 27,755 factory ordinals and constructor parameters. Match completed runs by label and parameter group. Count each ordinal once. [Case transcript](verification/ggml-factory-cases.tsv.gz) · [Verdict mapping](verification/ggml-factory-inventory.json)
- [ ] Cover legal variants outside the factory with the imported CPU or another pinned authoritative oracle. Track the aborting `ggml_flash_attn_back` constructor until an authoritative verdict exists. [Pinned constructor](https://github.com/ggml-org/llama.cpp/blob/9a080f44b503c64f66c84a6274b9d9ee5d5b09d5/ggml/src/ggml.c#L5529)
- [x] Pin the submitted Verdant primitives. VT-02 checks Cargo's exact upstream source and the 15 descriptors. [Engine PR #12](https://github.com/afflom/verdant-engine/pull/12) · [Stacked Engine PR #13](https://github.com/afflom/verdant-engine/pull/13) · [Engine PR #14](https://github.com/afflom/verdant-engine/pull/14) · [Engine PR #15](https://github.com/afflom/verdant-engine/pull/15) · [Engine PR #16](https://github.com/afflom/verdant-engine/pull/16) · [Engine PR #17](https://github.com/afflom/verdant-engine/pull/17) · [Engine PR #18](https://github.com/afflom/verdant-engine/pull/18) · [Engine PR #19](https://github.com/afflom/verdant-engine/pull/19) · [Engine PR #20](https://github.com/afflom/verdant-engine/pull/20) · [Engine PR #21](https://github.com/afflom/verdant-engine/pull/21) · [Engine PR #22](https://github.com/afflom/verdant-engine/pull/22) · [Engine PR #23](https://github.com/afflom/verdant-engine/pull/23) · [Engine PR #24](https://github.com/afflom/verdant-engine/pull/24) · [Engine PR #25](https://github.com/afflom/verdant-engine/pull/25) · [Engine PR #26](https://github.com/afflom/verdant-engine/pull/26) · [Engine PR #27](https://github.com/afflom/verdant-engine/pull/27) · [Patch record](model/verdant-development.json)
- [x] Check the compiled provider’s GGML imports in VT-02. A planted dependency on the GGML matrix kernel is rejected. [Boundary evidence](VTENSOR-VERIFICATION.md#provider-linkage)
- [ ] When Verdant releases these primitives, switch to the published production packages, verify provenance and checksums, and build a clean consumer. Keep the PR pin until then. Verdant v2.18.0 lacks the two Gaussian activation exports used by VT-75 and VT-77. [Release source](https://github.com/afflom/verdant-engine/tree/ba1425942f2cdc0f1d647df2fb71ab1e57bca507/crates/verdant-tensor/src)
- [x] Move the 864 allocation inputs, 1,024 transfer inputs and 40 fixture queries to Prism. [Certificate and oracle results](VTENSOR-VERIFICATION.md#fixture-probes)
- [x] Generate all 1,872 layout requests in Prism and retain the original input digest and independent GGML byte-span oracle. [VT-06 fixture evidence](VTENSOR-VERIFICATION.md#vt-06-layout-fixtures)
- [ ] Recover **1,588 non-Prism lines** (12,137 → at most 10,549). Move repeated native fixture construction and runner dispatch into shared Prism recipes. Preserve frozen inputs, refusal probes and CPU comparisons. [Original ceiling and VT-51 measurement](verification/typed-arithmetic-authoring.json)

## Bring up a real llama.cpp consumer

- [x] Load the plugin via `GGML_BACKEND_PATH` in unmodified llama.cpp. The isolated host selects vTensor for model layers 0–6. [Consumer evidence](VTENSOR-VERIFICATION.md#llamacpp-consumer)
- [x] Match the upstream scheduler's compute-bit selection and reuse an allocated two-operation graph across active, inactive and reactivated runs (VT-52). [Oracle evidence](VTENSOR-VERIFICATION.md#scheduled-graph-selection)
- [ ] Complete preallocation admission, views, buffer placement, transfers and synchronization through the scheduler. Verify required ABI callbacks and defaults for optional capabilities. [Backend contract](vendor/llama.cpp-oracle/ggml/src/ggml-backend-impl.h)
- [x] Encode Q8_0 activation blocks in Verdant. All 271 blocks match the imported reference byte for byte. [Engine oracle and certificate](https://github.com/afflom/verdant-engine/blob/32cbf30db6d3e0f708aebb7d2adb55f38c9d381a/ENGINE-VERIFICATION.md#vtk-16-symmetric-block-encoding)
- [x] Place `output.weight` in llama.cpp. Run all 122 Q8_0 × F32 MUL_MAT cases through the Verdant row product (VT-53). [Factory and consumer evidence](VTENSOR-VERIFICATION.md#quantized-matrix-execution)
- [x] Load the quantized GGUF into 17.50 MiB of vTensor model storage. Run all 122 Q4_0 × F32 MUL_MAT factory cases through Verdant (VT-54). [Factory and consumer evidence](VTENSOR-VERIFICATION.md#packed-matrix-execution-and-kv-cache-placement)
- [x] Pass all 126 scalar SET_ROWS factory cases, including all 20 F32 → F16 / I64 cases. Confirm llama.cpp admits all 12 KV-cache writes (VT-55). [Factory and scheduler evidence](VTENSOR-VERIFICATION.md#scalar-indexed-row-writes)
- [x] Pin Verdant’s GELU and GELU_QUICK after 257,548 scalar oracle comparisons. [Verdant PR #26](https://github.com/afflom/verdant-engine/pull/26)
- [x] Compare the 24 SNAKE_FUSE, 42 MUL_MAT_ADD and 36 FLASH_ATTN_EXT graphs admitted on replay with the original GGML CPU evaluator. [Original comparisons](verification/factory-admitted-compositions.json)
- [x] Pass all 64 GELU, GELU_QUICK and composed factory graphs against GGML CPU. [VT-75 results](verification/factory-gaussian-activations-vt75.json)
- [x] Supply the shared binary32 exponential for SwiGLU and attention in Verdant. All 166,856 imported scalar comparisons pass. [Engine oracle and certificate](https://github.com/afflom/verdant-engine/blob/5b2f0c66e4e51a6e34d5deaf9212738924747912/ENGINE-VERIFICATION.md#vtk-19-binary32-exponential)
- [x] Pass all 108 REGLU, GEGLU and GEGLU_QUICK factory cases. [VT-77 results](verification/factory-gated-activations-vt77.json)
- [x] Pass all 36 split, combined and swapped SWIGLU factory cases. Confirm llama.cpp admits all six feed-forward gates (VT-56). [Factory and scheduler evidence](VTENSOR-VERIFICATION.md#swiglu-rows)
- [x] Supply shared binary32 sine and cosine for rotary position encoding in Verdant. All 520,568 imported scalar comparisons pass. [Engine oracle and certificate](https://github.com/afflom/verdant-engine/blob/db211ab9efc25855162be786bc2db6034a0c483a/ENGINE-VERIFICATION.md#vtk-21-binary32-sine-and-cosine)
- [x] Verify rotary frequency scaling and plane rotation in Verdant. Pass 24,578 logarithms, 260 frequency ratios and 7,680 rotated coefficient/coordinate pairs against upstream GGML. [Frequency oracle](https://github.com/afflom/verdant-engine/pull/19) · [Rotation oracle](https://github.com/afflom/verdant-engine/pull/20)
- [x] Certify the GGML rotary address plan for normal, NEOX, multimodal, vision and interleaved layouts. Connect it to Verdant execution under VT-57 below. [Prism export](https://github.com/afflom/vTensor/blob/7774ee045cf44352cdb85531ef752af1fde6394e/rotary-kernel.lock)
- [x] Run all 510 ROPE and 424 ROPE_BACK cases on the plugin. Confirm llama.cpp admits all 12 stories15M rotary nodes. [VT-57 evidence](VTENSOR-VERIFICATION.md#rotary-graph-execution)
- [x] Verify the Verdant streaming weighted mean against 5,940 coordinates from the imported CPU attention evaluator. [Upstream patch](https://github.com/afflom/verdant-engine/pull/21)
- [x] Execute all six stories15M FLASH_ATTN_EXT nodes on vTensor. Compare the prefill and both decode cache updates across six layers. [Host, logits and cache evidence](verification/llama-stories15m-vt58.json)
- [x] Run the unmodified `llama-eval-callback` at one and two threads through creation, inference and teardown on the VT-59 implementation. [Source](https://github.com/afflom/vTensor/commit/d232e00ae9dcf383c900d8a3792d1201ef19b26e) · [CPU comparison](verification/llama-threadpool-vt59.json)
- [ ] Integrate VT-59 on main. Recover the 349 additional native lines measured against its main base and pass the complete locked-SDK gate.
- [x] Pass all 252 GET_ROWS factory cases on the plugin. [Factory results](verification/factory-checkpoint-vt57.json)
- [x] Pass all 694 F16/BF16 scalar MUL_MAT cases across six operand pairs and the high-range F32 precision regression. [VT-64 factory results](verification/factory-scalar-matrix-vt64.json)
- [x] Pass all 87 CPU-supported scalar OUT_PROD cases with the Verdant matrix primitive. [VT-69 results](verification/factory-scalar-outer-products-vt69.json)
- [x] Pass all 144 directly CPU-supported packed OUT_PROD cases through Verdant. [VT-71 results](verification/factory-packed-outer-products-vt71.json)
- [x] Pass the 1,312 CPU-refused OUT_PROD graphs against the upstream decoder and F32 CPU contraction. All 1,543 factory outer products now have passing comparisons. [VT-73 evidence](verification/factory-decoded-outer-products-vt73.json)
- [x] Pass 68 additional dense matrix and activation compositions across F32, F16, Q4_0, Q8_0 and Q2_K. [Full-graph comparisons](verification/factory-dense-matrix-fusions.json)
- [ ] Complete embedding, projection and KV-cache operations. Implement the 641 refused SET_ROWS factory cases, CPY/casts, block encoding and quantized dot products, the 1,775 refused standalone MUL_MAT cases, precision modes and 1,225 refused MUL_MAT_FUSION cases. Reuse the verified decoders and F32 matrix path.
- [x] Pass all 230 Q4_0, Q8_0 and Q2_K MUL_MAT_ID graphs, including skipped experts and offset index views. [VT-83 results](verification/factory-indexed-products-vt83.json)
- [x] Pass the 130 newly admitted packed expert compositions, including six shared-expert graphs and 124 matrix fusions. [Original full-graph comparisons](verification/factory-indexed-expert-compositions.json)
- [x] Execute all 251 F32, F16 and BF16 MUL_MAT_ID cases, including invalid expert IDs and strided index views. [VT-84 results](verification/factory-scalar-experts-vt84.json)
- [ ] Pass all 128 scalar expert compositions. 126 pass. [Original graph comparisons](verification/factory-scalar-expert-compositions.json)
- [x] Pass all 157 MUL_MAT and 73 MUL_MAT_ID graphs with E4M3 weights. [VT-85 comparisons](verification/factory-e4m3-products-vt85.json)
- [x] Pass all 164 newly admitted E4M3 matrix fusions. [Full-graph comparisons](verification/factory-e4m3-compositions.json)
- [ ] Pass the remaining 699 block-encoded MUL_MAT_ID cases. [Original factory cases](verification/ggml-factory-cases.tsv.gz)
- [x] Pass all 77 ROPE_SET_ROWS and 281 RMS_NORM_MUL_ROPE graphs on the plugin. [Composition results](verification/factory-rotary-compositions.json)
- [x] Execute BF16 key/value attention with Verdant query conversion, dot products and weighted means. 246 upstream cases pass, covering every mask/sink combination. [VT-63 results](verification/factory-bf16-attention-vt63.json)
- [x] Pass all 533 zero-bias, zero-softcap Q4_0 and Q8_0 attention cases. 533 pass. Verdant supplies the encoder, decoder and row products. [VT-66 oracle results](verification/factory-packed-attention-vt66.json)
- [x] Pin the Verdant head slope and validate the provider boundary with VT-02. All 40,452 upstream scalar comparisons passed, within one binary32 step. [Engine PR #27](https://github.com/afflom/verdant-engine/pull/27)
- [ ] Pass the 1,574 FLASH_ATTN_EXT cases with positive head bias. 1,120 of 1,190 cases in the five implemented storage formats pass. Add Q4_1, Q5_0, Q5_1 and IQ4_NL for the other 384. [VT-78 comparisons](verification/factory-biased-attention-vt78.json)
- [ ] Complete masked softmax and FLASH_ATTN_EXT. Of 5,775 attention cases, 4,222 pass, 1,453 are refused and 100 still need an evaluator verdict.
- [x] Execute all 39 LOG, SIN, COS and EXP cases through Verdant, including the upstream storage and view variants. [Factory verdicts](verification/factory-elementary-vt60.json)
- [x] Execute the 122 sigmoid and SiLU standalone, multiplied and added graphs on the pinned Verdant primitives. [Factory verdicts](verification/factory-logistic-vt61.json)
- [x] Supply binary32 hyperbolic tangent in Verdant. All 128,870 GGML scalar comparisons pass. [Verdant PR #25](https://github.com/afflom/verdant-engine/pull/25)
- [x] Pass all 12 TANH graphs and the composed SOFTCAP graph on Verdant PR #25. [VT-67 results](verification/factory-hyperbolic-vt68.json)
- [x] Pass all 241 zero-bias F16 attention graphs with soft cap 10. 241 pass. [VT-68 results](verification/factory-hyperbolic-vt68.json)
- [ ] Complete the feed-forward path. Close remaining binary formats, transcendental and unary functions, SWIGLU_OAI, GEGLU_ERF and SWIGLU_CLAMP, and their composed graphs. Close the 14 refused standalone ADD cases. The other 159 pass. All 427 SUB, MUL and DIV cases passed on the plugin. [Factory results](verification/factory-checkpoint-vt57.json)
- [x] Run checksum-pinned F32 and Q4_0 GGUFs. Compare prompt prefill, two decode steps and KV writes to upstream CPU. Confirm every scheduled node used vTensor. [F32](verification/llama-stories15m-float-vt58.json) · [Q4_0](verification/llama-stories15m-vt58.json) · [Node placements](verification/llama-stories15m-vt58-placement.tsv.gz)

## Finish the arbitrary-provider contract

- [x] Encode Q8_K activations in Verdant. All 38,544 bytes from 132 blocks matched the imported GGML reference. [Engine PR #23](https://github.com/afflom/verdant-engine/pull/23)
- [x] Execute all 55 Q2_K × F32 MUL_MAT cases and the graph combining MUL_MAT, GET_ROWS, REPEAT and ADD (809) with the pinned Verdant row product. [VT-65 results](verification/factory-grouped-matrix-vt65.json)
- [x] Execute BF16 GET_ROWS_BACK on vTensor. The two factory graphs match 10,240 CPU-reference bytes, and 1,680 strided, empty and invalid-index cases pass. The direct factory reports CPU-only refusal on both ordinals. [VT-62 evidence](verification/factory-bf16-row-gradient-vt62.json)
- [x] Compare all 63 CPU-refused identity copies across seven IQ formats with the pinned CPU copy kernel. All destination bytes match, including permuted layouts. [63 direct comparisons](verification/factory-iq-copy.json) · [IQ2_XXS regression](VTENSOR-VERIFICATION.md#iq2_xxs-identity-copy)
- [x] Pass all 534 TOP_K and 98 ARGSORT factory cases, including ties and infinities. [VT-86 results](verification/factory-row-ranking-vt86.json) · [Verdant row ranking](https://github.com/afflom/verdant-engine/pull/28)
- [x] Pass 172 TOPK_MOE and all four TOPK_QSA graphs unlocked by row ranking. [Full-graph comparisons](verification/factory-row-ranking-compositions.json)
- [ ] Pass 38 TOP_K_MASK and the remaining 520 TOPK_MOE cases. Implement SOFT_MAX for 344 graphs and SOFTPLUS for 176. Complete the indexed matrix compositions, LIGHTNING_INDEXER and DSV4 operations.
- [x] Pass all 2,051 IM2COL_3D factory cases. The 1,539 previous refusals now match the CPU for F32/F16 output, padded and dilated inputs and views. [VT-70 results](verification/factory-image-columns-vt70.json)
- [x] Pass all 113 IM2COL factory cases, both supplemental F16-input graphs and all nine CONV_1D_DW compositions. [VT-72 evidence](verification/factory-planar-columns-vt72.json)
- [x] Pass all 2,030 directly CPU-supported CONV_2D cases through the Verdant matrix product. 2,030 pass. [VT-74 results](verification/factory-direct-convolution-vt74.json)
- [x] Compare the 1,114 CPU-refused CONV_2D layouts with a contiguous GGML CPU reference. 1,114 pass. [VT-76 results](verification/factory-contiguous-convolution-vt76.json)
- [x] Pass all 240 POOL_1D and 144 POOL_2D forward graphs and four supplemental F16 graphs. [VT-79 comparisons](verification/factory-pooling-vt79.json)
- [x] Pass all 292 direct CONV_3D graphs, including packed batches, offset kernels and empty kernels. 292 pass. [VT-80 results](verification/factory-volume-convolution-vt80.json)
- [x] Pass all 33 COL2IM_1D overlap-add graphs in F32, F16 and BF16. [VT-82 results](verification/factory-column-overlap-vt82.json)
- [ ] Pass the remaining spatial factory graphs (116 CONV_TRANSPOSE_1D, 8 CONV_TRANSPOSE_2D, 8 CONV_2D_DW, 24 UPSCALE, 6 TIMESTEP_EMBEDDING). [Case inventory](verification/ggml-factory-inventory.json)
- [ ] Compare IM2COL_BACK and POOL_2D_BACK gradients with the pinned GGML CPU using original forward inputs, padding and overlapping windows. [Imported declarations](model/ggml-declarations.toml)
- [x] Pass all 75 state convolution graphs and 120 compositions with SiLU and optional bias. 195 pass. [VT-81 results](verification/factory-state-convolution-vt81.json)
- [ ] Complete SSM scan, RWKV, gated attention, gated delta, triangular solve and state/cache compositions.
- [ ] Complete remaining gradients, cross-entropy, SGD and AdamW. Exercise applicable upstream gradient checks and parameter updates.
- [ ] Implement MAP_CUSTOM1/2/3, CUSTOM and SLEEP effects. Verify callback inputs, task indices, user data, aliasing and observable completion against upstream.
- [ ] Close every residual legal variant in the ledger, including partial families from VT-01–51. Cover strides, empty axes, broadcasting, overlapping views, precision and typed failures where applicable.

## Accept and ship

- [ ] Reconcile all 27,755 factory records to individual verdicts. Require every applicable case to execute and pass on the actual library. Resolve omissions, unsupported required cases and missing-oracle rows. Link supplemental evidence for constructor and ABI behavior outside the factory.
- [x] Pass the complete locked-SDK `just vv` for the VT-52 checkpoint on the Verdant PR #12 pin. All 57 backend contract tests passed. [Verification](VTENSOR-VERIFICATION.md#verdant-q8-dependency)
- [x] Pass the complete locked-SDK `just vv` at the VT-55 checkpoint on the Verdant PR #15 pin. All 60 backend contract tests passed. [Verification](VTENSOR-VERIFICATION.md#vt-55-complete-checkpoint)
- [x] Pass the full locked-SDK gate at VT-56. All 61 backend contract tests passed on the SwiGLU implementation. [Checkpoint](VTENSOR-VERIFICATION.md#vt-56-complete-checkpoint)
- [x] Pass the complete locked-SDK gate at VT-57. All 63 backend contract tests passed, including all 934 forward and backward rotary factory cases. [Gate record](VTENSOR-VERIFICATION.md#vt-57-complete-checkpoint)
- [x] Pass the full locked-SDK gate at VT-58. All 66 backend contract tests passed. [Checkpoint](VTENSOR-VERIFICATION.md#vt-58-complete-checkpoint)
- [x] Pass the full locked-SDK gate at VT-63. All 70 backend contract tests passed. [Checkpoint](VTENSOR-VERIFICATION.md#vt-63-complete-checkpoint)
- [x] Pass the full locked-SDK gate at VT-65. All 72 backend contracts passed on the grouped two-bit matrix implementation. [Checkpoint](VTENSOR-VERIFICATION.md#vt-65-complete-checkpoint)
- [x] Pass the full locked-SDK gate at VT-68. All 75 backend contracts passed on Verdant PR #25. [Checkpoint](VTENSOR-VERIFICATION.md#vt-68-complete-checkpoint)
- [x] Pass the full locked-SDK gate at VT-72. All 79 backend contracts passed. [Checkpoint](VTENSOR-VERIFICATION.md#vt-72-complete-checkpoint)
- [x] Pass the full locked-SDK gate at VT-76. All 83 backend contracts passed on Verdant PR #26. [Checkpoint](VTENSOR-VERIFICATION.md#vt-76-complete-checkpoint)
- [x] Pass the full locked-SDK gate at VT-80. All 86 backend contracts passed. [Checkpoint](VTENSOR-VERIFICATION.md#vt-80-complete-checkpoint)
- [x] Pass the full locked-SDK gate at VT-82 on Verdant PR #27. All 89 backend contracts passed. [Checkpoint](VTENSOR-VERIFICATION.md#vt-82-complete-checkpoint)
- [ ] Pass the complete `just vv` in the locked SDK from a clean checkout with production dependencies. Attach the exact build, proof and oracle evidence to the installable backend and its llama.cpp invocation.

For each implementation batch, register the scenario and failing case first. Export only changed Prism roots, reuse hash-checked oracle objects, and run the affected cases together. Check a representative planted defect before closing the batch. [Repository workflow](AGENTS.md) · [Oracle cache](verification/native-oracle-cache.json)
