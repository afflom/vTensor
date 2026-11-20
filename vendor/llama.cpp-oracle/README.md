# llama.cpp oracle snapshot

These files are copied verbatim from
[`ggml-org/llama.cpp`](https://github.com/ggml-org/llama.cpp) at the commit in
[`upstream.toml`](./upstream.toml). The snapshot contains the public GGML backend API, the pinned backend
implementation interface, and upstream backend-operation consistency tests.
It is an acceptance oracle and specification source, not a vTensor
implementation or runtime dependency.

The files retain the upstream MIT license in [`LICENSE`](./LICENSE). Their
per-file SHA-256 values are checked by `oracle_import_vt_01`; the complete
directory is independently pinned by `model/authorities.toml`.

`source.tar.gz` is the unmodified `git archive --format=tar.gz` export of
`ggml`, `tests/test-backend-ops.cpp`, and `LICENSE` from that same revision.
Its archive digest and export paths are pinned separately in `upstream.toml`.
It supplies compilable upstream sources for test-only oracle executables;
no upstream computation is linked into the vTensor runtime. Extracted files
and native objects are disposable build-cache artifacts.
