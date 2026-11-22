# vTensor completion checklist

Target the complete backend contract at llama.cpp `9a080f44`. Close each box with a commit and evidence link. Track remaining **contract variants**, **factory cases without a verdict**, and **release blockers** separately. The upstream factory lists 27,755 cases under 154 labels. Overall verified coverage has yet to be counted. [GGML pin](vendor/llama.cpp-oracle/upstream.toml) · [Factory inventory](verification/ggml-factory-inventory.json) · [Existing component evidence](README.md#current-evidence)

## Establish coverage and unblock release

- [ ] Build the coverage ledger from the imported headers, constructors and backend interfaces. Cross-reference 100 opcodes (including NONE), 22 unary variants, all seven GLU variants, 36 live storage types, legal type tuples, flags, views and in-place forms with existing VT IDs, Prism roots, Verdant entry points and oracle evidence. [Current declaration inventory](model/ggml-declarations.toml)
- [ ] Partition the factory by ordinal, label and parameters. Assign each case once, including composed graphs. Cover missing legal variants with the imported CPU or another pinned authoritative oracle. Track upstream blockers such as the aborting `ggml_flash_attn_back` constructor until an authoritative verdict exists. [Pinned constructor](https://github.com/ggml-org/llama.cpp/blob/9a080f44b503c64f66c84a6274b9d9ee5d5b09d5/ggml/src/ggml.c#L5529)
- [x] Submit the certified tensor primitives against Verdant main and pin vTensor to the PR commit. VT-02 checks Cargo's exact upstream source and the 15 descriptors. [Engine PR #12](https://github.com/afflom/verdant-engine/pull/12) · [Patch record](model/verdant-development.json)
- [ ] When Verdant releases these primitives, switch to the published production packages, verify provenance and checksums, and build a clean consumer. Continue implementation on the PR pin until then.
- [ ] Recover **1,235 non-Prism lines** (11,784 → at most 10,549). Move repeated native fixture construction and runner dispatch into shared Prism recipes. Preserve frozen inputs, refusal probes and CPU comparisons. [Original ceiling and VT-51 measurement](verification/typed-arithmetic-authoring.json)

## Bring up a real llama.cpp consumer

- [x] Load the plugin via `GGML_BACKEND_PATH` in unmodified llama.cpp. The isolated host selects vTensor for model layers 0–6. [Consumer evidence](VTENSOR-VERIFICATION.md#llamacpp-consumer)
- [x] Match the upstream scheduler's compute-bit selection and reuse an allocated two-operation graph across active, inactive and reactivated runs (VT-52). [Oracle evidence](VTENSOR-VERIFICATION.md#scheduled-graph-selection)
- [ ] Complete preallocation admission, views, buffer placement, transfers and synchronization through the scheduler. Verify required ABI callbacks and defaults for optional capabilities. [Backend contract](vendor/llama.cpp-oracle/ggml/src/ggml-backend-impl.h)
- [ ] Unblock the first llama.cpp weight-placement refusal. Implement the Q8_0 × F32 `MUL_MAT` path for `output.weight`, including right-operand quantization and block dot products in Verdant. Compare execution with the imported CPU before admitting the operation. [Consumer probe](VTENSOR-VERIFICATION.md#llamacpp-consumer)
- [ ] Complete embedding, projection and KV-cache operations. Close GET_ROWS, SET_ROWS, CPY/casts, block encoding and quantized dot products, MUL_MAT precision modes, MUL_MAT_ID and OUT_PROD across their legal formats. Reuse the verified decoders and F32 matrix path.
- [ ] Complete RoPE, its backward and cache-update variants, masked softmax, and attention. Run the 510 ROPE, 424 ROPE_BACK, 77 ROPE_SET_ROWS, 281 RMS_NORM_MUL_ROPE and 5,775 FLASH_ATTN_EXT factory cases in their assigned batches.
- [ ] Complete the feed-forward path. Close remaining binary formats, transcendental and unary functions, all GLU variants, and their composed graphs. Exercise typed arithmetic against the 600 standalone ADD/SUB/MUL/DIV factory cases.
- [ ] Run a checksum-pinned small GGUF in floating-point and quantized form. Compare prompt prefill, successive decode logits and KV-cache reuse to the separate upstream CPU run. Record device assignment for every compute node and require zero numerical fallback.

## Finish the arbitrary-provider contract

- [ ] Complete sorting, TOP_K and masks, expert routing, indexed matrix compositions, LIGHTNING_INDEXER and DSV4 operations.
- [ ] Complete convolution, im2col/col2im, transpose/depthwise forms, pooling, upscaling, timestep embeddings and their defined backward paths.
- [ ] Complete SSM convolution/scan, RWKV, gated attention, gated delta, triangular solve and state/cache compositions.
- [ ] Complete remaining gradients, cross-entropy, SGD and AdamW. Exercise applicable upstream gradient checks and parameter updates.
- [ ] Implement MAP_CUSTOM1/2/3, CUSTOM and SLEEP effects. Verify callback inputs, task indices, user data, aliasing and observable completion against upstream.
- [ ] Close every residual legal variant in the ledger, including partial families from VT-01–51. Cover strides, empty axes, broadcasting, overlapping views, precision and typed failures where applicable.

## Accept and ship

- [ ] Reconcile all 27,755 factory records to individual verdicts. Require every applicable case to execute and pass on the actual library. Resolve omissions, unsupported required cases and missing-oracle rows. Link supplemental evidence for constructor and ABI behavior outside the factory.
- [ ] Pass the complete `just vv` in the locked SDK from a clean checkout with production dependencies. Attach the exact build, proof and oracle evidence to the installable backend and its llama.cpp invocation.

For each implementation batch, register the scenario and failing case first. Export only changed Prism roots, reuse hash-checked oracle objects, and run the affected cases together. Check a representative planted defect before closing the batch. [Repository workflow](AGENTS.md) · [Oracle cache](verification/native-oracle-cache.json)
