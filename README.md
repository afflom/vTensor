# vTensor

vTensor is a Prism-modeled GGML device backend whose only tensor-compute
substrate is Verdant Tensor. Its target is the complete public GGML backend
contract, so arbitrary GGML applications can select it as a device without an
operation-support gap. The pinned llama.cpp host loads the plugin through GGML and runs the stories15M models. llama.cpp owns model loading and graph construction.

vTensor owns the GGML ABI, devices and buffers. Its Prism models decide which
graphs it can execute and map tensor views, strides and indices to the bytes
each operation reads and writes. Verdant Engine owns the numerical primitives,
including storage conversion, row products and reductions. The provider calls
Verdant for that computation. VT-02 checks the plugin’s imported GGML symbols
against its host-service allowlist. The test suite runs the imported GGML CPU
backend independently for expected results. [Linkage check](VTENSOR-VERIFICATION.md#provider-linkage) · [Oracle source](vendor/llama.cpp-oracle/README.md)

At llama.cpp `9a080f44`, vTensor passes all 27,755 GGML factory cases. The stock scheduler assigns all 86,647 factory nodes to vTensor when it is the sole provider. The checklist tracks legal variants outside the factory and final release acceptance. [Factory verdicts](verification/ggml-factory-inventory.json) · [Scheduler evidence](verification/shared-host-storage-vt156.json) · [Provider burndown](BURNDOWN.md)

Cargo pins Verdant’s low-bit encoders and recurrent projections at `931139cb` until release. GGML supplies no importance operand on CPY or SET_ROWS. For IQ2_XXS, IQ2_XS and IQ1_S, the vTensor plan supplies unit weights and the original GGML oracle receives the same weights. IQ1_M uses the unweighted GGML encoding. [Verdant PR #62](https://github.com/afflom/verdant-engine/pull/62) · [Dependency record](model/verdant-development.json)

Verdant Tensor supplies Prism-generated F64 addition, F32 widening and
narrowing, and unsigned-count conversion for vTensor reductions. All 1,213,940
comparisons against the imported Berkeley SoftFloat oracle passed. Prism now
binds SUM, SUM_ROWS and MEAN graphs to this foundation and the existing F32
and storage-conversion primitives, matching 4,080 imported CPU comparisons.

## Run vTensor with llama.cpp on Linux

From the vTensor checkout, build the plugin in the locked SDK. Set `LLAMA_SRC` to an absolute path to llama.cpp at `9a080f44b503c64f66c84a6274b9d9ee5d5b09d5` and `MODEL` to an absolute GGUF path. The example installs vTensor as the CPU device for an unmodified `llama-simple`. GGML searches the executable directory and the working directory for CPU plugins, so run the example in its own directory. [GGML loader](https://github.com/ggml-org/llama.cpp/blob/9a080f44b503c64f66c84a6274b9d9ee5d5b09d5/ggml/src/ggml-backend-reg.cpp#L488-L501)

```sh
cargo build --locked --release -p vtensor-backend
prefix="$HOME/.local/share/vtensor"
host_build="$LLAMA_SRC/build-vtensor"
install -Dm755 "${CARGO_TARGET_DIR:-target}/release/libvtensor_backend.so" "$prefix/lib/libvtensor_backend.so"

cmake -S "$LLAMA_SRC" -B "$host_build" -DCMAKE_BUILD_TYPE=Release -DBUILD_SHARED_LIBS=ON \
  -DGGML_BACKEND_DL=ON -DGGML_NATIVE=OFF -DLLAMA_BUILD_EXAMPLES=ON
cmake --build "$host_build" --config Release --target llama-simple --parallel
install -Dm755 "$host_build/bin/llama-simple" "$prefix/bin/llama-simple"
cd "$prefix/bin"
LD_LIBRARY_PATH="$host_build/bin${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}" \
  GGML_BACKEND_PATH="$prefix/lib/libvtensor_backend.so" \
  ./llama-simple -m "$MODEL" -n 8 -ngl 0 "The small model"
```

The installed x86-64 release library loaded as the sole backend for GPT-2 and Mamba. llama.cpp assigned every layer to vTensor, and both eight-token outputs matched the imported CPU results byte for byte. The receipt records the library, executable, model and output hashes. [Installed-provider verification](verification/llama-installed-provider.json)

## Architecture

- **Verdant Tensor** is the sole source of tensor computation. Backend glue may
  manage GGML device and buffer lifetimes, but must not introduce a second
  tensor-kernel implementation.
- **GGML** defines the device, buffer, transfer, synchronization, graph, and
  operation contracts through its upstream headers and test suites.
- **Prism** holds the normative model, imported authority records, executable
  acceptance vectors, and kernel-checked invariants.
- **vTensor** owns the GGML ABI, device and buffer lifetimes, graph admission,
  address plans and dispatch. It passes words, blocks or matrix slices to
  Verdant. GGML graph objects and callbacks stay on the vTensor side of that
  interface. The imported GGML CPU and SoftFloat implementations supply test
  results. Verdant owns runtime format data, including quantization codebooks.
  Submit missing reusable numerical primitives to Verdant and pin the submitted
  patch revision in vTensor until a published release contains the primitive.

## Current evidence

| ID | Level | Status |
| --- | --- | --- |
| `VT-01` | build | Pins and checks the imported GGML contract and oracle source. |
| `VT-02` | build | Checks the exact Verdant dependency, fifteen ordered kernel descriptors and the compiled provider’s GGML host imports. [Linkage check](VTENSOR-VERIFICATION.md#provider-linkage) |
| `VT-03` | build | Delegates one fixed 4×4 F64 matrix primitive to Verdant. |
| `VT-04` | build | Checks GGML operations, unary and GLU variants, and storage-type declarations against the pinned header. |
| `VT-05` | build | Executes upstream storage-size functions from the pinned source. |
| `VT-06` | build | Compares certified Prism byte spans against upstream. |
| `VT-07` | build | Compares the certified storage catalog against every declared live type. |
| `VT-08` | build | Checks tracked and untracked source accounting and exact imported bytes. |
| `VT-09` | build | Compares certified transfer admission with upstream set/get guards. |
| `VT-10` | build | Compares certified aligned tensor placement with upstream’s linear allocator. |
| `VT-11` | build | Loads the native device and validates buffer lifetimes, transfers, ABI layouts and metadata graphs through upstream GGML. |
| `VT-12` | build | Certified execution schedule matches 1,280 upstream CPU matrix cases, including independent source/output padding and strides, empty dimensions, broadcasting and IEEE edge values. |
| `VT-13` | build | Development graph path matches 120 CPU arithmetic/unary graphs with padding, broadcasting and IEEE edge values; malformed descriptors fail before writes. |
| `VT-14` | build | Matches 864 empty copy graphs across all 36 live storage types and every empty axis, including large remaining dimensions, without operand buffers. |
| `VT-15` | build | Matches 3,456 nonempty same-type DUP, CONT and CPY graphs across all 36 live types, with strided inputs, reshape and preserved padding. |
| `VT-16` | build | Matches 160 REPEAT graphs for all five CPU-oracle types, with four-axis repetition, strided inputs and preserved output padding. |
| `VT-17` | build | Matches 10,368 CONCAT graphs across all 36 types and four axes, including strided inputs, empty operands and preserved padding. |
| `VT-18` | build | Checks 2,056 F32 padding/reflection/roll CPU comparisons and 200 circular empty-source refusals, including signed crops, extreme words and padded output. |
| `VT-19` | build | Matches 4,608 SET/ACC graphs with copied/in-place outputs, aliased sources, empty patches and overlapping/backward update strides. |
| `VT-20` | build | Matches 672 F32 REPEAT_BACK graphs across four-axis folding, empty inputs/outputs, strided inputs and preserved output padding. |
| `VT-21` | build | Matches 384 batched DIAG graphs across F32, F16 and BF16, including strides, empty dimensions, exact diagonal encodings and zero-filled off-diagonals. |
| `VT-22` | build | Checks 1,536 TRI CPU comparisons and 384 nonsquare refusals across all four variants and F32/F16/BF16, with batches, strides and empty dimensions. |
| `VT-23` | build | Matches 15,360 F32/F16 FILL graphs, including copied/in-place output, exact constants, rounding boundaries, strides, empty dimensions and unused inputs without allocations. |
| `VT-24` | build | Matches 1,920 cast/copy graphs across all six mixed F32/F16/BF16 pairs, both constructors, strides, reshape, empty dimensions and preserved padding. |
| `VT-25` | build | Window partition/join covers 2,808 CPU comparisons and 216 bounded-source refusals, including strides, padding, empty dimensions and independent oversized/reshaped windows. |
| `VT-26` | build | Matches 18,432 numerical and 384 empty SCALE/SCALE_BIAS/CLAMP/LEAKY_RELU graphs across F32/F16/BF16, copied/in-place output, strides and exact padding. |
| `VT-27` | build | Matches 4,992 graphs for thirteen typed unary operations across all five supported float pairs, copied/in-place forms, strides and empty dimensions. |
| `VT-28` | build | Matches 528 F32 CUMSUM graphs with exact first-word copies, ordered row-local prefixes, independent strides, signed-zero and NaN edges, empty axes and preserved padding. |
| `VT-29` | build | Matches 4,080 SUM/SUM_ROWS/MEAN graphs with F64 or F32 accumulation as upstream requires, packed/strided SUM ordering, narrowing before MEAN division, all supported float types, empty axes and preserved padding. |
| `VT-30` | build | Matches 2,264 ARGMAX/COUNT_EQUAL graphs, including last ties, NaN recovery, all-NaN rows, exact I32/I64 outputs, independent strides, four-axis equality, empty inputs and large-empty planning probes. |
| `VT-31` | build | Matches 3,360 F16/F32 GET_ROWS_BACK graphs with I32 indices, duplicate and prefix rows, strided layouts, empty outputs, zero-index clears without gradient buffers, and atomic invalid-index refusals. |
| `VT-32` | build | In progress: GET_ROWS passes 384 CPU graphs with strided views and atomic refusals; source and production release checks remain open. |
| `VT-33` | build | GET_REL_POS passes 192 CPU graphs with exact F16/BF16 source-word preservation in F16 output, independent strides, higher source dimensions, empty widths and atomic descriptor refusals. |
| `VT-34` | build | ADD_ID passes 164 CPU graphs and descriptor/index refusals, including three-source addition, exact minus-one copies, strided views, ignored poisoned index slices and empty tables; source and release gates remain open. |
| `VT-35` | build | Prism generates the complete frozen matrix fixture corpus while imported GGML computes every expected result. |
| `VT-36` | build | Prism generates the complete frozen arithmetic fixture corpus with independent GGML CPU numerical results. |
| `VT-37` | build | Zero-source ARANGE passes all ten unchanged upstream cases, three supplemental cases and nineteen atomic refusal probes. |
| `VT-38` | build | L2 normalization passes all 61 unchanged upstream ordinary/batched cases, 68 supplemental cases and 28 atomic refusal probes through the shared Prism/Verdant executor. |
| `VT-39` | build | ADD_REL_POS passes 1,315 CPU comparisons and 42 descriptor refusals, including copied/in-place constructors, aliased biases, exact update order, IEEE values and empty outputs. |
| `VT-40` | build | Freezes graph construction, aliases, initialized bytes and refusal mutations for 70,392 existing native fixture cases across 16 families, before their Prism recipe migration. |
| `VT-41` | build | Prism supplies all 3,456 COPY fixture recipes, layouts and initial bytes; frozen transcripts and the independent CPU oracle remain unchanged. Universal model laws bind the byte-pattern cache to recipe semantics. |
| `VT-42` | build | Prism supplies all 160 REPEAT fixture recipes, source/target/output layouts and initial bytes, preserving frozen transcripts and independent CPU results. |
| `VT-43` | build | Prism supplies all 10,368 CONCAT fixture recipes, both operand layouts, output layouts and initial bytes, preserving frozen transcripts and independent CPU results. |
| `VT-44` | build | Prism supplies 15,360 FILL and 1,920 CAST fixture recipes, typed layouts, fill constants and physical byte patterns; frozen transcripts, FILL source-release states and independent CPU results are preserved. |
| `VT-45` | build | Certified Verdant F64 division and unsigned-count conversion preserve the imported normalization mean-order regression. |
| `VT-46` | build | F32/BF16 RMS_NORM passes 121 factory and 256 supplemental cases, including 168 atomic refusals. |
| `VT-47` | build | A separate certified Prism package preserves all 2,256 spatial constructor/input recipes and independent CPU results. |
| `VT-48` | build | NORM passes 61 factory and 184 supplemental cases, including 55 atomic refusals and stride-selected rounding. |
| `VT-49` | build | GROUP_NORM passes five ordinary/fused factory cases and 224 supplemental cases, including 112 atomic refusals, nested F64 row subtotals and physical output spacing. |
| `VT-50` | build | RMS_NORM_BACK passes ten factory and 353 supplemental cases, including 115 atomic refusals, 12 mutation probes, independent source strides, narrowing counterexamples and a 65,537-batch regression. |
| `VT-51` | build | All 187 non-RoPE composed normalization factory cases and 977 typed binary supplemental cases pass, including 240 atomic refusals, 12 metadata probes, every scalar storage tuple and a 65,537-batch case. |
| `VT-52` | build | The upstream scheduler reuses an allocated graph across active, inactive and reactivated nodes. Both output buffers match the independent CPU on all three runs. |
| `VT-53` | build | All 122 Q8_0 × F32 MUL_MAT factory cases match the independent CPU. The llama.cpp loader places the Q8_0 output projection. |
| `VT-54` | build | All 122 Q4_0 × F32 MUL_MAT factory cases match the independent CPU. llama.cpp loads the quantized model and allocates its F16 KV cache on vTensor. |
| `VT-55` | build | Prism validates scatter indices and plans destination writes. Verdant converts each scalar. All 126 selected SET_ROWS cases pass the imported CPU evaluator. |
| `VT-56` | build | Prism selects the two SwiGLU operands and their row addresses. Verdant evaluates the gated product. All 36 imported factory cases pass. |
| `VT-57` | build | Passes all 510 GGML ROPE and 424 ROPE_BACK factory cases using Verdant rotations. |
| `VT-58` | build | Executes dense floating-point attention through Verdant. The quantized stories15M prefill and two decode steps match the reference top token and have maximum absolute logit error below 0.000008. [Consumer evidence](verification/llama-stories15m-vt58.json) |
| `VT-60` | build | Dispatches LOG, SIN, COS and EXP to Verdant. All 39 imported GGML factory cases pass across F32, F16 and BF16. [Oracle results](verification/factory-elementary-vt60.json) |
| `VT-61` | build | Executes sigmoid, SiLU and their addition/multiplication compositions with Verdant. All 122 imported GGML cases pass. [Oracle results](verification/factory-logistic-vt61.json) |
| `VT-62` | build | Executes BF16 GET_ROWS_BACK through Verdant. Both factory graphs and 1,680 boundary cases match the imported GGML cast and F32 accumulation reference. [Oracle results](verification/factory-bf16-row-gradient-vt62.json) |
| `VT-63` | build | Executes BF16 key/value attention with Verdant, including masked rows and sinks. [Factory results](verification/factory-bf16-attention-vt63.json) |
| `VT-64` | build | Executes six F16/BF16 matrix operand pairs. The imported high-range F32 case confirms that F16 weights retain F32 activations. [Factory results](verification/factory-scalar-matrix-vt64.json) |
| `VT-65` | build | Executes all 55 Q2_K matrix products and the upstream graph that gathers from the same weights. [Factory results](verification/factory-grouped-matrix-vt65.json) |
| `VT-66` | build | Executes packed Q4_0 and Q8_0 attention through Verdant. [Oracle results](verification/factory-packed-attention-vt66.json) |
| `VT-67` | build | Executes all TANH and composed SOFTCAP graphs through the pinned Verdant tangent. [Oracle results](verification/factory-hyperbolic-vt68.json) |
| `VT-68` | build | Places attention soft caps before masks and compares F16 outputs with GGML CPU. [Oracle results](verification/factory-hyperbolic-vt68.json) |
| `VT-69` | build | Executes all 87 directly CPU-supported F32/F16 outer-product graphs. [Oracle results](verification/factory-scalar-outer-products-vt69.json) |
| `VT-70` | build | Executes all 2,051 three-dimensional image-column factory cases. [Oracle results](verification/factory-image-columns-vt70.json) |
| `VT-71` | build | Executes all 144 directly CPU-supported packed outer products. [Oracle results](verification/factory-packed-outer-products-vt71.json) |
| `VT-72` | build | Executes all planar and linear IM2COL factory cases, F16 inputs and all nine depthwise 1D convolution graphs. [Oracle results](verification/factory-planar-columns-vt72.json) |
| `VT-73` | build | Executes 1,312 outer-product graphs for which the direct CPU kernel refuses the operands. [Decoded F32 CPU comparisons](verification/factory-decoded-outer-products-vt73.json) |
| `VT-74` | build | Prism gathers strided image tiles and filters for direct GGML convolution. Verdant evaluates the matrix product. [Factory results](verification/factory-direct-convolution-vt74.json) |
| `VT-75` | build | Executes the 64 Gaussian unary and composed factory graphs using the Verdant primitives and the GGML binary32 lookup-table precision profile. [Results](verification/factory-gaussian-activations-vt75.json) |
| `VT-76` | build | Runs original channel-major convolution graphs against independently materialized CPU operands. [Case-level comparisons](verification/factory-contiguous-convolution-vt76.json) |
| `VT-77` | build | Executes split, combined and swapped REGLU, GEGLU and GEGLU_QUICK rows using Verdant. [All 108 original cases](verification/factory-gated-activations-vt77.json) |
| `VT-78` | build | Maps biased attention masks to the Verdant head slope with original GGML softcap and mask order. [Factory comparisons](verification/factory-biased-attention-vt78.json) |
| `VT-79` | build | Executes forward average and maximum pooling. All 384 factory graphs and four F16 graphs match the imported CPU oracle. [Oracle results](verification/factory-pooling-vt79.json) |
| `VT-80` | build | Executes tiled CONV_3D through the same Verdant matrix product as CONV_2D. [Completed factory comparisons](verification/factory-volume-convolution-vt80.json) |
| `VT-81` | build | Executes all 75 state convolutions and 120 compositions with SiLU and optional bias through Verdant. [Factory comparisons](verification/factory-state-convolution-vt81.json) |
| `VT-82` | build | Gathers overlapping, cropped and gapped signal windows through Verdant. [All 33 original comparisons](verification/factory-column-overlap-vt82.json) |
| `VT-83` | build | Selects indexed expert rows in Prism and contracts Q4_0, Q8_0 and Q2_K weights with Verdant. [230 factory comparisons](verification/factory-indexed-products-vt83.json) |

These are component checks. The native backend registers one device, supports
owned and borrowed buffers, executes metadata nodes, and delegates admitted F32
matrix products, ADD, SUB, MUL, DIV, SQRT, SQR, NEG and ABS graphs to Verdant.
Prism selects and plans the numerical graph operations; scalar arithmetic is
itself generated from a certified Prism bit model inside Verdant. Same-type
DUP, CONT, CPY, CONCAT, admitted REPEAT graphs and F32 PAD, PAD_REFLECT_1D
and ROLL use Prism address plans and Verdant byte transfers. SET uses the same
transfer path; ACC delegates addition to Verdant and preserves original-source
versus in-place alias behavior. REPEAT_BACK uses certified zero-initialization
and accumulation instructions with Verdant addition in upstream input order.
DIAG uses certified address selection and Verdant transfers for all three
CPU-supported types, preserving diagonal encodings and writing positive zeros.
SUM retains packed-row versus strided-flat F32 accumulation order; half-storage
SUM accumulates in F32. SUM_ROWS and MEAN accumulate F32 rows in F64, with
F32 narrowing before MEAN division. Empty reductions retain their output semantics.
CUMSUM copies the first word of each row, then uses certified F32 addition
in CPU prefix order. It supports independently strided inputs and outputs.
ARGMAX preserves GGML’s last-index ties and NaN reset behavior. COUNT_EQUAL
compares raw I32 words by logical position across all four axes and writes an
I64 total. Its CPU reference uses raw valid layouts and an explicit contiguous
normalization for shapes affected by the imported multidimensional index bug.
TRI shares that selection path for all four upstream variants and rejects
nonsquare descriptors before writes. FILL converts constants once through
Verdant and uses the transfer path for F32/F16, including in-place aliases.
Mixed F32/F16/BF16 copies use Prism ordinal-address plans and Verdant encoding
conversion. Remaining conversions, operations and numeric types are unfinished.
The development dependency also carries the certified F32/F16/BF16 scale-bias,
clamp and leaky-ReLU transform primitive, checked against 35,964 imported CPU
cases. Prism now binds SCALE, SCALE_BIAS, CLAMP and LEAKY_RELU graphs to it,
including strided F32/F16/BF16 tensors and in-place aliases.
Storage and empty-graph coverage for all 36 live types do not establish their
nonempty numerical behavior.
The development dependency now contains a Prism storage-block decoder for all
30 executable CPU formats. Its kernel proof, 2,250 exact block executions and 16,944 exact arithmetic-word
comparisons pass, including NaN sign/payload bits. Row-gather execution uses this
decoder and passes 384 CPU graphs with strided views and atomic index refusals.

The development Verdant dependency also provides a certified F32/F16/BF16
storage conversion root, independently checked against 1,397,260 imported GGML
comparisons. FILL and admitted mixed F32/F16/BF16 copies use this primitive.

The VT-52 pinned-SDK `just vv` pass used Verdant PR #12.
An earlier full contract run passed 40 checks and VT-02 rejected the local
Verdant override. That run included the expanded 1,280
matrix cases, 384 GET_ROWS cases, 192 GET_REL_POS cases and 164 ADD_ID cases.
All ten optional GGML tensor-source slots are now carried to the Prism planner;
zero-source ARANGE now uses Prism preflight and exact output-count validation
with Verdant F32 primitives. All ten unchanged upstream ARANGE cases pass,
including a million-element output, plus empty and padded-layout checks and
nineteen atomic refusal probes. L2 normalization now uses the same typed Prism
instructions and existing Verdant primitives. All 61 unchanged upstream L2
cases pass, including batched graphs and a 65,536-row case, plus padded outputs,
both copied and in-place constructors, epsilon and IEEE edges, empty outputs
and atomic metadata refusals.
ADD_REL_POS now caches bias words and reproduces the CPU height/width update
order, including bias views that alias an in-place output. Its oracle uses the
unchanged upstream evaluator through a reusable constructor callback bridge;
the pinned factory itself has no ADD_REL_POS cases.
Development builds optimize dependency arithmetic and the pointwise kernel
while keeping assertions and overflow checks. The complete contract group takes
25.55 seconds with 43 passes and the explicit VT-02 refusal; these timings are
observations, not controlled benchmarks. Other imported operation families remain unfinished. Prism now generates all
1,280 matrix and 120 arithmetic fixtures; both generated data and actual GGML
tensor bindings match the frozen prior corpora. They share binary framing and
cache each model replay, manifest and generated batch once per test process.
Before migrating the remaining native recipes to Prism, VT-40 freezes 70,392
existing construction/input cases and 1,920 late FILL source-release states.
The numerical oracle remains imported GGML CPU code. The existing tests emit
and check each transcript during their normal execution; successful transient
files are removed. COPY, REPEAT, CONCAT, FILL and CAST now consume certified Prism enumeration,
operand/output layouts and byte recipes through a shared test-only C ABI bridge.
Their 31,264 cases preserve the complete frozen construction and input transcripts,
including all 1,920 FILL source-release states.
Model-named byte patterns are cached under universal coherence laws; model-only
edits reuse the C++ driver. Other native fixture families remain to be migrated.
Native oracle build identities bind the emitted C++ and compiler/linker recipe;
host-only fixture orchestration edits reuse the driver. Imported evaluator edits
invalidate only drivers that link it. Warm executions retain their upstream,
executable and current-source integrity checks. Mutation evidence is recorded in
[`verification/native-oracle-cache.json`](verification/native-oracle-cache.json).
The VT-51 checkpoint measures 11,738 non-Prism lines, 1,189 above the
original ceiling of 10,549.
The complete release checks and remaining imported operation coverage remain
open. Detailed evidence is recorded in
[`VTENSOR-VERIFICATION.md`](VTENSOR-VERIFICATION.md).

## Development

Use the [completion checklist](BURNDOWN.md) to track the remaining GGML contract,
Verdant release, llama.cpp integration and acceptance work.

During iteration, run the cheapest affected model/code check, export only
changed Prism roots, and batch numerical cases through the shared oracle.
Certified artifacts are replayed without reproving unchanged roots. Native
reference builds are keyed by source, compiler, SDK, architecture and normalized
build implementation; cached bytes are checked and behavior is always rerun.
Run workspace checks after a coherent batch and the full gate at acceptance.

The next source-budget reduction should move constructor and operand recipes
into Prism. Freeze each existing corpus before migration and require identical
construction and input transcripts afterward. Reuse the shared imported
constructor/evaluator bridge; retain direct CPU comparisons for IEEE cases
that the upstream evaluator rejects. Preserve padding, alias, refusal and
corruption probes while removing duplicated host fixture logic.

Before each operation family, audit its public constructors, parameter flags
and CPU implementation. Factory coverage alone is insufficient: L2 omitted an
in-place constructor, and ADD_REL_POS had no factory cases at all.
Verdant's certified F64 division and unsigned count conversion now supply the
RMS_NORM arithmetic prerequisite (VT-45). The imported mean regression preserves
division before F32 narrowing; narrowing the accumulator first changes rounding.
RMS_NORM now executes the F32/BF16 copied and in-place paths (VT-46): all 121
imported cases and 256 Prism-generated supplemental cases pass. Its exact-word
comparisons and atomic numeric/overlap refusals are recorded in the verification
evidence. Full acceptance and source-budget recovery remain open.
Spatial pad/reflect/roll fixture parameters now come from a separate certified
Prism package (VT-47); all 2,256 frozen construction/input cases and imported
CPU comparisons are preserved. Binding and certificate faults are rejected.
NORM (VT-48) now uses separate certified Prism planner and instruction roots
with Verdant arithmetic. All 61 imported factory cases and 184 supplemental
cases pass, including 55 atomic refusals. Its model preserves the imported
SSE2 grouped variance, scalar stride path and mean narrowing point. The same
normalization package carries L2's existing instruction stream to keep every
certificate within the unchanged SDK limit. The pinned oracle compiler and
source audit are recorded in `verification/normalization-next-audit.json`;
GROUP_NORM (VT-49) now preserves F64 row subtotals, clamped channel groups
and physical F32 output spacing through the same certified normalization roots.
All five ordinary/fused factory cases and 224 supplemental cases pass, including
112 atomic refusals. Evidence is in `verification/group-normalization-authoring.json`.
RMS_NORM_BACK (VT-50) now passes all ten factory and 353 supplemental cases
through a separate certified instruction root. NORM and L2 instruction bytes
are preserved; source-1 geometry is captured once for execution. Its narrowing
order and sign composition are recorded in `verification/backward-normalization-authoring.json`.
The pinned factory inventory contains 27,755 cases. VT-51 now registers all
187 NORM_SCALE, RMS_NORM_SCALE, NORM_MUL_ADD, RMS_NORM_MUL_ADD and ADD_RMS_NORM
graphs, including 69 BF16 cases. Its registered failure exposed mixed BF16/F32
arithmetic. A new certified Prism typed-arithmetic root now plans all four
binary operations and all seven scalar storage tuples, including F32 safety
guards. All 187 composed cases pass the actual imported factory comparison.
The imported constructors
and parameters are recorded in `verification/composed-normalization-cases.json`.
The implementation uses Verdant conversions and F32 operations, with rounding
at each graph node and identity conversions omitted by the Prism schedule.
The existing 120 F32 arithmetic/unary cases and 353 backward-normalization
edge cases pass; planted native encoding, model encoding and certificate
binding defects are rejected. A separate certified Prism fixture root supplies
977 additional cases across all seven tuples: 737 positive cases and 240
atomic refusals, plus 12 metadata probes. Non-NaN words and padding match the
CPU exactly; NaN outputs are compared by classification. A frozen transcript
binds actual constructors and initialized bytes, and rejects all-zero fixture
substitution. Evidence is in `verification/typed-arithmetic-authoring.json`.
Its authoritative type, stride and counter audit is in
`verification/typed-binary-source-audit.json`. Another 281 factory graphs combine
RMS normalization with RoPE and require separate coverage.
Full acceptance remains open.

The normative repository gate is `just vv`, run using the SDK image pinned in
[`prismpm.lock`](prismpm.lock). Focused checks include:

Cargo resolves the patch directly. Use `just verdant-prepare` to prefetch
locked dependencies for offline work.

```sh
cargo test --locked -p vtensor-backend --test contract oracle_import
cargo test --locked -p vtensor-backend --test contract verdant_api
```

The machine-readable oracle pin is
[`vendor/llama.cpp-oracle/upstream.toml`](vendor/llama.cpp-oracle/upstream.toml).
The source snapshot retains llama.cpp's MIT license.

The workspace manifest pins the submitted Verdant patch until its release. The graph also contains Verdant's pinned `uor-matmul` upstream;
`deny.toml` permits only those two Git origins.

## Historical typed unary checkpoint

The pinned Verdant development bundle now includes VTK-06: certified typed
unary primitives for thirteen operations and all five GGML CPU float pairs.
The isolated Engine checkpoint is `054df8ead9a0eb47b634c4cbc34263ec4690ca16`.
Its four-root Prism bundle passed certificate replay and 3,675,256 comparisons
with the authoritative imported GGML CPU implementation. Source/object drift
and a wrong unary dispatch were rejected; a driver-only change reused every
CPU object unchanged. Existing VTK-01..05 and four integration regressions pass.

The vTensor graph integration passes 4,992 imported CPU comparisons, including
in-place graphs, half-to-F32 promotion, independent strides and empty tensors.
Promotion, address and theorem defects are rejected. This development bundle
retains the explicit VT-02 release-source refusal; it is not a published
Verdant dependency or evidence of complete backend conformance.

The M20 development checkpoint merged the Tensor extension with Verdant
v2.12.0 upstream `34c827d`. Its twenty-four certified SDK packages share one imported
Prism bundle and one certificate replay per process. Native source lifting now
captures the exact generated package sources and separated test fixtures; the
new omission checks reject missing roots and package manifests. VT-02 rejected the local source at that checkpoint.
The isolated Engine integration passes its complete exact-SDK gate, including
225 workspace tests and full Prism application verification. The actual Wasm
artifacts fit stricter M20 ceilings; no budget was raised. Exact evidence is in
[`verification/verdant-m20-integration.json`](verification/verdant-m20-integration.json).

Verdant's native packager now includes the certified production dependency
closure (36 crates), with exact registry versions and preserved SDK provenance.
Its registry check requires a successful real Cargo consumer build and rejects
missing or unresolvable dependencies. VT-148 uses the published v2.21.5
release. All 103 resolved archives match the [release manifest](vendor/verdant-crates-manifest.json).

The earlier Verdant extension added certified packed-scale and F8 word primitives
for the remaining row-gathering work. Its 244,492 imported GGML comparisons pass;
all twenty previous numerical libraries remain byte-identical. Full packed-block
decoding and GET_ROWS graph support are still under development.

The next packed-block foundation adds certified packed-field extraction and
bounded integer-to-F32 encoding, with 778,496 comparisons against the imported
GGML CPU callbacks. All eleven CPU codebooks from the pinned upstream header
are imported as 34,984 canonical bytes with exact table spans, hashes and the
original license. Validation compares them with the actual compiled upstream
arrays; source lifting and production packaging retain all three resources.
The prior twenty-one SDK runtime libraries remain byte-identical. These
primitive and data checks support the remaining full decoders and GET_ROWS
implementation; they do not establish complete quantized graph support.

The next checkpoint registers full GET_ROWS and storage-block decoding before
implementation. A separate RowGather model verifies shared shape and index
conditions; its authoring evidence is retained under verification/. Verdant's
new reference corpus executes the upstream CPU operator over all 30 decodable
source formats and nine byte patterns. The implementation tests are deliberately
failing at this stage. Decoder execution will remain inside Verdant, with
vTensor following Prism's graph and address plans. The added native harness
currently exceeds the source budget and must be consolidated before acceptance.
