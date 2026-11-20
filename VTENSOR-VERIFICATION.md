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
