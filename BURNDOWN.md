# vTensor completion checklist

Target the complete backend contract at llama.cpp `9a080f44`. Close each box with a commit and evidence link. Track remaining **contract variants**, **factory cases without a verdict**, and **release blockers** separately. The upstream factory lists 27,755 cases under 154 labels. The imported evaluator reports **3,631 passed**, **3 refused** and **24,121 untested**. [GGML pin](vendor/llama.cpp-oracle/upstream.toml) · [Factory inventory](verification/ggml-factory-inventory.json) · [Existing component evidence](README.md#current-evidence)

## Establish coverage and unblock release

- [ ] Build the coverage ledger from the imported headers, constructors and backend interfaces. Cross-reference 100 opcodes (including NONE), 22 unary variants, all seven GLU variants, 36 live storage types, legal type tuples, flags, views and in-place forms with existing VT IDs, Prism roots, Verdant entry points and oracle evidence. [Current declaration inventory](model/ggml-declarations.toml)
- [x] Index all 27,755 factory ordinals and constructor parameters. Match completed runs by label and parameter group. Count each ordinal once. [Case transcript](verification/ggml-factory-cases.tsv.gz) · [Verdict mapping](verification/ggml-factory-inventory.json)
- [ ] Cover legal variants outside the factory with the imported CPU or another pinned authoritative oracle. Track the aborting `ggml_flash_attn_back` constructor until an authoritative verdict exists. [Pinned constructor](https://github.com/ggml-org/llama.cpp/blob/9a080f44b503c64f66c84a6274b9d9ee5d5b09d5/ggml/src/ggml.c#L5529)
- [x] Pin the submitted Verdant primitives. VT-02 checks Cargo's exact upstream source and the 15 descriptors. [Engine PR #12](https://github.com/afflom/verdant-engine/pull/12) · [Stacked Engine PR #13](https://github.com/afflom/verdant-engine/pull/13) · [Engine PR #14](https://github.com/afflom/verdant-engine/pull/14) · [Engine PR #15](https://github.com/afflom/verdant-engine/pull/15) · [Engine PR #16](https://github.com/afflom/verdant-engine/pull/16) · [Engine PR #17](https://github.com/afflom/verdant-engine/pull/17) · [Engine PR #18](https://github.com/afflom/verdant-engine/pull/18) · [Engine PR #19](https://github.com/afflom/verdant-engine/pull/19) · [Engine PR #20](https://github.com/afflom/verdant-engine/pull/20) · [Engine PR #21](https://github.com/afflom/verdant-engine/pull/21) · [Patch record](model/verdant-development.json)
- [ ] When Verdant releases these primitives, switch to the published production packages, verify provenance and checksums, and build a clean consumer. Continue implementation on the PR pin until then.
- [x] Move the 864 allocation inputs, 1,024 transfer inputs and 40 fixture queries to Prism. [Certificate and oracle results](VTENSOR-VERIFICATION.md#fixture-probes)
- [ ] Recover **1,618 non-Prism lines** (12,167 → at most 10,549). Move repeated native fixture construction and runner dispatch into shared Prism recipes. Preserve frozen inputs, refusal probes and CPU comparisons. [Original ceiling and VT-51 measurement](verification/typed-arithmetic-authoring.json)

## Bring up a real llama.cpp consumer

- [x] Load the plugin via `GGML_BACKEND_PATH` in unmodified llama.cpp. The isolated host selects vTensor for model layers 0–6. [Consumer evidence](VTENSOR-VERIFICATION.md#llamacpp-consumer)
- [x] Match the upstream scheduler's compute-bit selection and reuse an allocated two-operation graph across active, inactive and reactivated runs (VT-52). [Oracle evidence](VTENSOR-VERIFICATION.md#scheduled-graph-selection)
- [ ] Complete preallocation admission, views, buffer placement, transfers and synchronization through the scheduler. Verify required ABI callbacks and defaults for optional capabilities. [Backend contract](vendor/llama.cpp-oracle/ggml/src/ggml-backend-impl.h)
- [x] Encode Q8_0 activation blocks in Verdant. All 271 blocks match the imported reference byte for byte. [Engine oracle and certificate](https://github.com/afflom/verdant-engine/blob/32cbf30db6d3e0f708aebb7d2adb55f38c9d381a/ENGINE-VERIFICATION.md#vtk-16-symmetric-block-encoding)
- [x] Place `output.weight` in llama.cpp. Run all 122 Q8_0 × F32 MUL_MAT cases through the Verdant row product (VT-53). [Factory and consumer evidence](VTENSOR-VERIFICATION.md#quantized-matrix-execution)
- [x] Load the quantized GGUF into 17.50 MiB of vTensor model storage. Run all 122 Q4_0 × F32 MUL_MAT factory cases through Verdant (VT-54). [Factory and consumer evidence](VTENSOR-VERIFICATION.md#packed-matrix-execution-and-kv-cache-placement)
- [x] Pass all 126 scalar SET_ROWS factory cases, including all 20 F32 → F16 / I64 cases. Confirm llama.cpp admits all 12 KV-cache writes (VT-55). [Factory and scheduler evidence](VTENSOR-VERIFICATION.md#scalar-indexed-row-writes)
- [x] Supply the shared binary32 exponential for SwiGLU and attention in Verdant. All 166,856 imported scalar comparisons pass. [Engine oracle and certificate](https://github.com/afflom/verdant-engine/blob/5b2f0c66e4e51a6e34d5deaf9212738924747912/ENGINE-VERIFICATION.md#vtk-19-binary32-exponential)
- [x] Pass all 36 split, combined and swapped SWIGLU factory cases. Confirm llama.cpp admits all six feed-forward gates (VT-56). [Factory and scheduler evidence](VTENSOR-VERIFICATION.md#swiglu-rows)
- [x] Supply shared binary32 sine and cosine for rotary position encoding in Verdant. All 520,568 imported scalar comparisons pass. [Engine oracle and certificate](https://github.com/afflom/verdant-engine/blob/db211ab9efc25855162be786bc2db6034a0c483a/ENGINE-VERIFICATION.md#vtk-21-binary32-sine-and-cosine)
- [x] Verify rotary frequency scaling and plane rotation in Verdant. Pass 24,578 logarithms, 260 frequency ratios and 7,680 rotated coefficient/coordinate pairs against upstream GGML. [Frequency oracle](https://github.com/afflom/verdant-engine/pull/19) · [Rotation oracle](https://github.com/afflom/verdant-engine/pull/20)
- [x] Certify the GGML rotary address plan for normal, NEOX, multimodal, vision and interleaved layouts. Connect it to Verdant execution under VT-57 below. [Prism export](https://github.com/afflom/vTensor/blob/7774ee045cf44352cdb85531ef752af1fde6394e/rotary-kernel.lock)
- [x] Run all 510 ROPE and 424 ROPE_BACK cases on the plugin. Confirm llama.cpp admits all 12 stories15M rotary nodes. [VT-57 evidence](VTENSOR-VERIFICATION.md#rotary-graph-execution)
- [x] Verify the Verdant streaming weighted mean against 5,940 coordinates from the imported CPU attention evaluator. [Upstream patch](https://github.com/afflom/verdant-engine/pull/21)
- [x] Execute all six stories15M FLASH_ATTN_EXT nodes on vTensor. Compare the prefill and both decode cache updates across six layers. [Host, logits and cache evidence](verification/llama-stories15m-vt58.json)
- [ ] Implement the CPU registry threadpool lifecycle in vTensor. Run the unmodified llama-eval-callback executable through threadpool creation, inference and teardown. [Imported host call](https://github.com/ggml-org/llama.cpp/blob/9a080f44b503c64f66c84a6274b9d9ee5d5b09d5/common/common.cpp#L1813)
- [x] Pass all 252 GET_ROWS factory cases on the plugin. [Factory results](verification/factory-checkpoint-vt57.json)
- [ ] Complete embedding, projection and KV-cache operations. Close the remaining 641 SET_ROWS factory cases, CPY/casts, block encoding and quantized dot products, MUL_MAT precision modes, MUL_MAT_ID and OUT_PROD across their legal formats. Reuse the verified decoders and F32 matrix path.
- [x] Pass all 77 ROPE_SET_ROWS and 281 RMS_NORM_MUL_ROPE graphs on the plugin. [Composition results](verification/factory-rotary-compositions.json)
- [ ] Complete remaining masked softmax variants and 5,772 FLASH_ATTN_EXT cases.
- [ ] Complete the feed-forward path. Close remaining binary formats, transcendental and unary functions, all GLU variants, and their composed graphs. Close the remaining 173 standalone ADD cases. All 427 SUB, MUL and DIV cases passed on the plugin. [Factory results](verification/factory-checkpoint-vt57.json)
- [x] Run checksum-pinned F32 and Q4_0 GGUFs. Compare prompt prefill, two decode steps and KV writes to upstream CPU. Confirm every scheduled node used vTensor. [F32](verification/llama-stories15m-float-vt58.json) · [Q4_0](verification/llama-stories15m-vt58.json) · [Node placements](verification/llama-stories15m-vt58-placement.tsv.gz)

## Finish the arbitrary-provider contract

- [ ] Resolve factory refusals 809 (Q2_K ADD), 1,069 (BF16 GET_ROWS_BACK) and 8,368 (IQ2_XXS CPY). Identify the refusing backend and validate each case against its authoritative oracle. [Exact inputs](verification/factory-checkpoint-vt57.json)
- [ ] Complete sorting, TOP_K and masks, expert routing, indexed matrix compositions, LIGHTNING_INDEXER and DSV4 operations.
- [ ] Complete convolution, im2col/col2im, transpose/depthwise forms, pooling, upscaling, timestep embeddings and their defined backward paths.
- [ ] Complete SSM convolution/scan, RWKV, gated attention, gated delta, triangular solve and state/cache compositions.
- [ ] Complete remaining gradients, cross-entropy, SGD and AdamW. Exercise applicable upstream gradient checks and parameter updates.
- [ ] Implement MAP_CUSTOM1/2/3, CUSTOM and SLEEP effects. Verify callback inputs, task indices, user data, aliasing and observable completion against upstream.
- [ ] Close every residual legal variant in the ledger, including partial families from VT-01–51. Cover strides, empty axes, broadcasting, overlapping views, precision and typed failures where applicable.

## Accept and ship

- [ ] Reconcile all 27,755 factory records to individual verdicts. Require every applicable case to execute and pass on the actual library. Resolve omissions, unsupported required cases and missing-oracle rows. Link supplemental evidence for constructor and ABI behavior outside the factory.
- [x] Pass the complete locked-SDK `just vv` for the VT-52 checkpoint on the Verdant PR #12 pin. All 57 backend contract tests passed. [Verification](VTENSOR-VERIFICATION.md#verdant-q8-dependency)
- [x] Pass the complete locked-SDK `just vv` at the VT-55 checkpoint on the Verdant PR #15 pin. All 60 backend contract tests passed. [Verification](VTENSOR-VERIFICATION.md#vt-55-complete-checkpoint)
- [x] Pass the full locked-SDK gate at VT-56. All 61 backend contract tests passed on the SwiGLU implementation. [Checkpoint](VTENSOR-VERIFICATION.md#vt-56-complete-checkpoint)
- [ ] Pass the complete `just vv` in the locked SDK from a clean checkout with production dependencies. Attach the exact build, proof and oracle evidence to the installable backend and its llama.cpp invocation.

For each implementation batch, register the scenario and failing case first. Export only changed Prism roots, reuse hash-checked oracle objects, and run the affected cases together. Check a representative planted defect before closing the batch. [Repository workflow](AGENTS.md) · [Oracle cache](verification/native-oracle-cache.json)
