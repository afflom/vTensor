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
The declared upstream base is Verdant v2.4.0 commit `87c3256326b9`; the
reproducible matrix/scalar/storage extension checkpoint is `26ba1eaa1369`.

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
| `VT-12` | build | Development graph path matches 640 upstream CPU matrix cases, including empty dimensions, padding, broadcasting and IEEE edge values. |
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
TRI shares that selection path for all four upstream variants and rejects
nonsquare descriptors before writes. FILL converts constants once through
Verdant and uses the transfer path for F32/F16, including in-place aliases.
Mixed F32/F16/BF16 copies use Prism ordinal-address plans and Verdant encoding
conversion. Remaining conversions, operations and numeric types are unfinished.
Storage and empty-graph coverage for all 36 live types do not establish their
nonempty numerical behavior.

The development Verdant dependency also provides a certified F32/F16/BF16
storage conversion root, independently checked against 1,397,260 imported GGML
comparisons. FILL and admitted mixed F32/F16/BF16 copies use this primitive.

The last full pinned-SDK `just vv` pass covers VT-11 before the numerical work.
The current development tree passes twenty-three contract tests; VT-02 refuses the
unpublished local dependency. Formatting, model/source audits, workspace Clippy
and dependency checks pass. No release gate has been weakened to admit it.
Detailed evidence is recorded in
[`VTENSOR-VERIFICATION.md`](VTENSOR-VERIFICATION.md).

## Development

The completion sequence uses shared models, execution primitives and the same
imported oracle throughout:

1. Keep the complete imported contract inventory, including nested operation
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
