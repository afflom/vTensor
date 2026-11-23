# vTensor verification

## VT-01: pinned GGML oracle import

The oracle snapshot, including the private backend implementation interface,
is taken from
[`ggml-org/llama.cpp`](https://github.com/ggml-org/llama.cpp) commit
`9a080f44b503c64f66c84a6274b9d9ee5d5b09d5`. It includes the GGML backend API
headers, `ggml/src/ggml-backend-impl.h`, the upstream backend-operation
consistency test, and the upstream MIT license. Per-file SHA-256 values are in
[`vendor/llama.cpp-oracle/upstream.toml`](vendor/llama.cpp-oracle/upstream.toml);
the complete snapshot is pinned by `model/authorities.toml`.

Positive check:

```text
cargo test --locked -p repo-conformance --test oracle_import
1 passed; 0 failed
```

Falsifiability check:

1. Changed the first line of `ggml/include/ggml-backend.h`.
2. Ran `cargo test --locked -p repo-conformance --test oracle_import oracle_import_vt_01`.
3. The test failed with `upstream source drifted: ggml/include/ggml-backend.h`,
   reporting different observed and pinned SHA-256 digests.
4. Restored the upstream byte and reran the positive check successfully.

The model artifact check also passed with
`cargo run -q --locked -p xtask -- check-model`.

The full Rust workspace suite passed with `cargo test --workspace --locked`.

## VT-02: pinned Verdant API inventory

vTensor consumes `verdant-tensor` from Verdant Engine commit
`bbcb465a79f7f52cfff2a9c0b4acac549529c5bc` through
the test-side adapter in [`contract.rs`](crates/conformance/tests/contract.rs). The conformance test checks that
Cargo resolved this exact revision and that the API returns all fifteen
unified kernel descriptors in contiguous index order.

Positive check:

```text
cargo test --locked -p repo-conformance --test verdant_api
1 passed; 0 failed
```

Falsifiability check:

1. Temporarily changed the expected inventory size from 15 to 14.
2. Ran `cargo test --locked -p repo-conformance --test verdant_api verdant_api_vt_02`.
3. VT-02 failed with `left: 15, right: 14`.
4. Restored the expected size and reran the positive check successfully.

This proves dependency resolution and API availability only. It does not
establish any GGML backend callback, tensor-type mapping, or operator support.

## VT-03: Verdant-backed fixed 4x4 F64 matrix multiplication

the test-side adapter in [`contract.rs`](crates/conformance/tests/contract.rs) exposes a vTensor adapter that
uses Prism's generated `supportsF64Matmul4` predicate and directly delegates
fixed 4x4 column-major F64 matrix multiplication to
`verdant_tensor::linear::multiply_mat4_cols_exact`. The conformance test checks
all predicate rejection cases, the result against an independent scalar
column-major calculation, and that non-finite input is rejected with Verdant's
typed error.

Positive and negative checks:

```text
cargo test --locked -p repo-conformance --test verdant_api verdant_api_vt_03
1 passed; 0 failed
```

Before implementation, this test failed to compile because
`multiply_matrix4_f64_cols` was not present in the adapter. After adding the
direct Verdant delegation, the matrix-result and NaN-refusal assertions passed.
This is only a fixed-shape kernel primitive, not a registered GGML backend or
general `MUL_MAT` implementation.

## Prism-generated adapter predicate

The LexLean module
[`app/src/BackendCore.lex.tex`](app/src/BackendCore.lex.tex) now models the
fixed F64 4x4 matrix case and proves `supportsF64Matmul4Decides`. The existing
dispatch root references the predicate so Prism emits it in the generated
Rust package at [`crates/prism-kernel/src/lib.rs`](crates/prism-kernel/src/lib.rs).
the conformance adapter calls that predicate before delegating the operation to
Verdant Tensor.

The pinned SDK verified the model with attestation
`1cc4bc33bb302035846052b485370860cac5e8df207a4a5491e1947f10510bcd`.
[`prism-kernel.lock`](prism-kernel.lock) binds the generated Rust file's
SHA-256 digest to that attestation. The `app` recipe regenerates and checks the
lock after a fresh verification.

The theorem proves the modeled predicate's Boolean/Proposition equivalence; it
does not prove that a GGML implementation satisfies the predicate. The holo
still contains the inherited template contract checker, so no complete GGML
backend claim is established. The full `just vv` gate has not yet been rerun
after adding the generated-kernel synchronization.

## Scope of this evidence

VT-01 proves only that the imported reference files match their recorded
digests and that the authority is registered. VT-02 proves only that the
pinned Verdant API exposes its verified inventory through vTensor. VT-03
verifies one fixed-shape matrix primitive. None of these claims establishes
GGML tensor-format, buffer, device, general operator, or graph-execution
support. The Prism theorem verifies the completeness predicate's logic, not
that any implementation satisfies it.

## VT-04: authoritative declaration inventory

`model/ggml-declarations.toml` records the pinned `ggml_op` and `ggml_type`
enums, including their numeric values, order, reserved-value gaps and count
sentinels. There are 100 operation entries and 36 tensor-type entries before
their respective sentinels. This is a declaration inventory, not a semantic
requirements inventory or evidence that any operation or type is implemented.
The external header remains authoritative; the test rejects inventory drift
and refuses unfamiliar enum syntax instead of silently omitting it.

The registered scenario and scaffold test were created before implementation.
The initial focused run failed with `VT-04 is registered and not built`.
After implementation, removing the first operation row from the real inventory
and running `cargo test --locked -p repo-conformance --test ggml_inventory`
failed with `operation: inventory differs from pinned upstream declarations`.
Restoring the row restored the passing result. The test additionally exercises
omission, duplication, reordering, numeric mutation and an invented
`implemented` field for each enum so its comparator must reject those defects.

This adds no tensor implementation and changes no dependency pins. Development
validation uses the focused inventory and scenario meta-tests; the new test is
also automatically included by the existing full-workspace acceptance recipe.
The complete SDK acceptance gate remains outstanding.

## Authoritative oracle investigation for subsequent implementation

A separate read-only investigation checkout at
`/tmp/vtensor-ggml-authority` resolves to the same upstream revision
`9a080f44b503c64f66c84a6274b9d9ee5d5b09d5`. It is development scratch,
not a repository dependency or a substitute for a pinned oracle import.

The authoritative `tests/test-backend-ops.cpp` test loop explicitly continues
past `SKIPPED` and `NOT_SUPPORTED` cases and compares successful cases only
against the cases that ran. Therefore exit status alone cannot establish
complete backend coverage. The vTensor oracle integration must additionally
account for selected, executed, skipped and unsupported cases and require the
intended vTensor device. The upstream suite provides CSV/SQL output,
`--show-coverage`, backend selection, and focused operation filters. Focused
filters serve development iteration; complete acceptance must account for the
entire required case set.

Upstream `ggml/src/ggml.c` also establishes details that the current fixed
matrix primitive does not model: `ggml_mul_mat` constructs an F32 output with
extents `{a.ne[1], b.ne[1], b.ne[2], b.ne[3]}`; `ggml_nbytes` handles empty
extents and block-encoded strides separately. These sources must be included
in the reproducible authoritative import before checks depend on them. The
existing F64 primitive remains primitive evidence only.

Focused validation for VT-04 passed: its conformance test, the four BDD
meta-tests, model regeneration consistency, formatting, and Clippy for the
new test target. Build artifacts were reused through
`CARGO_TARGET_DIR=/tmp/vtensor-analysis-target`; no full workspace test or
Prism proof was repeated for this declaration-only change.

## VT-05: executable authoritative storage oracle

The same upstream revision now supplies a 3.9 MiB compressed Git archive of
`ggml`, `tests/test-backend-ops.cpp`, and `LICENSE`. `upstream.toml` binds its
export paths and digest, and the registered authority binds the entire import
directory. This is strictly a test dependency, not a vTensor compute substrate.

The transport in `crates/conformance/tests/support/storage_oracle.c` populates
an upstream `ggml_tensor` and calls the actual upstream `ggml_nbytes`,
`ggml_blck_size`, and `ggml_type_size`. No storage-span formula is copied into
the harness. The test builds the unmodified `ggml.c` and `ggml-quants.c` from
the archive using the native compiler. Function/data sections and linker
section collection keep the storage oracle executable limited to its required
upstream closure. It needs no CMake installation or extra SDK package.

The registered scaffold failed before implementation. The positive fixtures
cover F32 scalar and padded rows, a four-dimensional F64 tensor, Q4_0 padded
block rows, and zero/negative extents. Malformed transport exits with code 2.
A planted `+ 1` on the transported upstream byte count caused all six expected
results to differ and the test failed; restoring the harness restores the
original results. Transport mutation invalidated the build cache as intended.

The native build cache binds the source archive, harness, build helper,
compiler version outputs, target architecture, and running SDK inventory.
Host development builds use a separate identity. Every reuse checks the
executable digest and executes the oracle cases again. The oracle uses one
process for a batch of cases to avoid per-case process overhead. Full backend
operation coverage and Prism-generated span validation remain outstanding.

VT-05's positive tests, oracle import check, and BDD meta-gate passed inside
the pinned SDK image. Focused SDK Clippy and formatting passed. A batch of
8192 scalar requests also passed through a single process; input is written
concurrently with output draining so batches cannot deadlock on pipe buffers.
The test verifies that a second unchanged build request preserves the
executable modification time. These are focused development results; they do
not establish a passing full `just vv`.

The next Prism storage draft is
`prism-app/1352e9b5d7a35097453582a8b51771bae8617c321480c5448bf86d6a7847b4ed`.
It passed LexLean check with semantic ID
`de38d2684c77cf878e434d50c78379a7f74e415dd79bd1947cf0c01c45074444`.
It defines four-dimensional nonnegative layouts, scalar/block byte spans,
zero-extent behavior and invalid-block refusal, with concrete padded F32 and
Q4_0 theorem fixtures. Verification task `task-238` remains queued as of the
last status read. No proof or generated implementation is claimed. A local
scratch copy is `/tmp/vtensor-tensor-layout.lex.tex`; it is not yet integrated
into the authoritative application. The pinned SDK's
`lexlean production build --request` supports certified function-root exports,
so integration can use that supported route rather than inserting a dummy
reference in the template application's dispatch function.

## VT-06: certified Prism storage spans against GGML

`app/src/TensorLayout.lex.tex` is the authoritative executable definition of
nonnegative four-dimensional storage spans. It distinguishes invalid block
sizes from spans. Scalar and block-encoded rows follow upstream `ggml_nbytes`;
empty dimensions short circuit arithmetic. The two concrete padded-span
fixtures are proved by definitional equality with no declared axioms.
They do not constitute a universal proof of GGML equivalence.

The pinned SDK production exporter verified this root and generated the
`rust-core` package in `generated/layout`. Result ID:
`1c4913db6e0858eb73e3a0fa9a6caa1d4d337641eac32ec9dfcb8a30ed7a4bb6`.
The bundle carries kernel-checked lowering, Rust-realization and composition
certificates. Their recorded axiom closure is `Classical.choice`, `Quot.sound`,
and `propext`; these preservation certificates are distinct from the two
axiom-free concrete source fixtures. `layout-kernel.lock` binds the generated
Rust bytes to the bundle result and verification attestation.

VT-06 checks current source, semantic and model-lock identities before using
the artifact, then asks the SDK to replay the complete certified bundle.
It compares the generated function against unmodified GGML across 1872
layouts covering all 36 declared tensor types, with padding, three-dimensional
and four-dimensional batches and zero extents. Type sizes and block sizes
come from the upstream oracle. Negative cases exercise invalid block sizes,
scalar and block multiplication overflow, sum overflow, and empty-dimension
short circuiting. No handwritten span implementation is present in vTensor.
Signed GGML extent normalization at the ABI boundary is not implemented yet.

The earlier remote draft verification task `task-238` was cancelled while
queued because the exact-SDK production export performs the required local
verification and code-preservation checks. The repository model supersedes
that scratch draft.

`just layout-export` regenerates through the supported SDK API and selects the
bundle by its returned content address. Routine conformance replays its
certificates and compares current model identities without a second proof
build. `just fmt` formats every maintained workspace member, preserving the
SDK-rendered package bytes that the artifact verifier requires.

VT-06 validation results: 1872 oracle comparisons and the typed negative cases
passed inside the pinned SDK; the BDD meta-gate, focused Clippy, model-write,
and repository formatting checks also passed. Appending a comment to the
actual generated Rust caused the generated-byte binding assertion to fail.
After restoring the generated file, renaming a theorem in the source model
caused the stale source-identity assertion to fail. Both original files were
restored exactly. The restored positive result is pending the full gate.
The export synchronization portion of `just layout-export` was smoke-tested
against the existing certified SDK output in an isolated temporary directory;
it reproduced the bundle and kernel lock without rerunning proofs.

The full pinned-SDK `just vv` run in session `79875` is terminal. Prism
application verification succeeded with attestation
`56f77df4266b93e1db460f5498423534a3feb7a5fd444c6f01c7b6742a018f52`.
The run then stopped because the tracked-source ratchet counted four added
handwritten lines. Moving the generated-lock catalog into
`model/prism-locks.json` restored its passing result without disabling the
rule; all nine source-ratchet unit tests passed. The remaining gate components
were subsequently run without repeating the unchanged application proof.
Template verification, SDK lock verification, workspace Clippy and all
workspace tests passed. Dependency checking initially rejected the SDK-rendered
package's absent license metadata, then passed after an exact-source-hash
licensing clarification. No single complete `just vv` success is claimed for
the current source, which has since gained VT-07.

Dependency investigation: the live registry at
`https://afflom.github.io/verdant-engine/crates/index/` advertises
`verdant-tensor` 0.1.0 while the previously fetched upstream source declares
workspace version 2.1.0. The published tarball matches registry checksum
`377141f476f6bba5ed38426e881942ddff0b44c91dbe6aa2dc9529bf7a49a788`,
but its manifest retains workspace-only inheritance including
`version.workspace = true` and `uor-matmul.workspace = true`. It is not yet a
standalone production dependency. A verified scratch copy is
`/tmp/vtensor-verdant-published.crate`; the vTensor dependency pin is unchanged.

## VT-07: complete finite GGML type storage catalog

`app/src/TensorTypes.lex.tex` defines all 36 imported GGML type identifiers
and their exact block and storage sizes. Values were obtained by executing
unmodified `ggml_blck_size` and `ggml_type_size` in the pinned oracle. Removed
identifiers, the count sentinel and out-of-range inputs return a modeled
`unknownType` decision. This describes storage metadata; it does not implement
quantization, conversion, operators or device registration.

The registered scaffold failed before implementation. The exact SDK exported
both the byte-span and type-catalog roots in one certified bundle:
`bae1be375d02fd7820fb4958766fdc87df071c372e01cc335df5c2ca80b44501`
with verification attestation `5f964322725e74340e9567e8df99b0758051f7dabd062be1025085284f40a5eb`.
The generated `vtensor_types` package contains the executable catalog; there
is no handwritten type-size implementation in vTensor.

The conformance suite shares one certificate replay and one oracle setup via
`OnceLock` across VT-06 and VT-07. All 36 valid descriptors match the external
oracle, all holes through the sentinel are rejected, and further boundary
inputs including `u64::MAX` are refused. A planted `+ 1` on the oracle
transport's reported type size made VT-07 fail for GGML type 0, comparing
`[0, 1, 4]` with `[0, 1, 5]`. Restoring the transport restored the passing
result. Both storage tests together passed in 1.57 seconds after integration
back into the primary worktree. BDD and model consistency passed there too;
focused Clippy, dependency checks and formatting passed in the isolated
worktree before integration.

The SDK-generated packages retain their exact certified bytes. Their missing
manifest license fields are addressed with version-specific cargo-deny
clarifications bound to the actual `src/lib.rs` hashes and the repository's
MIT/Apache licensing. A wrong source hash was demonstrated to fail license
validation. This follows the file-bound clarification mechanism documented by
[cargo-deny](https://embarkstudios.github.io/cargo-deny/checks/licenses/cfg.html#clarify),
without excluding packages or broadening the license allowlist.

## Remaining source-accounting integration work

The current source ratchet reads tracked files only. Therefore its passing
result is insufficient for this worktree's untracked additions. Before release
it must account for untracked source files too, while recognizing only
byte-verified imported and generated artifacts as external implementation.
The inherited browser frame also lacks its intended upstream source pin:
`bootstrap/frame-sync.mjs --pin <commit>` is the existing authoritative
updater for that purpose. The template remote currently resolves exactly to
`b64cc73c94773ed5bcc9d0549b1ae2c836fa1e0b`, the inherited source revision.
Do not treat the partial ratchet result as proof of the complete revision.

No validation processes from sessions `79875`, `28175` or `65113` remain live.
The isolated development worktree `/tmp/vtensor-next` remains available, but
its VT-07 changes are integrated into `/home/alex/Desktop/vTensor`. It uses
its own native Rust target directory because compiled infrastructure can
retain absolute manifest-root paths. The content-addressed standalone C oracle
cache is shared safely by digest; no Rust target cache is shared across roots.

## VT-08: complete candidate-source accounting

The source gate now enumerates tracked and untracked non-ignored source
files. Its new regression test first failed because the tracked-only scan
omitted `src/new.rs`, then passed after the scan was strengthened. Imported
and generated bindings are validated independently before measurement: any
missing or drifted bound file fails even when source-budget headroom remains.
This includes the imported checker's own implementation. Tests cover both
recognition of exact imported bytes and rejection of altered bytes.

The browser frame was pinned with its existing authoritative updater to
`UOR-Foundation/template@b64cc73c94773ed5bcc9d0549b1ae2c836fa1e0b`.
All 52 files were fetched from that immutable upstream revision and verified
against its manifest before writing. Their bytes are unchanged from the
inherited frame. The source pin is registered in `model/authorities.toml`,
and `just model` now runs the frame's file-integrity check as well.

`oracle-import.lock` is derived from the existing GGML import manifest and
contains only its six exact authoritative files. The oracle-import conformance
check requires that exact derived inventory. A planted extra exemption for
our own `contract.rs` was rejected. A planted untracked Rust file was also
rejected by the actual source gate. Both defects were restored.

New test drivers, the test-only Verdant adapter and the C oracle transport
are consolidated into the existing `crates/conformance/tests/contract.rs`.
They remain handwritten, counted validation infrastructure; they are not
misrepresented as generated code. The provisional standalone Verdant adapter
crate was removed, with a backup in `/tmp/vtensor-pre-consolidation`.
All eight existing contract/product tests still execute. The native oracle
is initialized once per test process; its cache binds only the native oracle
module and transport, so unrelated test edits do not force a native rebuild.

Both SDK-produced Rust certificate-source artifacts are now included in
`layout-kernel.lock`, alongside the generated package sources. Their inventory
is checked against the certified bundle, preserving all SDK bytes.

Restored focused results: ten source-audit unit tests, four BDD meta-tests,
eight contract/product tests, and full workspace Clippy passed in the pinned
SDK. The eight contract/product tests take 2.00 seconds with the native cache
warm. Source accounting now includes all current candidate files and reports
8577 handwritten lines in 51 files versus 13423 in 77 baseline files.
No exemption for handwritten code or unsupported-operation allowance was introduced. Imported code is excluded only through exact, validated provenance bindings.
A complete `just vv` for this newer checkpoint remains outstanding.

Current focused commands use `cargo test --locked -p repo-conformance --test
contract`; filter by the existing test name when iterating. Earlier per-file
commands in this document describe the historical layout. The previous
"Remaining source-accounting integration work" section is superseded by VT-08.
The separate worktree `/tmp/vtensor-next` is now an older snapshot and must not
be copied over the primary worktree.

## VT-09: Prism transfer admission against upstream set/get

`app/src/BufferTransfer.lex.tex` defines admission for a live tensor from its
observed buffer/data state, logical byte span, offset and transfer size.
Missing buffers are refused before zero-length handling. With a buffer, a
zero-byte request is a no-op regardless of allocation and offset. Nonempty
requests require data and a mathematically in-bounds range. The subtraction
form of the bound check avoids unsigned addition wrapping. Universal
source theorems establish missing-buffer refusal and zero-length behavior
without additional axioms.

The exact SDK generated the `vtensor_transfer` Rust-core package in certified
bundle `5a2ed31380e4a53a7a4c5aa28eb2e23e3c2d1966f070c405c4eb0079dc4d0f64`, attestation `6d3b8fd63ceb69fd4f145f37ff58a0e409e84fbd7e4fbf48761b339b12a22bea`.
The same bundle retains the storage-span and type-catalog roots. Conformance
binds every requested root and every generated Rust source, including the
certificate-source artifacts; no kernel implementation was handwritten.

The test-only native oracle now also compiles unmodified upstream
`ggml/src/ggml-backend.cpp` from the already pinned archive. Its transport
calls the actual `ggml_backend_tensor_set` and `ggml_backend_tensor_get`
functions. Callback markers observe successful dispatch; GGML's own abort
callback reports guard refusals in forked children. Ordinary and view tensors
are covered, with actual backing allocations for non-null data observations.
No reference guard is copied into the test transport.

VT-09 compares 1024 calls over set/get, views, buffer and allocation presence,
empty spans, boundary offsets and out-of-bounds ranges. Additional generated
checks reject overflowing ranges, which are not sent through the upstream
unsigned-add assertion. A planted transport defect forwarded offset zero
instead of the requested set offset. VT-09 failed at transfer 213, comparing
the modeled out-of-bounds result 12 with admitted result 1. Restoring the
transport restored all nine contract/product tests. BDD, model validation,
source accounting, frame integrity, workspace Clippy and dependency checks
also passed in the selected SDK. The final backing-allocation refinement
passed all nine tests and workspace Clippy again. The complete `just vv`
checkpoint remains outstanding.

This is transfer admission, not allocated-buffer ownership, byte-copy
correctness, asynchronous synchronization or device registration. Those
remaining contracts must use the modeled decision rather than duplicate it.
The cached storage and transfer oracle executables share the pinned source
closure and are built once per input identity; certificate replay and oracle
setup are shared by all three generated components in the contract test.

Upstream investigation was refreshed: Verdant `origin/main` has advanced to
`0b07aca` (v2.2.0). Its tensor changes add lifted scalars and retain the paged
arena, but the inspected public API still does not provide a general GGML
operation executor or device backend. The vTensor dependency pin is unchanged.
No source or publication change was made in the user's Verdant worktree.

## VT-10: certified tensor placement against upstream allocation

`BufferAllocation.lex.tex` defines placement for an aligned buffer and an
aligned allocator offset: validate a nonzero power-of-two alignment, pad the
requested allocation size, check remaining capacity, and return the tensor
offset, padded size, and next offset. Invalid alignments and offsets are typed
decisions; arithmetic overflow is the certified renderer's `Overflow` result.
This plans placement; it does not allocate or own a runtime buffer.

The Prism MCP checked the model. The official `prism-code axioms --write`
audited all 42 declarations and set the exact observed axiom policies. The
pinned SDK exported all four roots together in certified result
`9530e8333b6181a4809b005bedeadca1565ab29b93fd27f953158559a419b364`,
attestation `5bdbfc02d4e42dea4481b85929658c0caa3a866fbff668b450272169fa588781`.
The bundle and every generated Rust artifact are bound in `layout-kernel.lock`.

The native oracle additionally compiles unmodified upstream `ggml-alloc.c`
and `ggml-backend-meta.cpp`. Its transport calls `ggml_tallocr_new` and
`ggml_tallocr_alloc` on real aligned backing allocations. It observes tensor
addresses and allocator offsets, and identifies upstream capacity aborts
through the abort callback. Other aborts fail the harness. A zero-size buffer's
base is taken from GGML itself, which reports null, rather than the transport's
unused allocation. No placement formula is copied into the transport.

The test compares 864 cases across eight alignments, three initial offsets,
six capacities, and six sizes. Additional model cases cover invalid alignment,
misaligned offsets, maximum unsigned values, overflow, and alignment 2^63.
All ten contract/component tests passed in the selected SDK. The new
capability's initial test failed before its model was added. A planted defect
reset the native allocator offset to zero; VT-10 rejected placement 36, where
capacity=0, offset=1, size=0, alignment=1. The transport was restored.

A license clarification for the unchanged SDK package binds its exact
`src/lib.rs` hash `0xa7d2a978`; an initial wrong hash was rejected as unlicensed.
The export recipe now prints its SDK diagnostic receipt on failure rather
than discarding it. Native oracle initialization and certificate replay remain
shared within the component test process.

The inspected LexLean production specification explicitly excludes FFI,
pointers, threads, floating point and external I/O from its certified Rust
calculus. Its pure planning certificates do not establish native effect
correctness. Verdant's tensor crate also forbids unsafe Rust. A mutable aligned
byte owner can still use safe overallocated storage, but the native ABI effect
boundary needs explicit modeling and independent validation.

Verdant upstream is now `d8dc674b317efcd03da11dd2a72600796d803e35`
(v2.3.0). An isolated, clean detached worktree at `/tmp/vtensor-verdant-buffer`
was created for buffer capability work; no source was changed in the user's
Verdant checkout. vTensor's production dependency pin is unchanged.

## Complete VT-10 integration checkpoint

The full `just vv` completed successfully inside the exact SDK selected by
`prismpm.lock`, including fresh Prism application verification, generated-kernel
synchronization, formatting, model/source accounting, frame integrity, template
and SDK lock checks, workspace Clippy, the complete workspace test suite, and
dependency checks. Application attestation:
`fc3c75dc3a277b068db8182333edaf698a47a1e9f1f6e15dc359e0ddd4eca942`.
All ten component tests passed in 1.76 seconds with a warm native cache.
The source audit counted 8894 handwritten lines in 51 files against the
13423-line, 77-file parent baseline.

Before this run, the full workspace suite exposed two scaffold tests that
copied product authorities into an otherwise empty scratch model. The frame
authority referenced absent FR IDs. The fixture now creates mutually
consistent empty ID, authority and ledger registers. The production model
validation was not weakened; all 62 xtask tests passed after the repair.
The new scenario also now carries the required `@VT-10 @build` tags.

This acceptance checkpoint covers the registered component claims only. It
does not establish a native GGML backend. The isolated Verdant worktree now
contains a mutable aligned byte owner and its own certified storage planner;
that work remains separate from vTensor's unchanged Verdant dependency pin.

## VT-11 native device and buffer integration

The SDK exported the separate NativeBackend project as result
`db2a2fe0d030bbe895777a46fce0e55032954f740596c0d8f4ca797afba602da`,
attestation `f752320b09f53e6db73db105af24f5ba472423733cd9d4859eef3e83c1eb4ff1`.
Configuration, memory planning, and metadata admission are certified roots;
`native-kernel.lock` binds their exact SDK outputs. Native lifetime and pointer
effects remain outside the pure certificate and are tested independently.
The existing conformance package now builds the native `vtensor-backend`
rlib/cdylib. BDD tooling moved into the development-only model package without
adding handwritten source paths.

The oracle compiles unmodified pinned GGML frontend sources and loads the actual
Cargo-built shared library. It compares all recorded Rust ABI sizes, alignments,
and field offsets with the authoritative C headers. It exercises owned, empty,
and borrowed buffers, allocator placement, zero initialization, clearing,
set/get, partial memset, 2D transfers, views and buffer copies over three layouts
for each of 36 live storage types (108 cases). Typed invalid allocation and
initialization paths and an upstream abort for out-of-bounds access are included.
It registers exactly one device and no CPU compute backend. A real metadata
graph succeeds; numerical ADD is explicitly refused.

VT-11 originally failed before the native entry points existed. A planted
setter defect changed the destination offset to zero. Running
`cargo test -p vtensor-backend --locked --test contract native_backend_vt_11 -- --exact`
inside the pinned SDK failed the upstream 2D gather comparison:
`native check line 225: !memcmp(gathered, columns, 3)`.
Restoring the setter passed. Separately, replacing the shared library with that
older defective build while keeping the current Rust test executable failed
before behavior: `expected_source.size() == vtensor_build_sources_size()`.
The current library was restored. An incidental lockfile-format change also
correctly triggered `Rust test library is stale`; model semantics were identical.
The test binds live inputs to both the Rust test library and the loaded DSO,
and checks the loaded library path. Cached native objects and executables are
digest-checked, while each oracle execution remains live.

All 11 contract tests pass. Workspace tests, Clippy, model/source accounting
and dependency checks pass. Source accounting is 10076 handwritten lines in
51 files before documentation/export-writer updates, below the 13423-line,
77-file parent baseline. The generated native packages need explicit licensing
clarifications because their renderer omits metadata; each clarification binds
its exact unchanged source hash. Incorrect hashes failed the license gate.
The export writer now handles layout and native scopes without reproving the
other project. Replaying it against the existing SDK results preserved both
locks semantically and all certified source bytes.

This is not full acceptance: a new full `just vv` remains required. Numerical
graph kernels, all-operation conformance, complete numeric type behavior, and
production dependency packaging remain open. The isolated Verdant byte-owner
prototype is not integrated and is unnecessary for GGML-owned host buffer glue.

The native ABI adapter now refuses negative signed extents instead of silently
mapping them to the model's empty natural-number case. The new direct buffer
initialization check failed before this correction and passed afterward. Empty
nonnegative extents remain covered by the upstream comparison. This boundary
refusal is not a claim that GGML's byte-span helper rejects every malformed
negative descriptor.

## Complete VT-11 integration checkpoint

The full `just vv` completed successfully in the exact vTensor SDK. It freshly
verified the Prism application, synchronized the kernel, and passed formatting,
model/source audits, frame and template integrity, SDK lock validation,
workspace Clippy, every workspace test, and dependency checks. All 11 contract
tests passed. Application attestation remains
`fc3c75dc3a277b068db8182333edaf698a47a1e9f1f6e15dc359e0ddd4eca942` because
its model inputs are unchanged; the separate native certificate above is
replayed by VT-11. Source accounting is 10081 handwritten lines in 51 files.

The next numerical capability is being developed in an isolated Verdant
worktree `/tmp/vtensor-verdant-execution` based on upstream v2.4.0 commit
`87c3256326b9ec9f8d591855394c0bd0265588ef`. It adds general row-major F32
matrix products through Verdant's existing uor-matmul substrate. Its separate
Prism shape planner result is
`cdfaec443ae65a74ea981529eda0c0efc0d01f5aa32c8e473c12917a35dc7d1e`,
attestation `2bc0a642221aa22b601885422dccf2ae0856336a19c228710cec44dc1bc3e3b6`.
The planner proves shape behavior only; numerical behavior is independently
compared with unmodified GGML `ggml_vec_dot_f32` from the same pinned source.
All 624 observed contractions match (616 finite/empty contractions and eight
IEEE edge cases, comparing NaNs by class). A planted zero-output defect failed
the numerical comparison; a canonical model restriction to rows <= 1 failed
certificate freshness with `stale matrix model: source_id`. Both were restored.
Five Verdant Tensor crate tests, focused Clippy, dependency checks, and model
validation pass. The wider Engine conformance test is building separately.
This extension has not passed Engine's full acceptance/fidelity gates and is
not vTensor's production dependency. No general GGML numerical graph operator
is claimed implemented by this checkpoint.

## VT-12 numerical graph development checkpoint

The first whole-graph test failed at `ggml_backend_dev_supports_op(dev, product)`:
the native device correctly refused the unimplemented MUL_MAT operation.
MatrixGraph and TensorIndex were then checked through Prism MCP and exported
by the exact vTensor SDK. Matrix planning result:

- `matrix` result `8f602a05182492a3ed71403a1b2e6d5d124a7a92677a698caf34a948aca1ab8e`, attestation `b62d38cd630499f83e702402228a475697b4355b145d67a37814cf94b751ec72`.

- `index` result `bae0387d48ecd02d335b1566df1e463bc9f418c57c95cc715eeaf97f337c7c21`, attestation `86b37bcc7d4f9be5c0f14e6e55c4c85e0bf281eb5e8283791e8256dbbfb1dc61`.

Their exact outputs are bound in `matrix-kernel.lock` and `index-kernel.lock`.
MatrixGraph defines F32 operation admission, output shapes, batch divisibility,
broadcast ratios and workspace extents. TensorIndex defines byte addressing
from four indices and strides. The native adapter uses the generated address
function instead of a second offset formula. Graph preflight retains each
numerical plan before execution; unsupported tails fail before prior nodes
write output. Buffer bounds are admitted before decoding native F32 values.
All matrix arithmetic is delegated to Verdant's general matrix primitive.

The oracle now compiles GGML's complete CPU reference using the pinned SDK's
C/C++ compilers. The SDK lacks CMake; the common CPU source list is read from
upstream's own CMake file and the matching architecture sources are added.
No upstream source is modified. The CPU backend is linked only into the test
host. It computes the same real GGML graph and supplies every expected value;
the transport does not duplicate multiplication or batch-mapping formulas.
Only vTensor is registered as a selectable device, including after reference
execution. The production shared library does not invoke the CPU reference.

All 640 graphs match: four row counts, four column counts, five inner sizes,
two storage layouts and four batch configurations. This includes empty
products and eight graphs containing NaNs, infinities, signed zero,
subnormals and overflow. NaNs are compared by class. Negative checks reject
shape mismatch, absent sources, missing nonempty output buffers, foreign
buffer contexts and a numerical prefix followed by unsupported ADD. The last
case checks that the output sentinel remains unchanged. Kernel admission is
also compared with every imported operation declaration.

A planted broadcast defect used modulo instead of the certified grouping
ratio. The matrix test failed its upstream output comparison. Restoring the
adapter passed. Cached oracle source/header inputs are now checked against
fresh bytes from the authoritative archive, including lengths and SHA-256.
Appending one newline to cached `ggml/src/ggml-cpu/ops.cpp` failed with
`cached upstream source differs: ggml/src/ggml-cpu/ops.cpp`; restoring it passed.
Cached objects and executables retain their separate digest checks.

The expensive reference build also binds its own implementation. Rust token
normalization ignores formatting and ordinary comments while preserving code
and string literals, so formatting no longer needlessly recompiles GGML.
A comment-only probe passed with the native executable's exact modification
time unchanged. Cold compilation took about 40 seconds; the warm matrix test
completed in 5 seconds, including live certificate and numerical comparisons.

Model/source audits, workspace Clippy and dependency checks pass. Eleven of
twelve contract tests pass. VT-02 intentionally still refuses the local
Verdant override; its diagnostic now explains that a development override is
not release evidence. This is not full repository acceptance. The unchanged
upstream v2.4.0 base passed both existing Verdant API tests before the override.
The current source dependency remains unpublished and must be resolved before
release; other numerical operators and numeric type behavior remain open.

In the isolated Verdant extension, the generated planner is a standalone
SDK-certified dependency, verified by live certificate replay rather than
retranslated as native workspace code by uorust. The native duplicate
`verdant_core::math::pow2_u64` is now re-exported from Verdant Tensor, preserving
its implementation and public path. The actual native frontier remains at
203 lifted, 1192 residue, 73 foreign and 78 effect functions, exactly within
its unchanged ceilings. The crate tests, VE-60 conformance and focused Clippy
pass after this change. No ratchet limit was raised.

The standard Engine ratchet workflow progressed through model/fidelity and
frontier refresh, then failed `verdant-uorc.wasm` initialization. A controlled
rebuild restored all modified production Rust and Cargo files to upstream
HEAD, built the decoder in the same SDK, then restored every extension file
byte-for-byte. Both the unchanged-source build and the extension build return
0 from initialization; the binary committed at upstream HEAD returns 1.
Thus the initialization failure is reproducible from upstream source in this
SDK and is not evidence that the matrix extension caused it. Full Engine
acceptance remains unproven; the failure has not been bypassed.

The final workspace run used `cargo test --workspace --locked --no-fail-fast`
to observe every target without suppressing the known source-pin failure.
Every other workspace target passed; the contract target reports eleven
passed and the one explicit unpublished-source refusal. Model/source audit,
Clippy and dependency checks passed. Current accounting is 10492 handwritten
lines in 51 files, within the unchanged parent baseline. The restored numeric
matrix test passed after the broadcast and source-cache defects, and the
comment-only cache probe reused the exact native executable. These results
remain development evidence, not a new full `just vv` acceptance checkpoint.

## Initial reproducible GitHub checkpoint

The initial Matrix-only Verdant source checkpoint was preserved as Git commit
`8e4e4ed670f3ea8337e7f66ae971f7b37d8cc2b2` in
`vendor/verdant-development.bundle`, based on upstream
`87c3256326b9ec9f8d591855394c0bd0265588ef`. Its SHA-256 and source metadata
are in `model/verdant-development.json`. `just verdant-prepare` verified and
materialized this exact commit into a clean `.verdant-engine` checkout. A
second invocation verified the existing checkout without changing it. Cargo
explicitly excludes that nested workspace from vTensor's own workspace.

This removes the machine-specific `/tmp` dependency from the repository.
That initial bundle included the source, imported reference, Prism model and
exact certified output needed to reproduce the Matrix primitive; unfinished
scalar work and unaccepted Wasm binaries were excluded. The development override
remains visible and the exact-source release guard remains enforced.

## Initial pointwise model checkpoint

VT-13 initially received its registered requirement, scenario and executable imported-oracle
comparison before runtime implementation. The fixture covers 120 F32 graphs
across eight operations, contiguous and padded storage, ordinary and exceptional
bit patterns, and binary broadcasting. The independent CPU reference uses
upstream contiguous copies for operations whose CPU kernels require them.
The nested GGML unary-operation enumeration is now checked against the imported
header alongside operation and type inventories. Existing unsupported-tail
tests use GGML_OP_COUNT so implementing ADD does not invalidate their purpose.

The exact pinned SDK exported PointwiseGraph.planPointwise and rightOffset as
result `3b2823fd4173282753c1e08b4ff1de1b4baa2a8969340a4ba3a285c0b0084ae6`,
attestation `3a34ab6ffdc535b81871c419bd415dd331d12fca4e42604136099ad82eaa7245`.
The generated packages and evidence are preserved unchanged and bound by
pointwise-kernel.lock. Live source identity, artifact replay and compilation
passed in the VT-13 test; the test then failed at the intended native
supports-operation assertion before arithmetic execution was implemented.
`cargo check -p vtensor-backend`, formatting and `just model` passed.
That was an implementation checkpoint, not a passing capability or release.

The pointwise packages also pass `cargo deny --all-features check`. As for
the existing certified packages, their omitted SDK license metadata is
clarified under this repository's license using exact generated-source
hashes; the certified package bytes remain unchanged.

## Certified F32 arithmetic graph checkpoint

The current Verdant extension source is Git commit
`ce916e89450986f78c2ba13df1d6954a39e2ffa3`, preserved in the hash-bound
`vendor/verdant-development.bundle`. It contains the Matrix primitive and the
new ScalarF32 model, certified export, thin bit-conversion binding and imported
GGML scalar reference tests. The prepared `.verdant-engine` checkout is clean
and matches that commit. The checkpoint excludes the unaccepted Wasm changes
in the isolated development worktree. Its upstream base remains the explicit
v2.4.0 revision; upstream main `c57a687722204d5e1c0153eae45bf1d73e3dc8b5`
was inspected for newer lifecycle facilities. Integration IDs VTK-01/VTK-02
avoid upstream's independently allocated VE-60/VE-61.

Verdant's ScalarF32.evaluate result is
`9f40d825356dc94901322a57f4c442ad8c0d3cd35ecf7c6eaae7daf679193e79`,
attestation `5f4a83fe7dcdbcf3eafc7c3fac709485773a8bcc00bef0e8e53d3f8ceedf9514`.
All scalar arithmetic is emitted from Prism's integer encoding: classification,
normalization, guard/round/sticky bits, nearest-even rounding, subnormals,
signed zeros, infinities and NaNs. Native binding code converts bits and typed
failures. Four theorem examples remain checked. Initial constant/dispatch
representations exceeded the SDK preservation kernel's recursion bound;
inlining constants and introducing a typed operation enum resolved this
without raising limits, removing a theorem or disabling an operation.

The independent primitive reference compiles the pinned, unmodified GGML vector
functions. It compares 10,592 scalar cases, including every pairing of 24 edge
encodings for binary operations and deterministic bit-pattern samples. Matrix
and scalar tests share one memoized reference executable. Its identity binds
the imported archive, transport, compiler/flags, SDK, architecture and normalized
builder tokens. Executable hashes are checked and comparisons always run live.
The explicit 624 and 10,592 counts reject shortened corpora. Eight Verdant crate
tests, both VTK conformance cases, model validation, Tensor/Core Clippy and all
dependency checks passed. The fresh frontier projection remains 203 lifted,
1192 residue, 73 foreign and 78 effect functions, within unchanged ceilings.
Full Engine acceptance and final ratchet synchronization remain unproven.

The final vTensor planner composes MatrixGraph and pointwise admission in Prism.
It returns a single typed graph plan; native code no longer selects between
independent handwritten planner branches. The updated pointwise result is
`7c5c1c018e71a9afcb285c2a328db8442538cf8a354a2fd3d5ad21a3c2f66815`,
attestation `8e97d6f7b213f7b75a9fd25e284f28a5fac01d2e77d2bcccc0fd2d3511c4c066`.
The bundle and generated packages are byte-bound by pointwise-kernel.lock and
replayed through the official SDK. Unchanged ScalarF32, layout, native, matrix
and index roots were not re-exported during the planner composition.

VT-13 executes 120 actual GGML graphs for ADD, SUB, MUL, DIV, SQRT, SQR, NEG and
ABS. Cases vary dimensions, padded input strides and binary broadcasting, with
ordinary and exceptional encodings. The upstream reference uses its own CONT
nodes where CPU unary kernels require contiguous inputs. A shared constructor
creates actual and reference operations; explicit imported opcode and unary
variant checks ensure every intended operation is present. Missing binary
operands, incompatible repeats, inconsistent output dimensions, invalid types
and invalid unary variants are refused; exercised failures preserve sentinel
output bytes. Only the vTensor device is registered as selectable. Finite values
and signed zeros compare by encoding; NaNs compare by class.

Falsifiability checks observed and restored:

- Adding one bit to Verdant's returned encoding failed the direct scalar oracle
  at positive-zero addition (actual 1, expected 0).
- A canonical model admission change refused the final valid operation; the
  compiled export was rejected as `stale scalar model: source_id`.
- Forcing the right tensor's row coordinate to zero failed the actual graph
  oracle with an elementwise ADD mismatch.
- Tampering one byte of the loaded native source snapshot failed the reused
  C++ host's live memcmp provenance guard. Original library bytes were restored.

Backend code edits no longer rebuild the C++ oracle host: its key binds the
loaded library path, ABI observations and verification implementation, while
the executable verifies that path and live source snapshot on every run. The
restored graph run reused unchanged oracle executables after the planted
broadcast defect. This removes a redundant rebuild without bypassing the
wrong/stale-library checks.

The source ratchet initially rejected native-glue growth. Moving dispatch into
Prism, sharing configuration callback bindings, removing redundant test-only
adapters and sharing duplicate fixture/metadata helpers preserved all cases
within 10,592 handwritten lines and 51 files, exactly the parent ceiling.
The final full workspace observation reports twelve passed contract tests and
one explicit failure: VT-02 refuses the unpublished development dependency.
All other workspace targets passed. Formatting, model/source audits, frame
synchronization, workspace Clippy and dependency checks passed. The full
`just vv` acceptance boundary remains unmet; the production-source guard and
Engine gate have not been weakened. These are finite component observations,
not full GGML conformance or exhaustive IEEE equivalence claims.

## Empty-node admission checkpoint

The imported `ggml/src/ggml-cpu/ggml-cpu.c` returns from
`ggml_compute_forward` when a tensor is empty, before operation-specific
execution. vTensor previously sent empty copy operations through unsupported
operation and operand checks. NativeBackend.decideNative now receives the
certified output byte span and defines a valid zero-span node as a no-op before
source descriptors or numeric workspace are accessed. Its `emptyOperation`
theorem proves the zero-span result for an arbitrary operation identifier.

The exact SDK NativeBackend export is result
`395dacf6872bd25d8a470992f92117d771fa363efc696100d3c54a2585d53fe3`,
attestation `60cacbe2623da9b5d3747ee11fbc1e00fb5228d3f1d60f53ef50a6b1128eb8d7`.
The native bundle and source lock were updated unchanged from the SDK output.
Existing configuration and memory semantics are preserved. No scalar,
pointwise, matrix, index or layout export was repeated for this change.

VT-14 compares actual GGML CPU and vTensor execution for 864 empty graphs:
36 live storage types, four empty axes, ordinary and INT32_MAX remaining
dimensions, and DUP, CONT and CPY. Every graph is checked to contain one
executable node, preventing a vacuous empty graph pass. Inputs and outputs
remain unallocated with null data throughout both executions. The reference
is the unmodified CPU backend compiled from the existing pinned archive.
These cases establish empty-copy behavior; nonempty copy execution remains
part of the completion scope. Existing nonempty invalid-operation tests remain
active in the full regression run.

The initial test failed at `ggml_backend_dev_supports_op(dev,result)` after the
CPU reference accepted the empty DUP. After implementation, all 864 cases
passed. A planted native binding defect passed span+1 into the certified
predicate and failed at that same support check. The binding was restored
before the workspace regression.

Repeated JSON/TOML fixture readers now share path-reporting helpers inside the
source-bound oracle module. Their code remains part of the normalized build
identity. Fixed configuration words use typed array construction for their
little-endian ABI representation. These changes preserve the imported cases
and bring handwritten accounting to 10,591 lines in 51 files, below the
unchanged parent ceiling.

The final workspace observation has thirteen passing contract tests and the
one explicit VT-02 development-source refusal. All other workspace targets
passed. Formatting, model/source audits, frame synchronization, workspace
Clippy and dependency checks passed. The full release acceptance remains
unproven and no guard was weakened.

Upstream Verdant main has advanced to v2.7.0 commit
`8f560d975e12fd6f599a51c4d78487b1e1f3c710`. Inspection found its in-Verdant
ratchet-cycle and lifecycle orchestration in `lifecycle.rs`; its current stage
paths and fidelity commands target the Engine repository. They are useful
references for the remaining automation work, not an already validated
vTensor integration. The prepared dependency remains the explicitly recorded
scalar/matrix checkpoint until an updated source is validated.


## VT-15: same-type storage graphs through Prism and Verdant

VT-15 was registered with its scenario before execution support. The initial
oracle run failed at `ggml_backend_dev_supports_op(dev,result)` after the CPU
reference accepted the first nonempty copy. No previously unsupported numeric
family is presented as completed by this checkpoint.

The Prism PointwiseGraph model now exposes a typed graph request containing
tensor type, shape, strides and byte span. Its copy decision looks up the
imported storage catalog, checks block alignment and equal element counts,
and supplies paired source/destination addressing contexts. The native
adapter executes those plans and delegates each encoded block to Verdant's
safe `memory::copy_storage_block` API. One scratch block is reused per node;
no block allocation occurs inside the element loop. Empty nodes retain their
separate certified no-op path before operand inspection.

The exact pinned SDK exported PointwiseGraph.evaluate and rightOffset with
result `b1c67d28021623aa81d54fafd11573d920f2fed965606723a00cfba9d52ff9b4`
and attestation
`c5403093084c840a1148766af0fa6dc9a47154813fd3de00bbdb72b728224187`.
The export preserves the SDK-produced source and proof artifacts. The new
closed examples check padded and reshaped block offsets; they are not a
universal proof of all descriptor behavior.

The source bundle now reproduces Verdant checkpoint
`26ba1eaa1369e9f6539c3ca40ec9124103d666e4`. Its storage primitive admits
lengths through a separate certified Prism model, preserves the Tensor
crate's `forbid(unsafe_code)`, and rejects unequal lengths before mutation.
The primitive's imported GGML vector-copy comparison covers 1,539 cases and
393,984 bytes, including zero length and guard bytes. A planted flipped byte
failed with `GGML storage byte 0`; the restored ten Tensor tests pass.
Tensor/Core Clippy, Engine model/format and dependency checks pass. A fresh
MIR projection retains 203 lifted, 1,192 residue, 73 foreign and 78 effect
functions. The existing Engine Wasm release blocker remains unresolved.

The native oracle covers all 36 live GGML types, three operations, four
layout variants, two block counts, two row counts and two batch counts:
3,456 cases. Cases include noncontiguous block/outer strides, reshaped CONT
and CPY, raw quantized encodings, and whole-allocation comparisons preserving
padding. Malformed output shapes and missing required operands are refused
before writes. Type conversions remain in the completion scope.

The pinned upstream `ggml_compute_forward_dup_bytes` contiguous-destination
branch multiplies `ne[0]` by type size without dividing by quantization block
size for noncontiguous sources. Inspection also found this branch unchanged
at upstream commit `919794500475862f6768c353255469dc661cb734` in
`ggml/src/ggml-cpu/ops.cpp`. The reference therefore uses upstream CPY to a
padded `[blockSize,totalBlocks]` destination followed by same-shape CONT.
This selects the upstream generic block-copy and row-size paths, providing
a flat logical-byte oracle without editing upstream source or excluding
quantized layouts. vTensor's actual operation and destination retain the
layout under test.

Shared typed ABI declarations preserve every imported field's order and
signature. Digest helpers centralize cached-object checks; every behavioral
comparison and native source-snapshot comparison still executes live.
The source audit measures 10,583 handwritten lines in 51 files, below the
unchanged parent ceiling. The complete release gate remains unproven;
VT-02 continues to reject the development dependency override.

The graph-level planted byte fault failed at
`copy mismatch type=0 op=0 layout=0`; the source was restored before regression.
The final workspace run passed every other target and fourteen contract tests;
VT-02 alone failed with the explicit production-source refusal. After adding
the missing safety comment, the complete native contract target reproduced
fourteen passes and that same refusal in 5.96 seconds. Model and source audits,
formatting, frame synchronization, workspace Clippy and all cargo-deny checks
passed. The final source audit is 10,583 lines in 51 files. The full release
`just vv` remains unproven while the development dependency is refused; no
unchanged expensive main-app proof was repeated for this component checkpoint.


## VT-16: modeled four-axis REPEAT addressing

The VT-16 register entry and behavioral scenario preceded execution support.
The imported CPU reference accepted the initial graph; vTensor failed at
`ggml_backend_dev_supports_op(dev,result)`. The test shares the copy fixture,
its source-bound oracle build, and its complete output/padding comparison.
It exercises all five types implemented by the pinned CPU REPEAT dispatcher:
F32, F16, BF16, I16 and I32. The 160 cases vary source width, rows, batch count,
padding and repetition across all four dimensions. A contiguous upstream copy
normalizes strided sources before the CPU REPEAT, preserving the same logical
encoded values while admitting layouts that its direct REPEAT kernel cannot
read. The actual vTensor graph retains its source strides and padded output.
Nondivisible repeat extents and a missing source are checked before writes.
The unchanged copy cases retain unequal-element-count and missing-operand
checks.

Prism extends the existing block-copy decision with an explicit repeat mode.
It admits exactly the five CPU types, requires matching output type and
per-axis divisibility, then maps output coordinates modulo each input extent.
The shared index context now carries the fourth source extent. A closed
four-axis offset example checks a padded source address of 124 bytes.
Verdant's already certified storage transfer primitive remains the execution
substrate, with one reusable block per node.

The Prism MCP draft is
`prism-app/f9f4575eeafa88772f53e067a19d99a1c6285359948e6e8ad50da491a893232b`.
The remote check returned `engine.check failed: project-refused`; the exact
pinned SDK accepted the four source modules. Its production export is the
required source of the executable artifact and certificate.

The adapter now shares initialized workspace allocation between owned buffers
and graph scratch, retaining one initialization pass and the existing typed
allocation/overflow errors. Owned and borrowed buffers also share GGML wrapper
construction without changing their ownership. This deduplication is covered
by the existing allocation, host-buffer, transfer and graph oracle cases.

The SDK export completed with result
`857786898ba3003611cff6956133153116a192accfa99c1b952dfb3671e87c93`
and attestation
`26457a9196171ecab5291d964f5706d3a818cb32606370423fbce54ecaa28bb1`.
The exact generated bindings carry the repeat mode and fourth source extent;
the imported certificate and source freshness checks remain active. The
source audit measures 10,579 handwritten lines in the existing 51 files.

The initial certified execution passed all 160 repeat cases in 4.05 seconds.
A planted adapter fault replaced every repeat source offset with zero, which
stays inside the admitted source allocation. The live reference comparison
rejected it at `copy mismatch type=0 op=0 layout=0`; the binding was restored
before the workspace regression. This demonstrates detection of incorrect
repeat addressing, independently of the earlier byte-transfer corruption test.

This is the CPU-oracle repeat checkpoint, not completion of general storage
coverage or the whole backend. Other repeat types, copy conversions and the
remaining imported operation families remain within the original goal.

The final workspace regression passed all other targets and fifteen native
contract tests in 10.83 seconds; VT-02 alone retained the expected refusal of
the unpublished Verdant development source. Formatting, model/source audits,
frame synchronization, workspace Clippy and dependency checks passed. The
source audit remains 10,579 lines in 51 files, below both the gate's Git-parent
ceiling and the preceding checkpoint's own size. No release gate was weakened
and no unchanged main-app proof was repeated.


## VT-17: CONCAT source selection across all live storage types

VT-17's register, scenario and live CPU comparison preceded the executable
model extension. The initial test failed at
`ggml_backend_dev_supports_op(dev,result)` after the reference accepted the
first graph. CONCAT's shape/type rules come from the pinned `ggml_concat`
constructor in `ggml/src/ggml.c`; byte movement is compared with the pinned
CPU implementation in `ggml/src/ggml-cpu/ops.cpp`.

Prism extends the shared copy mode with a typed concatenation mode carrying
its axis and right source's addressing context. Admission requires matching
types, a valid axis, matching nonconcatenated dimensions, summed concatenated
dimensions, and block-aligned leading extents. Every output block's source
selection and source/destination addresses are evaluated by Prism. Native
glue retains the required source buffers; Verdant performs the encoded-byte
transfer through the existing certified storage primitive. Closed examples
cover an inner-axis boundary, an outer-axis boundary and an empty left side.

The fixture exercises all 36 live types on all four axes, four layout variants,
three widths, three row counts and two batch counts: 10,368 cases. Inputs have
unequal concatenated extents and distinct encoded bytes. Cases include empty
left and right operands, entirely empty graphs, strided block/outer storage,
and padded outputs. The actual graph retains its layout. The CPU reference
normalizes each input through the previously validated padded CPY/CONT path,
then uses upstream CONCAT. This avoids the already documented upstream
quantized contiguous-copy stride bug without editing or excluding its source.
The existing copy and repeat cases retain their original parameter sets.

The test also checks wrong output shape, a missing right source and an invalid
axis before writes. Shared native/storage oracle queries now use the same
pipe-safe batch runner, including stderr capture and exit-status assertions.
Descriptor and span queries share the same model-backed metadata constructor;
this removes repeated shape conversion without changing typed errors.

The Prism MCP draft is
`prism-app/20f4fa65a03e50e464144a6d0e5170bd74715d52eb17f21e659cccfb89f4560a`.
Its remote check again returned `engine.check failed: project-refused`; the
pinned SDK accepted all four modules. A local authoring script syntax error
briefly started an export of the unchanged prior model. That process was
stopped, and only the corrected CONCAT source is used for this checkpoint.

The completed SDK export has result
`9283c9bfbccf8063281f8e1ad0b059814193d6c8a37877ecd0a7da7b7f3b0efa`
and attestation
`13193ed69b1a9bb56da91e18e361aea7ba6bf32c83bcbccacab1e277ddcacce1`.
The adapter bindings were checked against the actual emitted types before
running the graph oracle. Certified source and all proof artifacts remain
byte-preserved; license clarification binds the new emitted source digest.

Fixture tracing found a dangling temporary initializer-list range in its new
conditional extent selection. The fixture now owns one extent vector for the
whole run. The next run exposed upstream's requirement that even zero-byte
set/get calls have an allocated buffer. Empty operands and outputs therefore
perform no transfers; both the reference and vTensor graphs still execute and
are counted. After those fixture corrections, all 10,368 CONCAT cases passed
in 5.15 seconds. No type, layout, empty graph or oracle case was excluded.

A planted native binding defect swapped the first output block's source only
when both source buffers were nonempty. Its source offset was zero and valid
in either allocation, so the experiment introduced no out-of-bounds read.
The live CPU comparison rejected it at
`copy mismatch type=0 op=0 layout=0`. The adapter was restored before regression.

The final workspace run passed every other target and sixteen contract tests
in 13.00 seconds. VT-02 alone retained its explicit refusal of the unpublished
Verdant development dependency. Formatting, model/source audits, frame
synchronization, workspace Clippy and dependency checks passed. Handwritten
source remains 10,579 lines in the same 51 files. The changed model was exported
once successfully; unchanged expensive main-app proofs were not repeated.
This component checkpoint does not establish complete backend conformance or
resolve the existing Engine/release dependency blockers.


## VT-18: signed spatial storage graphs

The ID, scenario and live comparison preceded execution support; the initial
run failed at `ggml_backend_dev_supports_op(dev,result)` after the CPU accepted
the first nonempty graph. The scope includes the pinned GGML PAD variants
(`ggml_pad_ext` and `ggml_pad_ext_circular`), PAD_REFLECT_1D and ROLL, with the
F32 type implemented by their CPU oracle.

The Prism request now carries all sixteen raw 32-bit GGML parameter words.
Native glue preserves their bits as unsigned words; Prism interprets signed
padding/cropping and shifts from the imported int32 ABI. The model validates
per-axis output extents, reflection bounds and roll shift bounds. It defines
zero padding, circular wrapping, signed crops and reflection as source
addressing decisions. Zero padding selects a private zero block; Verdant's
existing certified opaque storage transfer remains the byte-movement substrate.
Circular padding and ROLL reuse the same wrapped-coordinate definition.
Closed examples cover signed wrapping, left cropping and both reflected edges.

The test has 2,256 spatial scenarios: 1,584 PAD scenarios, 128 reflected-padding
graphs and 544 roll graphs. PAD scenarios vary all four axes, asymmetric left
and right padding, signed crops, circular/zero mode, zero source dimensions,
and balanced INT32_MAX/INT32_MIN values. Reflection covers both boundaries;
ROLL covers zero shifts and every sign combination at the largest admitted
shift on each axis. The actual outputs retain padded strides. Sources use
contiguous and padded layouts; constructor-required contiguous sources for
reflection and roll are produced through vTensor CONT. The CPU reference uses
the existing upstream canonical-copy path and the same upstream constructor.

Two hundred circular PAD scenarios have an empty source and a nonempty
output. The CPU would divide by a zero extent; those graphs instead exercise
modeled refusal before writes. The fixture asserts the exact refusal count.
The other 2,056 scenarios execute the imported CPU oracle. Empty outputs still
execute and count both graphs without issuing invalid zero-byte transfers.
Wrong shapes, absent operands, mismatched crop parameters, negative reflection
parameters and out-of-range roll shifts have explicit refusal checks.

The shared compiler helper replaces duplicated command construction for
storage, allocation, transfer, frontend and CPU reference objects. It pins
C11/GNU and C++17 compilation, optimization and section flags and the header
search roots. Its implementation is inside the normalized source bound into
the oracle cache identity, so changing the helper invalidates affected cached
objects once. Source archive and object digests remain checked on cache hits,
and behavioral comparisons remain live.

The corrected Prism MCP draft is
`prism-app/e6f182c3b809f21268d2a1718ca2ba03a3970cf74b296e112568b667097c6d62`.
Remote checking again returned `engine.check failed: project-refused`. The
exact pinned SDK accepted the four source modules and is exporting the
executable implementation and proof artifacts.

The exact SDK export completed with result
`acdfccb715fa2518d0d26b223dcd5850d277030e1c1d872f122dfc70c19ce88c`
and attestation
`fbe80c42722c6b17bbb4d241b76a28d568a3138ba7c0e530e7066c1b85a2272f`.
The native bindings were checked against the emitted sixteen-word parameter
record and spatial copy-mode variants before execution.

The first data comparison differed for circular left padding INT32_MAX. The
pinned `ggml_wrap_around` computes `(coord + size) % size`; one added extent
cannot make a large negative coordinate nonnegative. This lets the CPU kernel
read before the source allocation. The model already uses a mathematical
modulo with a nonnegative source index. For the reference only, the fixture
now normalizes each circular left pad by the source extent and adjusts the
right pad to preserve their sum and the output shape. Signed normalized left
pads lie strictly between minus and plus one extent, where the upstream
helper is safe. The adjusted right pad is checked against int32 bounds.
The actual vTensor graph retains its original signed/extreme parameters.
No model or certified artifact changed for this reference correction, and no
original case was removed or upstream source edited.

With that equivalent upstream reference, all 2,256 scenarios passed in 3.38
seconds, including exactly 200 typed refusals. This is finite component
evidence for the declared F32 checkpoint, not proof of all storage-type
coverage or complete backend conformance.

Two independent planted native binding defects demonstrated detection. Changing
the private zero source from byte zero to byte one failed at PAD pattern 2,
shape `[0,0,2,1]`, output byte zero (`actual=1 expected=0`). Replacing admitted
roll shifts with zero failed at ROLL pattern 1, shape `[2,2,2,2]`, byte zero
(`actual=0 expected=172`). Both experiments stayed within admitted memory
regions, and the original adapter was restored before regression.

The final workspace run passed every other target and seventeen contract tests
in 9.83 seconds. VT-02 alone retained the expected production-source refusal
for the unpublished Verdant development override. Formatting, model/source
audits, frame synchronization, workspace Clippy and dependency checks passed.
Handwritten source is 10,577 lines in the same 51 files, below the preceding
checkpoint. The changed model was certified once; the unchanged main-app
proof and Engine release gates were not rerun for this component checkpoint.
The complete backend and production dependency requirements remain open.


## VT-19: bounded SET and ACC update views

The ID and scenario preceded execution support. The initial graph comparison
failed at `ggml_backend_dev_supports_op(dev,result)` after the pinned CPU
accepted the graph. The implementation covers CPU SET with F32/I32 left and
right storage (including raw mixed-type copies), and CPU ACC with F32 operands.
It includes copied and in-place results, aliased update sources, empty patches,
noncontiguous source rows, sparse/overlapping target rows and backward strides.

The descriptor model now records the data address. SET/ACC admission checks
matching output shape/type, contiguous left/output storage as required by
GGML, four-byte source words, constructor element-count/offset bounds and
in-place address equality. Signed update strides are interpreted from the
raw int32 parameter words. The model bounds the minimum and maximum addresses
across every view corner, and retains the CPU's strict ACC/non-strict SET
last-address condition for empty patches. Full scalar writes must fit their
allocations; the model does not reproduce unsafe one-byte-only bounds checks.

A shared copy mode first initializes out-of-place outputs from the left tensor,
then applies right-source updates in logical row order. SET moves opaque words
through Verdant's certified storage transfer. ACC reads the original left
source at each modeled update address and invokes Verdant's certified F32 ADD.
For in-place results that source aliases the destination, so later overlapping
updates observe earlier writes. Out-of-place results continue reading the
unchanged left allocation. This preserves GGML's serial alias behavior without
an alternate arithmetic implementation.

The 4,608 graph cases cover the supported type pairs, both output ownership
modes, standalone and aliased right sources, three offsets, four target stride
patterns and varying four-dimensional patch shapes (including zero leading
extent). Backward views use negative int32 strides with a bounded positive
base. Finite binary-fraction inputs isolate addressing and alias effects;
the unchanged Verdant scalar certificate/oracle already exercises IEEE edge
behavior. Invalid bounds, backward views below the base, absent right sources
and false in-place declarations are checked before writes.

A shared mutation guard snapshots the full valid output, applies one invalid
field, checks refusal, restores the descriptor and compares all output bytes.
It replaces duplicated refusal code across matrix, arithmetic and storage
graphs, preserving their cases and strengthening checks after each mutation.
Typed ABI record and interface declarations now share their struct expansion.
The source-bound oracle builder also shares executable linking/publication,
retaining exact object and executable digest checks. Its storage object was
renamed consistently with its executable stem; changing the builder invalidates
the affected cache identity once. SHA-256 formatting uses its fixed 32-byte
output type and two padded native integer formatters without per-byte allocation.

The corrected MCP draft is
`prism-app/a38069f3ed9f6dcd23f58cdc90bdf61a348d080983a41bfc34546f3b9b7825ff`.
Remote checking returned `engine.check failed: project-refused`. The pinned SDK
accepted and exported the model with result
`905a72c45769b2894924480a262b6b10840a0ed80f8aa4b27db859d1cf7644a5`
and attestation
`99f0054a6f7527d01ebf31b328a9dd948f1b41fb7b54e6a7f3b76c584907e5c2`.
The actual emitted eleven-field descriptor and update-mode/response variants
were inspected before binding them. All 4,608 scenarios pass; the final
source-bound fixture observed a 37.34-second cold reference build and run.
Handwritten source is 10,576 lines in the existing 51 files.

A disk-full write interrupted the test-file edit before export started. The
file was restored from Git plus the saved exact R3 edit, and the diff was
checked before continuing. Only owned, rebuildable package outputs were
cleaned; committed sources, reference sources and proof evidence were retained.
Subsequent source edits used temporary files and atomic replacement.

Three independent planted defects were rejected. Skipping the out-of-place
initialization prefix failed the empty-patch SET case at shape `[0,1,1,1]`.
Reading ACC's destination instead of its original left source failed the
out-of-place overlapping-row case at shape `[1,2,1,1]`. Binding every descriptor
address to zero failed the false in-place-declaration refusal. Each experiment
used bounded memory or stopped at capability refusal, and the adapter was
restored before the complete regression.

The final workspace run passed every other target and eighteen contract tests
in 8.10 seconds. VT-02 alone retained its explicit refusal of the unpublished
Verdant development dependency. Formatting, model/source audits, frame
synchronization, workspace Clippy and dependency checks passed. Handwritten
source is 10,576 lines in the existing 51 files, below the preceding checkpoint.
The expanded shared refusal guard also passed all existing matrix, arithmetic,
copy, repeat, concat and spatial regression cases. Full backend conformance
and production dependency acceptance remain unfinished.


## VT-20: F32 REPEAT_BACK through certified fold instructions

Registered VT-20 and its behavioral scenario before implementation. The first
`cargo test --locked -p vtensor-backend --test contract fold_graphs_vt_20 -- --exact --nocapture`
failed at the live device support query after the imported CPU graph succeeded.
The pinned upstream kernel initializes output elements to positive zero, then
accumulates in linear input order. Prism models that initialization prefix,
F32 admission, divisibility, input addresses and modulo output addresses.
Verdant executes each addition; the native adapter binds memory effects only.
Two closed address examples and the generated implementation preservation
certificate accompany the model. These are distinct from general bounds proofs.

MCP draft `prism-app/77d26fe44c69f7411e51b957f5c6ea8cc8629b2f5e435cd27cab1b26c9427bf1`
returned `engine.check failed: project-refused`. The exact pinned SDK checked
all four modules and exported result
`b71e71a78ca3a9c830135171b795164079ca999e0499a2815602e9c701cb1e68`
with attestation
`eb2612fa67400251605885756653b7f556c631d0820ae016c80db2b370208bb9`.
The emitted folded-mode and accumulator-response variants were inspected
before binding them. No SDK output was edited. An accidental workspace-wide
format pass was caught by the exact-byte audit; affected generated files were
restored from Git before export, then the repository's scoped formatter passed.

All 672 imported CPU comparisons pass: sixteen combinations of four-axis
repetition, empty source with nonempty output, each empty output axis, four
layouts, two widths, two row counts and two batch counts. Input values include
2^24, 1 and -2^24 to expose accumulation-order differences. Reference input
canonicalization preserves the actual strided input; complete output comparison
checks unchanged padding. Malformed extents and missing inputs are refused
before writes. The first implemented run completed in 1.85 seconds using the
source-bound reference cache.

Skipping the initialization prefix was rejected at pattern 16 (empty source):
byte zero remained 165 instead of zero. Replacing each running accumulator with
zero was rejected at pattern 1: byte three was 63 instead of 75. Both defects
retained bounded memory access and were restored before regression.

The final workspace run passed nineteen contract tests and every other target;
VT-02 alone rejected the unpublished Verdant development override. Contract
execution took 9.50 seconds. Formatting, model/source audits, frame synchronization,
workspace Clippy and dependency checks passed. The shared set/get/memset callback
binding passed the existing buffer oracle. A fixed four-byte zero block replaces
a separate allocation for the F32-only padding/fold initialization modes.
Handwritten source remains 10,576 lines in the existing 51 files. Full backend
conformance and production dependency acceptance remain unfinished; unchanged
main-application proofs were not redundantly rebuilt at this component checkpoint.


## VT-21: DIAG across every CPU-supported type

Registered VT-21 and its scenario before implementation. The initial
`cargo test --locked -p vtensor-backend --test contract diagonal_graphs_vt_21 -- --exact --nocapture`
failed at `ggml_backend_dev_supports_op` after the imported CPU computation
succeeded. The authoritative constructor requires a vector in dimension one
and produces a square matrix for each batch. The CPU kernel preserves raw
F32/F16/BF16 encodings on the diagonal and writes positive-zero bits elsewhere.
Prism models the complete shape/type decision, diagonal predicate, strided
input/output addresses and zero-source selection. Verdant executes every
admitted storage transfer. Two closed examples check batched two-byte diagonal
and off-diagonal addresses; preservation certificates accompany the generated
implementation. Finite oracle evidence is not a general bounds proof.

MCP draft `prism-app/6371236314d39c3fa9f85f09e173202ff6d763c680897ffd5e08e4a42df8edb7`
was refused with `engine.check failed: project-refused`. The attempt to store
the final SDK-formatted draft returned `store.put failed: unwritable`.
The exact pinned SDK accepted all four modules and exported result
`5b4b32e61b42c31aac75faaa6a8ad97409c845bf4e009887b26aa105de086f0b`
with attestation
`6eb0860c7665afba4e288a3a6c90c8eb62a6368139dbec79d0f48a35a77bd1a9`.
The emitted zero-field diagonal mode was inspected before binding it. The
native change selects the existing four-byte zero block for that mode; F16
and BF16 transfers use its first two bytes. No handwritten numerical kernel,
conversion, arithmetic or generated-source edit was added.

All 384 imported CPU comparisons pass: three supported types, four layouts,
two vector widths, two dimension-two batch counts, two dimension-three batch
counts, and four normal/empty-dimension patterns. Actual strided input is
retained; only the CPU reference input is canonicalized. Complete output
comparison verifies raw diagonal encodings, zeros and untouched padding.
Malformed source shape, either square output extent, unsupported/mismatched
type, and missing source are refused before mutation. The initial implemented
run completed in 2.88 seconds using source-bound reference caching.

Two bounded planted defects were rejected: filling the diagonal mode's zero
source with byte one failed at an off-diagonal output (actual 1, expected 0);
forcing every diagonal source offset to zero failed on a later batch
(actual 0, expected 148). Both faults were restored before regression.

Model checking and certificate replay now share the existing command runner,
which verifies exit status and reports stdout/stderr. The fixture chooses
whether it needs a type-input stream once per batch instead of once per type.
The full regression passed twenty contract tests and every other workspace
target; VT-02 alone still rejected the unpublished Verdant development override.
Contract execution took 8.99 seconds. Formatting, model/source audits, frame
synchronization, workspace Clippy and dependency checks passed. Handwritten
source is 10,574 lines in the existing 51 files. Full backend conformance and
production dependency acceptance remain unfinished. Unchanged main-application
proofs were not redundantly rebuilt for this component checkpoint.


## VT-22: all four TRI variants through shared selection plans

Registered VT-22 and its behavioral scenario before implementation. The initial
`cargo test --locked -p vtensor-backend --test contract triangular_graphs_vt_22 -- --exact --nocapture`
failed at the live device support query after a valid imported CPU computation.
Prism now shares one selection mode between DIAG and TRI: it defines square
shape/type admission, exact upper/lower and strict/inclusive predicates,
strided addresses, retained source selection and positive-zero selection.
The native adapter binds that mode's zero block; Verdant transfers every block.
Closed examples cover strict/inclusive diagonal decisions and both directions,
in addition to the retained batched DIAG address examples. Generated-code
preservation evidence remains distinct from general semantic/bounds proofs.

The first implementation exposed a fixture/model error: the upstream TRI
constructor asserts contiguous square input, whereas the initial fixture tried
rectangular input directly. The corrected fixture constructs a valid node on a
contiguous descriptor before reattaching the actual strided input. The CPU
reference uses the canonicalized input. The model now requires square TRI
shape; a new nonsquare-descriptor case failed against the initial generated
implementation at `!ggml_backend_dev_supports_op`. It checks admission before
execution, restores original extents before reading output, and verifies the
complete unchanged output. No invalid upstream computation is invoked or
silently filtered. The corrected root was exported again after this finding.

The remote MCP draft store returned `store.put failed: unwritable`, including
for the corrected model. The exact pinned SDK checked four modules and exported
result
`c95380447d6d6a5f17726f0eafa99af44f8f467deb201b843e04753170635850`
with attestation
`99436efc8bd2d9f4f82ed600017736d4b6070415820e18c624a2a8d826144c56`.
The emitted parameterized selection-mode variant was inspected before binding.
No generated source was edited and no handwritten numerical kernel was added.

All 1,920 scenarios pass: 1,536 imported CPU comparisons across all four enum
variants, F32/F16/BF16, four layouts, two square widths, two dimension-two batch
counts, two dimension-three batch counts, and normal/empty dimension patterns;
plus 384 nonsquare refusals. Comparisons preserve retained raw encodings and
check positive-zero masks and unchanged padding. Invalid variant words -1 and
4, missing source, malformed extents and unsupported/mismatched type are also
refused before writes. The final implemented run took 1.16 seconds using the
source-bound reference cache.

Two bounded planted defects were rejected. XORing the variant with two swapped
upper/lower direction and failed at width three (actual byte 0, expected 148).
XORing it with one changed diagonal inclusion and failed at width one (actual
byte 0, expected 37). Both faults were restored before regression.

A verifier defect was also corrected: checking the size of the collected ABI
map could hide identical duplicate records. The gate now counts raw records;
its existing per-key and case-count checks still establish the required set.
Injecting a second identical registry-size record passed under the former map
size check, then failed under the current check with 129 records instead of
128. The fixture was restored after both experiments. This is one count check,
not an additional validation pass replacing an unchanged count check.

The two existing malformed matrix cases now share their assertion loop without
changing inputs. Expected graph case counts have one named binding. Native
operand binding and typed GGML abort handling were simplified without changing
memory or error behavior. Workspace regression passed twenty-one contract
tests and every other target; VT-02 alone retained its refusal of the unpublished
Verdant development override. Contract execution took 9.69 seconds. Formatting,
model/source audits, frame synchronization, workspace Clippy and dependency
checks passed. Handwritten source is 10,572 lines in the existing 51 files.
Full backend conformance and production dependency acceptance remain unfinished;
unchanged main-application proofs were not redundantly rebuilt here.


## Verdant float conversion foundation (VTK-04 dependency checkpoint)

The isolated Verdant source is now
`d6674663941f88789911131d180ea553ac8be5c7` on
`refs/heads/vtensor/storage-checkpoint`. Its thin bundle remains based on the
original pinned upstream commit, with SHA-256
`fa1c3dcf03d3a35d14680cecc23a395d1acf19ca32de7585faa549416d9a3a93`
and 4,793,982 bytes. The previous clean prepared checkout was preserved at
`/tmp/vtensor-verdant-source-26ba1-preserved`; `just verdant-prepare` then cloned
upstream, verified the bundle/base/ref/hash, prepared the exact new commit and
confirmed a clean source tree. No files were copied into the prepared dependency
by hand. The isolated Engine's two unaccepted Wasm investigation outputs were
excluded from its commit and from this bundle.

The new `verdant_tensor::linear::convert_float_storage` directly re-exports the
certified Prism `FloatStorage.convert` root. It accepts GGML F32/F16/BF16 type
codes and encoded words, returns `None` for unsupported types or invalid input
widths, and has typed checked overflow. Identity conversions preserve raw bits.
No handwritten production conversion function was introduced; Tensor retains
its unsafe-code prohibition. The SDK result is
`6b0c18a4cb889057de720147862a0813a8480e5a128717f057bb7ad279694206`
and attestation
`44377072eeaed84f50a5c87c77ad8db0fb4e37803493d33689fd1e6a9846c225`.
MCP draft storage returned `store.put failed: unwritable`; the pinned official
Engine SDK checked, exported and replayed the model and implementation evidence.

Engine VTK-04 began with a registered ID/scenario and an observed missing-API
compilation failure. Its imported GGML authority uses the pinned unmodified
archive and upstream conversion functions; F16 widening is the function used
to populate the CPU lookup table. All sixteen-bit words across six type pairs
supply 393,216 comparisons. Verified widened results seed narrowing boundaries;
1,048,576 generated requests are sorted/deduplicated to 1,004,044 distinct F32
comparisons. All 1,397,260 results match bit for bit, including NaN encodings.
The corpus asserts both its complete input topology and unique record counts.
Invalid source/destination types and sixteen/thirty-two-bit width boundaries
are separate fast negative tests. Closed model examples and preservation
certificates are not presented as a general conversion-semantics proof.

A tie-rounding defect returned 15361 instead of 15360 and failed the numerical
oracle. Masking oversized sixteen-bit input to fit failed the typed refusal.
Changing the F16 NaN model encoding without exporting again failed artifact
freshness at `source_id`. All faults were restored. The final Engine Tensor
suite passed thirteen unit and four integration tests, with unit execution in
7.23 seconds. Model checks, formatting, Core/Tensor Clippy and dependency checks
passed. The fresh uorust projection stayed at 203 lifted, 1,192 residue, 73
foreign and 78 effect functions; no ceilings were raised. Its Rust-tree pin was
refreshed, and the tensor models gained explicit stratum ledger entries.

The broader Engine self-hosting ratchet still refused stale hashes in its
separate generated manifest and missing lifted tensor-module entries; it also
reported the unaccepted Wasm investigation output exceeding its unchanged size
watermark. That dirty Wasm output is absent from the accepted source bundle.
The manifest was not hand-edited and the full Engine release gate is not
claimed. Its previously observed source-rebuild Wasm initialization discrepancy
and production-source packaging remain unresolved.

The fresh upstream clone observed `5b11dde69fdf03100eb493e331609cd54f1d028e`
(v2.7.0), whose latest change decomposes seven large Rust files. Inspection of
its tensor/model tree found none of these conversion-root/API names; no claim
is made that the development extension is published upstream.

vTensor's new dependency built successfully. The complete workspace regression
passed twenty-one contract tests and every other target; VT-02 alone retained
its exact-source refusal of the unpublished development override. Contract
execution took 16.73 seconds. Formatting, model/source audits, frame synchronization,
workspace Clippy and dependency checks passed. Source remains 10,572 lines in
51 files. Existing Prism roots and the unchanged main application were not
redundantly re-exported. No new native graph capability is claimed here: this
verified conversion primitive enables subsequent mixed-type copies and F16 fill.


## VT-23: copied and in-place F32/F16 FILL

Registered VT-23 and its behavioral scenario before implementation. The initial
`cargo test --locked -p vtensor-backend --test contract fill_graphs_vt_23 -- --exact --nocapture`
failed at the expected live device support query after the imported CPU graph
succeeded. Upstream's constructor admits contiguous F32/F16 input and chooses
a duplicate or aliasing view; its CPU kernel uses the descriptor's type but does
not read the input payload. F16 constants pass through the pinned GGML conversion
function. The fixture constructs valid nodes before applying tested strided
layouts, and the in-place output retains its input strides and physical alias.

Prism defines FILL type/word/shape admission, output block count and offsets,
and the constant-source instruction. It preserves all 32 parameter bits and
refuses words beyond that domain. Verdant converts the constant once per
nonempty node, then transfers each encoded block. Native bindings preserve
host byte order for both four-byte and two-byte words. Unused input payloads
are not accessed or required to have buffers. Closed examples cover an F16
exceptional input word and a width refusal; preservation evidence is distinct
from a general semantic/bounds proof.

Pointwise arithmetic now lowers to the same block executor through Prism
arithmetic instructions. The model chooses operation code, source slots and
all operand/output addresses. ACC's original-source reads and REPEAT_BACK's
running-output reads use the same instruction with different modeled slots.
Unary operands use a local zero word. The separate native pointwise loop was
removed; the remaining interpreter binds raw memory effects and calls Verdant
scalar arithmetic. No handwritten numerical kernel was introduced.

MCP draft storage again returned `store.put failed: unwritable`. The pinned SDK
accepted all four modules and exported result
`72ac1710d1bb80909ad8bcf23177392fd152e7fbfe78a463ee5a2c52622a5eb0`
with attestation
`800206a99d844424c100cdbaf40c5552501d91008d6106c27be3def71aaeda77`.
The actual constant/arithmetic mode variants and six-field arithmetic response
were inspected before validating the native bindings. No SDK output was edited.

All 15,360 imported CPU comparisons pass: both supported types, copied and
in-place outputs, 24 constant encodings, normal plus four empty-axis patterns,
four layouts, two widths, two row counts and two batch counts. Constants include
signed zeros, normals, subnormals, infinities, signaling/quiet and signed NaNs,
F16 underflow/overflow thresholds and even/odd ties. Complete byte comparison
checks exact encodings and untouched padding. Out-of-place layout-three cases
remove the unused input's buffer and data after reference execution; nonempty
outputs still succeed. Invalid shape, missing descriptor and same-type BF16
FILL are refused before writes. The first implemented run took 5.78 seconds.
The migrated 120 arithmetic/unary graphs passed independently before fault tests.

Three bounded defects were rejected. Converting a F16 constant as F32 failed
at negative zero (byte one was 0 instead of 128). Requiring the unused source's
data failed the expected successful computation for an unallocated input.
XORing the arithmetic opcode with one failed the migrated arithmetic oracle
(expected `bf800000`, actual `3f800000`). All faults were restored before the
complete regression; none relied on out-of-bounds memory access.

The final workspace regression passed twenty-two contract tests and every other
target, including the existing 4,608 SET/ACC and 672 REPEAT_BACK scenarios through
the shared arithmetic interpreter. VT-02 alone retained its exact-source refusal
of the unpublished Verdant dependency. Contract execution took 17.56 seconds.
Formatting, model/source audits, frame synchronization, workspace Clippy and
dependency checks passed. Handwritten source is 10,571 lines in the existing
51 files, below the previous checkpoint. The unchanged Verdant conversion proof
and its large corpus were not rerun; this checkpoint validates their graph
composition. Unchanged main-application proofs were not redundantly rebuilt.
Full backend conformance, production source acceptance and the previously
recorded broader Verdant release blockers remain unfinished.


## VT-24: mixed F32/F16/BF16 casts and copies

Registered VT-24 and its scenario before implementation. The initial
`cargo test --locked -p vtensor-backend --test contract cast_graphs_vt_24 -- --exact --nocapture`
failed at the live support query after the imported CPU graph succeeded.
The upstream copy constructor requires equal element counts and retains a target
descriptor; cast uses the CPY operation with a self target. The CPU float-copy
kernel converts through F32. Prism now chooses mixed-float admission, equal
logical element counts, source/destination types and independent ordinal byte
offsets. Same-type copies retain their existing opaque-block path for all 36
types. Missing CPY targets and unsupported mixed type pairs remain typed refusals.

The native adapter reads a bounded two- or four-byte host word and calls
Verdant's certified conversion primitive. FILL and casts share the host-word
conversion/binding helper. Converted bytes join the existing staged Verdant
transfer path, preserving source-before-destination ordering. Destination
encoding uses native byte order for both word widths. A read-only source-pointer
type annotation resolved a Rust mutability mismatch before execution. The
existing query callback macro gained typed additional arguments, preserving
the buffer-type callback ABI while removing repeated declaration boilerplate.
No handwritten numerical conversion kernel or new source file was introduced.

All 1,920 CPU comparisons pass: six mixed pairs, both cast/copy constructors,
five normal/empty-axis patterns, four layouts, two widths, two row counts and
two batch counts. CPY includes a reshaped padded destination. Raw input words
include signed zeros, subnormals, finite limits, infinities, signaling/quiet and
signed NaNs, and narrowing boundaries. Complete byte comparison checks exact
encodings and untouched padding. Only the CPU reference input is canonicalized;
the actual strided source and destination are retained. Invalid counts, missing
source/target and unsupported source/destination types fail before writes.
The first implemented run took 4.51 seconds. Forcing destination conversion to
F32 failed the F16 cast at byte seven (0 instead of 128). Forcing source offset
zero only for a reshaped copy failed at byte five (0 instead of 128). Both faults
were bounded and restored before regression.

The sampled hundred-opcode matrix dispatch loop was replaced by
`MatrixGraph.scalarDispatchForEveryOpcode`, a kernel-checked theorem quantified
over every Nat opcode at the same fixed scalar shape/type. Its result is the
scalar product exactly under opcode 28's boolean guard and unsupported otherwise.
The positive scalar API call and both existing shape/batch refusal calls remain,
as do the 640 live CPU matrix comparisons and the separate imported opcode
inventory check. A deliberately wrong opcode 27 in the theorem failed Lean
with LLV7002 (`rfl` failed). The correct model was restored byte for byte.

MCP draft storage returned `store.put failed: unwritable`. The pinned SDK checked
and exported both changed projects. Matrix result
`44067ee57128e8a1feab7c084b0501fe77c119e1df4060e18de463015a7ed14c`
has attestation
`90ad5047f80ea45e7819a751db8a81fead115a6f1cd1a246e7e83eb5de523952`.
Pointwise result
`a9f37f0c5823fd15b4357d46e0df49e99017271a71c6cafc3fbfb81069e70140`
has attestation
`68f4015c7d1d45f5bf78a722b63c70a3bc5368bc7e99d1d1a859e2ab1c675b41`.
The emitted conversion mode/response variants were inspected before native
validation. The matrix source theorem and its axiom audit were present in the
SDK's successful production proof. After the deliberate theorem fault, a
restored `verify` began repeating the already successful preservation proof;
that redundant process was stopped. A current source/semantic identity check
matched the kept matrix artifact and `production verify-artifact` verified its
bindings. Its output explicitly says axiom audit, Lean elaboration, leanchecker
replay and named-root extraction were not rerun. Only the changed pointwise
root then needed a fresh export; no kept proof is described as a fresh proof.

The final workspace regression passed twenty-three contract tests and every
other target. VT-02 alone retained its exact-source refusal of the unpublished
Verdant dependency. Contract execution took 18.59 seconds. Formatting,
model/source audits, frame synchronization, workspace Clippy and dependency
checks passed. Handwritten source remains 10,571 lines in 51 files. Existing
same-type copies, FILL, arithmetic and buffer metadata regressions passed.
The unchanged large Verdant conversion corpus and main-application proofs were
not redundantly rerun. Other conversions, numeric types, operation families,
complete lifecycle conformance and production source acceptance remain unfinished.


## VT-25: window partition and join

Registered VT-25, its scenario and the window contract test before implementation.
The initial `cargo test --locked -p vtensor-backend --test contract window_graphs_vt_25 -- --exact --nocapture`
failed at the live support query after the CPU reference completed successfully
(`native check line 367: ggml_backend_dev_supports_op(dev,result)`).
Prism now defines both window plans and ordinal source/destination offsets.
Partition checks the constructor's grid and window parameters and supplies
positive-zero padding. Join follows the CPU kernel's actual ordinal formula;
it permits oversized and reshaped source windows when every referenced element
exists. Its admission computes the exact maximum over complete and final partial
windows on each axis, including cases where the final coordinate is not the
maximum. Signed parameter and grid arithmetic limits are checked before execution.
The native executor binds its shared zero literal for unused secondary sources;
all address decisions remain in Prism and byte transfers remain in Verdant.

The authoritative GGML CPU window kernels assume packed storage and ignore
strides. Only the reference input is canonicalized before running those kernels;
the actual vTensor input and output retain their independent strides. This
checks vTensor's logical strided extension against the CPU's packed semantics.
Join inputs are generated independently, rather than obtained by partitioning.
The cases include normal, oversized, compressed and exactly sufficient sources,
and sources one element below the required bound. The fixture never executes
an out-of-bounds CPU case. Negative admission checks require an unchanged output
buffer after a failed graph computation.

Prism MCP draft storage returned `store.put failed: unwritable`; the exact pinned
SDK is used for source checking, Lean verification and certified export.

The hundred-opcode runtime metadata-classification sample is replaced by
`NativeBackend.metadataClassification`, universally quantified over every Nat
opcode and span. Actual ABI discovery, buffer and graph checks remain; the
metadata graphs now call all five authoritative constructors (NONE, RESHAPE,
VIEW, PERMUTE and TRANSPOSE). The successful native production export yielded
result `5747e89f0b25948b3777579d18890c5b3f74d60685d065aeb0498e095633fb38`
and attestation `aee2ca3e6f11724bff66d3284c6a17fe8e7a962d0bf42add1a0fd5c0ca1930a0`.
Changing opcode 38 to 39 in the theorem's statement failed LLV7002 (`rfl` failed).
After byte-for-byte restoration, the SDK source/semantic identities matched the
kept export and `production verify-artifact generated/native` verified all three
outputs' bindings. Its output explicitly reports that axiom audit, elaboration,
leanchecker and root extraction were not rerun. No redundant positive production
verification was launched after the successful export.

The window export produced result
`cf5bd6c3f230ce25c43b20cdd64b903f365eb1414c081a7de56ed13f64ce3d3f`
and attestation
`58f2dddc1a1bfa62240775ef0933763c6145920a7b8604f0e2e772a346f30ea6`.
Its emitted `CopyMode.windowed` binding was inspected before native execution.
The focused test passed in 5.46 seconds: 864 partition CPU comparisons, 1,944
join CPU comparisons and 216 one-element-short refusals. Window sizes are 1,
2 and 4, channel/width extents include 1, 3 and 5, and four independent storage
layouts include padded outputs. Five-element extents exercise complete-window
maxima that exceed the final coordinate. Empty channel, image-width and
image-height cases preserve the metadata no-op contract. Invalid signed/zero
window sizes, grid parameters, channel counts, source types and missing sources
are refused without output writes.

Replacing partition's shared padding literal with F32 one failed at byte ten
(actual 128, expected zero). Forcing only join's source offset to zero failed
at byte twelve (actual zero, expected 148). Both faults accessed bounded memory,
were observed by the actual CPU comparison and were restored before regression.
The generated source's changed license fingerprint was updated to the value
reported by cargo-deny, without editing SDK-generated bytes.

The final workspace contract run passed twenty-four tests in 18.18 seconds.
VT-02 alone retained its exact-source refusal of the unpublished Verdant
extension. Because Cargo stopped at that binary, the remaining workspace
members and the already-tested members' doc targets were then run separately;
all passed without rerunning the contract corpus. The five live metadata
constructors, earlier graph families and buffer tests passed. Formatting,
model/source audits, frame synchronization, all-target/all-feature Clippy and
cargo-deny passed. Handwritten source remains 10,571 lines across 51 files.
Only changed native and pointwise projects were exported. Unchanged main-app
proofs and Verdant's large conversion corpus were not rebuilt. Full imported
operation coverage, complete lifecycle support, production dependency acceptance
and the previously recorded Verdant release blockers remain unfinished.


## Certified float-transform dependency foundation

The reproducible Verdant extension is advanced to
`45bb03d603838a3289400f36753c743a289ef492`. Its new VTK-05 capability adds
`linear::transform_float_storage` for scale-bias, clamp and leaky ReLU across
F32, F16 and BF16. `FloatTransform.lex.tex` composes the existing certified
storage conversion and scalar arithmetic models, including unordered comparison,
signed zeros, reversed clamp bounds and the F32 zero-bias scale branch.
The pinned SDK export has result
`e9e77b4bbfe6d9ec6aac3f513ef975ab7f3c60ba6ca569940d0c1baafa4d8310`
and attestation
`7afb29d539884299def8d2d56bcd7ab9f87607e22bbba58bb26bf4ee375e55df`.
The remote Prism draft store again refused writes; the exact SDK supplied the
source check, Lean proof and generated-code preservation certificate.

The isolated Engine checkpoint registers its capability and scenario before
the observed missing-API failure. Its oracle compiles the imported GGML CPU
kernels directly with the SDK C/C++ compilers and baseline architecture flags.
The 35,964 cases span input/parameter combinations, exceptional values and all
three storage types. Finite values, infinities and signed zeros require exact
bits; numerical NaNs use the existing scalar classification policy. Reversed
clamp bounds and NaN selection are exercised, rather than filtered. Swapping
the leaky-ReLU slope and unused argument failed the negative-one/unit-slope
comparison (actual zero, expected `bf800000`). The binding was restored, and
all three new tests passed, including typed refusals and certificate replay.

All 16 Tensor unit and four integration tests passed in the Engine checkpoint.
The complete numeric corpora were rerun because the shared oracle builder
changed; the 75.18-second unit run includes a fresh CPU executable build.
Formatting, model checks, Core/Tensor all-target/all-feature Clippy and
cargo-deny pass. A fresh uorust projection retains the existing 203 lifted and
1,192 residue functions without raising ceilings; its 49-file source pin is
`74acfe14d176770b0846e108a283d272db0d276a574026c95d77e542e2432c6e`.
Previously recorded broader Engine release blockers remain open.

The vTensor bundle is 5,179,847 bytes, SHA-256
`69dd4ca8204b225d0913c5a1cda9f39e3ea4fd7983f57b9692d97a42ebe7c839`.
Its Git prerequisite and exact ref were verified. The clean prepared checkout
was advanced by a fast-forward; `just verdant-prepare` confirms the exact new
commit and bundle. The isolated checkout's two unaccepted rebuilt Wasm files
remain outside the commit and bundle. The lockfile and generated-source license
fingerprint bind the added package without changing its emitted bytes.
Graph bindings for these three transforms remain to be implemented in Prism;
this dependency foundation does not add native graph support by itself.

The vTensor workspace regression used `cargo test --workspace --locked
--no-fail-fast`: twenty-four contract tests passed in 17.06 seconds, every other
target passed, and VT-02 alone refused the unpublished development override.
Formatting, model/source audits, frame synchronization, workspace Clippy and
all dependency checks pass. Handwritten source remains 10,571 lines across the
existing 51 files. Unchanged vTensor Prism roots were replayed without fresh
exports. Full backend coverage, the new transforms' graph bindings, complete
lifecycle support and production source acceptance remain unfinished.


## VT-26: typed scalar transform graphs

Registered VT-26 and its scenario before implementation. The initial
`cargo test --locked -p vtensor-backend --test contract transform_graphs_vt_26 -- --exact --nocapture`
failed at `native check line 371: ggml_backend_dev_supports_op(dev,result)`
after the imported CPU reference graph succeeded. Prism now admits SCALE,
CLAMP and LEAKY_RELU for all three CPU-supported storage types (F32, F16, BF16),
requires equal source/output type and shape, and retains the raw F32 parameters.
Its transform mode selects logical source/output byte offsets through the
shared word response. The universal `transformOffsetsPreserveWordTypes` theorem
connects that mode to equal source/destination word types for every index,
context, transform kind and parameter value.

The native word binding calls Verdant's certified transform for that mode;
other conversion/FILL modes continue through the conversion API. The shared
binding preserves host byte order, maps generated errors to typed backend
failures and stages each source word before writing an aliased destination.
All arithmetic, comparison and rounding remain in Verdant's certified model.
Repeated transfer callback declarations now share one typed macro table, and
the CPU reference backend is initialized once per oracle invocation. No native
numeric kernel, new handwritten source path or ratchet exemption was added.

The graph fixture covers copied/in-place SCALE, SCALE_BIAS, CLAMP and LEAKY_RELU,
24 parameter encodings, four storage layouts, two widths, two row counts,
two batch counts and all three float types for every nonempty case. Four empty
axes retain every form, layout and type at one geometry and parameter setting;
the universal metadata theorem covers the irrelevant parameter/extent variations.
All 18,432 nonempty graphs are retained, alongside 384 empty graphs. In-place
cases retain the original input bytes and strides; copied outputs have separate
padding. Reference inputs are canonicalized because the imported scale/leaky
kernels impose packed-row assumptions, while vTensor's Prism model defines the
logical strided extension. Every finite value, infinity and signed zero requires
exact output bits. Numerical NaNs use the existing scalar classification policy
only at logical output locations; complete byte comparison still checks all
padding. Missing sources, wrong shapes/types and unavailable input allocations
must fail before output writes.

Prism MCP returned `store.put failed: unwritable`. The pinned SDK checked the
new model and supplied the production export for the pointwise project.

The first export stopped with LLB6003 while writing preservation process evidence
because the filesystem filled; it produced no accepted artifact. Cleaning only
this task's rebuildable `vtensor-backend` target freed 605.7 MiB. The retry
completed the SDK's production pipeline with result
`1b05f6b1d8ad80b7fb43176532b8d36b291da5f2f7735e25850cf1e4bba6e849`
and attestation
`d129893d8f0f28bca6ba7e9a893d0f04b2955f4e11199cb680111409cdca213f`.
The emitted transform variant was inspected before native execution. Its source
fingerprint `526182f2` updates cargo-deny's existing license record without
changing generated bytes.

The implemented graph test passed all 18,816 cases in 12.31 seconds, including
native driver preparation. Empty graphs deduplicate the parameter/geometry
cross-product while retaining every type, form, layout and empty axis; all
18,432 numerical cases from the initial fixture remain. Dropping SCALE_BIAS's
bias failed at byte two (actual zero, expected 128). Forcing transform source
offset zero failed the signed-zero comparison at byte fifteen (actual zero,
expected 128). Both faults stayed within allocated memory and were restored.
Changing the universal theorem's destination type to F32 failed Lean with
LLV7002 (`rfl` failed). After byte-for-byte restoration, source ID, semantic ID
and lock digest matched the kept result. `production verify-artifact` verified
both outputs' bindings and explicitly reported no repeated axiom audit, Lean
elaboration, leanchecker replay or named-root extraction.

The final `cargo test --workspace --locked --no-fail-fast` run passed twenty-five
contract tests in 21.93 seconds and every other workspace target. VT-02 alone
retained its exact-source refusal of the unpublished Verdant dependency.
Earlier graph families, buffers and all ABI layouts passed through the shared
callback and reference setup. Formatting, model/source audits, frame
synchronization, all-target/all-feature Clippy and dependency checks passed.
Handwritten source remains 10,571 lines across the existing 51 files. Only the
pointwise project received a fresh export, including its companion offset root;
unchanged layout/native/matrix/index and main-app proofs were not rebuilt.
The unchanged Verdant primitive corpus was not redundantly rerun; this checkpoint
checks its graph composition. Other operations, numeric types, lifecycle
coverage and production dependency acceptance remain unfinished.


## VT-27: typed unary graphs and integer rounding

Registered VT-27 and its scenario before implementation. The initial graph
fixture failed at the live supports-op check. The Verdant VTK-06 foundation
is pinned by development commit 054df8ead9a0eb47b634c4cbc34263ec4690ca16;
its four certified roots and 3,675,256 imported GGML CPU comparisons are
recorded in that bundle's README. vTensor uses its typed adapter without
handwritten numerical arithmetic. The upstream reference remains the imported
GGML CPU unary functions from commit 9a080f44b503c64f66c84a6274b9d9ee5d5b09d5.

Prism maps ABS, SGN, NEG, STEP, RELU, SQR, SQRT, HARDSIGMOID, HARDSWISH,
FLOOR, CEIL, ROUND and TRUNC to Verdant's model. The plan admits F32/F32,
F16/F16, BF16/BF16, F16/F32 and BF16/F32; source and destination element
widths remain independent throughout addressing and transfer. Shapes must
agree. Native code binds raw host-endian words and effects; Verdant defines
all numerical behavior. A universal theorem retains both type codes in every
unary offset response.

Only the changed pointwise project was exported, including its companion
repeat-offset root. Certified result
16fec5454c51e69a19708a6366fce5b9d0056193833e5576ec9f91d67034031c,
attestation 69c9bee1327a54125654f740c35be83fa3cd917fd8b4d2c6b0752487cbe3154b.
The emitted source remains untouched; cargo-deny's observed source fingerprint
is e9447206.

All 4,992 graph cases passed: 3,328 nonempty numerical graphs and 1,664 empty
graphs. These cover thirteen operations, the five admitted pairs, copied and
same-type in-place forms, four layouts, two widths, two row extents and two
batch extents. Empty cases retain every type/form/layout/empty-axis while
avoiding redundant geometry. CPU inputs are canonicalized to test the logical
extension to arbitrary admitted strides. Every finite value, infinity and
signed zero requires exact output bytes; numerical NaNs compare by class.
Destination padding is checked byte-for-byte. Wrong pairs, unsupported unary
variants, invalid storage, shape drift and absent source data are refused.

The first positive run took 46.27 seconds including the fresh reference build.
A promotion defect that encoded the result in the source type failed F16 to
F32 ABS at byte 29 (actual 56, expected zero). A source-offset-zero defect
failed F32 ABS at byte 31 (actual zero, expected 63). Both were bounded within
allocated memory, restored byte-for-byte, and the full 4,992-case graph check
then passed in 2.32 seconds using the unchanged CPU reference. A wrong theorem
that replaced the destination type with the source type was refused by Lean
with LLV7002. After restoration, source and semantic IDs matched the accepted
bundle. Artifact replay verified both outputs' bindings and explicitly did
not repeat axiom audit, Lean elaboration, leanchecker or root extraction.

Shared oracle row parsing, exact file hashing, and atomic compiled-object
publication replace duplicated plumbing. The transfer-boundary table retains
all previous negative inputs. Handwritten source is 10,569 lines across the
same 51 files, below the 10,571-line parent ceiling. No implementation-accounting
exemption or limit was added. Main-app and unchanged Prism projects were not
reproved, and the unchanged exhaustive Verdant corpus was not rerun for graph
binding faults.

The final workspace run passed 26 contract tests in 20.74 seconds and every
other workspace target. VT-02 alone retained its refusal of the unpublished
Verdant development dependency. Formatting, model/source audits, frame sync,
all-target/all-feature Clippy and dependency checks passed. Complete imported
operation/type/lifecycle coverage and production dependency acceptance remain
unfinished; this checkpoint does not claim full vTensor or Engine acceptance.


## VT-28: row-local cumulative sums

The imported GGML implementation admits F32 CUMSUM, copies each row's first
word directly and adds later words in F32 prefix order. SUM and MEAN instead
use an F64 accumulator; their implementation requires that wider model and
is not inferred from CUMSUM. VT-28's registered scenario and initial live
supports-op failure preceded this implementation.

Prism defines a scan plan and response sequence using the existing copy and
arithmetic instructions. No native numerical implementation or new Verdant
primitive is introduced: first words use the existing byte-transfer path,
and subsequent words call Verdant's certified F32 addition. Each row restarts
from its own source word; both source and destination strides remain intact.
The model rejects non-F32 operands and mismatched extents. Universal routing
and concrete row/plane reset and predecessor-offset theorems are checked.

The changed pointwise project was exported once; result
8acd4b65dca09c0c6ba288fc48f330f829511ff33c4f3fcd8b4abfdc5d30279b,
attestation afd4eca32e4e95820c23cf2a5550fb166e23c0e53c016843697d89f7cd0795e0.
The generated source remains byte-for-byte SDK output. The observed cargo-deny
source fingerprint is 34e5211c. Other projects and the unchanged main-app proof
were not rebuilt.

All 528 imported CPU graph comparisons passed in 2.52 seconds: 512 numerical
graphs across eight data patterns, four widths (1, 3, 17, 65), two row extents,
two batch extents and four layouts, plus sixteen empty graphs retaining every
empty axis and layout. Data include cancellation, signed zeros, infinities,
signaling/quiet NaNs, finite overflow, subnormals and seeded raw words. Source
and destination element strides vary independently. The first word requires
exact bits, including NaN payload; later numerical NaNs compare by class.
Every finite result, infinity, signed zero and padding byte requires exact
bytes. Invalid shape/type and allocation states retain the shared refusal
checks before writes. CPU inputs are canonicalized for the logical strided
extension; the imported CPU scan itself requires packed elements within rows.

Replacing the first-word transfer with addition of positive zero failed the
negative-zero case at byte three (actual zero, expected 128). Reading the first
output word in place of the previous prefix failed at byte eleven (actual
zero, expected 64). Both defects were bounded within admitted memory and
restored exactly; all 528 cases then passed in 2.26 seconds using the cached
reference. Shared file hashing and exact source-snapshot comparisons remove
duplicated plumbing without dropping integrity or inventory checks.

A plane-reset theorem changed to the wrong source offset failed Lean with
LLV7002. Exact restoration reproduced the kept source and semantic IDs;
artifact replay verified both generated bindings and explicitly did not repeat
axiom audit, Lean elaboration, leanchecker or named-root extraction.

The harness checks loaded-library provenance once, before execution: canonical
library path, exact source length and a live byte comparison against current
inputs. It no longer writes that same snapshot back to disk for a duplicate
post-execution check. Corrupting one byte of the shared library's embedded
source snapshot failed that live memcmp guard. The original library bytes were
restored exactly and all 528 graphs passed in 2.22 seconds with the unchanged
oracle host. The Rust test library also retains its independent freshness
check against current inputs.

The final workspace run passed 27 contract tests in 18.84 seconds and all other
workspace targets. VT-02 alone still refused the unpublished development
dependency. Formatting, model/source audits, frame sync, all-target/all-feature
Clippy and dependency checks passed. Handwritten source is now 10,560 lines in
the same 51 files, below the preceding 10,569-line checkpoint. Complete imported
operation/type/lifecycle coverage, wider reduction arithmetic and production
dependency acceptance remain unfinished.

## Wide reduction foundation: Verdant VTK-07

The development dependency advances to 940ae5b8fe404805a002ba6a0e2fb1067b642ff9.
Its hash-bound Git bundle retains the Prism models, ten exact generated
packages, certificates, imported oracle, failure scenarios and tests.
ScalarF64 defines encoded F64 addition, F32 widening, F64 narrowing and
unsigned-count conversion with nearest-even rounding. A separate pure word
specification and universally refined field exports define the two-word
binding. Native effects dispatch certified stages and assemble bytes; they
provide no floating-point implementation.

The six-root arithmetic result is
b44050e4408a17672de7b4d199b234ce0d00da2eef1ab6d93c326467076947bf
(attestation 5c52caba9d2ee04cadcaea0b8c479f650f9cf452a69ce378678516e1210e6a5e).
The four-root encoding result is
091f4c9728cfbbca866f7b63822d578aee8539cd66cc6c2c57b953a08b18b9a5
(attestation 03578f78b630a5c1f48e6f8c40e456fafbe74557a48432ef8dec5cdc2c3d6e5b).

Berkeley SoftFloat Release 3e is imported from its authoritative repository
at commit 07ebd29db0cfb55723a79fbbffe99103ef38e305. Its retained source archive
SHA-256 is a43936117a4b62b2c942b62144ab903f7d93c5fb313389caecbd6e790d416e34.
All 1,213,940 comparisons passed: 851,968 additions, 118,712 narrowings,
70,656 widenings and 172,604 count conversions. Exact encodings are required
except for numerical NaNs, compared by classification. Certificate freshness
and replay check every requested root and compiled binding.

Planted source, library and executable cache corruption each failed their
byte-integrity gate. A driver-only change reused every unchanged oracle
library/object byte and mtime. Swapped high/low words and wrong operation
dispatch failed the imported oracle; exact restoration passed all 152 smoke
cases. Wrong rounding and concrete high-word theorems failed Lean with
LLV7002; changed source also failed certificate freshness. Restored identities
match the kept artifacts, replayed without repeating kernel proof phases.
The unchanged million-case corpus and unrelated graph proofs were not rerun.

This foundation does not yet bind SUM, SUM_ROWS or MEAN graphs. GGML requires
F64 accumulation for F32 reductions, with observable packed/strided SUM
ordering and F32 narrowing before MEAN division. Half-storage SUM has distinct
F32 accumulation. Those semantics require their own Prism routing and live
GGML comparisons. Engine full acceptance still refuses stale self-host
manifest records and an unaccepted oversized UORC Wasm build. Both dirty Wasm
files were excluded; no ceiling or exact-source gate was weakened.

With this dependency pinned, the complete workspace test command passed all
27 behavioral contract tests in 32.30 seconds and every other target; VT-02
alone rejected the unpublished development override. Scoped formatting,
model/source audits, frame synchronization, all-target/all-feature Clippy and
cargo-deny passed. Handwritten source remains 10,560 lines in the same 51
files. No unchanged Prism project was re-exported for this dependency update.

## VT-29: SUM, SUM_ROWS and MEAN graphs

The ID, behavioral scenario and failing live supports-operation test preceded
implementation. Prism now admits SUM for F32/F16/BF16 and SUM_ROWS/MEAN for
F32, matching the imported CPU type matrix. SUM requires a scalar output of
the source type; row reductions require width one and equal remaining axes.
Unsupported types and malformed shapes fail before execution. Empty output
nodes retain the shared no-op rule, while empty input with a nonempty output
still executes its reduction: SUM/SUM_ROWS produce positive zero, and MEAN of
zero-width rows performs the specified zero-over-zero division.

The model defines the full instruction sequence, including accumulator resets,
logical source addresses, scratch registers, row/global merges, conversions
and final writes. Packed F32 SUM accumulates each row in F64 then merges the
row totals; strided F32 SUM accumulates all elements flat in F64. F16/BF16 SUM
uses F32 row/global accumulation and converts only the final total. SUM_ROWS
narrows each F64 row total to F32. MEAN narrows that same total first, converts
the unsigned width to F32 and divides in F32. Destination strides are retained
without writing padding. Native effects bind bytes and execute the certified
instructions through Verdant; they do not implement floating arithmetic.
The existing F32 arithmetic effects share the same word interpreter.

The changed pointwise export is result
c5224ee75a511d5e625fc3f1149c06b1373a7f9cbc2dd69771d062cb822b3bf6,
attestation 75014e89dea5020a4d52dd644b2b7716b97457aa1d4c40b39d036daea0ead22c.
Its source identity is 904e530496b686986e6b434c178c789593d86141fdbe102f40a5138a9f5a2558;
semantic identity 2cb168d89cc01248d9f80604518336d55c497daebc94958d0b90c4220349270a.
Generated bytes remain exact SDK output, with observed license fingerprint
40aebce3. The other projects and unchanged main application were not rebuilt.

All 4,080 comparisons against the imported GGML CPU passed: 4,000 numerical
graphs cover five admitted type/operation combinations, ten word patterns,
five widths (1, 2, 3, 17, 65), two row extents, two batch extents and four
layouts; eighty cases cover every empty axis. Patterns include cancellation,
signed zeros, subnormals, infinities, signaling/quiet NaNs, overflow and seeded
words. Precision cases include rows [2^54, 0] and [-2^54, 1], which distinguish
packed row grouping from flat strided addition, and maximum F32 words that
require MEAN narrowing before division. Values are placed by logical ordinal
so the strided cases retain those same adversarial values. Numerical NaNs
compare by classification; every finite result, infinity, signed zero and
padding byte requires exact bytes. The CPU reference retains actual F32 SUM
strides; its other reduction kernels require packed elements and use the
harness's explicit logical-stride extension.

Dropping the second wide-add operand failed the first SUM graph at byte two
(actual zero, expected 64). Routing MEAN division to addition failed the first
MEAN graph at byte two (actual zero, expected 128). An earlier attempted
MEAN fault changed a different slot-mapping arm and passed; it was not counted
as refusal evidence. The corrected mutation targeted the arithmetic dispatch.
Both actual defects were restored byte-for-byte. A false theorem replacing
MEAN narrowing with wide addition failed Lean by rfl with LLV7002. Restored
source/semantic identities matched the kept artifact; replay verified both
bindings without repeating axiom audit, Lean, leanchecker or root extraction.

The final workspace run passed 28 contract tests in 32.50 seconds and every
other workspace target; VT-02 alone rejected the unpublished development
dependency. This also exercised every existing arithmetic/scan/fold graph
through the consolidated word effects and retained model-validation negative
tests. Scoped formatting, model/source audits, frame synchronization, Clippy,
cargo-deny, template checks and SDK lock checks passed. Handwritten source is
10,558 lines in the same 51 files, below the preceding 10,560-line checkpoint.
Repeated validation error construction, document writes and setup bindings
were consolidated without changing their guards or generated documentation.
Full imported operation/type/lifecycle coverage and production dependency
acceptance remain unfinished.

## Row comparison foundation: Verdant VTK-08

The development dependency advances to 61ccbe98d084643c53ca2fb020e5ae51e82a32a9.
Its single new Prism root classifies F32 less/equal/greater and which operand
is NaN, compares bounded U32 words, adds U64 words with typed overflow, and
preserves word identity. The result is
7f0a468d8de62f47d9693d14755752546f976dffaf7e1e67f9453b2cb38cc256,
attestation 469944e7bf3b3d1bd78e3fce3175e5b9716469b840d84868e854c0e8c5201466.
The exact generated package fingerprint is 3e01166f. The model was authored
and checked through LexLean MCP, then proved/exported once in the pinned SDK.

All 573,440 comparison cases matched the retained authoritative SoftFloat
f32_eq/f32_lt predicates. Both signs, every exponent, fraction boundaries,
zeros, infinities, both NaN classes and seeded words are represented. Width,
unknown-operation and integer overflow refusal tests and certificate freshness
and replay passed. Reversing the imported less-than operands failed while
all SoftFloat library/object hashes and mtimes remained unchanged. A false
signed-zero theorem failed Lean with LLV7002; exact restoration reproduced
kept identities and replayed bindings without repeating proof phases. The
unchanged arithmetic corpus was not repeated; the 152-case wide smoke passed.

Engine formatting, scoped Clippy and cargo-deny passed. The new surface
projection retains the previous counts and ceilings: 205 lifted, 1192 residue,
73 foreign and 78 effects. Full Engine acceptance still refuses stale self-host
manifest records and the retained oversized UORC build; both dirty Wasm files
are excluded. The dependency source and its bundle are hash-bound and prepared
without resetting local changes. Row graph acceptance is recorded separately.

Upstream main was inspected at e3abe5bbadaa67ac1cffa10fd951396bfd0e2b2d,
Verdant v2.9.0, whose native build automation and lifting work inform the
remaining Engine acceptance/migration work. This bundle retains its declared
base and does not claim upstream migration or production dependency acceptance.

## VT-30: ARGMAX and COUNT_EQUAL graphs

The registered scenario and initial supports-operation failure preceded
implementation. Prism admits F32 matrix ARGMAX to an I32 vector with one
index per row, requiring matrix extents and the imported INT32_MAX width
limit. It admits COUNT_EQUAL for equal-shaped I32 operands to an I64 scalar.
Type/shape/source/allocation refusals precede writes. Source and destination
strides are independent; COUNT_EQUAL has a separate right indexing context.

ARGMAX models the imported sequence `max = MAX(max, x)` followed by the index
update when `max == x`. Equal maxima choose the last index, including either
signed zero and negative infinity. A right NaN replaces the maximum but keeps
the index; the next finite value replaces that NaN and updates the index even
when smaller than a prior finite maximum. An all-NaN or zero-width row returns
zero. Prism emits comparison, maximum selection, index selection and final
writes; the new typed selection request carries only the comparison flag.
Numerical classification and word operations execute solely through Verdant.
COUNT_EQUAL compares raw I32 encodings, so opposite sign bits and float-looking
NaN words are ordinary distinct integers. Its certified checked integer sum
writes the I64 scalar through the native word binding. The admitted count is
retained in the plan, avoiding repeated extent products during execution.

The final pointwise result is
9f5dce5498200d553ad81c5ad217267471a9e7822a58ba84b66ab96817e0e137,
attestation 120c8c6fac0e25292116c64f49af0a069e1d8d93c06444ba03f2a12cdd58c588.
Source identity d3dbbf8860c43a99cee7145eda9dfd1dbbd36e408dbfd65c19a6469e079953ff;
semantic identity f7ee6716870c9a107b5db2ec240af7e5742e834702091922ec16b8c0a076d5a8.
Generated output is unchanged SDK output, fingerprint 1787daf3. Only the
changed pointwise project was exported. An initial accepted row model exposed
an empty-input overflow on new direct plan probes: unused extents 2^40 could
multiply before recognizing zero elements. The model now checks emptiness
first; scalar SUM and COUNT_EQUAL emit two instructions for those probes.
Universal zero-width theorems cover arbitrary remaining natural extents.
This necessary model correction was re-exported; unrelated proofs were kept.

All 2,264 imported CPU comparisons passed: 1,120 numerical ARGMAX and 1,120
numerical COUNT_EQUAL graphs, plus eight empty ARGMAX and sixteen empty count
graphs. Numerical cases combine fourteen patterns, five widths (1, 2, 3, 17,
65), two row choices, two batch choices and four layouts. ARGMAX varies its
matrix row count; COUNT_EQUAL exercises all four axes and independently
strided operands. Data include repeated maxima, opposite zeros, all negative
infinities, all NaNs, NaNs before and between finite values, subnormals,
negative values and seeded words. Equality patterns cover all equal, all
different, alternating and sparse differences. Exact integer outputs and
all padding bytes are required, with no NaN normalization of integer data.
The two large-empty plan probes pass in addition to those CPU graphs.

The imported count kernel computes its multidimensional row coordinates with
`i03*ne03` and `i02*ne02` where the general logical decomposition requires
other products. Arbitrary shapes can therefore read incorrect or out-of-bound
rows. The public header specifies the number of equal elements. Reference
comparisons retain raw 2D layouts and balanced 4D layouts where the imported
index equations are valid: n1=n2=k, n3=k*k, with k=1,2,3,4 and mixed equality
results. Other shapes use the imported CONT operation to normalize both
operands to contiguous 2D tensors before calling the unchanged COUNT_EQUAL
kernel. That normalization preserves logical equality while avoiding undefined
upstream accesses; it is explicit oracle scope, not evidence that the buggy
raw CPU kernel implements arbitrary strides/shapes. ARGMAX uses the raw CPU
input whenever its element stride is admissible, normalizing only the strided
element extension. Output comparison always retains actual output strides.

Changing the live comparison flag so ties retained an earlier index failed
ARGMAX (actual 1, expected 13). Treating a left NaN as a right NaN failed the
recovery case (actual zero, expected one). Dropping COUNT_EQUAL's integer
addend failed the first equality graph (actual zero, expected two). Native
sources were restored byte-for-byte. A false last-tie theorem failed Lean by
rfl with LLV7002. Restored source/semantic IDs match the kept result; artifact
replay verified both outputs without repeating axiom audit, Lean, leanchecker
or named-root extraction.

The workspace command passed 29 contract tests in 25.60 seconds and every
other target, with VT-02 alone rejecting the unpublished development override.
The final raw-layout reference refinements and explicit ARGMAX width-limit
refusal were then checked by the affected row corpus only (4.16 seconds). Scoped formatting, model/source audits, frame sync, Clippy,
cargo-deny, template checks and SDK locks passed. Handwritten code remains
10,558 lines in the same 51 files. Scenario tests now own their expected case
counts directly, removing duplicate mode dispatch from the common harness.
Digest formatting is shared, and ID uniqueness uses one ordered-set insertion.
Scratch bindings use the same bounds-checked indexing convention as admitted
pointer slots; Prism emits only the three literal register indices.
Complete imported coverage and production dependency acceptance remain open.

## Verdant 2.9 upstream migration checkpoint

The development source now extends upstream
`31c93199d2e77c16cf6044ad3b61654c92e66c68` at
`5ebdafd58f114c8c272177c99134593762dd6567`. Its Git bundle is SHA-256
`1be003f9142542f2e2616bf17e3ca316ff94501dcf4bdd54b0a71f393c10a962`.
The preparation recipe verifies that bundle and clean exact checkout; the old
clean dependency commit is preserved locally as `vtensor/preserved-61ccbe9`.
The universal policy files and release checks remain unchanged.

Verdant combines the ten Tensor models into a twenty-root SDK export:
result `3df6d8268ddd7716fa9fbc43dc425922c623b2b175e583beb0949e109462d8fa`,
attestation `38e9980830f040c8cdd3c28379a7a3ee4e7e1e8fbf6c7b5fcb461006a629eaa5`.
Every generated library matches the previously validated numerical source byte
for byte. Nine certificate tests passed with one shared replay (1.68 seconds);
all expected roots, source/semantic identities, lock and compiled bytes remain
checked. The earlier full numerical migration run passed its 22 numerical
tests; stale legacy certificate locks were rejected, then replaced by this
unified project. Unchanged mathematical roots were not reproved again.

New VTK-09 failed first on a missing separated test fixture, then passed against
87 exact source-DAG and manifest entries in 89.41 seconds. Removing one exported
root was rejected as 19 versus 20; removing a generated package manifest was
rejected by native source lifting with a typed missing-source error. Both
faults were restored. The shared-artifact checksum regression rejects a second
authority with a conflicting checksum. The existing ten-catalog runtime oracle
check passed; GGML and SoftFloat archives retain their independent import
validation instead of being treated as runtime JSON catalogs.

Native Wasm rebuild measured 157313-byte core and 3919275-byte UORC artifacts.
An earlier 3923259-byte UORC was rejected by the unchanged 3919872-byte ceiling.
Consolidating repeated GGML authority records preserved citations and oracle
coverage. Native source/ratchet generation retained all upper limits and
measured 282 lifted functions, 1177 residue and 1334 uncertified functions. These
are surface measures, not new numerical correctness proofs.

Verdant's exact-SDK template, model, fidelity and workspace Clippy checks passed.
Its first full acceptance run passed 70 engine checks and rejected two: a fixed
authority count and a UI check aimed at the pre-extraction source file. Their
corrected checks pass in the isolated migration worktree; a subsequent
checkpoint must capture those corrections and updated native source evidence.
Production packaging still needs the certified dependency closure and removal
of development path/Git dependencies. VT-02 continues to reject the development
override; neither Verdant nor vTensor is claimed release-complete here.

For this vTensor pin, workspace compilation, scoped formatting, model validation,
all-target/all-feature Clippy and cargo-deny passed. The full contract run
reported 29 passed and exactly one failure: VT-02 rejected the development
source override. This is recorded as a failing release boundary, not waived.
Handwritten Rust remains 10558 lines across 51 files; only its exact upstream
revision constant changed.

The follow-up development pin is
`f4d3dd44bdbd8b41ebd18f24c15347046157cd28`, bundle SHA-256
`c973ae23c8d0ba01224b5fb3ed04a8e76ce3e0db326aed25c77d0336f2c7064d`.
It captures both corrected Verdant checks and regenerated native source
manifest/frame evidence. A conflicting GGML authority checksum and a
disconnected gameplay-panel import each failed the corresponding corrected
check; original bytes were restored. The two positive checks, nine certificate
replays and conformance Clippy passed. Native generation retained zero unlifted
workspace/UI modules and all ratchet limits. No production numerical source or
certified library changed between these two development pins, so their large
oracle corpora and unchanged proofs were not repeated for the follow-up.

## Certified production dependency packaging

Development pin `4d52da880b1b383ddbd3028113da4268b4707daa` captures VTK-10.
Its source bundle SHA-256 is
`1e386f73cda75729d00a17572919667bb0109b82ef037a12a521766383dafccd`.
The registered scenario first failed because native packaging omitted
`verdant_float_storage`. Verdant now follows its resolved runtime/build graph,
including direct imported features, and packages 33 crates: four shipped crates,
twenty certified Tensor libraries and nine non-registry imports. Normalized
manifests use exact dependency versions and registry references; original
manifests and SDK package/provenance metadata remain present. The test checks
all twenty generated libraries and their metadata against the authoritative
SDK export byte for byte. Embedded core resources are relocated into its crate.

Cargo from the immutable Verdant SDK is the imported validation oracle. The
Cargo Book registry-index specification is imported with SHA-256
`11660f5678ee206d0997f1fe1712a2ba83fbc08a367a9af72a9c333a774d422e`.
A live loopback sparse registry must now produce a successful real Cargo
consumer build using the shipped APIs. The earlier verifier could report
success after a failed Cargo process; the new gate rejects that result.
VTK-10 passed in 22.93 seconds, including missing-SDK-index and syntactically
valid unresolved-dependency defects. The latter must name the missing package
and report `real Cargo consumer failed`, proving that endpoint responses alone
do not satisfy the gate. A real 33-crate registry was generated natively.

Native Wasm and source generation passed without increasing any limit: core
157313 bytes, UORC 3919723 bytes (ceiling 3919872), zero unlifted workspace/UI
modules, and frontier 282 lifted / 1177 residue / 1334 uncertified. Source
capsule identity is
`eaebf567bcd553a837f2316725f2893290f6308503f51f2690d5373bda052679`.
The full exact-SDK Verdant acceptance run is in progress for this checkpoint.
No numerical model or generated Tensor library changed, so the vTensor
numerical corpora and proofs are retained. This packaging improvement does
not publish a release or satisfy VT-02's upstream production-source boundary.

The corrected follow-up pin is `1f827a7112cebdf1b98aba24d3eb696aed5dce3b`, bundle SHA-256
`dcea515b8a20273560b5367ac242d2f242924a889bcb6591f0f4d05f0377c2bc`. Fidelity rejected a raw sparse-schema `2`; Verdant now
uses its existing certified `two_u32()` value and regenerated native source
evidence. No rule or limit was relaxed. Restored capsule identity is
`46756621499633e505b5b443b40e3b9e39d59791ef5973300778064f4a54460c`.
The full restored run passed all workspace lint/tests (73 engine and 31 Tensor
tests), template, model, experience, fidelity and dependency checks. Final Prism
verification remains active; complete acceptance is still pending. The updated
33-crate registry was regenerated from the corrected source.

Verdant's restored full `just vv` subsequently passed with exit 0. Final
attestation is
`a83ef1d53cae059f70e59ab7978050c69fef04f1f7436eee24cdccea422843bc`.
The current evidence pin is `5fd03d498df926f7770ad131ea8c828f4b657e86`; bundle SHA-256
`0231025623a4636eeee4bcd0c4915e8b1d2c83f3364ee38b38d4e49b4d5590f5`. The exact SDK acceptance, Lean attestation, build
manifest and input receipt are committed in the dependency's
`verification/tensor-delivery/` directory. They cover the main application
artifact closure, Cargo package, core Wasm, browser projection, modeled vectors
and independent kernel replay, separately from the twenty Tensor root
certificates. Only evidence and documentation changed after the passing run;
its numerical sources and proof inputs were retained. Full Verdant acceptance
does not establish complete GGML coverage or waive vTensor's VT-02 boundary.

The reported final identifier is the SHA-256 of the SDK verification manifest.
The preserved independent LexLean attestation has ID
`3779a2f8f076e8414ba8228b77743b97ecc8d6e86b628f0de0ffc70e52d6cd33`;
both it and the application acceptance bind source
`102bd4890f71f4e14829a64b9789d52de23f298dc0c233ccb9d9f0afdd46e8bc`.

## Indexed row-gradient accumulation (VT-31)

Prism defines `GET_ROWS_BACK` admission, the index-validation phase, logical
zero initialization and ordered F32 accumulation. F16 inputs widen through
Verdant's existing certified storage conversion; F32 inputs retain their bits
before the same certified addition. I32 indices are checked as signed values
and bounded by the destination row count before any output word is written.
The primitive bindings add no handwritten tensor arithmetic.

The imported constructor/kernel permits an index vector shorter than the
source matrix. The revised plan traverses that prefix rather than assuming
all gradient rows participate. An empty index vector selects a constant-zero
plan and reads no gradient buffer, including when unused gradient rows exist.
The corpus removes that buffer before vTensor execution to test this property.
Nonempty accumulation retains the imported CPU loops' signed-int extent bounds;
checked planning overflow and invalid descriptors remain typed refusals.

The 3360 graphs cover both input types, widths 0/1/3/17/65, index counts 0/1/3/9,
destination row counts 0/1/4/7, three independent stride layouts and seven index
patterns. Prefix patterns leave poisoned unused gradient rows. Cases include
cancellation, signed zeros, subnormals, infinities and NaNs. Every finite output
bit and every padding byte is compared; arithmetic NaNs retain the established
classification comparison policy. Direct CPU kernels are used when their
word strides are admissible; otherwise the imported copy/CONT path normalizes
the logical inputs for the authoritative CPU kernel. No oracle source is edited.

There are 432 nonempty bad-index graphs (negative or upper bound, placed last)
that must fail with the complete output allocation unchanged. Focused descriptor
probes reject I64 indices, insufficient gradient rows, non-F32 output and output
aliasing. An injected one-bit error after Verdant addition failed with
`oracle mismatch type=0 width=1 rows=1 byte=0 actual=1 expected=0`; the original
source bytes were restored. The initial implementation check failed because
GET_ROWS_BACK was unsupported; the prefix extension also failed before its
model correction. All 3360 restored graphs pass, including unused-buffer cases.

Final pinned-SDK production result:
`4837c1653cba871693654a0b016bb94fbabc19f2ebe5006bf444f15d9574243b`.
Attestation:
`9fffe3cb9315e90b15dc275f942d862178a93bc1f43270aabf1675d791684e8c`.
Source:
`bc8ac1ece94e34a4092cde08b39aca71139a711b2d713d2fb8b71d18c9aae8e1`.
Semantic:
`400a160ca682ea0dcf4fdef139b0f21158e0d5632ff5f10c59cfee46d9234bd5`.
The kernel lock and generated outputs are copied from the SDK export unchanged.
The generated-source license clarification retains its expression and binds
the new pointwise file fingerprint `0xa82506e1`.

Existing arithmetic/update/fold/scan word-bank selection now resides in Prism;
the host's duplicate bank remap is removed. The oracle comparison helper shares
complete allocation checks while preserving scan-first-word NaN payload checks
and exact integer comparisons. Shared fixed-width hexadecimal validation and
SHA-256 formatting replace duplicate host implementations without relaxing
syntax, checksums, source freshness, artifact binding or CI policy.

Formatting, model validation, all-target/all-feature Clippy and cargo-deny pass.
All 62 tooling tests pass, including immutable-pin and proof-cache defect tests.
The final workspace contract run reports 30 passed and one failure: VT-02 still
rejects the declared development dependency. Handwritten implementation remains
10558 lines across 51 files, equal to the unchanged shrink-only ceiling.
The numerical primitive proofs/corpora are unchanged and were not reproved.
Complete imported coverage and production source acceptance remain unfinished.

The final prefix-aware authoring draft also passed independent MCP verification:
attestation `d69f672c5e605cd7e41a86647e27ebecd6a9de7706297c1191fbd824bfd58bba`,
source `ca5f618aeb082b9fe133915944867b40ff50c5740dd50863cbc946e6005f0c58`,
semantic `2fcddc7df8d52f681b24fee739ef75fcb66e55c545ea3413b950c005a568e953`.
Those identifiers belong to the authoring profile; the deployed code remains
bound to the pinned-SDK production identifiers above. The unchanged repeated
right-offset library was retained byte for byte across the export.


## Packed-scale and F8 execution foundation

Verdant extension `79d86c94170b88c5a52e3180466a3bab51a99c5b` adds VTK-11
without changing the bytes of the twenty existing numerical SDK libraries. The
new SmallFloat Prism root handles E8M0, E8M0-half, unsigned E4M3-half, signed F8,
the two upstream F32 encoders, and doubled E2M1 coefficients. This foundation is
for complete packed row gathering; no GET_ROWS graph claim is added here.

The current twenty-one-root SDK bundle is
`9e0295585e59ecee5c12da182f0e6a232210440c61a7774c9dff4bf838536a9a`,
with independent Lean/leanchecker attestation
`c7bc0cd2d04ad9d8128b3aefb6aa2d736d600603a9774f04327d4e50b1ca6ba4`.
Its 1,040 exhaustive byte/nibble and 243,452 encoder comparisons against the
pinned imported GGML helpers passed. A planted one-bit E8M0 defect was rejected.
The final generated implementation has no normalization or exponentiation
loops. Certificate replay and typed refusal checks pass. Full Engine workspace
verification passed 74 Engine and 35 Tensor tests, plus all model, fidelity,
formatting, Clippy, dependency and production-registry checks. The final main
application proof remains running at this implementation checkpoint.

The authoring MCP service timed out at its unchanged five-minute child limit;
it issued no successful attestation. The pinned SDK completed the model, Rust
target, composition and independent checker stages. The successful SDK
evidence above is distinct from the remote authoring outcome.

Native source lifting retained zero unlifted workspace and UI modules. Core
Wasm shrank to 155562 bytes and UORC to 3919290 bytes; the core ceiling tightened
to 155648 bytes and no bound increased. The native source capsule is
`e5b6127b75c27bd02fb023b9ca6bb5ddfe63eac7480792977d415b82068521b2`.
The new package remains in the explicit development source bundle. VT-02 must
continue to reject that override as production release evidence.

After pinning the extension, all 30 current vTensor contract checks passed;
VT-02 alone rejected the explicit development source, as intended. All 62
xtask tests, formatting, model/frame/source audits, workspace Clippy and
dependency checks passed. The non-Prism source total remains exactly 10558
lines in 51 files, equal to the unchanged ceiling.

The subsequent full Engine `just vv` acceptance passed. Extension commit
`927d2863c320c9d88c5ff9b6929eeb1790e57e79` records the final result and exact
current proof inputs. The application verification manifest remains
`a83ef1d53cae059f70e59ab7978050c69fef04f1f7436eee24cdccea422843bc`,
with independent attestation
`3779a2f8f076e8414ba8228b77743b97ecc8d6e86b628f0de0ffc70e52d6cd33`.
The unchanged main application has source
`102bd4890f71f4e14829a64b9789d52de23f298dc0c233ccb9d9f0afdd46e8bc`
and semantics
`a6a9da96e587973aa53f4052b5687f2339e6c206f0fdf245d6d3d2af004d7451`.
This application evidence is separate from the new twenty-one-root Tensor
certificate and is not a claim that all vTensor operations are complete.
Only documentation and the saved proof-input receipt changed after the tested
79d86c9 implementation, so its numerical and integration results are retained.

## Packed fields and authoritative GGML codebooks

The Verdant development extension now carries VTK-12 and VTK-13. Its
23-root Tensor bundle is
`7bed106bf36ece828868bb6a64e276d2e676cd1127939e4cae441d0bd7366d4b`,
with independent SDK attestation
`a1f97ba78c6a5009a237a731669e0d87dc514add7e9a70e6dc977b513c8c50ab`.
The authoring MCP verification also succeeded, with attestation
`09fe01512447485be085023287757b8f22d579e1026cc85816a1db34a379a037`.
All 21 prior SDK runtime libraries retain their exact bytes.

Certified field extraction and bounded integer-to-F32 encoding passed 778,496
comparisons with the pinned GGML CPU callbacks, covering all byte values,
packed lanes, high-bit planes, signed coefficients and ternary wrapping.
Typed invalid-divisor, modulus, bounds and intermediate-overflow cases pass.
Changing the coefficient encoding and the field factor made the numerical
gates fail; the certified files were restored.

The compiler-preprocessed upstream header enumerates all eleven CPU codebooks;
the original compiled arrays export their values. The import contains 34,984
canonical little-endian bytes, exact table spans and per-table hashes, and the
original MIT license. Data SHA-256 is
`4c7dda6f97acca70654a93868dda58faa3e299b9e3044b95afa9a55147c96c2a`.
Both compiled and on-disk resources are compared with the authoritative export.
Planted byte and offset defects were rejected. Validation writes only its target
cache; importing source resources requires an explicit copy. Native lifting and
production-package tests retain the binary, catalog and license.

The complete Engine evidence, including each planted fault and restored result,
is retained in the pinned extension's TENSOR-VERIFICATION.md. This is a decoding
foundation. Complete packed-block execution and GET_ROWS graph support remain
unfinished, and VT-02 continues to refuse the unpublished development dependency.

The implementation checkpoint is Engine
`1af128086268c5ce1890463d8cd9501795c311af`. Its complete workspace run passed
76 Engine and 40 Tensor unit tests, formatting, all 93 model IDs, template/lock,
experience, fidelity, Clippy and cargo-deny. The final application proof failed
during a host disk-exhaustion incident; it remains pending a successful retry.
No full Engine acceptance is claimed for this checkpoint. Disposable Cargo
caches were cleared; source, oracle builds and certificate artifacts were kept.

After importing that checkpoint, all 30 numerical/integration contract checks
passed; VT-02 alone rejected the development dependency. All 62 tooling tests,
formatting, model/frame/source audits, workspace Clippy and cargo-deny passed.
The source ceiling remains unchanged at 10,558 non-Prism lines in 51 files.

The final application component subsequently passed its unchanged pinned-SDK
`just app` retry. Engine evidence commit
`276404d37427dfb844eedc2e92f0652ba5176629` saves the exact current proof-input
receipt. The three application evidence JSON files remain unchanged, with
manifest `a83ef1d53cae059f70e59ab7978050c69fef04f1f7436eee24cdccea422843bc`
and independent attestation
`3779a2f8f076e8414ba8228b77743b97ecc8d6e86b628f0de0ffc70e52d6cd33`.
Every component of the original Engine acceptance run has passed for the same
implementation; the original full invocation failed during disk exhaustion
and its application component passed on retry. A successful full release
invocation is not claimed here. Only documentation and the saved receipt
changed after the tested implementation, so its numerical evidence is retained.
An additional 120 baseline ABI comparisons passed for both camera modes,
wrapped nonzero yaw and extreme signed camera intents. The unchanged vTensor
source still requires complete decoders, GET_ROWS and the remaining imported
operations, plus resolution of the production dependency and VT-02.

## Row-gather preparation: shared fixture enumeration

The allocation, transfer and byte-span fixtures now share a lazy Cartesian
iterator. Their parameter domains and nesting order are preserved; explicit
counts retain all 864 placement and 1,024 transfer cases. The byte-span oracle
comparison retains all 36 types and all original nonempty and empty layouts.
The three affected imported-oracle checks pass, along with tooling tests,
workspace Clippy, model/frame checks and the source audit. No implementation,
model or certified runtime library changed. Handwritten source decreases from
10,558 to 10,549 lines in the same 51 files. This supports the remaining gather
work without increasing a ceiling or removing validation cases.

## GET_ROWS contract and failing implementation gates (VT-32)

VT-32 is registered before its implementation. The native regression derives
30 source formats from the pinned GGML type traits plus F32/I32, builds an
actual upstream GET_ROWS graph, and checks the CPU result before asking
vTensor to run it. All 174 upstream cases succeed: 30 nonempty cases and
four empty-axis cases for each of the 36 live storage types. vTensor passes
all 144 empty cases and fails all 30 nonempty cases at support and execution.
The test accumulates failures across the full corpus before refusing, so
every format runs even while GET_ROWS execution remains unimplemented.

RowGather.lex.tex defines the four-dimensional constructor conditions,
I32 signed-index bounds, logical index addressing, source block addressing,
and the imported codebook spans. Empty outputs admit all 36 known storage
types while nonempty outputs use the 30-format CPU oracle domain. General
shape/index decision theorems and
boundary examples passed authoring verification. Canonical authoring
attestation is
`cc029e4b5152ac6df80226a0c97b3122d8a8f411c183dc2ed0352583ee248d04`.
The exact signed envelope, source hashes and separate pinned-SDK structural
check identities are in verification/row-gather-authoring.json. The model has
its own locked project; it is not yet wired into the graph planner or exported
as a runtime implementation. The authoring proof is not SDK runtime acceptance.

The pinned Prism arithmetic proof form closes the general index bounds.
The upper-bound theorem's exact observed axioms are Quot.sound and propext;
the other two general decision proofs require propext. No axiom policy or
proof limit was relaxed. The compiler's fixed proof form is documented in
[LexLean v0.4.0](https://github.com/UOR-Foundation/LexLean/blob/v0.4.0/SPEC.md).

Q8_1's missing CPU to_float callback and Q8_K's absence from this row-operator
domain remain explicit reference gaps. The cpuSource predicate describes the
executable oracle domain, not a claim that every imported numerical requirement
is implemented. All storage types retain their existing storage/copy contracts.

The new regression is an unfinished TDD checkpoint. The source audit currently
rejects its added handwritten harness (10,603 lines versus the measured
10,558-line parent). The immediately preceding fixture-refactor checkpoint
is at 10,549 lines, which becomes the parent budget after this commit.
The harness must be consolidated and the final implementation must fit the
monotonic budget. No ceiling or acceptance gate was changed. Full operation
coverage, production dependency resolution, and VT-02 remain outstanding.

The initial Engine R3 checkpoint was
`00fd83066677e43dcdf0060ecdef53426eaad30e`. Its new external reference mode
passed all 270 cases across the 30 formats. The production-root check and
public API check remain failing (missing StorageBlock root and E0432 for
missing decode_storage_block). The new expected-value corpus is initialized
once per process and reused by the pending numerical and refusal tests.
The prior runtime libraries and production request are unchanged. Neither this
Engine stage nor VT-32 is accepted as implemented; the explicit development
bundle retains their failing checks for the next implementation step.


## Storage-block execution checkpoint for GET_ROWS (VT-32)

The pinned development Engine checkpoint is
`eceb2a6b3efe5da2091c5005694a213581902112`.
Verdant's StorageBlock model now contains instruction plans for all 30 CPU
formats. Prism authoring verification and the exact pinned SDK 24-root production
export pass. Authoring attestation is
`249ff2b2afaf6e962b1d8ae227b193096418c69c6d0b6507ebdae714ba454c90`;
native export attestation is
`0c5d6357f64238400705938c50773d6b40f0ae7de32c2ae041a5521acc3e93d8`.
All 23 previous runtime libraries remain byte-identical.

The new effect adapter loads modeled offsets, calls existing certified Verdant
primitives and stores modeled words. It carries no handwritten codec formulas.
All 810 imported CPU block executions now run: 732 match every byte; the other
78 differ only in 15,360 NaN payload/sign words. The complete diagnostic checks
every logical word and padding byte and reports zero other word differences
and zero padding differences. The strict equality assertion remains failing.
The reference test and all 227 typed/atomic refusal cases pass. Focused tensor
Clippy and the wasm32 bridge type check pass. The dependency's authoring record
contains the signed envelope, native export identities and numerical diagnostic.

This is an unfinished implementation checkpoint. NaN word behavior must be
resolved before numerical acceptance; no result has been normalized or skipped.
The complete Engine gate, capsule and Wasm budgets, native proof frontier and
production package closure remain to be refreshed. Root GET_ROWS execution is
not wired to the decoder yet, and its 30 nonempty graph cases still fail. Root's
10,603-line harness still exceeds its 10,549-line parent budget. No acceptance
ceiling was changed and this checkpoint is not a full `just vv` pass.


## Initial GET_ROWS graph execution passes (VT-32)

The main PointwiseGraph model now imports RowGather and emits a complete
preflight pass before any output write. A second pass reloads each selected
index and emits either raw F32/I32 word copies or typed Verdant block-decoding
requests. Batched source addresses and independent output strides are defined
in Prism. A packed source stride shorter than its physical block is refused
before constructing any native slice. Native glue performs only the admitted
load/store effects and calls decode_storage_block.

The authoring proof passed with attestation
`58865e17005c5cd8e0a9cb64df17f272af8bb1e599fa598d9b4b2ba11dd1ee6b`.
The corrected pinned-SDK export passed with result
`53013e44c3d97888b240830a309fe66b434e1c457ad5c7a6c8232046ca45e6e7`,
attestation `844652c2a565d87554b18876def265fd7de839ccccabb9f9d1f5f8f401ed129c`.
The repeat-offset runtime library is byte-identical to its previous export.
An earlier unaccepted export preceded the packed-stride guard; it was replaced
by this corrected export before native execution. The authoring envelope,
canonical file hashes and production identities are recorded in
verification/row-gather-runtime.json.

The native VT-32 run passed all 174 graphs: 30 nonempty formats and four empty
axes for each of 36 known types. It also passed 175 static descriptor/alias
refusals and 60 bad-final-index cases, including unchanged output bytes after
each late refusal. The run used the existing pinned development decoder and
took 25.18 seconds after compilation. This initial graph corpus is finite-valued;
it does not resolve the separately recorded decoder NaN-word failures.

This remains an unfinished development checkpoint. Source audit reports 10,641
non-Prism lines, versus 10,603 in its measured parent. The last accepted fixture
budget was 10,549 lines, which remains the completion target; intermediate red
commits do not establish a higher accepted ceiling. The harness and glue must
be consolidated without removing coverage. Independent graph layout/view
coverage, remaining decoder exceptional values, representative planted defects,
all prior graph regressions and the complete release gate remain open.


Upstream was rechecked during this stage: afflom/verdant-engine main is now
`1c78b377c57a0714b74170d00bc1580067846efb` (v2.10.0, M18). Its additions
cover sovereign governance, native source modularity and 321 MIR-lifted
functions; they do not provide this storage-block decoder. The local main
Engine worktree also has separate uncommitted changes, which were left
untouched. Decoder work remains in its isolated development branch. Upstream
integration and preservation of the stricter local budgets remain open.


## Expanded GET_ROWS and exact decoder integration (VT-32)

The pinned Engine checkpoint is now
`3635dcf766665509caef009e5a8f89856556a7e1`. Its decoder passes all 2,250
exact-word/padding executions (750 blocks, three strides), 16,944 exact
arithmetic-word comparisons, and the native and model refusal cases. NaNs are
compared without normalization. A planted load-endianness defect rejected
2,163 block executions; restoring its source restored all four component tests.
The pinned bundle retains those proofs, regression cases and fault evidence.

The final main-model authoring attestation is
`5dc5bf964007e6d2b92d5263b3fcd1ce129dda19691555d300cb355a51b5e7b7`;
production result `7208140b23408b7e92dd1b0324c5659e6a07234718cb080cad11b0531df44041`,
attestation `77fce78dbfe583e222347097fc24e9fc08808e230e505fc7f00cbb6b2fbcc8c8`.
An intermediate export triggered Clippy's collapsible-match lint. Optional-input
handling and concrete descriptor admission were separated in the model, and
unknown storage retains its unsupported decision. The final SDK export and
Clippy pass without altering generated code or lint policies.

The expanded graph corpus passes 384 cases: 240 nonempty cases over all 30
CPU formats and four layouts/two selection patterns, plus the original 144
empty cases over all 36 storage types. It covers padded source views with row
and block offsets, independently strided I32 indices and outputs, repeated and
reversed selections varying by batch, and asymmetric batch dimensions. It also
passes 175 static descriptor/alias refusals and 240 bad-final-index cases with
unchanged output. The final full contract run passed 31 tests and failed only
VT-02's required refusal of the unpublished development dependency; zero tests
were ignored or filtered. That invocation took 109.66 seconds.

A planted native stride-binding defect passed stride+4 to the decoder. The full
384-case gather regression rejected 200 nonempty packed-format executions with
supported=1/computed=0. Restoring the exact pre-fault source restored the passing
384-case corpus before its final batch-data strengthening; the final full suite
also passed the strengthened corpus. This demonstrates that the oracle rejects
an erroneous native effect binding rather than just checking modeled examples.

The repeat-offset library remains byte-identical. Formatting, focused Clippy,
model generation (38 IDs) and frame synchronization pass. The source audit
still rejects 10,653 non-Prism lines against its 10,603-line measured parent;
the completion target remains the last accepted 10,549-line budget. Restoring
that budget, refreshing the Engine frontier/footprints/package closure, resolving
VT-02 through a real production dependency, completing the remaining imported
GGML operation coverage, and a complete `just vv` pass remain required.

## Shared authority validation and structured policy parsing

The BDD CM-03 test now uses `Model::check` as the single validator for authority
resolution, citations and checksum reasons, then retains its existing ledger
non-vacuity guard and artifact validation. Its second handwritten pass over the
same authority conditions was removed. TC-01 now reads the universal policy list
through the existing JSON reader rather than splitting the serialized text.
No tensor algorithm, graph domain, oracle case or generated output changed.

A temporary valid `some-true` claim first passed the BDD authority check, then
four independent planted defects were rejected: a missing referenced authority,
an empty citation, an empty checksum reason, and an incorrect artifact digest.
Both model files were restored byte for byte. All four restored BDD tests,
TC-01, formatting and backend Clippy over all targets at `-D warnings` pass.
The numeric suites were not rerun because their models, bindings and corpus
were unchanged. Logs are `/tmp/vtensor-authority-consolidation-faults.log`,
`/tmp/vtensor-harness-consolidation.log` and `/tmp/vtensor-harness-restored.log`.

The measured non-Prism count decreased by 20 lines to 10,633. This is still
84 lines above the last accepted 10,549-line budget; the source gate remains
unsatisfied against that budget. This checkpoint does not claim release
acceptance or resolve the developmental Verdant dependency.

## Relative-position oracle registered (VT-33)

VT-33 is registered with its behavioral scenario and actual GGML CPU graph
comparison before implementation. Its first behavioral run fails at native
capability admission for GET_REL_POS (`native check line 555:
ggml_backend_dev_supports_op(dev,result)`), as expected. The test is part of the
existing contract harness and is configured for 192 cases over F16/BF16 source
words, relative widths, higher source dimensions, strided outputs and empty
widths. The CPU constructor always produces F16 output; its BF16 path copies
encoded 16-bit words without conversion. Expected words come from the imported
CPU implementation, not a second handwritten indexing formula. This is a
registered, failing development checkpoint, not implementation evidence.
The initial failure log is `/tmp/vtensor-relative-before.log`.

## Certified relative-position execution and shared ABI checks (VT-33)

GET_REL_POS now uses a Prism source-word selector composed with the existing
Verdant memory transfer. No handwritten native executor was added. The model
admits F16/BF16 source words and the upstream constructor's F16 output, including
higher source dimensions that the CPU selector does not read. It models the
reversed relative row and query-plane offset, independent strides, source-row
extent, output shape, minimum word strides and source/output separation. Its
universal theorem bounds every selected source row for bounded row and plane
coordinates. The exact observed axiom policy is `Quot.sound, propext`; the first
proof attempt refused an incomplete declaration, which was corrected to the
reported set before the successful certifications.

Authoring draft
`prism-app/fab757c94474adb7c9dd8d1a886d54f3db3bb0d51c8c86145e97947a95f3934f`
verified with attestation
`e8af07d50d1f865066ec6cd0fe603413e06ad01c295f4da6f6089169a49fcf2d`.
The signed envelope in `verification/relative-position-runtime.json` binds all
six canonical module digests. The pinned SDK production result is
`eb5cd88649dade3543930b093a8fe6650425dc604ed2e61242a19ab71f013fbc`,
attestation `7c3d3f9c8e70405bfaaf0aaec02e78e380d631fc777ae9387eac747cfaf53b03`.
The repeat-offset library remains byte-identical to its previous certification.

All 192 imported CPU comparisons pass: 144 nonempty and 48 empty graphs,
with word widths 1/3/17, relative widths 1/3/7, higher source dimensions and
four layouts. Raw exceptional F16/BF16 encodings and padding compare exactly;
NaNs are not normalized. All 24 malformed descriptor/storage probes refuse
before changing output. Reversing captured word bytes before the native Verdant
transfer made the gate fail with `oracle mismatch type=1 width=1 rows=3 byte=0
actual=60 expected=0`. The runner was restored byte for byte, and VT-33 passes
again in the full suite.

Every GGML ABI size, alignment and field offset is now checked by a C++
`static_assert` against the actual imported header when the content-addressed
oracle is compiled. Live Rust/native source provenance and executable/object
integrity checks remain. Each graph invocation returns only its exact case-count
record, avoiding repeated ABI output/parsing. A planted `ggml_tensor` size+1
observation fails with `static assertion failed: ABI ggml_tensor.$size`; the
restored full suite passes native ABI validation. Type-list serialization also
avoids an intermediate vector and skips its inventory read for modes that do
not consume a type list. No oracle domain was removed.

The final full contract run reports 32 passed, one failed (VT-02's unpublished
development dependency), zero ignored and zero filtered, in 144.13 seconds.
All four BDD tests pass, with 39 IDs/scenarios and 107 test names. Backend
Clippy at `-D warnings`, formatting, model generation and the 52 frame-file
bindings pass. Source is 10,630 non-Prism lines, three fewer than the checkpoint
before VT-33 registration, but still 81 above the last accepted 10,549 budget.
The parent-relative source audit passing does not restore that earlier budget.

Verdant upstream was independently refreshed to
`294c92a3cf0c829a602b2a11be1d59e1122a9358` (v2.11.0/M19). It adds 351
MIR-lifted functions and further Rust/JS modularity, with no changes to the
FloatUnary/ScalarF32 extension roots or a replacement for the local packed
storage decoder. Its integration, preserved stricter footprint/frontier limits,
production dependency resolution, the remaining GGML specification coverage
and complete `just vv` acceptance remain required. The user's main Engine
worktree was left untouched.

## Prism matrix execution schedule and independent output layouts (VT-12)

`MatrixExecution.evaluate` now defines the complete per-batch sequence of
source loads, the Verdant kernel call and output stores. It owns broadcast
coordinates, byte offsets, workspace indices, schedule length and total scratch
size. Its zero-output branch returns an empty schedule before evaluating
potentially overflowing geometry. The native interpreter binds bounded byte
transfers and the existing `multiply_matrix_rows_f32` call; the old nested
matrix loops and standalone native index helper are removed. Three separately
allocated scratch arrays are replaced by one private F32 workspace, split at
the admitted matrix counts only when invoking Verdant. No numerical fallback
or new arithmetic implementation was added.

The existing VT-12 scenario was extended before implementation. Its initial
check rejected the old certified bundle's one root against the two requested
roots. The final authoring draft is
`prism-app/004ee33d374bfa0b9aadeaa0ef3e631be8fe0d70687a3a6a846c935e2eb30f18`,
attestation `991dfb8b13553103ea45540647c2809a439adfcb15268a395a8c72986f21109d`.
The signed envelope binds the canonical execution module. The pinned SDK
production result is
`58b516927e5610c6d0cd3b4ccda38242ee2a6f6c4b1e74f5069218372b76208a`,
attestation `48de2aa9ca57127fa2cc15b59dd1671a5e3189c883330225f0275f30953f6b82`.
The existing matrix admission library is byte-identical (SHA-256
`a9dabc1dae888635ebe4ae75ccc02b73dc5ed55dbe6b4eda230b3900ba3920e3`).
No pointwise or Verdant root was re-exported. The newly generated package is
included in the workspace and exact-output lock, and excluded from handwritten
Rust formatting. The standalone index package is now a test dependency only.

The new interpreter first passed all 640 existing GGML matrix cases. A planted
native binding sent `r3+1` to the schedule instead of `r3`. The resulting left
source coordinate is never larger than its correct value, so the defect is
bounded; the actual GGML output comparison rejected it. The runner was restored
byte for byte before further validation.

The old corpus varied source padding but left the result contiguous. The
expanded corpus independently varies source padding and destination word/axis
strides, yielding 1,280 actual CPU comparisons. A separate ordinary GGML CPU
result supplies expected values for the strided vTensor destination. The shared
byte comparator now checks every output padding byte and exact finite/infinite
F32 words, including signed zeros. Numerical NaNs retain the previous
classification comparison. All 1,280 cases pass in 9.33 seconds, including the
original dimensions, broadcast shapes, empty outputs, zero-inner products and
negative descriptor/buffer/unsupported-node probes. The shared comparator
removes the duplicated matrix-specific result reader rather than introducing
another expected-value implementation.

Formatting, regenerated/checked model evidence, four BDD tests (39 IDs and
scenarios, 107 test names), and backend Clippy over all targets at `-D warnings`
pass. The affected matrix suite was rerun; the other numerical suites were not
repeated for this isolated matrix change. The last full contract result remains
the preceding relative-position checkpoint (32 passed, VT-02 refused the
unpublished development dependency). These scoped results are not `just vv`
release evidence. Current non-Prism implementation is 10,626 lines, four fewer
than before this refactor and still 77 above the last accepted 10,549 budget.
The earlier target remains required even though the parent-relative audit passes.

Exact certificates, source and log hashes, domains and planted-defect evidence
are recorded in `verification/matrix-execution-runtime.json`. Logs:
`/tmp/vtensor-matrix-schedule-before.log`,
`/tmp/vtensor-matrix-schedule-export.log`,
`/tmp/vtensor-matrix-schedule-runtime.log`,
`/tmp/vtensor-matrix-ratio-fault.log`,
`/tmp/vtensor-matrix-schedule-layouts.log`, and
`/tmp/vtensor-matrix-schedule-final-checks.log`.

## Three-source indexed addition registered (VT-34)

The ADD_ID scenario and imported CPU oracle are registered before implementation.
The existing indexed-row harness now shares GGML's own unravel-index helper for
fixture addressing. Its new mode is configured for 164 cases: 20 nonempty ADD_ID
cases and 144 empty cases over every live table storage type and empty axis.
The nonempty cases cover widths 1/3/17/65, padded table views, independently
strided values/indices/output, repeated/reversed indices, exact minus-one
exceptional-word copies, empty tables used only for copies, numerical exceptional
values, and poisoned higher index slices that the CPU operator does not read.
The initial run reports all 20 nonempty cases unsupported and fails its final
completeness check; the empty cases retain their established no-op behavior.
This is a registered failing checkpoint, not completion evidence. The log is
`/tmp/vtensor-indexed-add-before.log`.

## Certified ten-slot input transport and ADD_ID execution (VT-34)

The graph planning request now carries all ten optional GGML tensor-source
slots. Native marshalling validates each present descriptor through the existing
metadata function without allocating a temporary source vector. Missing sources
reach the Prism model as `None`; supported operators still enforce their own
requirements. This does not implement zero-source numerical generators. The
existing two-source planner was preserved as a shared model definition rather
than duplicated or replaced by a handwritten dispatcher.

`IndexedAddition` defines signed I32 admission including the exact `0xffffffff`
copy sentinel, usable table rows, and bounded execution length. Its universal
sentinel and invalid-negative theorems, empty-width theorem and concrete bound
examples kernel-check. The graph model validates all required descriptors,
shapes, word strides and output separation. It preflights precisely the first
index rows/planes consumed by the imported CPU function, then repeats those
selections across value batches. Higher index slices are not consumed. Table
higher dimensions are not selected; empty table storage permits only minus-one
copies. Minus-one rows use exact Verdant byte transfers; selected rows use the
existing Verdant F32 addition primitive. The native extension only binds the
third data pointer and modeled instruction request.

Authoring draft
`prism-app/004eb897a308c8e19164be8c70069e675a5f4fe7ab57c1057d90601818151b80`
verified with attestation
`e9ec6a91a6c69986a261d41142e62188f060a62173e499ff6263f7467798af9d`.
Its signed envelope binds all seven canonical model digests. The pinned SDK
production result is
`c47a91aea96a6f78c331b8de8e41e6df8199e8f8f6d84fc62e27f7ba3818ce16`,
attestation `9296a5ae3e04c236b66374150604ab3efabfcc27f7b52295f9a9f84da1e181e1`.
The repeat-offset library remains byte-identical. The matrix execution package
and Verdant dependency were not re-exported.

The shared indexed-row fixture now uses the actual imported
`ggml_unravel_index` to locate logical blocks; both GET_ROWS and ADD_ID retain
their original operator constructors and actual CPU execution. The empty ADD_ID
fixtures retain block-aligned widths for quantized table types. All 164 cases
pass: 20 nonempty F32 cases across widths 1/3/17/65 and four layouts, and 144
empty cases across all 36 live table types and every empty axis. Nonempty
patterns cover repeated/reversed selections, mixed addition/copy rows,
all-copy rows with an empty table, and numerical exceptional values. Ignored
higher index slices contain invalid -2 values, so reading or validating those
slices would reject a required passing case. Padding and minus-one copy words
(including signaling NaNs and signed zeros) compare exactly. Numerical NaNs in
the addition-only exceptional pattern retain classification comparison.
Sixteen malformed descriptor/stride/alias/missing-source probes and eight late
invalid-index cases refuse without changing output.

A planted native defect flipped the lowest runtime index bit sent to the model.
All 20 nonempty cases were rejected after capability admission
(`supported=1 computed=0`), through the bounded preflight checks. The runner was
restored byte for byte. A formatting-induced Clippy safety-comment location was
also corrected without suppressing the lint. The final full contract run passes
33 tests and fails only VT-02's refusal of the unpublished development override;
zero tests are ignored or filtered. It takes 140.63 seconds and includes all
1,280 expanded matrix cases, 384 GET_ROWS cases and 192 GET_REL_POS cases, as
well as the other existing graph families.

Formatting, regenerated/checked model evidence, four BDD tests (40 IDs and
scenarios, 108 test names), and backend Clippy over all targets at `-D warnings`
pass. The source gate correctly fails: implementation is 10,684 non-Prism lines,
58 more than the previous completed checkpoint and 135 above the last accepted
10,549 budget. This checkpoint does not establish a new accepted ceiling.
Further oracle fixture construction must move into Prism with its full case
coverage preserved; the source policy, production dependency and complete
release gate remain unsatisfied. The broader imported GGML operation inventory
also remains unfinished.

Exact identities, domains, logs and the signed authoring envelope are recorded
in `verification/indexed-add-runtime.json`. Principal logs:
`/tmp/vtensor-indexed-add-before-fixed.log`,
`/tmp/vtensor-indexed-add-export.log`,
`/tmp/vtensor-indexed-add-runtime.log`,
`/tmp/vtensor-indexed-add-flag-fault.log`,
`/tmp/vtensor-indexed-add-full-contract.log`, and
`/tmp/vtensor-indexed-add-final-checks.log`.

## Matrix fixture generation registered and prior corpus frozen (VT-35)

VT-35 is registered before its implementation. Its first test refuses the
missing certified fixture bundle. The existing matrix oracle was instrumented
without changing its case domains or input generation, then passed all 1,280
cases while capturing actual tensor dimensions/strides, control flags, and
logical encoded input words. The canonical transcript has 2,515,840 bytes.
`verification/matrix-fixture-baseline.json` records every case's key and digest,
the complete transcript digest, and the prior fixture source identity. These
are local fixture-preservation evidence; expected numerical results remain the
actual imported GGML CPU computations. The baseline includes the negative-probe
case at ordinal 416. Its source/data domain must survive the Prism migration
byte for byte. This is a registered failing checkpoint, not fixture-generation
completion or release evidence.

Capture: `/tmp/vtensor-matrix-fixture-capture.log` (1,280 cases passed).
Initial refusal: `/tmp/vtensor-fixture-before.log` (missing fixture bundle).

## Certified matrix fixture generation (VT-35)

`MatrixFixtures.lex.tex` now defines all 1,280 matrix cases, their 24 metadata
fields, both source layouts and every encoded input word. The separate locked
fixture project exports a certified `vtensor_fixtures` SDK package. The native
harness imports that data, constructs real GGML tensors and executes the
imported GGML CPU backend for expected numerical results. No expected numerical
result comes from the fixture model. All five matrix refusal probes remain
mandatory at case 416. Invalid families, fields, cases, sources and empty-word
queries return the modeled `None` result.

Every generated case and every actual constructed tensor/input-word record
matches the frozen prior corpus digest. This checks both model migration and
the native binding independently of numerical agreement. A planted low-bit
input-word defect in the Rust transport fails at case 416 in 2.21 seconds.
A second defect in the C++ word binding also fails at case 416 in 15.61 seconds,
even though both numerical backends receive the same altered input. Both
changes were restored byte for byte. The restored scoped pair passes, including
all 1,280 CPU matrix comparisons and the five refusal probes.

Fixture generation and its certificate check execute once per contract-test
process through `OnceLock`, shared by VT-35 and VT-12. Existing integrity-bound
GGML object and native-program caches remain in use. Only the new fixture
project was exported; unrelated product kernels were not regenerated.

Formatting, model regeneration/checking, the four BDD meta-tests and backend
Clippy over all targets pass. The source audit correctly fails at 10,787
non-Prism lines: 103 above the previous development checkpoint and 238 above
the last accepted 10,549 budget. The shared fixture transport and preservation
checks add infrastructure at this stage. Further fixture families must reuse
it while removing their old handwritten generators. This does not establish
a new accepted ceiling, complete imported operation coverage or satisfy R6.

The final full contract run after restoring both planted defects passes 34
tests and fails only VT-02's production-dependency check. No tests are ignored
or filtered. It takes 165.93 seconds, including fixture certificate replay,
all 1,280 matrix oracle cases and every other existing graph family. This is
component validation; the complete pinned-SDK `just vv` acceptance remains open.

Signed authoring provenance, production identities and defect/run logs are in
`verification/matrix-fixture-runtime.json`. The frozen baseline remains local
corpus-preservation evidence; the pinned external GGML CPU remains the numerical
validation authority.

## Arithmetic fixture corpus frozen before migration (VT-36)

The new fixture-family test initially refuses family 1 (the certified package
returns `None`). The unchanged prior arithmetic generator passes all 120 GGML
CPU cases while capturing the complete actual descriptor/input-word transcript.
`verification/arithmetic-fixture-baseline.json` freezes 1,313,024 bytes in 120
per-case digests, including all five refusal probes at cases 0 and 108.
The baseline is local corpus-preservation evidence, not numerical authority.
Registration, initial failure and capture: `/tmp/vtensor-arithmetic-fixture-before.log`.

## Verdant upstream M20 reference review

The isolated migration repository fetched upstream main at
`34c827db624f951609e49f57bbf324671876eeff` (M20, v2.12.0). Its checked-in core
and UORC WASM blobs are 154,334 and 3,802,305 bytes respectively, below the
previously recorded local ceilings. This is an observation of upstream Git
blobs, not a rebuild or validation of the local tensor extension. The upstream
frontier reports 387 lifted functions and 1,167 residue functions. The upstream
tensor crate still lacks the local scalar evaluation and storage-block APIs.
No local checkout, production dependency pin, bundle or acceptance policy was
changed by this review. Exact references and limitations are recorded in
`verification/verdant-upstream-m20-review.json`.

## Certified arithmetic fixtures and shared binary framing (VT-36)

`ArithmeticFixtures.lex.tex` models all 120 prior arithmetic cases, including
eight operations, widths 1/17/257, both padding modes, all four binary broadcast
layouts, all descriptors and every encoded input word. Its independent locked
project exports the certified `vtensor_arithmetic` package. The original GGML
constructors and actual GGML CPU execution still supply numerical expectations.
The matrix model and certified package are reused byte for byte.

Both generated and actual constructed descriptor/input-word records match the
complete frozen prior corpus, including the five refusal probes at cases 0
and 108. All 120 arithmetic and 1,280 matrix numerical cases pass through the
shared binary transport. A changed Rust transport word fails at arithmetic
case 0 in 2.16 seconds. Changing the C++ word binding also fails at case 0 in
12.38 seconds after both numerical backends received the altered input. Both
defects were restored exactly. The latter check detects a corrupted corpus
even when its two numerical executions agree.

The strict SDK initially rejected the word generator with LLV7013 deep
recursion. Small helpers and operand ordering resolve the certificate without
raising limits: the pinned Lean `Nat.mul` and `Nat.add` definitions recurse on
their second operand, so the model puts large constants first. The final SDK
certificate and signed authoring verification pass. No generated Rust was
edited or reformatted.

The transport writes one canonical little-endian binary buffer, eliminating
per-word decimal formatting and parsing. Each fixture project replays its
certificate once, generates its batch once and reads/checks its frozen manifest
once per contract process. Generated records and actual GGML-bound records
retain their separate comparisons. Existing imported reference object and
native harness cache integrity checks remain in force. The scoped run includes
the first reference rebuild after changing the shared batch helper; its timing
does not establish a controlled performance comparison.

The final full contract run passes 35 tests and fails only VT-02's unpublished
development-dependency refusal. It takes 108.07 seconds, with no ignored or
filtered tests. Formatting, model consistency, all four BDD meta-tests (42 IDs,
42 scenarios, 110 test names) and backend Clippy over all targets at
`-D warnings` pass. Non-Prism implementation is 10,840 lines: 53 above the prior
completed checkpoint and 291 above the last accepted 10,549. This remains a
failing source budget, not a new ceiling. Imported operation coverage,
production dependency closure and complete `just vv` acceptance remain open.

`verification/arithmetic-fixture-runtime.json` records identities, the signed
authoring envelope, corpus coverage, planted defects and logs. The M20 reference
review also records a merge-tree analysis with 15 conflicting paths; no working
tree merge or production pin change has been made.


## Verified Verdant M20 adoption

The isolated tensor branch now merges upstream M20 v2.12.0
`34c827db624f951609e49f57bbf324671876eeff` and the previous tensor extension
at commit `774424e472fb6e41dcaafd13a96ab5816eb4fc13`. Its complete exact-SDK
`just vv` passes: 225 workspace tests, including 80 Engine and 44 tensor tests,
zero failures, ignored or filtered tests, and full Prism package verification.
The Engine application attestation is
`a83ef1d53cae059f70e59ab7978050c69fef04f1f7436eee24cdccea422843bc`.
VE-17 independently checks the fifteen unified kernel descriptors.

The actual regenerated core/UORC Wasm files are 153,529/3,798,351 bytes. The
frontier is 390 lifted, 1,167 residue, 74 foreign and 83 effect functions.
The upstream ceilings were preserved or tightened. Native authority bootstrap,
bridge binding, supported RNG lifting and existing StorageBlock grounding were
corrected, with before/fault/restored evidence. No numerical generated source
was edited, reformatted or replaced with host tensor arithmetic.

The complete-history bundle verifies and materializes the exact clean detached
commit through `just verdant-prepare`. Its SHA-256 is
`f310f870ad7625eb684f2a511827dd6c13fe94b395f1433920b8d106de2d58ef`.
The declared upstream base, imported authority citation, version, source guard
revision and lockfile now agree with M20. The lockfile changes only Verdant's
version. The original separate Engine source checkout was not changed by this
integration.

The full root exact-SDK `just vv` verifies the Prism application (attestation
`fc3c75dc3a277b068db8182333edaf698a47a1e9f1f6e15dc359e0ddd4eca942`), formatting,
model/frame/template/lock consistency and all-target/all-feature workspace
Clippy. All 35 other contract tests pass, with zero ignored or filtered tests,
in 156.40 seconds. VT-02 alone fails because the dependency is a local
development override. The full gate therefore remains red. The run completes
five model unit tests and four BDD meta-tests before that refusal.

The separate complete dependency audit passes after adding/refreshing exact
source-bound license clarifications for the generated packages; deliberately
wrong source hashes are rejected. Generated bytes remain intact. The source
audit reports 10,840 lines against an intermediate parent at the same count.
This mechanical pass does not establish the last accepted 10,549-line budget:
the 291-line excess remains open. The observed contract timing includes a
cache/dependency transition and is not a controlled performance comparison.

`verification/verdant-m20-integration.json` records the merge, artifacts, SDKs,
attestations, actual gate results and log digests. This is a development
foundation for the remaining imported GGML operations, not full vTensor or
production dependency acceptance.


## Registered upstream ARANGE regression (VT-37)

The new operation regression uses the unchanged imported upstream test factory
and `test_case::eval`, with the CPU reference mode explicitly enabled. It
requires all ten ARANGE cases, including the 1,048,576-element case, rejects
selected unsupported/failed cases and enforces exact execution/result counts.
The suite object is cached separately from the normal regression driver and
reuses the existing authoritative CPU build. Cached suite sources and program
provenance are checked before execution.

The first run fails on the actual upstream default ARANGE case with
`not supported` (93.74 seconds including the initial suite compilation). All
four registration meta-tests pass. No production generator has been implemented
at this checkpoint. The source budget is now 10,912 lines, 363 above the last
accepted 10,549; this remains unsatisfied. `verification/arange-runtime.json`
records the initial failure, scope, cache boundaries and implementation sequence.


## Implemented zero-source ARANGE (VT-37)

`Arange.lex.tex` models exact positive-integer F32 count decoding, bounded binary
units, execution length and flat output addresses. `PointwiseGraph` admits the
zero-source operation and preflights Verdant comparison/subtraction/division/ceil
before the first write. Decoding the computed ceil word prevents a rounded F32
count from aliasing an incorrect count above 2^24. Each value uses the existing
Verdant unsigned-index conversion, multiplication and addition in CPU order.
The native adapter adds an instruction binding and a source-free request route.
It contains no new host floating arithmetic.

The canonical model passes authoring verification, with attestation
`4c43bcea3b5e304a99f19c202d56c89d9fab5ad3031d7adc084c71c30ed25d0d`. All eight
local file hashes occur in its signed input set. The general store-bound theorem
and exceptional/count-alias checks are included. Strict SDK production export
and certificate verification pass with attestation
`a1ad957b0b532fb058a4c9059492f50279c77ada5ac13c0c69f5ce40b6be4394`.
The repeat-offset Rust source and all imported Verdant libraries are unchanged.
No generated Rust was edited or reformatted.

All ten unchanged upstream factory cases pass, including the 1,048,576-element
case. Two additional layouts verify the CPU's flat writes through padded
higher-dimensional descriptors against actual CPU execution and all physical
padding bytes. An empty-output case preserves the CPU no-op behavior even for
invalid parameters. Nineteen parameter/type/stride/count refusal probes verify
failure before writes. Exact counters require thirteen positive cases and all
nineteen refusal probes. The Pointwise certificate is checked once per test
process and shared by arithmetic and ARANGE tests.

Binding ceil to floor fails the imported 3.0/3.5/1.0 comparison in 3.42 seconds.
Changing the extracted upstream source is rejected before execution in 3.47
seconds; selecting no required cases is rejected in 3.95 seconds. All changes
were restored byte for byte. The restored scoped test passes in 59.77 seconds.
The full contract run passes 36 tests and fails only the unchanged VT-02
production-source guard in 170.81 seconds, with zero ignored or filtered tests.
All-target/all-feature workspace Clippy, dependencies, formatting and four BDD
meta-tests pass. These timings are observations, not controlled benchmarks.

The source budget remains unsatisfied: 10,952 lines is 403 above the last
accepted 10,549. Full repository acceptance, production dependency closure and
remaining imported operations are still open. The complete gate was attempted twice. The first exhausted disk in the SDK
browser-adapter build. After removing obsolete task-owned caches, the retry
with debug symbols disabled passes full Prism application verification and
formatting, then correctly refuses source growth. Assertions and gate scope
remain enabled. Separate SDK template and lock checks pass. No budget was
raised or acceptance claimed. Exact identities, signed authoring inputs,
coverage, faults and log hashes are in `verification/arange-runtime.json`.

### Shared typed execution protocol

Prism now defines source bindings, literal storage, scalar widths and stateful
stepping for all 24 existing pointwise modes. Its instructions carry typed
Verdant operations, source/scratch/immediate/output operands and typed write
targets. The native adapter binds these instructions to existing Verdant
primitives and admitted memory regions. It no longer chooses a CopyMode route,
remaps numerical opcodes or packs transform parameters. This refactors existing
capabilities; no additional imported operation is claimed.

The exact SDK production export passes kernel and preservation checking with
result `3ffce9b4cdb8ed34598cf35736cbeb4ecec4d5c6d9dbd7c1e2bfee62214ad172`
and attestation
`967a22967ba4ebd29ccd8cbd2e1cbd418996f56e46013bc082b2528059fabefb`.
Generated Rust is untouched by manual editing and formatting. Repeat-offset
Rust and all 24 imported Verdant generated libraries remain byte-identical.
All 18,816 scoped transform comparisons pass against imported GGML CPU kernels
in 6.18 seconds. Replacing state feedback with zero causes the first unchanged
upstream ARANGE case to fail in 2.77 seconds. The fault is restored byte for byte.

Clippy across every workspace target and feature, formatting, model audits and
dependency checks pass. The generated-source license clarification is refreshed
using the hash reported by cargo-deny. Source accounting falls by 34 lines to
10,918, still 369 above the last accepted 10,549; the original budget remains
unsatisfied. The full contract passes 36 tests and fails only the existing
VT-02 production-source guard in 143.83 seconds, with zero ignored or filtered
tests. The full exact-SDK `just vv` also completes Prism verification,
formatting, model/frame audits, template/SDK lock checks and workspace Clippy.
It then passes the same 36 contract tests and fails only VT-02 in 138.65
seconds, with zero ignored or filtered tests. The full gate exits 101 before
its dependency stage; the separate dependency check passes. Full repository
acceptance remains open. No proof-event reuse or budget waiver was used.

All three remote authoring attempts failed: two lost their worker and the
third exhausted disk on the verifier service. No remote authoring attestation
is claimed. The successful exact-SDK production attestation already checks all
652 declarations, including all 24 universal unified-mode route theorems.
Its build manifest is imported byte for byte into
`verification/unified-execution-build-manifest.json`; its digest matches the
attestation, and all eight current model source hashes match its inputs. This
uses the completed kernel check without repeating it. Current identities and
log hashes are recorded in `verification/unified-execution-runtime.json`.

### VT-38: L2 normalization through shared typed instructions

VT-38 begins with its registered ID, scenario and a real failing imported test:
the first unchanged upstream L2_NORM case reports not supported in 2.75 seconds.
The unchanged factory and test_case::eval select both L2_NORM and L2_NORM_BATCH,
with mandatory CPU reference mode. All 31 ordinary and 30 batched cases are
required, including views, noncontiguous rows and a 65,536-row shape. Missing,
unsupported or failed required cases refuse the run.

A header audit finds a constructor missing from the factory corpus:
ggml_l2_norm_inplace at ggml/include/ggml.h:1245. Its real test first fails in
12.26 seconds after the upstream cases pass. The model and oracle are expanded
before closing this milestone. Every required public constructor is part of
the completion audit; a green factory corpus alone is insufficient.

Prism models F32, matching shapes, nonnegative epsilon including negative zero
and positive infinity, and four-axis addressing. Output may be separate or an
exact matching in-place view; partial overlap and mismatched alias strides
are refused. Each input is squared in F32, widened and added in F64, then the
sum narrows before F32 square root, epsilon maximum, reciprocal and scaling.
Existing Verdant primitives execute every numerical step. No native production
adapter or imported Verdant library changes are needed. Empty outputs retain
the upstream no-op behavior even with invalid parameters.

The final exact-SDK export passes with result
`9b732f59b323c76a1971f273f933de8303966a85a0de58d2d5f385bab038bc4a`
and attestation
`e640e8f181efafd71d11b4e42b966fd67a6f1e2152f2ec021ad3d848a7acf3a1`.
All 672 declarations pass kernel checking. The imported build manifest binds
all nine current model source files. Repeat-offset Rust and all 24 imported
Verdant libraries are byte-identical. Generated Rust is not manually edited
or formatted. The remote authoring service exhausts its own disk and produces
no attestation; the independent pinned-SDK model and preservation proof succeeds.

All 61 required factory cases pass. Another 64 cases exercise both actual CPU
constructors across contiguous and padded sources/in-place views and padded
copied outputs. Four source fill classes cover zero, finite values, overflowing
squares and NaNs; epsilon covers +0, -0, 1e-6 and +infinity. Four empty copied
and in-place cases preserve all bytes. The comparison follows actual reference
strides and preserves padding; NaNs compare by class. Twenty-eight refusal
probes check epsilon, source/output types, missing source, shape mismatch,
unsafe aliases, negative extents and overflowing strides before any output
write. Exact totals require 129 positive cases and all 28 refusals. The final
scoped test passes in 11.17 seconds.

Binding square root to Verdant squaring fails the first imported case with
ERR 8898.370948965 above 0.000000100 in 3.08 seconds during the earlier copied
stage. The native source is restored byte for byte. An initial edit assertion
failed before applying a defect; its mistakenly started unchanged test was
stopped and is not defect evidence. Restoring the old contiguous-reference
assumption fails the final padded in-place comparison at byte 10, actual 192
versus expected 0, in 11.60 seconds. That fault is restored byte for byte before
the final workspace run.

Development dependency arithmetic and the pointwise kernel use optimization
level 2. Native effects and tests retain their development profile. Compiler
invocations show optimization with debug assertions, and overflow checks keep
their development defaults. No proof, oracle case, assertion or provenance
check is removed. The final workspace contract run passes 37 tests and fails
only the existing VT-02 production-source guard in 17.31 seconds, with zero
ignored or filtered tests. Five model unit tests and four BDD meta-tests pass.
The earlier unoptimized copied-only L2 test took 265.82 seconds; the first
optimized profile build took 75 seconds. These observations differ in scope
and are not a controlled speedup benchmark. Final all-target/all-feature
Clippy, formatting, template/SDK lock and dependency checks pass.

The complete pinned-SDK just vv passes full Prism application verification
and formatting, then the source audit rejects 54 lines of oracle-test growth
against ec9f9ef, 10,918 to 10,972. No proof-event reuse, reduced gate or budget
waiver is used. The original accepted 10,549-line budget is still exceeded by
423 lines. Production dependency closure and all remaining imported operations
and API variants remain open. Current source-bound evidence and historical
iteration measurements are in verification/l2-runtime.json and the imported
build manifests beside it.

### VT-39: relative-position addition and a shared constructor oracle

The pinned header exposes copied and in-place ADD_REL_POS constructors, but the
unchanged upstream factory contains no cases for either. The registered ID,
scenario and actual first-case failure precede implementation. The initial
unsupported result takes 49.11 seconds including bridge compilation; the final
calibrated pre-implementation run fails as unsupported in 6.26 seconds.

A shared bridge delegates constructed graphs to the unchanged test_case::eval
and actual CPU backend, with reference mode mandatory. It uses the upstream
tensor-creation methods, including their per-tensor sentinels, and retains
inherited initialization and determinism checks. Its successful-case counter
requires exactly one non-skipped record. Finite error tolerance is tightened
to zero. Two calibration cases run existing DUP and in-place ADD; an incorrect
operator callback must be refused. Additional constructor fixtures change the
small driver while the imported evaluator translation unit remains cached.
These are local fixtures using an imported oracle, not factory cases claimed
to exist upstream.

Prism models the initial raw copy, the CPU's signed 32-bit head-count conversion,
then both bias-cache reads before each group of alternating height/width writes.
This preserves F32 update order and data dependence when bias views alias the
in-place main tensor. Extra outer batches are copied or preserved without bias
updates, matching the CPU implementation. All arithmetic uses existing Verdant
primitives; the native production adapter is unchanged.

The exact SDK verifies all 697 declarations and exports result
`eed248d0abcb004062e5bab44495a4ffc56dc754df711efe9adb2b924493874f`
with attestation
`86046ea875f85de0c3e7d9b5be321bc0d5a7910ecfbf2b65eb20b34d0c539f1f`.
Its imported build manifest matches all ten current model files. The changed
Nat module also passes remote kernel verification with attestation
`68c79f60389f9cb1ba53117be1e48ff0a03fac18887294bf0e3247fbde4989a6`.
That remote proof covers RelativePositionAdd only; its signed inputs include
the same ten draft-file hashes. The full model and generated Rust preservation
proof is the separate pinned-SDK export. Repeat-offset Rust and all 24 imported
Verdant generated libraries remain byte-identical. No generated Rust is edited
or formatted manually.

The finite corpus has 1,280 constructed cases covering width 1/2/3/8, both
query axes, multiple heads and outer batches, both constructors, five bias-alias
arrangements and legal noncanonical unit strides. Another 25 actual CPU
comparisons cover cancellation order, negative zero, infinities, NaNs and signed
in-place flags. Modified-prefix NaNs compare by class; untouched outer batches
remain exact bytes, including NaN payloads. Ten empty cases cover each constructor
shape parameter without operand buffers. Exact totals require 1,315 positives,
42 atomic descriptor refusals and two rejected NaN-tail payload corruptions.
The final scoped run passes in 6.25 seconds.

Binding required DUP calibration to NEG fails the callback gate in 5.14 seconds.
Binding both cached biases to register zero fails the first ADD_REL_POS case
with ERR 0.802741709 above zero in 2.87 seconds. Both source changes are restored
byte for byte before final verification. The native runner matches its Git
baseline. The complete workspace contract run passes 38 tests and fails only
the existing VT-02 development-source refusal in 16.66 seconds, with zero ignored
or filtered tests. Five model unit tests and all four BDD meta-tests pass.
All-target/all-feature Clippy, formatting and dependencies pass. Timings are
observations, not controlled benchmarks.

The complete pinned-SDK just vv passes full Prism application verification
with attestation fc3c75dc3a277b068db8182333edaf698a47a1e9f1f6e15dc359e0ddd4eca942
and formatting, then rejects the 112-line source increase against 755dc29.
It exits 1 before the later stages. Separate current-source workspace, Clippy,
and dependency checks are recorded above; separate template and canonical SDK
lock checks also pass.
No proof-event reuse or reduced gate is used. Current source is 11,084 lines,
535 above the original accepted 10,549, and no ceiling is raised. Production dependency closure and all remaining imported operation/API
coverage remain open. Detailed source-bound evidence is recorded in
verification/relative-position-add-runtime.json and the signed authoring and
imported build manifests beside it.

## Native oracle cache invalidation

The native executable key previously included every Rust token in the native
oracle module. A semantically equivalent change to expected-source snapshot
allocation rebuilt the C++ driver, despite unchanged build inputs. The revised
key retains the compiler/linker recipe tokens, complete emitted C++ and ABI
assertions, source/compiler/SDK cache identity and dynamically loaded library
path. It includes imported evaluator adapter bytes only for drivers that link
the adapter. Source snapshot, upstream source, object and executable checks
still run at their existing boundaries.

Before the change, replacing `Vec::new()` with `Vec::with_capacity(0)` in the
host snapshot orchestration created a new executable; the scoped native test
took 5.10 seconds. After the change, the same edit preserved every executable
path and nanosecond modification time; the test passed in 2.39 seconds.
Reordering linker flags created exactly one new driver and passed in 5.12
seconds. Inserting a harmless static assertion into the imported evaluator
adapter created exactly one new upstream driver and passed ADD_REL_POS in
35.22 seconds, including adapter compilation. The same adapter edit preserved
the non-importing native driver. These are local observations rather than
controlled benchmarks.

Appending one zero byte to the warm native executable failed its digest check
before execution in 2.40 seconds. The executable and its original timestamps
were restored, and every temporary source mutation was restored byte for byte.
The numerical Prism model, native production runner and pinned evaluator
source remain unchanged. Detailed mutations and final gate results are in
`verification/native-oracle-cache.json`.

The complete pinned-SDK `just vv` freshly verifies the Prism application with
attestation `fc3c75dc3a277b068db8182333edaf698a47a1e9f1f6e15dc359e0ddd4eca942`.
Formatting, all 45 model IDs, error and deferral checks, bootstrap/frame
integrity, template/SDK locks and all-target/all-feature Clippy pass. All five
model unit tests and four BDD meta-tests pass. The contract group passes 38
tests and fails only VT-02 in 15.88 seconds, with no ignored or filtered tests.
Both generated fixture batches and observed GGML bindings still match every
frozen matrix/arithmetic record. The dependency audit passes separately; the
complete gate exits 101 at the development-source refusal before its deny
stage. This is not a release pass.

The source audit measures 11,083 lines in 51 files, one line smaller than the
parent. The original accepted 10,549-line budget remains exceeded by 534; no
ceiling is raised. Fixture generation migration, remaining imported operation
and API coverage, and production dependency closure remain open.

## Frozen native fixture recipes (VT-40)

The construction corpus is frozen before replacing its native recipe builders
with Prism. It contains 70,392 cases across sixteen existing families: 69,592
successful CPU-reference cases, 800 constructed refusals and 1,001 descriptor
mutation probes. These include the original empty and IEEE cases. The immutable
manifest records each case hash and complete transcript hashes; expected
numerical outputs continue to come from the pinned GGML CPU implementation.

Transcripts include creation-ordered tensor descriptors, source and view
identities, parameters, normalized buffer offsets, initialized physical bytes,
refusal mutations and each positive/refusal outcome. A separate immutable
supplement covers 1,920 copied-FILL execution states after releasing the unused
source pointer and buffer. The original construction manifest remains unchanged.

The baseline test initially fails because its manifest is absent. Adding one
to the input-byte pattern passes all 160 REPEAT CPU comparisons but fails the
construction hash in 2.82 seconds. Seeding a valid stale transcript and redirecting
the producer to another file fails the fresh-output requirement in 3.03 seconds.
Altering a frozen case hash fails manifest integrity in 0.03 seconds. Omitting
FILL source release passes all 15,360 CPU comparisons and the original construction
hash, then fails the supplemental execution-state hash in 3.61 seconds. Every
temporary source and corpus mutation is restored before final checks.

Each existing oracle test emits its transcript during its normal graph execution;
the additional VT-40 test checks immutable manifest structure without executing
the cases again. Manifests are cached once per process. Snapshot paths include
the process ID, prior output is cleared before invocation, and successful
temporary files are removed. Per-case hashes preserve migration diagnostics
without committing the 249 MB of raw construction traces. A pinned-SDK experiment
also compiles the existing certified arithmetic fixture root behind a thin C ABI
adapter and passes six C++ queries. It provides a tested path for the upcoming
recipe migration while keeping production backend APIs unchanged.

Complete pinned-SDK `just vv` freshly verifies the Prism application with
attestation `fc3c75dc3a277b068db8182333edaf698a47a1e9f1f6e15dc359e0ddd4eca942`.
Formatting, all 46 model IDs, typed-error/deferral checks and bootstrap checks pass.
The source audit rejects 134-line growth against 3a08947: 11,083 to 11,217.
No proof-event reuse or reduced acceptance boundary is used. The remaining
template, SDK-lock, all-target/all-feature Clippy and dependency checks pass
separately. The final workspace run passes five model unit tests, four BDD tests
and 39 contract tests; VT-02 alone rejects the development dependency. The forty
contract tests take 19.72 seconds, with no ignored or filtered tests. Timings
include cache effects and are not controlled benchmarks.

The original 10,549-line budget is still exceeded by 668. The instrumentation
enables preservation during migration; the native recipes are not yet Prism
generated. Source-budget recovery, complete imported operation/type/API coverage
and production dependency closure remain open. Detailed evidence is in
`verification/native-fixture-freeze-runtime.json` and its two frozen manifests.

## Prism COPY fixture recipes (VT-41)

Prism now supplies COPY case enumeration, source and target dimensions and
strides, byte-span and flatten counts, initial byte patterns and destination
chunk plans. The native harness binds those values to public GGML constructors
and literal byte-copy effects. COPY has no native recipe fallback. All 3,456
cases across the 36 live storage types match their frozen construction, alias,
input and refusal transcripts exactly. Other fixture families retain their
existing builders and match their original transcripts after the shared loop
refactor. The numerical reference remains unchanged imported GGML CPU code.

The existing arithmetic fixture root routes the new request range to
CopyFixtures, retaining its original requests and refusal boundary. CopyFixtures
imports the existing type catalog and byte-span model. Its test-only C ABI
adapter preserves Some, None and checked overflow separately, with no reserved
payload value. Both Rust compilations keep debug and overflow checks enabled.
The source/certificate, bridge, compiler recipe and compiler identity bind the
cached model library. Its digest is checked before loading. No production
backend exports or numerical execution code change.

The model exposes 144 reusable 256-byte patterns. Metadata names each pattern;
byte generation factors through that same identity. Two universal theorems
establish this cache contract. A planted per-case byte change for ordinal 1
leaves the earlier ground examples intact but fails bytesFollowThePattern with
LLV7002 in the isolated SDK verifier. This protects against model changes that
runtime memoization could otherwise hide. The isolated negative source is
restored afterward; the repository source is never mutated by that probe.

The final exact SDK export verifies all 338 declarations. Its result is
`98e1ecd719f829e332d767745ff3b1f755ec08f0dcdeb0abfc9e358888e3bc5e`,
with attestation
`a2e35406de8292863405bed12ac5672fd43c8da50f7799993792e41309c806a1`.
The final export uses an isolated project with identical model sources and
lock; every installed artifact byte is compared to the SDK output. Current
repository source and artifact checks pass in the final workspace run. Remote
MCP authoring checks pass, but both remote verification attempts fail on gateway
disk exhaustion. They supply no proof attestation; the full SDK proof is the
verification evidence.

XORing byte-query answers in the adapter passes all COPY CPU comparisons and the
RESULT count, then fails the frozen input hash in 2.14 seconds. Corrupting the
warm model library fails its digest check before oracle execution in 2.02
seconds. Both mutations are restored byte for byte, including library timestamps.
The adapter edit and the actual model-only cache-law refinement preserve every
C++ executable path and nanosecond modification time. The model library changes
without rebuilding the imported CPU oracle driver.

The final workspace passes five model unit tests, four BDD tests and 40 contract
tests; VT-02 alone rejects the development dependency. All 41 contract tests run
without ignored or filtered cases in 19.38 seconds. The original matrix,
arithmetic, 70,392-case native construction and 1,920-state FILL manifests remain
byte-identical. Final all-target/all-feature Clippy, template/SDK lock and
dependency checks pass. The license clarification binds the newly certified
arithmetic output hash 0x12f6dc30.

The complete pinned-SDK just vv passes the unchanged default application proof,
formatting, all 47 model IDs and ownership/error/deferral checks, then rejects
128-line native-source growth against cdd52c0. After the fixture-only cache-law
refinement, its SDK export and every affected workspace, ownership, lint and
dependency check are rerun. The default application proof inputs and native
code remain unchanged. No complete acceptance or release pass is claimed.
Current source is 11,345 lines, 796 above the original 10,549-line budget; no
ceiling is raised. Further recipe migration, all remaining imported requirements
and production dependency closure remain open. Detailed evidence is recorded
in `verification/copy-fixtures-runtime.json`.

## Prism REPEAT and CONCAT fixture recipes (VT-42, VT-43)

Prism now generates all 160 REPEAT and 10,368 CONCAT fixture recipes. The shared
test-only bridge binds their operand, unused target and output descriptors,
initial byte patterns and copy chunks to public GGML constructors. The unused
target remains represented because its allocation is part of the frozen
construction transcript. Both modes require the certified model library and
have no native recipe fallback. Numerical results still come from unchanged
imported GGML CPU code; production execution and backend exports do not change.

Both capabilities were registered with scenarios and failing tests before
implementation: the previous artifact returned None instead of Some(160) or
Some(10368). The final pinned-SDK arithmetic export verifies all 410 declarations.
Its result is
`a67b8f5a4b7a64ad854bd8037c6a6753365fde449fc4f625ae1e85584330c03d`,
with attestation
`66cf7647bfd201d25e963e5e0fbf07eeae81922ad4c1443892ed5f132f9bf6cb`.
Both canonical MCP authoring checks pass. Their remote proof tasks fail with
LLB6003 staging storage exhaustion and supply no proof evidence. The separate
SDK production proof supplies the attestation.

REPEAT reuses COPY's catalog projection and byte patterns. CONCAT models both
operands with distinct pattern identities, all four concatenation axes, packed
storage types, padded outputs and empty dimensions. Universal model theorems
bind cached patterns to metadata and byte generation. The model library remains
bound to its certificate, adapter and compiler identity and is checked before
loading. Assertions and overflow checks remain enabled.

Two planted adapter defects distinguish recipe preservation from numerical
agreement. Selecting REPEAT's contiguous target descriptor for its padded output
passes all 160 CPU comparisons, then fails the frozen transcript length:
563,768 instead of 583,928 bytes, in 2.45 seconds. Selecting CONCAT's left byte
pattern for its right operand passes all 10,368 CPU comparisons, then fails the
frozen input hash in 3.44 seconds. Both edits are restored byte for byte. Every
native driver path and nanosecond modification time remains unchanged across
these faults; the bridge recompiles without rebuilding the imported CPU driver.

The workspace run passes five model unit tests, four BDD tests and 42 contract
tests; VT-02 alone rejects the development dependency. All 43 contract tests run
without ignored or filtered cases in 24.48 seconds. These are local observations
with cache effects, not controlled benchmarks. The original 70,392-case native,
1,920-state FILL, matrix and arithmetic manifests remain byte-identical.
All-target/all-feature Clippy and dependency checks pass; license clarification
binds the new certified arithmetic output hash 0x210ef6da.

The remaining template and SDK-lock checks pass separately. Their initial
isolated model regeneration runs out of local disk space; removing an obsolete
ignored task-owned Prism build cache allows the retry to pass. The completed
proof, generated certified artifacts and native driver cache are preserved.

The complete pinned-SDK `just vv` passes the full default application proof,
formatting, all 49 model IDs and ownership/error/deferral checks, then rejects
36-line handwritten-source growth against 718d063169a7: 11,345 to 11,381.
The full gate therefore fails. The original accepted 10,549-line budget remains
exceeded by 832 lines; committing intermediate work does not establish a new
accepted ceiling. No proof-event reuse or acceptance waiver is used.

A read-only upstream reference check finds Verdant Engine 2.15.0 at
4e1e7bbe8b0554a2aac0681663c899a67b8b956c. Its linear module is unchanged from
the prior inspected revision. No dependency pin or development checkout is
changed, and this upstream observation establishes no additional vTensor
coverage. Remaining recipe migrations, complete imported operation/type/API
coverage, source-budget recovery and production dependency closure remain open.
Detailed evidence is recorded in
`verification/repeat-concat-fixtures-runtime.json`.

## Shared Prism FILL and CAST fixture recipes (VT-44)

The typed fixture model generates all 15,360 FILL and 1,920 CAST cases through
the existing arithmetic fixture root and shared test-only C ABI bridge. It
defines case order, storage types, copied/in-place variants, empty dimensions,
source/unused-target/output layouts, fill constants and physical input bytes.
FILL uses eight reusable 256-byte patterns; CAST uses three 192-byte patterns
that retain the original 24-word cycles for F32, F16 and BF16. Universal
metadata/pattern and byte/pattern theorems justify caching. Chunk offsets and
lengths remain model supplied. Neither mode retains a native recipe fallback.

VT-44's registered scenario and test precede implementation; the previous
artifact returns None where Some(15360) is required. Canonical MCP checks pass.
Remote proof task-44 fails with LLB6003 while staging
LexLeanPreservation.Values.json because the gateway has no disk space. That
failed task supplies no proof evidence. The separate pinned-SDK production
export verifies all 461 declarations, with result
`b9ef9c6477f466e0eef04b3bc1a58fbb5aa6bb0ca9b221725d9cc34b6b594b2d`
and attestation
`f29b6c7da5457b10a1ab8a59bced4aa3e941271a4b1674f1b42688020378563a`.
All installed artifact bytes are SDK output; generated Rust is not reformatted.

The affected tests pass: model queries in 1.47 seconds, all FILL CPU comparisons
and frozen records in 6.41 seconds, and all CAST CPU comparisons and frozen
records in 2.24 seconds. The FILL test also preserves all 1,920 late source
buffer-release states. These local timings include compilation/cache effects
and do not establish a controlled performance result. Expected numerical
results remain computed by unchanged imported GGML CPU code.

XORing one bit into FILL's constant metadata binding passes all 15,360 CPU
comparisons and the RESULT count, then fails the frozen construction/input hash
in 3.10 seconds. XORing one bit into CAST's byte binding passes all 1,920 CPU
comparisons and the RESULT count, then fails its frozen input hash in 2.80
seconds. Both mutations are restored byte for byte before the final gate. All
41 native driver paths and nanosecond modification times remain unchanged;
neither fault rebuilds the imported CPU driver. Existing arithmetic and native
fixture query assertions now share one helper with their original requests and
expected answers retained.

Production numerical execution is unchanged by this recipe migration. A separate
audit of imported RMS_NORM identifies the next numerical prerequisite: its F64
sum must be divided by the row count before narrowing to F32. The current
Verdant wide interface has addition, widening, narrowing and count-to-F32 but
no F64 division primitive. Narrowing the sum first changes the required rounding.
No RMS_NORM implementation or conformance pass is claimed.

An extracted, unchanged RMS_NORM accumulation/mean expression and its imported
ggml_float typedef reproduce the rounding distinction in the pinned SDK. For
the F32 row [4096, 1, 0, 0, 0], the F64 squared sum is 16,777,217. Dividing before
narrowing gives F32 3,355,443.5 (0x4a4cccce); narrowing first gives 3,355,443.25
(0x4a4ccccd). The source hashes, exact extracted snippet and observed values are
recorded in `verification/normalization-prerequisite.json`. This is a prerequisite
regression observation, not full operation conformance evidence.

The complete pinned-SDK `just vv` proves the default application again, with
attestation fc3c75dc3a277b068db8182333edaf698a47a1e9f1f6e15dc359e0ddd4eca942,
then passes formatting, all 50 model IDs, ownership/error/deferral checks, frame
sync, the template/SDK locks and all-target/all-feature Clippy. Its source audit
counts 11,380 lines in 51 files, down one line from the parent. The original
10,549-line budget is still exceeded by 831; the branch's parent comparison
does not establish an accepted increase.

The workspace stage passes five model unit tests, four BDD tests and 43 contract
tests, with VT-02 alone rejecting the Verdant development override. All 44
contract tests run without ignored or filtered cases in 25.55 seconds. The gate
therefore fails, and no release acceptance is claimed. Cargo stops at that
contract failure; remaining test binaries and checks are run separately to
avoid repeating the completed numerical oracle groups.

Those remaining checks pass: all five frame tests, all 62 xtask unit tests, all
available doctest targets and the dependency advisory/ban/license/source checks.
No tests are ignored or filtered in those runs. The new license clarification
binds certified arithmetic output hash 0x0bed3672. The original native, FILL
source-release, matrix and arithmetic baselines remain byte-identical. Detailed
evidence is in `verification/typed-fixtures-runtime.json`. Complete imported
coverage, remaining recipe migrations, original source-budget recovery and
production dependency closure remain open.

### VT-45: certified normalization arithmetic prerequisite

`fe2fd46` registered the capability, scenario and failing API test before its
implementation. Its half-division query originally returned `None`. The new
development bundle pins Verdant commit
`158402b84cf273dc303c16c70e3781809a568a37` and includes certified F64 division
and unsigned-count-to-F64 conversion. Existing operations zero through three
retain their semantics; operations four and five supply the new primitives,
and unsupported operations remain refused. Arithmetic is entirely in Prism
roots and the existing certified rounding/packing functions, sequenced through
Verdant's typed API. The regression uses that API for its count conversions too.

The unchanged ScalarF64 and FloatWord64 models are imported by F64Quotient.
Its exact locked SDK export contains six roots and 443 successful declarations:
result `e448fc0b0f4909f681fba7f436b41ed5428f397d013c0cbc65a2ad8fa154ebd9`,
attestation `ec57d3e381d25197fa7bc8ca0bdf34b6f9478ec90f8367dfe933df2fff78842a`.
The public operation dispatcher is defined in the model with interface
refinement equalities. Limits were not increased; the regression combines
separately checked quotient and narrowing equalities under observed `propext`.
Remote structural checking supplied no verification attestation; the actual
proof evidence comes from the complete pinned-SDK production export.

Imported Berkeley SoftFloat at
`07ebd29db0cfb55723a79fbbffe99103ef38e305` independently computes 1,277,924
divisions, 172,246 count conversions and 212,992 composed means: 1,663,162
comparisons. Non-NaN values match exact bits, and NaNs match by class. The
existing archive/source/compiler/SDK cache checks remain active. A planted
native binding supplied the numerator mantissa in place of the denominator
mantissa; division case 11 rejects it, actual `3cb0000000000001` versus expected
`3ca0000000000001`. Restored code passes all 46 tensor tests. The final interface
proof refinement leaves all six generated Rust files byte-identical, so it
requires certificate replay rather than another numerical corpus run.

The actual Verdant source lift and uorust projection were regenerated. Frontier
counts remain 390 lifted, 1,167 residue and 1,324 uncertified; no ceiling was
raised. Source-capsule and real Cargo registry-consumer tests include every new
package. The Engine full gate encountered shared-disk exhaustion in registry
validation; its isolated retry and every remaining stage pass, covering all
227 workspace tests and full application attestation
`a83ef1d53cae059f70e59ab7978050c69fef04f1f7436eee24cdccea422843bc`.
The interrupted full invocation is preserved as a failure, not relabeled green.
Task-owned SDK build caches were temporarily moved to RAM to finish compilation;
vTensor's cache was restored to its normal workspace path afterward. Obsolete
oracle executables/libraries were removed while retaining current outputs and
dependency caches. SDKs, policies, proof events and limits were unchanged.

vTensor reuses its existing current-model/artifact validator for the imported
six-root bundle, including package versions and complete certificate file
bindings. Native build provenance now captures Verdant's quotient lock and
binding source. VT-45 passes in 2.14 seconds and preserves the imported GGML mean
`4a4cccce`; prematurely narrowing the accumulator would produce `4a4ccccd`.
This is an arithmetic prerequisite, not a complete RMS_NORM operation pass.

The exact vTensor `just vv` passed the full application proof
`fc3c75dc3a277b068db8182333edaf698a47a1e9f1f6e15dc359e0ddd4eca942` and formatting,
then rejected initial source growth from 11,380 to 11,400 lines. Sharing frozen
JSON loading and certificate identity/path checks recovers all 20 lines while
retaining every frozen digest, descriptor and input transcript. The restored
51-ID model, error/ownership checks and 52-file frame sync pass at 11,380 lines.
Appending a newline to the frozen native baseline still fails its digest gate
in 0.03 seconds; exact restoration passes in 0.05 seconds. No numerical suite
was repeated for this metadata fault.

Only the unreached gate stages were then run. Template/SDK checks, all-target
Clippy, dependency checks, model tests, BDD, frame tests, 62 xtask tests, every
generated library target and all documentation targets pass. The complete
contract run has 44 passes and one failure in 19.45 seconds: VT-02 intentionally
refuses the development override as production release evidence. No release
pass is claimed. The original 10,549-line budget still needs 831 lines recovered;
meeting a later parent comparison does not raise that accepted ceiling.
Full RMS_NORM graph support and authoritative production dependency releases
remain open. Machine-readable evidence is
`verification/normalization-arithmetic-runtime.json`.


RMS_NORM (VT-46) now uses Prism to plan F32/BF16 normalization through Verdant's
certified scalar and F64 arithmetic. The graph preserves the imported CPU's
square/accumulate/divide/narrow order, caches every row factor before writing,
and checks non-nested output layouts with an occupancy preflight. Existing
operations retain their three stack scratch words without an additional Prism
request or heap allocation. Copied and exact in-place RMS paths share the model.

The production pointwise export verifies 746 declarations with attestation
`6372d917085faa07f6a3233610d78d7a31c8a2ae8535a26f444b1e77cfbadcb0`.
The arithmetic fixture export verifies 490 declarations with attestation
`02c63ca3596dab75e2e769fbc51a7938556f59e6e390ee83aad8847d7cb396c9`.
All 121 imported RMS factory cases and 256 Prism supplemental cases pass;
the latter include 168 atomic refusals and exact output/padding comparisons
with NaN classification. A wrong runtime flag accepts invalid input and is
rejected by the failure-status assertion. A planted early output write fails
the unchanged-output assertion. Both defects were restored, and the targeted
suite passes. The initial supplemental model omitted the shared chunk-copy
query; adding that query in Prism fixed the protocol without a native fallback.

The workspace contract suite reports 46 passes and the existing VT-02 failure
in 24.19 seconds. All-target Clippy, frame tests, 62 xtask tests, generated
library targets, documentation, dependency/license and template checks pass.
The full `just vv` invocation passed the application proof
`fc3c75dc3a277b068db8182333edaf698a47a1e9f1f6e15dc359e0ddd4eca942`
and formatting, then failed the source audit. It reports 11,402 lines, growth
of 17 from its immediate parent bfe5333 (11,385 lines), and 853 above the
original 10,549-line ceiling. The pre-commit comparison against 70d1bc5
reported 22 lines because it also included the registered failing test. Neither limit nor acceptance policy was relaxed.
Machine-readable details are in `verification/rms-normalization-authoring.json`.

Verdant upstream M24 at `55488e808bf88bd5ab1d62093f9417281d7b1f35` was
reviewed without changing the user's upstream working tree. It does not yet
provide the imported certified scalar/wide/storage interfaces required here;
its newer source ratchets need an isolated migration. The audit is recorded in
`verification/verdant-m24-review.json`. Full RMS acceptance, original source
budget recovery, production dependency releases and the remaining imported
operations are still open.

After the full gate exited, the task-owned SDK cache was restored to its normal
workspace path. Only superseded private oracle binaries/libraries were removed;
current outputs and dependency caches were retained.


VT-47 moves signed spatial fixture pad, reflect and roll arguments, including
the circular CPU-reference adjustment, from C++ into SpatialFixtures. The
existing typed fixture bridge loads the separate certified package. All other
generated bundles remain unchanged. The production export has 291 successful
declarations, result
`a674595a87fe122269442af5a4302d1a563bf553b2efd9d2705a31d7663878a6`,
and attestation
`502ac31b4f7a29457eaeea39827b4c7dba824346061470330f8768bb0c28b008`.

Two initial production attempts failed preservation with deep recursion. A
separate package alone did not resolve the failure. Inspecting the generated
preservation source identified padWord as the failing relation. Commuting the
large constant to the left of its addition resolved this case without changing
kernel/SDK limits, axioms, fixture theorem statements or generated code. The
first installation then exposed a missing writer package-name mapping. After
fixing it, the actual successful artifact was replayed and installed directly;
its proof was not rebuilt. Certificate replay validates the exported bindings;
the original production build supplies Lean and leanchecker proof evidence.

The unchanged imported CPU evaluator and frozen transcript checker preserve all
2,256 spatial cases, including construction and input bytes. Asking for axis
zero on every roll axis changes the frozen digest from
`a44c02245774e077b1f02a0217d0fba7da1fd065676c8e10abc5f1f24b770dfe` to
`e44ae834f64e88d94946a0a53a8070784d304bf42d6d753c99d8f3019542eeda`;
the gate rejects it in 4.23 seconds. A preceding compile error in the transport
was fixed and is not counted as mutation evidence. Replacing the spatial lock
attestation with zeroes is rejected in 0.45 seconds; exact restoration passes
in 1.00 second. Both faults are restored.

Before the final infrastructure consolidation, the complete workspace contract
run reports 47 passes and the existing VT-02 production-dependency failure in
26.97 seconds. After consolidation, all-target/all-feature Clippy, dependency
checks, seven fixture-generation tests, the imported storage and transfer
oracle tests, five frame tests, frame sync, 62 xtask tests and the new generated
library/documentation targets pass. Certificate checks share one cache keyed
by canonical repository root and model; certificate failure poisons it and
fails closed. Process-output checking is shared, and fixed-width oracle parsing
no longer allocates a temporary Vec per row. Existing arithmetic fixtures retain
their own certificate and cached bridge instance.

The source audit still refuses growth: 11,407 lines versus 11,402 in checked
parent 487d69c, 858 above the original 10,549-line ceiling. The registered failing
test checkpoint was 11,424 lines; shrinking from that checkpoint does not recover
the original budget. No ceiling or release gate was relaxed. Another full
`just vv` is left for the next acceptance batch because the default application
model is unchanged; full acceptance remains unproven, and the previous failed
full gate is retained. The temporary SDK cache is restored to its workspace
path. Details are in `verification/spatial-parameters.json`.

The next normalization source audit finds that NORM's mean and variance
rounding change with inner strides, and its contiguous variance uses native
SIMD grouping. The current imported oracle compiler enables SSE2 without AVX
or FMA flags. GROUP_NORM separately preserves F64 per-row subtotals. These
findings, source hashes and implementation prerequisites are recorded in
`verification/normalization-next-audit.json`; neither operation is claimed
implemented by this audit.

### VT-48: stride-selected NORM through certified normalization roots

NORM now uses a Prism metadata planner and instruction generator with Verdant
arithmetic. The imported source is GGML revision
`9a080f44b503c64f66c84a6274b9d9ee5d5b09d5`. The model preserves F64 summation,
F32 narrowing before mean division, the SSE2 four-square grouping before F64
variance accumulation, F64 variance division before narrowing, and the separate
sequential F32 path selected by nonunit input or output inner stride. The CPU
reference retains both strides, because canonicalizing its input would select
the wrong algorithm. Contiguous helper counts above their imported `int` bound
are refused; the scalar counter remains wide. Boundary proofs cover INT_MAX,
the next count, and the independent scalar domain.

All 61 imported factory cases and 184 Prism supplemental cases pass. The latter
include 129 successes and 55 atomic refusals across copied/in-place operation,
padded, strided, permuted and non-nested layouts, cancellation, grouped rounding,
nonfinite values, signed zero, tiny epsilon, empty axes, storage refusal and
output overlap. Comparisons require exact non-NaN words, NaN classification,
exact padding and unchanged bytes after refusals. The expected arithmetic comes
from the independently imported CPU backend, never from local fixture answers.

The initial monolithic and intermediate package designs exceeded the unchanged
4,194,304-byte certificate limit. Bit packing also increased certificate size.
The accepted design separates the NORM planner and NORM execution roots and
moves L2's unchanged instruction stream into that same normalization bundle.
Native code transports the certified ADTs and operands; it adds no numerical
implementation. Only NORM planning makes a second Prism call; instruction
execution makes one call per step, and existing modes retain their allocation
behavior. Removing unused raw query constructors does not remove any operation
from the generic stateful interface.

The actual SDK production exports are:

- Pointwise: result `a7d527fd057b5a48ed3c2e909da16dbdf8c00b0c2881cc10aa9e0450ae186591`, attestation `60b21e1fe3c0e75cbb6dad86743df9f99aa29f7891c02d549040824b60f54b28`, 749 verified declarations. Largest certificate B: 4,118,208 bytes.
- Normalization: result `f77c100def3d5e50591f41020cbe05abd0d461d3498eddbf4c94f1e88a078253`, attestation `fb52f0449d7a468a5787db2309973e9f6d4403a7e1be2ed013745a875e87efe3`, 411 verified declarations across three production roots.
- Supplemental fixtures: result `5f8eca73249754ddcbe9a329f9c198f87fc7fe82c5c5f3463151c234e47273e7`, attestation `313dbf2d3a5534a2e363d01de1985e941314ee43db2c4f5e30d08f5d3f7eead5`, 328 verified declarations.

The full integration batch caught an unintended source replacement in
`keepElement`: lower-triangle selector 2 had become 64. Its imported triangular
oracle failed at width 3, row count 3, byte 16. The correction restores selector
2 in Prism and adds `lowerTriangleIncludesBelowDiagonal` before exact SDK
regeneration. This regression was not waived or hidden by a filtered suite.

Three intentional defects establish the new gates. Passing zero instead of
occupancy readback into NORM fails the imported graph-refusal assertion in
4.90 seconds. Routing scratch target 4 into 5 corrupts grouped variance and
fails the CPU comparison at width 5, byte 0 in 4.60 seconds. Zeroing the lock's
attestation identity fails certificate validation before numerical execution in
2.30 seconds. The native source and lock were restored byte for byte afterward.
The restored full contract batch passes 49 of 50 tests in 44.74 seconds, including
all L2, RMS and triangular cases; VT-02 alone rejects the development dependency
override. Formatting, workspace/all-target/all-feature Clippy, cargo-deny, five
frame tests, 62 xtask tests, template/SDK-lock checks and frame synchronization
pass. No production dependency or complete vTensor acceptance is claimed.

Verdant M25 (v2.17.0), revision
`0d8ed4ed314cd09ff925deda6dd69d15082f263d`, was reviewed without changing the
user's upstream working tree. Its tensor changes add gateway predicates and
still omit `evaluate_wide`, `evaluate_f32_scalar`, `F64Quotient` and `StorageBlock`.
It cannot alone resolve VT-02. See `verification/verdant-m25-review.json` and
`verification/norm-authoring.json`. GROUP_NORM and the remaining imported
requirements, original source-budget recovery and production packaging remain
open.

The complete exact-SDK `just vv` attempt completed its actual application proof
and formatting checks, then exited 1 at the unchanged source ratchet. The kept
application attestation is
`fc3c75dc3a277b068db8182333edaf698a47a1e9f1f6e15dc359e0ddd4eca942`.
There are 11,532 non-Prism implementation lines versus 11,407 in parent
`fdc3dbf543da`: growth of 125, and 983 above the original 10,549-line ceiling.
Committing an implementation checkpoint does not recover that original ceiling.
All 54 model IDs and the error/deferral/bootstrap checks passed before this
refusal. Remaining gate components were checked separately as described above;
this is not a successful full acceptance run. No proof-event reuse, source
ceiling increase, certificate-limit increase or generated-code edit was used.

### VT-49: GROUP_NORM with nested F64 row subtotals

Registration `15edb59` added the ID, scenario and actual imported failing test
before implementation. The first ordinary GROUP_NORM case was unsupported.
The unchanged imported factory supplies three ordinary cases, including the
65,537-plane regression, and two whole-graph GROUP_NORM_MUL_ADD cases. All five
now pass. The 224 Prism supplemental cases also pass: 112 successes and 112
atomic refusals. They cover both public constructors, odd and excess group
counts, short and empty tail groups, padded/permuted/non-nested layouts,
readonly overlaps, nonfinite and negative epsilon, empty dimensions and
physical output column spacing. The fixture's +/-2^60 and unit counterexample
separates row-subtotal accumulation from a flattened F64 fold.

The planner and instruction stream extend the independent normalization bundle.
Separate F64 row and group accumulators preserve the CPU's subtotal order for
both mean and variance. Division occurs in F64 before F32 narrowing. The model
retains F32 centering/squaring and the F32 epsilon/sqrt/reciprocal/scale tail,
elides groups that perform no stores, bounds signed channel/group/denominator
arithmetic, and preflights all physical output words before writing. It does
not inherit NORM's epsilon assertion: GROUP_NORM's imported CPU has none.
Six scratch words are reused, with occupancy storage only where needed.

The certified copy result now carries its selected epsilon explicitly. Native
code forwards the operation and raw parameters and transports that result; it
adds no parameter-selection or numerical algorithm. Existing normalized modes
and L2 keep one instruction query per step. NORM, RMS and L2 remain covered by
the restored full contract batch.

Actual production exports from the unchanged pinned SDK are:

- Pointwise: result `ebdeece9bf545a05e6ce78f88f72ebb885d0c35b0a82b4d4614b83e2925e3606`, attestation `cc7962237c362b055d3da5d74ec0bd3f4f0aa0b251a44df24971beae5b12c363`, 750 verified declarations; largest certificate B is 4,123,314 bytes, below the unchanged 4,194,304-byte limit.
- Normalization: result `9c521081bcfc3f8f2e2c9fe29c2e594c74f0ef242d23eb0ddff878abd8a19de1`, attestation `f5c6e4751c75f4f4a85e65353a15a465d0c479de5843f6dc7fa8cafbb4c6c038`, 492 verified declarations. Its NORM/GROUP_NORM, L2 and planner certificate B sizes are 767,517, 194,497 and 532,406 bytes.
- Group fixtures: result `310614802ccf398d120a11c56edba4f7cd2ef69e9d03dcf9b89aa800cd7ce2d3`, attestation `c8c01d11bad3043223a32629ea4978e45bdd9f3704e7ef79f13c40b203dfcb82`, 382 verified declarations.

The first supplemental invocation exposed a missing fixture-library argument;
this was corrected in the shared host dispatch. The numerical comparison then
exposed a more important oracle-harness assumption. GROUP_NORM indexes output
columns as float pointers even when the descriptor says nb[0]=8. The old
comparison copied only logical payload words into a padding seed and therefore
misclassified a real CPU write at byte 4 as padding. The corrected comparison
seeds the CPU output with the same initial bytes, reads its complete physical
result, and adjusts only the descriptor view used to recognize NaNs at actual
F32 payload locations. Every other byte, including padding, is compared exactly.
The expected values still come solely from the imported CPU implementation;
production arithmetic did not change to accommodate the harness.

The planted row-reset defect exists only in MCP draft
`prism-app/cc4c5db133edb09b3c815068d63235e925ebad5ddccc818a0c980f453173029b`.
It resets the group accumulator instead of the row accumulator. Verification
`task-55` fails with LLV7002 at the fresh-row proofs for instruction indices 1
and 9. The correct repository model was never replaced by this draft.

Three transport/provenance faults also fail the intended gates. Moving physical
write offset 4 to 8 for metadata nb[0]=8 fails the CPU comparison at byte 4
(actual 165, expected 0) in 4.87 seconds. Bypassing occupancy readback fails the
expected refusal assertion in 4.42 seconds. Zeroing the fixture lock attestation
fails certificate validation before numerical comparisons in 3.84 seconds.
Native source and the fixture lock were restored byte for byte afterward.

The restored workspace contract batch passes 51 of 52 tests in 86.48 seconds;
VT-02 alone continues to reject the local development dependency override.
Formatting, all-target/all-feature workspace Clippy, cargo-deny, five frame
tests, 62 xtask tests, template/SDK-lock checks and 52-file frame synchronization
pass. This does not establish full vTensor acceptance. Machine-readable evidence
is in `verification/group-normalization-authoring.json`.

The next backward-normalization audit records ten imported RMS_NORM_BACK cases,
two source roles, F32 narrowing before mean division and coefficient division,
and physical output spacing. These are source observations, not implementation
claims: see `verification/normalization-backward-audit.json`. The remaining
imported requirements, original source-budget recovery and authoritative
production dependency packaging remain open.

The complete exact-SDK `just vv` attempt passes its actual application proof and
formatting, then exits 1 at the unchanged source ratchet. The kept application
attestation remains
`fc3c75dc3a277b068db8182333edaf698a47a1e9f1f6e15dc359e0ddd4eca942`.
The ratchet measures 11,564 lines against 11,532 in parent `9cf2df10c921`:
32 additional lines, comprising five in the native transport and 27 in test
transport/comparison. This remains 1,015 above the original 10,549-line ceiling.
All 55 IDs and the error, deferral and bootstrap checks pass before that refusal.
No ceiling or proof policy is relaxed; production dependency resolution and
full acceptance remain open.

A further numerical fault flattens only the first group's mean: it retains the
row accumulator between rows and copies that subtotal into the group sum.
The +/-2^60/unit fixture rejects this at output byte 4 (actual 171, expected 0)
in 4.10 seconds. This independently demonstrates the row-subtotal rounding
counterexample against the real imported CPU, in addition to the Lean reset
proofs. Native source was restored byte for byte to the full passing-component
baseline and its normal library rebuilt; no numerical change remained.

The SDK cache was restored to its ordinary workspace path with hard links
preserved. To retain disk space without losing prior evidence, obsolete
SDK export caches were archived under the task's temporary directory; current
exports and committed evidence are intact. Private compiled and historical
oracle caches remain in task RAM through their existing paths.

### VT-50: two-source RMS_NORM_BACK and its narrowing order

Registration `b26a61a` added the ID, scenario and actual imported failing test
before implementation. Its first F32 RMS_NORM_BACK factory case was
unsupported. All ten factory cases now pass, together with 353 Prism
supplemental cases: 238 successes and 115 atomic refusals. Twelve additional
mutations check required sources, source/output types, both inner strides,
all source-1 dimensions, missing source data and shifted output aliasing.
The supplemental corpus includes a 65,537-batch execution and four memory
configurations: copied output, output aliasing the gradient, output aliasing
the original input, and both sources/output sharing an effective view.
The backward public constructor remains copied; no backward in-place
constructor is invented by these buffer-view tests.

A separate backward instruction root reuses the existing six-word normalized
copy transport and Verdant primitives. Source-1 metadata is captured in planning
and its strides once during execution binding, rather than validating its
descriptor per instruction. The model requires matching F32 source/output
shapes, unit inner strides for both inputs, physical output bounds and safe
aliases against both inputs. Output injectivity is checked globally before any
write when the layout needs occupancy storage. It retains the CPU's int64
column domain rather than inheriting forward normalization's int helper bound.
There is no epsilon-admission or numeric-scale assertion in the imported CPU
backward implementation, and none is added here.

F32 products x*x and x*dz are widened into separate sequential F64 sums. The
squared sum is narrowed before F32 mean division; the gradient dot product is
narrowed before F32 coefficient division. Mean/epsilon, sum/epsilon and the
compound multiply/add/multiply output phases retain their separate rounding.
Physical output columns are spaced by sizeof(float), as in the imported CPU.
No handwritten numerical implementation or generated Rust edit is added.

The coefficient uses existing Verdant F32 negation after narrowing. Its imported
ScalarF64 narrowing model keeps sign separate from unsigned rounded magnitude,
and ScalarF32 negation toggles that sign. The general sign-composition law is
proved in Prism for sign <= 1 and magnitude < 2^31. MCP `task-57` verified it
under attestation
`9b214ce275b58b461f82820684a04b84542df83f2a14c060eb6d5f2b4860bc0a`.
This sign law, the pinned imported model-byte audit and the independent full
CPU execution establish the chosen composition; the sign law alone is not
restated as a proof of the entire imported floating-point implementation.

Actual exact-SDK production exports are:

- Pointwise: result `418cfd05fff116155d67325063473ad1aee672c2f397fe77e1bfe2ac79d74cff`, attestation `5f32a023e9d5fe81acaf9fe1ce418c6111d4167fb1f1649b262860adad9f4711`, 752 verified declarations; largest certificate B is 4,130,620 bytes, below the unchanged 4,194,304-byte limit.
- Normalization: result `688919e092c115620602401552ea51a8f0a24a5a6c2edcf8b39963ca2e786452`, attestation `73a992d5bc6c5dfe328bda96d25ddd95ddca28a661c91122b8b844715ee22a9c`, 643 verified declarations across NORM/GROUP_NORM, L2, metadata planning and backward execution. Certificate B sizes are 767,517, 194,497, 624,067 and 334,103 bytes. Existing NORM/GROUP_NORM and L2 generated instruction files are byte-identical to the parent.
- Backward fixtures: result `7028e9d97d20660a438127f470a310d64a3ffb56a4866ec79d7987bf80e7043e`, attestation `045011c8bc9b4fd856d6c18a5091e7b75eed356e640809d332fe3059d5165563`, 391 verified declarations.

Planted transport defects demonstrate the independent oracle gates. Rebinding
the original input to gradient source 0 fails at output byte 0 (actual 0,
expected 189) in 4.65 seconds. Delaying mean narrowing until after F64 division
while preserving the coefficient denominator changes that byte from 189 to 188
and fails in 4.61 seconds. Moving physical output offset 4 to 8 for metadata
nb[0]=8 fails at byte 4 (actual 165, expected 188) in 4.81 seconds. Bypassing
occupancy readback fails the expected refusal assertion in 4.83 seconds.
Zeroing the fixture lock attestation fails before numerical execution in 4.09
seconds. Each native/lock mutation was restored byte for byte afterward.

The distinct model defect in MCP draft
`prism-app/cdcf4211bcdead4ff67cb9b8c4ab32c9e6037ec61b4ea843bffe7e12e926bb1d`
changes coefficient division from F32 to F64. Verification `task-58` fails with
LLV7002 at instruction 42. The correct repository model was never replaced by
this mutant.

The restored workspace contract batch passes 53 of 54 tests in 92.70 seconds;
VT-02 alone rejects the development dependency override. Formatting,
all-target/all-feature workspace Clippy, cargo-deny, five frame tests, 62 xtask
tests, template/SDK-lock checks and 52-file frame synchronization pass. Clippy
required expressing three consecutive mode alternatives as an equivalent
inclusive range; the ordinary native library was rebuilt afterward.
Machine-readable details are in `verification/backward-normalization-authoring.json`.

The completion audit still distinguishes ordinary normalization from separately
labeled composed graphs. NORM_SCALE, RMS_NORM_SCALE, NORM_MUL_ADD,
RMS_NORM_MUL_ADD, ADD_RMS_NORM and normalization/RoPE combinations require their
own authoritative factory coverage, including mixed-storage operands and graph
aliases. No pass is inferred from component behavior. Source observations and
follow-up are in `verification/composed-normalization-audit.json`. The original
source budget, authoritative production dependency packaging and remaining
imported requirements remain open.

The complete exact-SDK `just vv` attempt passes its actual application proof and
formatting, then exits 1 at the unchanged source ratchet. It keeps application
attestation
`fc3c75dc3a277b068db8182333edaf698a47a1e9f1f6e15dc359e0ddd4eca942`.
All 56 IDs and the error, deferral and bootstrap checks pass before that refusal.
Non-Prism implementation measures 11,628 lines against 11,564 in parent
`0f105352d5cb`: 32 added native transport lines and 32 test transport lines,
64 total. The implementation remains 1,079 above the original 10,549-line
ceiling. A newer checkpoint parent cannot erase that debt. No limit, proof
policy or dependency rejection was weakened, and full acceptance remains open.

The SDK cache is restored to its workspace path with hard links preserved.
Three non-current cached SDK oracle executables are preserved separately in
task RAM; the active executable and common library cache are retained. Private
compiled and historical oracle caches remain accessible through their existing
task paths. This changes no committed proof, generated artifact or oracle source.

The one-time metadata inventory of the unchanged pinned `make_test_cases_eval`
constructs 27,755 graphs without allocating tensor data or evaluating arithmetic.
The inventory records every virtual operation label and count, plus the full
constructor parameters and factory ordinals for the 187 non-RoPE composed
normalization cases. There are 114 RMS_NORM_MUL_ADD, 28 NORM_MUL_ADD,
25 ADD_RMS_NORM and ten each NORM_SCALE and RMS_NORM_SCALE cases. Of these,
118 use F32 and 69 use BF16. Another 281 graphs combine RMS normalization and
RoPE. These counts establish construction coverage, not numerical conformance.

VT-51 registers its scenario and genuine failing test before implementation.
The exact-SDK command `cargo test --locked -p vtensor-backend --test contract
composed_normalization_vt_51 -- --exact --nocapture` fails in 7.53 seconds at
the BF16/F32-weight RMS_NORM_MUL_ADD case with shape [127,3,2,1], reporting
`not supported`. The current scalar arithmetic planner admits F32 only.
The source audit identifies the seven authoritative scalar storage tuples,
per-node F32 conversion/computation/encoding, broadcast geometry and distinct
contiguous/noncontiguous counter bounds. The next implementation remains
entirely Prism instruction generation and imported Verdant arithmetic.

The source audit rejected committing the diagnostic inventory driver as a new
non-Prism file. That one-time driver remains in task RAM; its digest and the
observed transcript digest remain recorded as diagnostic evidence. It is
removed from shipped source. The normalization tests now share certificate
setup, preserving all eight existing/new invocations and their case counts.
The exact-SDK `normalization_vt_` batch passes RMS_NORM, GROUP_NORM and
RMS_NORM_BACK; VT-51 alone retains the intended mixed-storage failure
(three pass, one fails, 41.05 seconds). Model validation passes all 57 IDs,
error/deferral/bootstrap checks and the parent source ratchet: 11,627 lines,
one below the prior implementation checkpoint. The original ceiling remains
10,549, leaving 1,078 lines to recover. No parent movement erases that debt.
Formatting passes. The unchanged full application proof was not repeated for
this red-test checkpoint; this is not a successful full acceptance run.

VT-51 now has a separate certified `TypedArithmetic.evaluate` root. Its 569
declarations and preservation certificate are verified in the exact pinned
SDK, with attestation
`7006f6825c8797226f3696673354fa143738628cf90956b9fbc070695979e140`.
The largest typed certificate is 530,577 bytes. Pointwise delegates all four
scalar binary operations, including F32, to this shared planner; its 753
declarations are verified with attestation
`df2e281f5825b6c3363a594f8b9cf2c5f84a247879a2f09f27eb0329e0e6a540`.
Its largest certificate is 4,140,076 bytes, below the unchanged 4,194,304 limit.
The remote MCP proof service failed twice with LLB6003 and an external
no-space diagnostic; these local SDK results are actual successful kernel
and preservation proofs, not inferred remote attestations.

The audit found that the legacy F32 path also needed the imported stride,
alias and signed-counter guards. The new Prism plan covers all seven scalar
type tuples, broadcasts, empty tensors, natural word alignment, permitted
aliases and output occupancy before any stores. It retains source/output
metadata once per graph operation. Its schedule emits one instruction per
F32 element, four for equal F16/BF16 storage, three for mixed small/F32
storage outputting small storage, or two for mixed input outputting F32.
All conversion and numerical operations remain imported Verdant calls.
The native adapter retains this plan and shares instruction transport with
the existing normalization roots.

The 120 frozen F32 arithmetic/unary cases pass in 4.75 seconds, and all 353
backward-normalization edge cases pass in 6.99 seconds. Redirecting BF16
encoding to F32 in the native transport fails the imported CPU comparison
in 10.28 seconds: ADD error 3.190350070 exceeds 0.000100000 on the BF16
[127,3,2,1] graph with F32 weights. Zeroing the typed lock attestation fails
before numerical execution in 0.59 seconds. Both files were restored byte
for byte. A separate temporary model encoding defect is rejected with
LLV7002 at phases 2 and 5; the correct repository model was never replaced.
The rejected project was removed from the repository after the check.

The initial composed run was deliberately stopped when its mixed-only
dispatch was superseded by the unified planner and minimal instruction
schedule. It establishes no pass. At the initial implementation checkpoint,
the corrected comparison and exact-word supplemental coverage were pending.
Full acceptance remained unproven. Formatting, workspace
Clippy and all-feature cargo-deny pass; source-bound license clarifications
were refreshed without editing generated code. Source accounting measures
11,708 non-Prism lines, 1,159 above the original ceiling and 81 above the
prior implementation checkpoint. The source ratchet still fails. Production
dependency packaging, original source-budget recovery, typed supplemental
coverage and the remaining imported requirements remained open.

The completed 187-case composed normalization comparison passes in 744.81
seconds. It executes the unchanged authoritative factory and its independent
CPU backend, including all 69 BF16 cases. The fixture transport work below
does not change the verified production planner or instruction packages, and
the factory run was not repeated for changes confined to supplemental test
transport and evidence.

The separate `TypedBinaryFixtures.evaluate` export verifies 340 declarations
and its preservation proof in the exact SDK, with attestation
`2f3d3e14ea7c307caa4180d44f0ee84ea8cd5e69aadf66cff00b65b3d6eecbb3`.
Its largest certificate is 732,417 bytes. All 977 generated cases pass:
864 ordinary layout/alias cases, 112 empty cases and one nonempty BF16/F32
case with shape [3,1,1,65537]. There are 737 positive cases and 240 atomic
refusals, with 12 additional malformed metadata probes. Every scalar storage
tuple and ADD/SUB/MUL/DIV are covered. The CPU executes actual source and
output layouts; non-NaN output words and padding bytes match exactly, while
both-NaN outputs are checked by classification. The raw patterns include
signed zero, subnormal/normal boundaries, maximum finite values, infinities,
quiet/signaling NaNs and rounding ties. Copied, left/right output views and
shared-input/output cases exercise padding, broadcast, readonly rows, outer
permutations and nonnested output occupancy.

The actual 2,491,136-byte constructor/input/refusal transcript hashes to
`77e15e44eee53cddf59c33756cd290a6c5c5801312e7fff9935efdc604648e7d`.
The source-bound baseline records construction and input bytes; every
numerical expected result still comes from the imported CPU. Bypassing typed
occupancy feedback fails the expected runtime refusal in 3.99 seconds.
Rebinding the right operand to source 0 fails an exact F32 comparison at byte
0 (actual 0, expected 1) in 4.28 seconds. A zeroed fixture attestation fails
before numerical execution in 3.39 seconds. Replacing every delivered input
byte with zero lets the numerical comparisons complete, then fails the
frozen transcript gate in 7.21 seconds. Its changed digest is
`a1ae4712da04c7c4a18e6e5dd159620955074e72042b00ea80a5a3409a41b4c1`.
Each mutation was restored byte for byte. The restored full corpus, including
the transcript comparison, passes in 4.70 seconds. The shared constructor
storage adapter also preserves all 4,992 frozen unary cases and their CPU
results (0.57 seconds).

Workspace Clippy, cargo-deny, five frame tests, template/SDK-lock checks and
52-file frame synchronization pass. The complete exact-SDK `just vv` attempt
passes its actual application proof and formatting, then fails the unchanged
source ratchet after all 57 IDs, error, deferral and bootstrap checks pass.
It keeps application attestation
`fc3c75dc3a277b068db8182333edaf698a47a1e9f1f6e15dc359e0ddd4eca942`.
Non-Prism implementation measures 11,738 lines, 1,189
above the original 10,549 ceiling; the source ratchet remains a genuine
failure. Production dependency packaging and the remaining imported
requirements, including RMS/RoPE compositions, remain open.

## Verdant PR dependency

Cargo resolves `verdant-tensor` and its 30 certified packages from Engine commit
`4654da8724f142a248b32739f1d930066e281bd5`. VT-02 and VT-03 passed in the
locked SDK. Substituting an all-zero revision in the patch record made the
same VT-02 executable fail with `wrong Verdant source`. VT-45 verified the
quotient artifact in Cargo's resolved checkout. VT-51 passed all 977 typed
runtime cases against the imported GGML oracle. The dependency audit passed.
The source audit measured 11,737 non-Prism lines. Full provider acceptance and
Verdant release publication remain open. [Engine PR #12](https://github.com/afflom/verdant-engine/pull/12)

## Scheduled graph selection

VT-52 drives the pinned GGML scheduler and an independent CPU backend through
the same three executions of a SCALE → ADD graph. Clearing the intermediate
node's compute flag at `b5fb916` failed `observed==expected[pass]`. With the
certified vTensor predicate the output pairs match the CPU on all three runs:
(6, 9), (−17, −13), (10, 15). The middle pass preserves the caller's −17.
Both nodes were assigned to the requested backend; each scheduler reused its
original allocation. The native bundle attestation is
`5eab556cdc2a68ef18f34af949854a9eceb75a747803f03eef5260f324def0f1`.
VT-11, VT-40, VT-51 and VT-52 passed in one executable. VT-40 previously
failed its frozen 16-mode count because the VT-51 aggregate had been appended
as a seventeenth per-case mode. Each baseline now keeps its original schema
and is parsed once on demand. The frozen inputs and checksums are unchanged.

## llama.cpp consumer

Built the unmodified upstream `llama-eval-callback` at `9a080f44` with
`GGML_BACKEND_DL=ON` and `GGML_NATIVE=OFF`. The upstream example pins
`stories15M-q4_0.gguf` to SHA-256
`66967fbece6dbe97886593fdbb73589584927e29119ec31f08090732d1861739`.
Use the same executable, model and arguments for both runs. For the reference
run use the build's bin directory. For vTensor, copy the executable into an
empty directory, run it there, set `LD_LIBRARY_PATH` to the build's bin
directory and `GGML_BACKEND_PATH` to the absolute vTensor shared library path.

```sh
./llama-eval-callback -m "$model" --prompt hello --seed 42 -ngl 0 -t 1 --verbose
```

The reference CPU decoded the prompt and produced 32,000 logits. The vTensor
run recorded `layer 0` through `layer 6 assigned to device vTensor` and stopped
during model loading with `failed to find a compatible buffer type for tensor
output.weight`. This GGUF stores the output weight as Q8_0 with dimensions
288 × 32,000. llama.cpp probes MUL_MAT with a 288 × 512 F32 right operand.
Per-node placement, prefill logits, and KV-cache reuse on vTensor remain open.
[Upstream weight probe](https://github.com/ggml-org/llama.cpp/blob/9a080f44b503c64f66c84a6274b9d9ee5d5b09d5/src/llama-model-loader.cpp#L1048)
· [Upstream example and model hash](https://github.com/ggml-org/llama.cpp/blob/9a080f44b503c64f66c84a6274b9d9ee5d5b09d5/examples/eval-callback/CMakeLists.txt)

At `f4c5923`, upstream factory ordinal 10,681 reproduced the same matrix
admission gap independently of model loading. Running the exact parameter
filter from the [case transcript](verification/ggml-factory-cases.tsv.gz) via
`test_case::eval` returned `not supported` and the oracle exited 2. It uses
one F32 activation column, 16 Q8_0 weight rows and inner dimension 256.

## Verdant Q8 dependency

Verdant PR #13 builds on #12 and provides `encode_symmetric_q8_block`.
Cargo resolves Verdant Tensor and all 31 numerical packages from Engine
commit `32cbf30db6d3e0f708aebb7d2adb55f38c9d381a`. The imported
quantizer supplied 9,214 expected bytes for 271 blocks. A corrupted lane
failed the external comparison. Q8_0 block dot products and graph admission
remain open. VT-02, VT-03 and cargo-deny passed for this pin. The rebuilt
plugin also reproduced the same llama.cpp layer placement and output-weight
refusal. Upstream fidelity still reports 1,168 native residue functions
against 1,160 and 1,326 uncertified functions against 1,318.
[Engine PR #13](https://github.com/afflom/verdant-engine/pull/13)

The complete vTensor `just vv` through `e3d5cf1` passed in the locked SDK on
the PR #12 pin, including all 57 backend contract tests. Its source audit
measured 11,784 non-Prism lines against the same immediate-parent total.
Recovering the original project ceiling of 10,549 remains on the burndown.
The subsequent PR #13 pin has the targeted dependency and host results above.

## Quantized matrix execution

VT-53 first failed at factory ordinal 10,681 in the registered test at
`75e3b69`. After implementation, all 122 Q8_0 × F32 MUL_MAT cases passed
the unchanged upstream evaluator against its independent CPU reference. The
test binds the ordinal, label and complete constructor parameters of every
passing case to SHA-256
`98ccc36df39f889744f83d3afec15c00e033031c5220e564565b54f30332eddd`.
These cases are a subset of the 3,213 MUL_MAT cases.
[Case inventory](verification/ggml-factory-inventory.json)

Verdant commit `17f04e0c21f814a49ff906bb640a747ca4cdda7c` provides the
activation encoder and the complete signed row product. Its VTK-17 test checks
256 rows against the pinned CPU type trait and rejects five malformed shapes.
vTensor's certified matrix root checks graph geometry and computes byte offsets.
The ABI adapter prepares each activation column once, calls the Verdant row
product for each weight row and stages output until all input reads finish.
Matrix attestation `b35189b57a8026bbe832c61a86acd63346d2da898e24ee985c387623073ddabf`.
[Verdant numerical evidence](https://github.com/afflom/verdant-engine/blob/17f04e0c21f814a49ff906bb640a747ca4cdda7c/ENGINE-VERIFICATION.md#vtk-17-symmetric-row-contraction)
· [Matrix certificate](generated/matrix/evidence/attestation.json)

Forcing vTensor to read row zero for every weight row made the VT-53 gate
fail on its first case with normalized mean square error 2.570212241 against
the upstream limit 0.0005. Restoring row indexing passed all three factory
instances sharing that exact parameter string. VT-02, VT-03, the 1,280 VT-12
F32 matrix fixtures, workspace Clippy and cargo-deny also passed.

With the same pinned model and command as the consumer probe above, llama.cpp
accepted `output.weight` and first refused `blk.0.attn_q.weight` (Q4_0,
288 × 288). The imported evaluator independently refused factory ordinal
10,637, a Q4_0 × F32 matrix with 16 output rows, one column and inner
dimension 256. vTensor prefill and decode logits remain unverified.

The VT-53 source audit measured 11,897 non-Prism lines, 113 above the
11,784 comparison baseline and 1,348 above the original 10,549 ceiling.

## Packed matrix execution and KV-cache placement

VT-54 first refused Q4_0 × F32 factory ordinal 10,637 at registration
`41a9b85`. vTensor's matrix plan reads the block geometry from the existing
GGML storage catalog and selects the Verdant contraction. Both weight formats
use the same buffer handling, broadcast schedule and activation preparation.
Verdant's four-bit path interprets the packed lanes and executes the two
rounded products and addition in the order specified by the upstream CPU.
[Matrix attestation](generated/matrix/evidence/attestation.json)
· [Verdant PR #15 and numerical evidence](https://github.com/afflom/verdant-engine/blob/97528328fe73591dc1528fa96d2dcad5291966bc/ENGINE-VERIFICATION.md#vtk-18-packed-four-bit-row-contraction)

On the PR #15 pin the upstream evaluator passed all 122 Q4_0 × F32 and
all 122 Q8_0 × F32 matrix cases. The new group's passing transcript SHA-256
is `dfa8e8d874e02e063285441939a67fcbc4a050a100a503706457ecd0390c578a`.
Together these account for 244 of 3,213 MUL_MAT cases. The issued matrix
attestation is `484ce89dffdcfcd6ba6b3485e6477ab55af06600ed5b8172447bd7855c1b5565`.

Swapping the Verdant entry points at the vTensor dispatch boundary made the
unchanged evaluator fail its first Q4_0 case. Restoring dispatch passed both
factory instances matching those exact parameters. VT-02 and VT-03,
workspace Clippy and cargo-deny passed on the PR #15 pin.

The default llama.cpp memory-fitting probe reached the preallocated
`cache_k_l0 (view)` and refused SET_ROWS. Repeating the same command with
`--fit off` exercised real allocation (`no_alloc = 0`), loaded a 17.50 MiB
vTensor model buffer and allocated 1.69 MiB for the six-layer F16 KV cache.
The scheduler reached the same SET_ROWS refusal before the first logits.
The standalone imported evaluator also refused factory ordinal 1,528, an
F32 source, F16 destination and I64 indices. This case uses already available
Verdant storage conversion. The missing behavior belongs to vTensor's
indexed GGML write plan.
[Upstream SET_ROWS contract](https://github.com/ggml-org/llama.cpp/blob/9a080f44b503c64f66c84a6274b9d9ee5d5b09d5/ggml/include/ggml.h#L1558)

The locked SDK verified the VT-53 application attestation
`fc3c75dc3a277b068db8182333edaf698a47a1e9f1f6e15dc359e0ddd4eca942`.
The retained proof and formatting passed when resumed in a separate Cargo
target directory. The release gate stopped at the source audit. The finalized
VT-53 commit measured 11,897 against its parent’s 11,793 lines. VT-54
measured 11,919 against the VT-53 baseline, 1,370 above the original 10,549.

## Scalar indexed row writes

VT-55 registered the scenario in `6873697`. The imported GGML factory
first refused ordinal 1,125. The typed Prism root now validates the
destination view, row layout, both index broadcast axes and every signed
index before issuing a store. It caches each index once. The backend passes
each scalar to Verdant's existing `convert_float_storage`. The destination
is the original GGML buffer, including rows untouched by the scatter.

`scalar_set_rows_vt_55` passed all 126 upstream F32/F16-source cases with
F32, F16 or BF16 destinations and signed I32/I64 indices. The 20 F32 → F16
cases with I64 indices include ordinal 1,528. The passing ordinal and full
parameter transcript hashes to
`cd0fe815fc5476210b0b483f1b6c6ea0f2fe1b0be16fe8eb16125b76f3af5595`.
A late I32 index of −1 and a late I64 index of 2³² each failed compute and
preserved the entire initialized destination. Forcing stores to offset zero
failed the imported comparison on ordinal 1,125 (NMSE 0.331520658 with limit
0.000000100). Restoring the adapter passed VT-55 and the 977 prior VT-51
typed cases. The typed root attestation is
`0a151edbfc1b7cdbea0a84b324a1fa33548237566338a8bf7223dfa6ebb35aea`.

The same isolated stories15M invocation from VT-54 now reaches the scheduler
assignment assertion at `ggml-backend.cpp:1711` during graph reservation.
A separate read-only observation queried upstream `ggml_backend_supports_op`
for each compute-flagged node immediately before forwarding that graph to the
scheduler. Of 195 nodes, 192 carried the flag and vTensor admitted 168,
including all 12 F16 cache writes. The 24 refusals were 12 ROPE, six
FLASH_ATTN_EXT and six split SWIGLU. Prompt evaluation stopped before logits.
The observation records graph index, support bit, operation, tensor name,
destination type, source types and output shape. [Reservation graph](verification/llama-stories15m-reserve.tsv)

The imported numerical evaluator independently refused representative factory
ordinals 660 (split F32 SWIGLU), 17,216 (F32 ROPE) and 19,242
(FLASH_ATTN_EXT with F16 K/V). The inventory counts a single verdict per
ordinal. [Factory inventory](verification/ggml-factory-inventory.json)

The fixture harness verifies the complete ordered matrix and arithmetic input
streams against the two frozen transcript digests, then compares the bytes
observed on the native tensors directly to those inputs. This removes a second
record parser, 1,400 repeated per-case hashes, and copies of both input streams.
Clippy, cargo-deny and the model gate passed. The source audit measured 11,918
lines against 11,919 at VT-54. The original 10,549-line ceiling remains on
the release checklist.
Flipping one bit in the generated matrix input failed the frozen transcript
digest. Flipping one bit in the observed elementwise binding independently
failed the byte comparison.

## Verdant exponential dependency

Verdant PR #16 supplies `linear::exponential_f32` for the activations and
attention observed on stories15M. The Engine compares the certified
implementation to unchanged `ggml_vec_exp_f32` from the pinned GGML source.
All 166,856 sampled binary32 inputs met the stated one-ULP or exceptional
class criterion. The final-term mutation failed that comparison. [Engine evidence](https://github.com/afflom/verdant-engine/blob/5b2f0c66e4e51a6e34d5deaf9212738924747912/ENGINE-VERIFICATION.md#vtk-19-binary32-exponential)

A fresh consumer build resolved the patch at
`5b2f0c66e4e51a6e34d5deaf9212738924747912`. VT-02 verified the exact Cargo
source and the 15 kernel descriptors. VT-55 passed its 126 imported factory
cases and both late-index refusal probes on the rebuilt backend. Cargo-deny
passed. The imported SWIGLU evaluator still refused combined F16 ordinal 596.
Graph planning and gated execution remain open in the provider checklist.
[Patch record](model/verdant-development.json) · [Burndown](BURNDOWN.md)

## VT-55 complete checkpoint

The locked SDK passed the complete `just vv` at vTensor `47632c4`, with
all 60 backend contract tests passing in 1,341.58 seconds. This run used
the immutable Verdant PR #15 pin. [Tested revision](https://github.com/afflom/vTensor/commit/47632c4a7e4b014e4a3382d43311e30a64521151)

## SwiGLU rows

VT-56 admits F32, F16 and BF16 GGML SwiGLU layouts. Its certified Prism
plan selects each input row, the half-row offset for packed inputs, and
the argument order. vTensor passes the selected words through Verdant's
storage conversions and `linear::swiglu_f32`. All scalar activation
semantics are defined in Verdant. The upstream scalar implementation
passed 110,700 independent comparisons with GGML. [Provider certificate](generated/typed/result.json) · [Verdant implementation and oracle](https://github.com/afflom/verdant-engine/blob/27c09ba01e661b5d4687dfc0f4e7b291277c6409/ENGINE-VERIFICATION.md#vtk-20-binary32-swiglu)

The original provider refused factory ordinal 596. All 36 cases from the
unchanged upstream evaluator now pass, including 12 split, 12 packed and
12 swapped packed cases. Reversing the Verdant arguments failed the first
packed F16 case with NMSE 1.072415188 against the upstream threshold of
0.000000100. The dependency check verifies the exact PR #17 Cargo source
and all 15 existing kernel descriptors. [Patch pin](model/verdant-development.json) · [Factory ledger](verification/ggml-factory-inventory.json)

The isolated, unmodified stories15M host still stops in graph reservation
at `ggml-backend.cpp:1711`. The scheduler admits 174 of its 192 compute
nodes, including all six SwiGLU and all 12 cache writes. It refuses 12
RoPE and six flash-attention nodes. Prompt execution and logits remain
unverified. [Per-node observations](verification/llama-stories15m-reserve-vt56.tsv)

Matrix and elementwise fixtures now share the native execution loop.
Their individual frozen transcript comparisons and five refusal checks
per family still run. The imported factory source is checked together
with the CPU sources against the pinned archive once per test process.
The source audit measured 11,910 non-Prism lines, eight fewer than VT-55.

Changing one byte in the cached upstream factory preserved the file length
and failed archive verification. The restored nine-test batch passed the
SWIGLU and scalar SET_ROWS groups, typed arithmetic, matrix and elementwise
regressions, their frozen fixtures, dependency provenance and the storage oracle.

### Rotary and attention reservation inputs

At `748b506`, all 12 RoPE nodes requested normal mode, 48 rotated F32
channels, frequency base 10,000, frequency scale 1, and extension factor 0.
All six attention nodes supplied F32 queries and F16 keys, values and mask.
The keys and values used non-contiguous head strides. The host requested
F32 precision with zero bias and softcap. A read-only scheduler observation
recorded each distinct descriptor set and its node indices during the same
stories15M reservation. [Parameter words, shapes and byte strides](verification/llama-stories15m-reserve-vt56-operands.tsv)

The unchanged upstream evaluator also refused ROPE_BACK ordinal 17604 on
this provider. VT-57 registers the 510 forward and 424 backward factory
cases separately. [Failing scenarios](https://github.com/afflom/vTensor/commit/d1f5741)

### Verdant circular-function pin

Cargo resolves `verdant-tensor` to `db211ab9efc25855162be786bc2db6034a0c483a`
from submitted Verdant PR #18. The new `linear::sin_cos_f32` shares one
phase reduction for two results. The pinned Engine passed all 520,568
independent GGML scalar comparisons. A fresh vTensor build passed VT-02,
the 36-case SwiGLU regression and cargo-deny. The rotary GGML graph work
remains open in VT-57. [Upstream numerical evidence](https://github.com/afflom/verdant-engine/blob/db211ab9efc25855162be786bc2db6034a0c483a/ENGINE-VERIFICATION.md#vtk-21-binary32-sine-and-cosine)

### Rotary address plan

Prism certified the admission decision, input/output row offsets, paired channel addresses and multimodal position selection for VT-57. The SDK replayed export `4ac3f3de27fdeca5640eb07367ca41ea0d361e5c6a4d689b74e7cf22e655c761`. The backend built with the exported package and passed dependency checks. The forward and backward GGML factory runs remain open pending numerical dispatch. [Model and certificate](https://github.com/afflom/vTensor/commit/7774ee045cf44352cdb85531ef752af1fde6394e)

### Verdant rotary numerical pin

VT-02 and cargo-deny passed on the exact upstream commit `22140d7209ea09d5e6dd596d1f8f0f345d37dc75`. Verdant's logarithm and reciprocal-root suite compared 24,578 logarithms and 260 ratios with GGML. Every tested degree matched bit for bit at bases 10,000 and 1,000,000. The plane-rotation suite ran 240 configurations in the imported CPU kernel, including both directions and fractional and rounded correction intervals. All 15,360 coefficients met the three-ULP bound. All 15,360 output coordinates matched bit for bit when Verdant used the observed GGML coefficients. Both suites rejected their planted numerical defects. [Engine frequency oracle](https://github.com/afflom/verdant-engine/pull/19) · [Engine rotation oracle](https://github.com/afflom/verdant-engine/pull/20)

The vTensor factory totals continue to count only actual provider executions. VT-57 remains open for GGML graph dispatch and the 510 forward and 424 backward cases.

### VT-56 complete checkpoint

The exact locked SDK passed `just vv` at `748b5066afe57ca86bfd8526a252fe144fe4926f`, including 61 backend contract tests with zero failures, skips or filters. The source audit measured 11,910 native lines and cargo-deny passed. [Validated implementation](https://github.com/afflom/vTensor/commit/748b5066afe57ca86bfd8526a252fe144fe4926f)

### Rotary graph execution

The locked SDK ran the release plugin against all 510 forward and 424 backward rotary graphs from the pinned GGML factory. Both suites passed. Reversing the direction in the adapter failed the first forward F32 graph. Prism supplies the byte addresses and selects the position axis. Verdant computes the frequencies, coefficients, coordinate rotation and storage conversions. Each token shares its coefficients across heads and batches.

The independent stories15M scheduler observation recorded 195 graph nodes and 192 compute nodes. The plugin admitted 186, including all 12 rotary nodes. Six FLASH_ATTN_EXT nodes still prevented graph reservation and prefill. [GGML oracle](vendor/llama.cpp-oracle/upstream.toml) · [Verdant plane rotations](https://github.com/afflom/verdant-engine/pull/20)

## Fixture probes

The certified `FixtureProbes.evaluate` root enumerates the 864 VT-10 allocation
inputs, 1,024 VT-09 transfer inputs and 40 boundary queries for the fixture
models. The test harness submits each allocation and transfer to the pinned
GGML oracle and compares its verdict with the layout model. The fixture
package is a development dependency of the backend. The ten affected fixture,
frozen-baseline and buffer tests passed in the locked SDK's release profile.
VT-02 and all four Cargo deny checks passed. [Prism lock](probes-kernel.lock)

Both VT-57 batches also passed on the release plugin built from
`fb2bada`, covering all 510 forward and 424 backward upstream rotary cases.

## Existing provider factory cases

The VT-58 registration branch exercised the VT-57 plugin against the pinned
factory across the exact label groups in the [run record](verification/factory-checkpoint-vt57.json).
The native driver checked the loaded library's identity and embedded source,
selected the separate GGML reference CPU backend, and invoked the unchanged
upstream evaluator. Each successful batch reported the full number of matching
ordinals in the pinned case transcript. The inventory counts only complete
passing groups and the individual refusals that the evaluator printed.

To replay a group with the upstream test executable, load the same plugin
through `GGML_BACKEND_PATH` and run `test-backend-ops test -b vTensor -o <labels> --output csv`
at the pinned llama.cpp revision. Check the device, case count, support and
comparison verdicts. The run record also identifies the SDK, exact provider
binary, source snapshot and oracle executable.

## Verdant weighted-row dependency

VT-02 resolves the submitted Verdant PR #21 at
`53dfc67d4382c6d9b01eb04ae053ba6a9cd1c862`. All fifteen descriptor
checks and cargo-deny's source, license, advisory and ban checks passed.
The numerical API takes scores and row slices. GGML objects, mask addressing
and scheduling stay in vTensor. All 5,940 weighted output coordinates matched
the imported reference within 0.000001. Reversing the rescaling branch
produced 0 for GGML's −0.75. The Verdant release gate still reports fidelity
failures in the earlier logarithm and rotary stack.
[Upstream evidence](https://github.com/afflom/verdant-engine/pull/21)

## Dense attention and stories15M inference

VT-58 admits dense F32 queries with F16 or F32 key/value storage. Prism checks
descriptors and selects row offsets, including the optional mask and sink.
Verdant converts storage, computes each matrix product, scales and biases the
score, and accumulates the weighted value row. The two Prism roots share one
export. [Prism lock](attention-kernel.lock) · [Verdant numerical oracle](https://github.com/afflom/verdant-engine/pull/21)

The release plugin passed the imported GGML factory at ordinals 19242, 19314 and
19394. Replacing the sink score with negative infinity in the provider made the
two-case test fail on its F16 sink graph. GGML measured 0.256118654 against
its 0.0005 limit. Restoring the real score passed the same test. Run
`cargo test --release --locked -p vtensor-backend --test contract attention_vt_58`
inside the locked SDK. [Factory inventory](verification/ggml-factory-inventory.json)

The pinned llama library decoded the quantized stories15M prompt and two fixed
continuation tokens in separate CPU-only and vTensor-only processes. The graph
reserved 195 nodes in one split. All 96,000 compared logits were finite and all
three top-token IDs matched. The scheduler callback read every written K and V
row across six layers. Four of the 6,912 prefill FP16 words differed by at most
0.00048828125. All 6,912 decode words matched bit for bit. The separate unmodified
`llama-eval-callback` executable reached its CPU registry threadpool lookup and
exited with SIGSEGV. Both threadpool entry points must be supplied for that
consumer. [Inputs, placement, hashes and per-step results](verification/llama-stories15m-vt58.json)

The F32 stories15M model from the pinned upstream quantization test completed
the same three steps. All top-token IDs matched (29892, 278, 6575). The largest
absolute differences were 0.00161553 for logits and 0.001953125 for stored FP16
cache values. The scheduler callback recorded the output buffer device of every
node for both models, including views. Each of the six graph executions had 195
nodes, including 192 marked for compute. Every node used a vTensor buffer and
the vTensor processes loaded only the vTensor plugin.
[F32 per-step results](verification/llama-stories15m-float-vt58.json) ·
[Q4_0 per-step results](verification/llama-stories15m-vt58.json) ·
[Per-node placement transcript](verification/llama-stories15m-vt58-placement.tsv.gz)

## VT-57 complete checkpoint

The complete locked-SDK `just vv` passed for VT-57 with the source at
`e527c3c` and Verdant pinned to `53dfc67d4382c6d9b01eb04ae053ba6a9cd1c862`.
All 63 backend contract tests passed, including the upstream rotary suite.
The run covered the application proof, format, model, template, lint, workspace
tests and dependency checks. The later dense attention implementation has its
own scoped oracle and consumer results above.

VT-02 also passed on the refreshed PR #21 pin
`ed88897c22e962f1ddcf97ea5e2596200d9e9fb5`. The independent rotary and
weighted-mean engine oracles passed on that patch stack. The vTensor SDK
confirmed all fifteen imported descriptors and cargo-deny accepted the resolved
sources and licenses. [Verdant rotary PR](https://github.com/afflom/verdant-engine/pull/20)
· [Current numerical dependency](model/verdant-development.json)

## IQ2_XXS identity copy

For factory ordinal 8368 the upstream evaluator names the CPU as the sole
refusing backend. The GGML CPU graph entry point can execute the underlying
same-format copy. The test builds the 256 × 2 × 3 × 4 graph with the pinned
GGML constructors, fills every encoded input byte and compares the complete
destination from both devices. Both matched the input byte for byte.
Clearing the block in the vTensor adapter made the comparison fail at
`observed==data`. Restoring the source block passed. Run
`cargo test --release --locked -p vtensor-backend --test contract iq2_identity_copy_vt_15`
in the locked SDK. The [factory inventory](verification/ggml-factory-inventory.json)
retains the admission refusal from its own upstream evaluator.

## VT-06 layout fixtures

The pre-migration executable emitted 1,872 requests across the 36 imported
storage types. The fixture model reproduces all 48 strided and four empty-axis
layouts per type from the upstream block size and byte width. The complete
serialized input to the GGML storage oracle retains SHA-256
`c16f7845ee80d5bcc0cdba789e92eff7395c84b28a95fd06407bef0e14172570`.
Swapping the first two extents changed that digest to
`a9ccdebad3b7084af999748d61b4632cd8857e5911ca52d977b861015e9992a5`
and the layout test failed. The imported executable still supplies every
expected byte span. [Prism result](generated/probes/result.json)

## VT-58 complete checkpoint

At `06b95286fb1aaeff1fb579eaeeb6094bf514d1c0`, the pinned SDK ran the full
`just vv` and regenerated the application proof because its inputs had changed.
All 66 backend contract tests passed in 519.17 seconds, including the dense
attention graphs and direct IQ2_XXS copy. Cargo dependency checks passed.
The native audit counted 12,166 lines in both this revision and its parent.
Recovery to the original 10,549-line ceiling and published Verdant dependency
acceptance remain open. [Implementation](https://github.com/afflom/vTensor/commit/06b95286fb1aaeff1fb579eaeeb6094bf514d1c0)

## Elementary function graphs

VT-60 sends GGML LOG, SIN, COS and EXP to the pinned Verdant logarithm,
circular-function and exponential primitives. Prism selects the operation,
storage formats and addresses. Verdant performs both storage conversions and
the numerical operation. All 39 factory ordinals previously refused by
vTensor now pass the imported upstream reference evaluator on the restored
library. Sending SIN to the cosine output failed the first F16 sine case
with error 2.261962970 against a limit of 0.000000100. Sharing the checked
native row transfer between matrix and attention execution reduced the
overall native count from 12,166 to 12,163. [Oracle, source and certificate](verification/factory-elementary-vt60.json)

## Sigmoid and SiLU graphs

VT-61 uses the certified binary32 sigmoid and SiLU entry points on the
immutable Verdant PR #22 pin. vTensor supplies GGML storage and graph dispatch.
The upstream factory previously reported a vTensor refusal for each of these
122 graphs. The paired evaluator now passes both unary functions, their
multiplied forms and their add/multiply compositions. Routing sigmoid to SiLU
failed the first F16 sigmoid case. Restoring the correct binding passed all
122 cases. VT-02 accepted the exact Git dependency. [GGML results](verification/factory-logistic-vt61.json) · [Verdant scalar oracle](https://github.com/afflom/verdant-engine/pull/22)

## VT-60 and VT-61 checkpoints

At `2146195`, the complete locked-SDK run proved the application and passed
all 67 backend contract tests in 578.52 seconds. Its final dependency check
rejected the old license clarification hash for the regenerated pointwise
kernel. Revision `ada8c2d` bound the clarification to the new certified bytes
and the dependency check passed.

On the VT-61 Verdant pin, the 122 logistic cases, the 39 elementary cases,
the complete 510-case forward rotary group, the two small attention cases
and VT-02 passed together. Scoped Clippy and the dependency check passed.
VT-58 remains the most recent full `just vv` checkpoint.

## BF16 indexed row gradients

VT-62 extends the existing Prism scatter plan to accept BF16. Verdant widens the encoded words and accumulates them into F32. The reference graph uses the pinned GGML CPU cast followed by its F32 GET_ROWS_BACK operator. The unmodified standalone evaluator attributes ordinals 1069 and 1070 to CPU refusal alone. Both direct vTensor graphs matched the composed reference byte for byte, including the repeated row index. [Factory parameters and binary hashes](verification/factory-bf16-row-gradient-vt62.json)

In the locked SDK, `bf16_row_gradient_vt_62` passed 1,680 cases and `row_gradient_graphs_vt_31` passed all 3,360 existing F16/F32 cases. The comparator requires exact finite words, signed zeros, infinities and padding, and checks NaNs by class. Invalid indices must leave the whole output unchanged. Before the implementation, the native admission assertion failed. Routing a BF16 word to Verdant's F16 decoder produced byte 192 where the CPU returned 128. Restoring the source format passed the batch.

Prism now locates a transfer within both the tensor span and its backing allocation. The host reuses a computed span for tensor initialization, copying and attention inputs. The exported locator covers an offset view, a tensor extending beyond its backing store and an empty transfer. The native-line count fell from 12,162 to 12,158 across the same 51 files. [Certified layout roots](layout-kernel.lock)

Removing the backing-view displacement failed VT-11's viewed-byte comparison. Restoring the Prism offset passed VT-11 and all 510 forward rotary cases. The final gradient plan separates optional-index admission from scatter planning. Scoped Clippy, the dependency audit and the 1,680-case gradient suite passed on the regenerated package.

## BF16 attention

Prism admits two-byte BF16 key and value rows under the dense attention plan. Verdant rounds each query once to the key's dot-product storage, widens the operands, computes the dot product and updates the streaming weighted mean. The native host shares the same bounded input regions with quantized matrix reads.

The pinned CPU factory initially reported a vTensor refusal for ordinal 19540. The original four mask/sink combinations pass on the implementation, as do the existing small attention probes. Misreading BF16 keys and values as F16 produced normalized error 0.698156352 against the upstream limit of 0.0005. Restoring BF16 decoding passed. The numerical transcript links the tested provider binary to individual factory ordinals. [Factory evidence](verification/factory-bf16-attention-vt63.json)

## Scalar matrix storage and dot-product precision

VT-64 admits six mixed scalar operand pairs in MUL_MAT. The shared Prism storage rule selects F32 activations for F16 and F32 weights, and BF16 activations for BF16 weights. Both matrix and attention execution consume this rule. Verdant converts the encoded operands and computes the products. [Upstream CPU traits](https://github.com/ggml-org/llama.cpp/blob/9a080f44b503c64f66c84a6274b9d9ee5d5b09d5/ggml/src/ggml-cpu/ggml-cpu.c#L219)

The locked SDK passed ten original dense and permuted matrix cases and the original high-range F32 case. Rounding the latter input to F16 produced NaN at coordinate zero. The GGML CPU produced −2,146,902.75. Restoring the storage rule passed all eleven comparisons. The standalone runs cover disjoint original factory ordinals. [Verdicts and binary identities](verification/factory-scalar-matrix-vt64.json)

The existing F32 matrix, BF16 attention, small dense attention, L2, standard and group normalization, and RMS backward contract tests passed. Normalization instruction selection now lives in a single certified Prism root. The native implementation decreased from 12,158 to 12,151 lines. The locked workspace Clippy and dependency checks passed.

## VT-63 complete checkpoint

The locked SDK ran `just vv` on commit `04fb169920030555d4f6fabba31642c346455d5c`. Prism verified attestation `8592fe790d1cf436393ee8a63901a372ba0e8b038725933257243d7056dc2f14`. All 70 backend contract tests passed, followed by the remaining workspace tests and cargo-deny. [Checked source](https://github.com/afflom/vTensor/commit/04fb169920030555d4f6fabba31642c346455d5c)

## Grouped two-bit matrix products

VT-65 directs GGML weight views, broadcast columns and destination writes through the existing Prism quantized matrix plan. Verdant owns activation encoding, packed two-bit coefficients, group minimums and numerical contraction. All 55 original Q2_K matrix cases and the original gather-after-product graph matched the imported GGML CPU. Their ordered case transcript is checksum-bound. [Factory and mutation results](verification/factory-grouped-matrix-vt65.json) · [Encoder](https://github.com/afflom/verdant-engine/pull/23) · [Row product](https://github.com/afflom/verdant-engine/pull/24)

Reusing the first weight row for subsequent rows produced normalized error 1.880437132 at the final ADD against an allowed 0.0005. Restoring the Prism row offset passed all 56 cases. All 244 earlier Q4_0 and Q8_0 matrix cases passed the same shared runner. The implementation measures 12,151 non-Prism lines. VT-02 and all 56 grouped graphs passed on the final Verdant pin, and the locked workspace Clippy and dependency checks passed on the grouped implementation.

The scalar matrix implementation also passed 18 Hadamard cases, 11 additional explicit precision cases and 15 fusion cases. The remaining 20 mixed scalar fusion cases in that selection were refused. [Standalone results](verification/factory-scalar-matrix-vt64.json)

### VT-65 callback error boundary

The full gate at `77d63f6` stopped at the model audit. Two private row-kernel callbacks named Verdant’s error type in their signatures. The adapter now translates rejected Verdant inputs to vTensor’s declared `BackendError::InvalidTensor` when it binds the callbacks. The locked model audit passes and measures 12,143 native lines. All 56 original grouped-matrix cases pass. The shared fixture-digest registry preserves the two complete matrix and arithmetic input transcripts and the native fixture baseline.

### Packed key and value attention

VT-66 uses the imported GGML tensor-type table for block strides and whole-row admission. Prism selects Q8_0 query storage for both packed key types. The backend calls the pinned Verdant encoder once per query row, contracts each key with the corresponding Verdant row product, and uses the Verdant decoder for value blocks. The eight mask/sink cases passed on the plugin. Reusing the first packed key row produced an error of 0.126766720 on ordinal 19541 against GGML’s limit of 0.0005. Restoring row selection passed all eight cases. [Factory results](verification/factory-packed-attention-vt66.json)

The imported evaluator now reads selected factory ordinals from its test input. Its output includes the actual upstream operation name and parameters. Each selection has a checksum-pinned transcript. Changing the selection can reuse the compiled GGML driver. Typed and normalization word dispatch now share the Prism operand and target model. The pointwise certificate required 4,207,708 bytes, so its project permits 8 MiB per file. All 15 compatibility tests passed, as did the additional SwiGLU and group normalization checks.

## Hyperbolic tangent and capped attention

VT-67 runs all twelve TANH graphs and the composed SOFTCAP graph from the pinned GGML factory. VT-68 selects four distinct F16 attention graphs with masks and sinks at cap 10. Both suites pass on the loaded vTensor plugin against the imported CPU. The pointwise and attention Prism models determine GGML operation admission, storage, and cap placement. Verdant PR #25 computes the hyperbolic tangent. Its independent scalar comparison covers 128,870 inputs. [Verdant implementation and oracle](https://github.com/afflom/verdant-engine/pull/25)

Changing pre-cap division to multiplication in the vTensor adapter made the original GGML evaluator reject ordinal 22994 with error 0.544802536 against its limit of 0.0005. Reverting that change restored all four attention graphs. The seven-test compatibility run also passed the TANH, Verdant dependency, BF16 attention, packed attention, small attention and grouped matrix contracts. The broader cap-10 census uses separate factory ordinals.

## VT-65 complete checkpoint

The locked SDK passed `just vv` at `31b0c5c589e3b3a25263d883d5f547be38d73fe3` using Verdant PR #24. The gate reused attestation `8592fe790d1cf436393ee8a63901a372ba0e8b038725933257243d7056dc2f14`. All 72 backend contracts passed, followed by the remaining workspace tests and dependency checks. [Checked source](https://github.com/afflom/vTensor/commit/31b0c5c589e3b3a25263d883d5f547be38d73fe3)

## Scalar outer products

VT-69 executes all 71 F32 × F32 and 16 F16 × F32 OUT_PROD factory cases for which the imported GGML CPU provides a direct verdict. The Prism model contracts the second axis of both inputs and retains each physical stride and batch ratio. F32 weights admit batch replication. The pinned CPU accepts F16 weights with matching batch axes. Verdant performs the storage conversion and the matrix product. [Upstream constructor and shape](https://github.com/ggml-org/llama.cpp/blob/9a080f44b503c64f66c84a6274b9d9ee5d5b09d5/ggml/src/ggml.c#L3435)

Reading every right-hand word at offset zero corrupted the second batch of ordinal 15238. The imported evaluator rejected it with error 1.721467990 and limit 0.0005. The preserved scalar unit-product theorem and the existing 1,280 GGML matrix comparisons cover the positive matrix case. The native test retains the malformed inner-dimension, indivisible-batch and overflow checks.

## Three-dimensional image columns

VT-70 models the IM2COL_3D stride, dilation, signed padding, batch and channel coordinates in Prism. It uses the input view's actual byte strides. Verdant converts each selected F32 word or padding zero to the requested output storage. The pinned CPU passed all six contract graphs and 1,533 disjoint standalone graphs. These 1,539 passes close the previous refusals. Another 512 factory ordinals already had passing verdicts. [Imported constructor](https://github.com/ggml-org/llama.cpp/blob/9a080f44b503c64f66c84a6274b9d9ee5d5b09d5/ggml/src/ggml.c#L4706)

Replacing every interior sample address with zero made ordinal 2466 fail with error 1.334158325 against its 0.0000001 limit. Restoring the Prism offset passed the contract and the spatial, capped-attention and hyperbolic regressions.

### Packed outer products

VT-71 passed all 144 directly CPU-supported packed OUT_PROD factory graphs, covering nine weight formats. Prism schedules each source block from its physical stride. Verdant decodes the block and performs the contraction. Redirecting all block reads to offset zero failed ordinal 15493 with error 1.739076707 against 0.0005. Restoring the implementation passed all nine affected contracts. The locked Clippy and dependency checks passed.

## VT-68 complete checkpoint

The complete locked-SDK `just vv` passed at `3f1cabd0b5de78bb31b8485bf1f1a1223a9f5377` on the Verdant PR #25 pin. All 75 backend contracts passed, including VT-02, followed by the remaining workspace tests and dependency checks. Prism verified attestation `8592fe790d1cf436393ee8a63901a372ba0e8b038725933257243d7056dc2f14`. [Checked source](https://github.com/afflom/vTensor/commit/3f1cabd0b5de78bb31b8485bf1f1a1223a9f5377)

## Planar image columns and depthwise 1D convolution

VT-72 passed all 113 original IM2COL cases and both supplemental GGML graphs with F16 inputs. The nine original CONV_1D_DW graphs also passed, including BF16 kernels. They compose the existing image-column and matrix primitives. Prism selects each image sample, and Verdant converts the encoded word. Reading every interior sample at offset zero failed ordinal 2361 with error 1.992249257 against 0.0000001. The restored implementation passed the planar, 3D, row, packed outer-product and buffer-transfer contracts. [Oracle records](verification/factory-planar-columns-vt72.json)

## Provider linkage

VT-02 reads the undefined dynamic symbols of the backend that Cargo built. Its GGML imports are the host abort hook, buffer initialization and host query, graph size and node lookup. The GGML CPU oracle lives in the separate test executable. The check runs once alongside the existing Verdant source and descriptor checks.

Changing the compiled graph-size binding to resolve `ggml_mul_mat` made VT-02 fail with `unexpected provider import ggml_mul_mat`. Restoring the binding passed VT-02 and the declaration-inventory regression in the locked SDK. [Approved GGML host imports](verification/native-oracle-bindings.json)

## Decoded outer-product reference

The pinned CPU refuses 1,312 OUT_PROD graphs covering 23 operand-type pairs and batch replication. VT-73 executes each original graph on vTensor, including transposed right operands. The external evaluator reads both initialized operands using GGML’s own storage decoder, runs the F32 CPU outer product on their logical values, and applies the original test’s normalized mean-square error threshold. All 1,312 passed. The original direct CPU refusal remains in the factory ledger. Reading each packed block at offset zero failed ordinal 15542 at 2.133897848 against 0.0005. [Recorded comparisons](verification/factory-decoded-outer-products-vt73.json)

### Direct 2D convolution

VT-74 runs the original CONV_2D graphs on both vTensor and the pinned GGML CPU reference. Prism resolves stride, dilation, padding, channel order, unaligned F16 or F32 kernel views, and output coordinates. vTensor stages up to 32 patches at a time. Verdant converts kernel words and evaluates the matrix product. Replacing interior image addresses with the first word failed ordinal 7658 with NMSE 2.477084885 against the upstream limit of 0.0005. The restored implementation passed the five permanent cases. [Per-case record](verification/factory-direct-convolution-vt74.json)

VT-03 uses a frozen column-major result from the pinned GGML CPU matrix multiplication. The test inputs, products and sums are exactly representable in both binary32 and binary64. VT-05, VT-09 and VT-10 retain their independent runtime oracles and frozen boundary inputs. The source cache compares the complete imported archive in one pass and reuses compiled CPU objects across test modes.

### Verdant Gaussian activation pin

VT-02 resolved the fifteen ordered kernel descriptors from Verdant commit `24d9acbd0520d56f6fc9b27e5e67f70d0de17774`, inspected the compiled provider imports and passed. Cargo-deny passed. The two Gaussian scalar operations passed 257,548 comparisons against the imported GGML functions in the engine. The complete engine gate still reports the numerical frontier counts and stale digest recorded on the parent patch. [Verdant PR #26](https://github.com/afflom/verdant-engine/pull/26)

### VT-72 complete checkpoint

The complete locked-SDK `just vv` passed on source `4bef86af3e78ccd0f4c02cfba4f5b009abba1bf0` using Verdant PR #25. All 79 backend contracts passed, followed by the workspace checks and cargo-deny. Prism verified attestation `8592fe790d1cf436393ee8a63901a372ba0e8b038725933257243d7056dc2f14`. [Checked source](https://github.com/afflom/vTensor/commit/4bef86af3e78ccd0f4c02cfba4f5b009abba1bf0)

### Channel-major convolution

The pinned CPU refuses these original kernel views. VT-76 gives the CPU a separate copy of the original leaf values, uses upstream tensor operations to materialize both operands in logical order, and runs the same convolution parameters. The provider executes the original graph. Setting the copied reference leaves to zero failed ordinal 4742 with infinite NMSE. Restoring the leaves passed all three permanent checks. Outputs consisting entirely of padding must match exactly because their reference energy is zero. [Factory comparisons](verification/factory-contiguous-convolution-vt76.json)

### Gaussian graph execution

VT-75 executes the pinned factory’s 12 GELU, 12 GELU_QUICK, 39 GELU_MUL and one ADD_GELU_MUL on the plugin. Prism selects the GGML binary32 GELU profile and resolves its two cutoffs. Verdant performs the half conversions and evaluates both activations. Replacing the F32 GELU input with zero failed factory ordinal 100 with error 0.000255365 against 0.000000100. Restoring the original input passed every case. [Factory and mutation record](verification/factory-gaussian-activations-vt75.json)

The storage, allocation and transfer tests now share one oracle executable and one link step. Each mode still calls the pinned GGML implementation. All three modes and the certified layout comparisons passed in the integrated check.

### Rectified and Gaussian gated rows

VT-77 compares all 36 original factory cases for each of REGLU, GEGLU and GEGLU_QUICK against GGML CPU. Prism resolves gate selection, operand order and addresses. Verdant performs the comparisons, storage conversions, activations and products. Swapping the activation and gate inputs failed REGLU ordinal 584 with error 0.967962058 against the upstream limit of 0.0000001. Restoring the operands passed all 108 cases. VT-04 also verifies the values and order of every GLU enumerator against the imported header. [Factory and mutation evidence](verification/factory-gated-activations-vt77.json)

### Shared GGML CPU oracle object

The SDK combines the five imported GGML core objects into one relocatable object. Storage and graph oracles link the same bytes. Each cached graph check verifies one shared object and its SHA-256 stamp. The loader still checks the executable independently. The storage oracle, fixed-matrix contract and three contiguous-convolution graphs passed. Changing one byte at offset 504,024 of the 1,008,048-byte object made the graph check reject its digest before execution. Restoring the original object passed the same graph check. The change reduced the measured native source from 12,117 to 12,099 lines.

### VT-76 complete checkpoint

The full locked-SDK `just vv` passed on vTensor `4231e4422b59cb75a5671ad05913c4ee44de076e` using Verdant PR #26. All 83 backend contract tests passed, including the Gaussian operations, direct convolution and the independent contiguous CPU reference. The workspace checks and cargo-deny passed. Prism verified attestation `8592fe790d1cf436393ee8a63901a372ba0e8b038725933257243d7056dc2f14`. [Checked source](https://github.com/afflom/vTensor/commit/4231e4422b59cb75a5671ad05913c4ee44de076e)

### Forward pooling

VT-79 covers all 240 POOL_1D and 144 POOL_2D factory graphs and four additional F16 graphs. The Prism model plans each window and requests Verdant storage conversion, addition, comparison, count conversion and division. One-dimensional average pooling divides by the valid width. Two-dimensional average pooling includes padded elements in the divisor. Swapping average and maximum at the parameter binding failed the padded F16 one-dimensional case. GGML CPU returned zero and vTensor returned negative FLT_MAX. Restoring the binding passed all four affected contracts. The provider retained the five approved GGML host imports. [Case ordinals, hashes and planted defect](verification/factory-pooling-vt79.json)

### Direct volumetric convolution

VT-80 runs the original GGML CONV_3D graphs through the existing Verdant matrix product. Prism maps each kernel, input and output address across the three spatial axes and the packed batch and channel dimension. The six permanent cases cover dilation, separate batches, offset F16 and F32 filters, and two empty-kernel axes. Sampling every tile from the first tile’s origin failed ordinal 7723 with NMSE 3.720924840 against the upstream limit of 0.0005. Restoring the coordinates passed the volumetric, planar convolution, channel-major convolution and pooling contracts. [Case ordinals and binary hashes](verification/factory-volume-convolution-vt80.json)

### State convolution

Prism maps each sequence, channel and filter tap to GGML byte offsets. Verdant supplies binary32 multiplication, ordered addition, bias addition and SiLU. Six permanent graphs and 189 additional original factory graphs passed the imported CPU evaluator. Replacing multiplication with addition failed ordinal 10260 at NMSE 1.201701697 against the upstream limit of 0.0000001. The restored state convolution, 3D convolution, pooling and logistic contracts passed. The tested binary and source revision are recorded with the per-case evidence. [VT-81 comparisons](verification/factory-state-convolution-vt81.json)

### Biased attention

Prism plans the query head, mask coordinates and skip condition for the imported FLASH_ATTN_EXT graph. Verdant supplies the geometric head slope and arithmetic for each soft-capped score and scaled mask. The seven permanent cases cover F32, F16, BF16, Q4_0 and Q8_0 with shared or per-head masks. Bypassing the slope multiplication failed original ordinal 19442 at NMSE 0.224837896 against the upstream limit of 0.0005. The restored bias, packed attention, softcap and state-convolution contracts passed. VT-02 also verified the upstream Cargo pin and the compiled plugin’s GGML host imports. [Original case verdicts](verification/factory-biased-attention-vt78.json) · [Verdant numerical oracle](https://github.com/afflom/verdant-engine/pull/27)

### VT-80 complete checkpoint

The complete locked-SDK `just vv` passed on `024b20c94de1069d68f0a57ec0624ea45bbc11ef` with Verdant PR #26. All 86 backend contracts passed in 479.51 seconds, including pooling, 2D and 3D convolution. Prism verified attestation `8592fe790d1cf436393ee8a63901a372ba0e8b038725933257243d7056dc2f14`. The workspace checks and dependency audit passed. [Checked source](https://github.com/afflom/vTensor/commit/024b20c94de1069d68f0a57ec0624ea45bbc11ef)

### Column-to-signal overlap

Prism finds the contributing columns in the upstream summation order. Verdant converts each F16, BF16 or F32 input, sums it in binary32 and converts the final value to the destination type. All 33 original COL2IM_1D cases passed. Replacing addition with subtraction failed ordinal 8093 at NMSE 4.0 against the CPU limit of 0.0000001. The factory driver ran the single-column cases first, preserved the requested transcript order and rejected repeated and out-of-range ordinals before comparing any graph. The native source check now reads each compiler input once per test process. Changing the live ABI source made a prebuilt test fail with the stale filename. After restoring it, the same prebuilt test passed. The F16 pooling and image-column supplemental graphs also passed through the shared evaluator. [Case-level evidence](verification/factory-column-overlap-vt82.json)


### Packed indexed matrix products

VT-83 compares all 230 original Q4_0, Q8_0 and Q2_K MUL_MAT_ID graphs with the pinned GGML CPU backend. Nine cases run in the permanent contract. The other 221 ran once against the same certified matrix root. The first comparison refused the graph before implementation. Forcing every active slot to select expert zero produced NMSE 2.164573898 at ordinal 13528 against the original limit of 0.0005. Restoring expert selection passed the comparison. [Ordinals, digests and mutation](verification/factory-indexed-products-vt83.json)

Prism validates shapes and strides, maps each index through its GGML view and resolves the weight, activation and destination offsets. A skipped expert yields a zero result. An invalid expert stops the operation before destination writes. The provider calls the existing Verdant row encoders and dot products. VT-02 checks the compiled plugin's GGML imports. The source audit measures 12,084 native lines across 51 files.

A metadata scan selected previously refused factory graphs for numerical replay. The original evaluator passed all 24 selected Snake activation graphs, all 42 selected matrix-plus-add graphs and all 36 selected attention graphs. The evidence records each completed ordinal and transcript. [Original comparisons](verification/factory-admitted-compositions.json)

### VT-82 complete checkpoint

The complete locked-SDK `just vv` passed at `685b50a364772537ad6ba33054df523bea32220b` on the Verdant PR #27 pin. All 89 backend contracts passed in 738.15 seconds. Prism verified attestation `8592fe790d1cf436393ee8a63901a372ba0e8b038725933257243d7056dc2f14`. The workspace lint, dependency and provider linkage checks passed. [Verified source](https://github.com/afflom/vTensor/commit/685b50a364772537ad6ba33054df523bea32220b)

### Matrix and arithmetic fixture streams

Prism generates the four header words, each case descriptor and every operand word in the matrix and elementwise fixture streams. VT-35 and VT-36 check the byte counts and SHA-256 digests against the frozen baselines. VT-12 and VT-13 execute the same cached inputs against the imported GGML CPU backend. Flipping one bit in the first matrix record changed its digest from `b9b926d7…` to `54e256fd…` and failed VT-35. Restoring the original word passed.

### Scalar expert selection and E4M3 products

VT-84 compares all 251 original F32, F16 and BF16 indexed matrix graphs. It also sets the final expert ID to an out-of-range number through a strided index view and checks that the destination retains every byte on refusal. Prism resolves the expert and the byte ranges. Verdant prepares the activations and computes the row product. [VT-84 results](verification/factory-scalar-experts-vt84.json)

VT-85 compares all 230 original E4M3 matrix and indexed matrix graphs and 164 additional matrix fusions. The plugin calls the existing Verdant eight-bit float decoder. Selecting the wrong format failed the original evaluator on ordinal 13232 with normalized squared error 1.0 against a maximum of 0.0005. Restoring the decoder passed. VT-02 checked the compiled plugin’s upstream dependency pin and GGML imports after integration. [Product evidence](verification/factory-e4m3-products-vt85.json) · [Fusion evidence](verification/factory-e4m3-compositions.json)

### Normalization admission

Prism now supplies the execution plan for NORM, GROUP_NORM and RMS_NORM_BACK directly from the GGML descriptors. Verdant still computes the numerical results. Eight locked-SDK contract tests passed, including the original GGML normalization and composition cases, the strided and invalid-descriptor fixtures and the E4M3 regression. Refusing every NORM graph failed the original CPU comparison. Restoring admission passed. The source audit measured 12,091 non-Prism lines across 51 files.

### VT-86 row ranking

Prism binds GGML sort direction, row pitch, selection width and output placement. Verdant PR #28 supplies stable binary32 row ranking. The original pinned GGML evaluators passed all 632 standalone cases and 176 newly supported compositions. Both padded, offset row-view comparisons passed. Invalid strides, result types and sort directions were refused with destination bytes preserved. Inverting the requested direction failed ARGSORT ordinal 18388 with error 0.307692308 against a limit of 0.000000100. Restoring it passed. The merged provider passed VT-86 and VT-02. The native source audit measured 12,137 lines, 46 above the preceding normalization checkpoint. [Standalone transcripts](verification/factory-row-ranking-vt86.json) · [Composition transcripts](verification/factory-row-ranking-compositions.json)

### Integrated llama.cpp CPU threadpool

The unmodified llama.cpp callback loaded the combined provider at 97027cd, ran inference at one and two threads and completed teardown. Both runs reported a logit sum of −298643.6875 for 32,000 vocabulary entries. All three paired-decode steps reproduced the previously CPU-compared provider logits and KV snapshots byte for byte. VT-02, VT-59, VT-85 and VT-86 passed together. The native source audit measured a further 356 lines for the callbacks, worker transport and tests. The combined release gate remains open. [Binary and output digests](verification/llama-threadpool-vt59.json)

### Normalization execution

Prism now selects the instructions for L2_NORM, NORM, GROUP_NORM and RMS_NORM_BACK using the source, destination and secondary strides. The native runner submits the same graph request across copy and normalization modes. Verdant performs the arithmetic. Eleven locked-SDK regression tests passed. Forcing L2_NORM execution to refuse failed the imported evaluator and restoring execution passed. Four combined checks covered normalization, row ranking and llama.cpp threadpools. The merged source audit counted 12,455 non-Prism lines across 51 files, down 38, and all model gates passed. [Integrated implementation](https://github.com/afflom/vTensor/commit/b025d52)

### VT-87 row softmax

Prism binds GGML score rows, mask broadcasting, per-head sinks, head slopes and in-place output. Verdant PR #29 supplies the normalized exponential over binary32 values. All 214 upstream SOFT_MAX factory cases and 344 full TOPK_MOE graphs passed the original numerical evaluators. A supplemental in-place graph matched the CPU with an oversized mask row dimension and arbitrary singleton strides. Missing mask, bad mask type, wrong sink width and an unexpected source were refused with the result preserved. Replacing the input scale with zero failed SOFT_MAX ordinal 16954 at 0.309546217 against a limit of 0.000001000. Restoring it passed. The integrated provider passed VT-02, VT-38, VT-86 and VT-87 together and passed workspace lint. The native source audit measured 12,586 lines. [Standalone results](verification/factory-row-softmax-vt87.json) · [Composed graphs](verification/factory-row-softmax-compositions.json) · [Verdant primitive](https://github.com/afflom/verdant-engine/pull/29)

### VT-85 complete checkpoint

The complete locked-SDK `just vv` exited zero at 89a650cc82d0eaabc5d16c211548e6b7af845dd7 on the Verdant PR #27 pin. The SDK rebuilt the application proof for the changed inputs. All 92 backend contracts passed in 556.40 seconds. The model, template, formatting, lint and dependency checks passed. [Checked revision](https://github.com/afflom/vTensor/commit/89a650cc82d0eaabc5d16c211548e6b7af845dd7)
