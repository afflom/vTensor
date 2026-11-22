# vTensor

vTensor is a Prism-modeled GGML device backend whose only tensor-compute
substrate is Verdant Tensor. Its target is the complete public GGML backend
contract, so arbitrary GGML applications can select it as a device without an
operation-support gap. The eventual llama.cpp integration uses GGML's backend
API; model loading, graph construction, and serving remain llama.cpp concerns.

Device and buffer integration has passed its complete repository checkpoint.
Numerical graph execution is under development and does **not** yet provide
full GGML backend support. The imported source and oracle are pinned in
[`vendor/llama.cpp-oracle`](vendor/llama.cpp-oracle/README.md).

The current F32 matrix and arithmetic graph implementations use an isolated Verdant extension
reproduced by `just verdant-prepare` from the hash-bound Git bundle in this
repository. The checkout lives at `.verdant-engine`. This override is explicit in
`Cargo.toml`; the unchanged exact-source guard rejects it as release evidence.
The declared upstream base is Verdant M20 v2.12.0 commit `34c827db624f`. The
exact extension commit and bundle digest are recorded in
[`model/verdant-development.json`](model/verdant-development.json).

The extension now includes Prism-generated F64 addition, F32 widening and
narrowing, and unsigned-count conversion for the reduction work. All 1,213,940
comparisons against the imported Berkeley SoftFloat oracle passed. Prism now
binds SUM, SUM_ROWS and MEAN graphs to this foundation and the existing F32
and storage-conversion primitives, matching 4,080 imported CPU comparisons.

## Architecture

- **Verdant Tensor** is the sole source of tensor computation. Backend glue may
  manage GGML device and buffer lifetimes, but must not introduce a second
  tensor-kernel implementation.
- **GGML** defines the device, buffer, transfer, synchronization, graph, and
  operation contracts through its upstream headers and test suites.
- **Prism** holds the normative model, imported authority records, executable
  acceptance vectors, and kernel-checked invariants.
- **vTensor** implements the GGML backend interface and validates it against
  the pinned upstream oracle.

## Current evidence

| ID | Level | Status |
| --- | --- | --- |
| `VT-01` | build | Pins and checks the imported GGML contract and oracle source. |
| `VT-02` | build | Checks the fifteen kernel descriptors and requires the pinned source; currently rejects the local development override. |
| `VT-03` | build | Delegates one fixed 4×4 F64 matrix primitive to Verdant. |
| `VT-04` | build | Inventories GGML operations, nested unary variants and storage-type declarations without claiming their implementation. |
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

The last full pinned-SDK `just vv` pass covers VT-11 before the numerical work.
The current full contract run passes 40 checks; VT-02 still refuses the local
Verdant development dependency for release. It includes the expanded 1,280
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
Non-Prism source is 11,407 lines, 858 above the original
accepted budget, so the source budget remains unsatisfied.
The complete release checks and remaining imported operation coverage remain
open. Detailed evidence is recorded in
[`VTENSOR-VERIFICATION.md`](VTENSOR-VERIFICATION.md).

## Development

The completion sequence uses shared models, execution primitives and the same
imported oracle throughout:

1. Keep the complete imported contract inventory, including public constructor and in-place variants as well as nested operation
   variants, tied to exact upstream bytes. Map each requirement to its Prism
   definition, Verdant implementation, oracle cases and observed failure test.
2. Complete the shared tensor execution interface: typed shapes, strides,
   views, addressing, ownership, layout conversion and numeric behavior for
   all 36 live storage types. Extend Verdant primitives as needed; production
   tensor arithmetic stays in Verdant.
3. Implement every operation family and variant from the imported inventory:
   elementwise/unary, reductions, row/index operations, copies and casts,
   matrix products, normalization, position/masking, convolution/pooling,
   attention, recurrent/state-space, optimizer and callback operations. Compose
   shared primitives and model their exact GGML edge conditions.
4. Complete backend lifecycle and graph interfaces, including allocation,
   transfers, synchronization, reusable plans and capability reporting. Prove
   model invariants and exercise real ABI effects and typed refusals against
   upstream implementations.
5. Run the imported upstream backend operation suite against the actual
   vTensor library. Refuse missing, stale, wrong-file, skipped or unsupported
   required cases. Exercise representative defects in data, layout, dispatch,
   ABI and provenance; distinguish finite oracle evidence from general proofs.
6. Resolve the authoritative Verdant extension and production dependency
   packaging, then run the complete pinned-SDK acceptance gate and audit every
   imported requirement. Component milestones cannot establish completion.

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
Composed labels such as NORM_SCALE and mixed-storage RMS_NORM_MUL_ADD require
their own coverage; see `verification/composed-normalization-audit.json`.
Full acceptance remains open.

The normative repository gate is `just vv`, run using the SDK image pinned in
[`prismpm.lock`](prismpm.lock). Focused checks include:

First run `just verdant-prepare` to materialize the recorded development source.
It verifies the bundle SHA-256, exact commit and upstream ancestry, and refuses
to overwrite an existing checkout with local changes. This preserves the
current development state without treating it as an accepted release.

```sh
cargo test --locked -p vtensor-backend --test contract oracle_import
cargo test --locked -p vtensor-backend --test contract verdant_api
```

The machine-readable oracle pin is
[`vendor/llama.cpp-oracle/upstream.toml`](vendor/llama.cpp-oracle/upstream.toml).
The source snapshot retains llama.cpp's MIT license.

The Verdant base revision is declared in the workspace manifest. The current
local extension must acquire an authoritative immutable dependency before
acceptance. The graph also contains Verdant's pinned `uor-matmul` upstream;
`deny.toml` permits only those two Git origins.

## Typed unary checkpoint

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

The current development checkpoint merges the Tensor extension with Verdant
M20 v2.12.0 upstream `34c827d`. Its twenty-four certified SDK packages share one imported
Prism bundle and one certificate replay per process. Native source lifting now
captures the exact generated package sources and separated test fixtures; the
new omission checks reject missing roots and package manifests. This remains a
development dependency until the authoritative production source and VT-02 pass.
The isolated Engine integration passes its complete exact-SDK gate, including
225 workspace tests and full Prism application verification. The actual Wasm
artifacts fit stricter M20 ceilings; no budget was raised. Exact evidence is in
[`verification/verdant-m20-integration.json`](verification/verdant-m20-integration.json).

Verdant's native packager now includes the certified production dependency
closure (36 crates), with exact registry versions and preserved SDK provenance.
Its registry check requires a successful real Cargo consumer build and rejects
missing or unresolvable dependencies. Production publication and VT-02 remain
part of completion.

The current Verdant extension adds certified packed-scale and F8 word primitives
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
