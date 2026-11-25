# `just vv` is the normative acceptance gate. Every other recipe is either one
# of its parts or a writer the gate checks the output of.
#
# The Prism app runs first because it regenerates the Rust kernel package. The
# remaining checks then validate that generated source and the native adapter.

default: vv

# The whole gate.
vv: app fmt-check model template-check lint test deny
    @echo "vv: the acceptance gate passed"

fmt-check:
    just fmt --check

# R1, R4, R5 and the bootstrap trust root: `check-model`, `audit-limits`,
# `audit-deferral` and `audit-bootstrap` (which includes
# `audit-dependency-updates`), in one `xtask` process.
model:
    cargo run -q --locked -p xtask -- validate
    node bootstrap/frame-sync.mjs --check

# VT-02 binds the packages Cargo resolved to their authority. VT-45 reuses
# that resolution and replays the separate upstream quotient certificate.
verdant-source-check:
    #!/usr/bin/env python3
    import hashlib, json, re, subprocess, tarfile, tomllib
    from pathlib import Path

    def digest(data):
        return hashlib.sha256(data).hexdigest()

    def require(equal, expected, name):
        if equal != expected:
            raise ValueError(f"{name}: expected {expected!r}, got {equal!r}")

    pin = json.loads(Path("model/verdant-development.json").read_bytes())
    metadata = json.loads(subprocess.check_output(["cargo", "metadata", "--locked", "--offline", "--format-version=1"]))
    packages = {p["name"]: p for p in metadata["packages"] if p["name"].startswith("verdant")}
    engine = packages["verdant-tensor"]
    upstream = "https://github.com/afflom/verdant-engine"
    authorities = tomllib.loads(Path("model/authorities.toml").read_text())["authority"]
    authority, = (row for row in authorities if row["id"] == "VERDANT-ENGINE-TENSOR")
    if pin["spec"] == "vtensor/verdant-release/1":
        mirror = pin["index_commit"]
        if not re.fullmatch("[0-9a-f]{40}", mirror):
            raise ValueError("Verdant registry requires a full mirror commit")
        registry = f"sparse+https://raw.githubusercontent.com/afflom/verdant-engine/{mirror}/v{pin['version']}/crates/index/"
        require(engine["source"], registry, "Verdant registry")
        require(engine["version"], pin["version"], "Verdant release")
        citation = f"{upstream}/releases/tag/v{pin['version']}"
        require(pin["release"], citation, "Verdant release citation")
        require(authority["citation"], citation, "registered Verdant authority")
        require(authority.get("artifact"), "vendor/verdant-crates-manifest.json", "registered Verdant manifest")
        require(authority["checksum"], pin["manifest_sha256"], "registered Verdant checksum")
        manifest = Path(authority["artifact"]).read_bytes()
        require(digest(manifest), pin["manifest_sha256"], "release manifest")
        records = json.loads(manifest)
        checksums = {(row["name"], row["version"]): row["sha256"] for row in records["crates"] + records["dependencies"]}
        lock = tomllib.loads(Path("Cargo.lock").read_text())
        for item in lock["package"]:
            if item.get("source") == registry:
                require(item["checksum"], checksums[item["name"], item["version"]], item["name"])
        for item in metadata["packages"]:
            if item["source"] == registry:
                directory = Path(item["manifest_path"]).parent
                cache = directory.parents[2] / "cache" / directory.parent.name
                archive = cache / f'{item["name"]}-{item["version"]}.crate'
                require(digest(archive.read_bytes()), checksums[item["name"], item["version"]], archive)
    else:
        require(pin["spec"], "vtensor/verdant-patch/1", "Verdant dependency kind")
        rev = pin["commit"]
        require(authority["citation"], f"{upstream}/tree/{rev}", "registered Verdant patch authority")
        require(engine["source"], f"git+{upstream}?rev={rev}#{rev}", "Verdant patch")
        require(authority.get("artifact"), "model/verdant-development.json", "registered Verdant patch record")
        require(digest(Path(authority["artifact"]).read_bytes()), authority["checksum"], "registered Verdant patch checksum")
    for item in metadata["packages"]:
        if item["name"].startswith("verdant"):
            require(item["source"], engine["source"], item["name"])
    binding = json.loads(Path("verification/native-oracle-bindings.json").read_bytes())["verdant_quotient"]
    archive = Path("vendor/verdant-quotient.tar.gz")
    require(digest(archive.read_bytes()), binding["archive_sha256"], archive)
    with tarfile.open(archive) as bundle:
        def contents(name):
            return bundle.extractfile(name).read()
        for name, expected in binding["sha256"].items():
            require(digest(contents(name)), expected, name)
        require(json.loads(contents(".devcontainer/devcontainer.json"))["image"], binding["sdk"], "producer SDK")
        sources = {}
        for project, directory in binding["projects"].items():
            result = json.loads(contents(f"generated/{directory}/result.json"))
            sources[project] = result["source"]
            for root in result["roots"]:
                for output in root["outputs"]:
                    name = output["package"]["name"]
                    installed = Path(packages[name]["manifest_path"]).parent / "src/lib.rs"
                    certified = contents(f"generated/{directory}/packages/rust-core/{name}/src/lib.rs")
                    require(digest(installed.read_bytes()), digest(certified), installed)
    checked = json.loads(subprocess.check_output(["just", "--quiet", "verdant-quotient-check", binding["sdk"]]))
    require(set(checked), set(sources), "certified Verdant projects")
    for scope, source in sources.items():
        for key in ["source_id", "semantic_id"]:
            require(checked[scope][key], source[key], f"Verdant {scope} {key}")
    print(json.dumps(sources))

verdant-quotient-check image:
    #!/usr/bin/env bash
    set -euo pipefail
    docker run --rm -i --network none --entrypoint sh --tmpfs /tmp:exec,mode=1777 -e HOME=/tmp -w /tmp {{quote(image)}} -ec '
      tar -xzf -
      printf "{"
      for scope in quotient reference-log; do
        project="app/$scope.lexlean.toml"
        lexlean --project "$project" lock --check >&2
        printf "\"%s\":" "$scope"
        lexlean --project "$project" --diagnostic-format json check
        case "$scope" in quotient) directory=tensor-quotient; printf "," ;; *) directory=$scope ;; esac
        lexlean production verify-artifact "generated/$directory" >&2
      done
      printf "}\n"
    ' < vendor/verdant-quotient.tar.gz

# The SDK's own checks of the contract and both locks. `template check` also
# regenerates `CONFORMANCE.md` in an isolated copy through `model-write`.
template-check:
    prismpm template check
    prismpm lock check

# Clippy at `-D warnings` over every target with every feature enabled. This is
# the one place an optional feature is compiled: a rename upstream of an
# optional dependency fails nowhere else until someone turns the flag on, and
# `--all-targets` because the tests behind a flag are code too. `test` compiles
# the default feature set.
lint:
    cargo clippy --workspace --all-targets --all-features --locked -- -D warnings

# The workspace suite, which includes the `repo-conformance` meta-gate (R3,
# `CM-02`, `CM-03`).
test:
    cargo test --workspace --locked

# R6: nothing shipped depends on a dev-only crate, no wildcard version
# requirement, no advisory against anything in the tree. `cargo-deny` is
# supplied by the locked SDK.
deny:
    cargo deny --all-features check

# The template's own Prism application, the contract checker. The model must
# be canonical and locked. `prismpm verify` builds the holo, replaying its
# acceptance vectors in the compiled guest, then has the Lean kernel and
# leanchecker check every theorem (`TC-01`) and the generated code against the
# model. It runs the build itself, so a separate `prismpm build` would repeat it.
#
# A pull request, a merge queue run and a main push may reuse a kept proof;
# a tag, a manual run and every local run prove in full. bootstrap.yml
# restores `.prism-proof` from a cache keyed by every proof input and passes
# `<event>:<ref>` in PRISM_PROOF_EVENT. The key only finds a candidate:
# `xtask proof-reuse` reuses it only if its attestation ID resolves to the
# attestation it names and that attestation binds this checkout's model, this
# image's Lean and Lake, the architecture, the SDK image and every input file.
# Anything else proves in full, and every proof is kept and read back.
app:
    #!/usr/bin/env bash
    set -euo pipefail
    (cd app && lexlean fmt --check && lexlean lock --check)
    case "${PRISM_PROOF_EVENT:-}" in
      pull_request:*|merge_group:*|push:refs/heads/main)
        if cargo run -q --locked -p xtask -- proof-reuse; then
          exit 0
        fi
        ;;
    esac
    rm -rf .prism-proof
    prismpm --project app verify | tee .prism-proof.log
    attestation=$(sed -n 's/^verified attestation: \([0-9a-f]\{64\}\)$/\1/p' .prism-proof.log)
    rm -f .prism-proof.log
    cargo run -q --locked -p xtask -- proof-keep "$attestation"
    just prism-kernel-sync "$attestation"

prism-kernel-sync attestation:
    #!/usr/bin/env bash
    set -euo pipefail
    source=$(find app/.prism/build -type f -path '*/cargo/package/src/lib.rs' -printf '%T@ %p\n' | sort -nr | head -n 1 | cut -d ' ' -f2-)
    if [[ -z "$source" ]]; then
      echo "no Prism-generated Rust package found under app/.prism/build" >&2
      exit 1
    fi
    cp "$source" crates/prism-kernel/src/lib.rs
    rustfmt --edition 2021 crates/prism-kernel/src/lib.rs
    digest=$(sha256sum crates/prism-kernel/src/lib.rs | cut -d ' ' -f1)
    printf '{"spec":"prism/kernel-lock/1","source":{"repository":"afflom/vTensor","attestation":"%s"},"output":{"path":"crates/prism-kernel/src/lib.rs","sha256":"%s"}}\n' "{{attestation}}" "$digest" > prism-kernel.lock

# Only the meta-gate, while iterating on a scenario. `test` already runs it.
bdd:
    cargo test --locked -p vtensor-backend

# Certified SDK output retains the renderer's exact bytes; VT-06 replays its
# certificate. Format every other workspace member without traversing dependencies.
fmt *options:
    cargo metadata --locked --no-deps --format-version 1 | node -e 'const fs = require("node:fs"), cp = require("node:child_process"); const m = JSON.parse(fs.readFileSync(0, "utf8")); const p = m.packages.filter(p => m.workspace_members.includes(p.id) && !["vtensor_layout", "vtensor_types", "vtensor_transfer", "vtensor_buffer_region", "vtensor_allocation", "vtensor_native_config", "vtensor_native_predicates", "vtensor_native_memory", "vtensor_native_route", "vtensor_matrix", "vtensor_matrix_execution", "vtensor_matrix_q8", "vtensor_fixtures", "vtensor_fixture_stream", "vtensor_arithmetic", "vtensor_spatial", "vtensor_norm", "vtensor_norm_plan", "vtensor_norm_dispatch", "vtensor_rms_backward", "vtensor_l2", "vtensor_normfixtures", "vtensor_groupfixtures", "vtensor_rmsbackfixtures", "vtensor_probes", "vtensor_attention", "vtensor_attention_mask", "vtensor_rotary", "vtensor_typed", "vtensor_vector_swiglu", "vtensor_typedfixtures", "vtensor_index", "vtensor_pointwise", "vtensor_repeat_offset"].includes(p.name)); const a = ["fmt", ...p.flatMap(p => ["--package", p.name]), "--", ...process.argv.slice(1)]; const r = cp.spawnSync("cargo", a, {stdio: "inherit"}); if (r.error) throw r.error; process.exit(r.status ?? 1);' -- {{options}}

# Regenerate everything the model owns: CONFORMANCE.md.
model-write:
    cargo run -q --locked -p xtask -- check-model --write

# R3's first step in one command: a register row, its scenario, a failing test
# named for the ID, and the regenerated CONFORMANCE.md.
#
#     just new-capability CX-01 build parsing "A parsed document round-trips"
new-capability id level suite statement:
    cargo run -q --locked -p xtask -- new-capability {{quote(id)}} {{quote(level)}} {{quote(suite)}} {{quote(statement)}}

# Export the selected Prism root through the SDK once, then preserve the exact
# certified bundle. VT-06 checks freshness and replays it without reproving it.
layout-export:
    just certified-export layout

native-export:
    just certified-export native

matrix-export:
    just certified-export matrix

index-export:
    just certified-export index

pointwise-export:
    just certified-export pointwise

certified-export name:
    #!/usr/bin/env bash
    set -euo pipefail
    name={{quote(name)}}
    case "$name" in
      layout) project=app/lexlean.toml ;;
      native|matrix|index|fixtures|arithmetic|spatial|norm|normfixtures|groupfixtures|rmsbackfixtures|pointwise|probes|attention|rotary|typed|typedfixtures) project="app/$name.lexlean.toml" ;;
      *) echo "unknown certified bundle: $name" >&2; exit 1 ;;
    esac
    receipt=$(mktemp)
    trap 'rm -f "$receipt"' EXIT
    if ! lexlean --project "$project" --diagnostic-format json production build --request "app/$name-production.json" > "$receipt"; then
      cat "$receipt" >&2
      exit 1
    fi
    artifact=$(node -e 'const r = JSON.parse(require("node:fs").readFileSync(process.argv[1], "utf8")); if (!r.success || r.artifacts.length !== 1) throw Error("SDK did not export one certified bundle"); process.stdout.write(r.artifacts[0]);' "$receipt")
    just certified-install "$name" "$artifact"

# Install a previously certified SDK bundle; this writer is not an acceptance gate.
certified-install name artifact:
    #!/usr/bin/env bash
    set -euo pipefail
    node - {{quote(name)}} {{quote(artifact)}} <<'JS'
    const fs = require('node:fs'), path = require('node:path');
    const name = process.argv[2], artifact = process.argv[3];
    const buildRoot = name === 'layout' ? '.lexlean' : `.lexlean-${name}`;
    if (!artifact.startsWith(`${buildRoot}/production/`) || !/^[0-9a-f]{64}$/.test(artifact.slice(buildRoot.length + 12))) throw Error('unexpected SDK bundle path');
    const source = path.resolve('app', artifact);
    const result = JSON.parse(fs.readFileSync(path.join(source, 'result.json'), 'utf8'));
    if (path.basename(source) !== result.result_id) throw Error('SDK result identity differs from its path');
    const lock = require('node:child_process').execFileSync('just', ['--quiet', 'certified-kernel-lock', name, source]);
    fs.mkdirSync('generated', {recursive: true});
    const staging = path.join('generated', `.${name}-${process.pid}`);
    fs.cpSync(source, staging, {recursive: true});
    fs.rmSync(`generated/${name}`, {recursive: true, force: true});
    fs.renameSync(staging, `generated/${name}`);
    fs.writeFileSync(`${name}-kernel.lock`, lock);
    JS
    # cargo-deny uses XXH32 for license evidence. Refresh existing decisions only for this export.
    python3 - {{quote(name)}} <<'PYTHON'
    import ctypes, re, sys, tomllib
    from pathlib import Path
    packages = Path('generated', sys.argv[1], 'packages', 'rust-core')
    xxh32 = ctypes.CDLL('libxxhash.so.0').XXH32
    xxh32.argtypes = [ctypes.c_char_p, ctypes.c_size_t, ctypes.c_uint32]
    xxh32.restype = ctypes.c_uint32
    policy = Path('deny.toml')
    original = policy.read_text()
    sections = re.split(r'(?=^\[\[licenses\.clarify\]\])', original, flags=re.M)
    for index, section in enumerate(sections[1:], 1):
        rule = tomllib.loads(section)['licenses']['clarify'][0]
        package = packages / rule['name']
        if not package.is_dir():
            continue
        manifest = tomllib.loads((package / 'Cargo.toml').read_text())['package']
        if rule.get('version') != '=' + manifest['version'] or [f['path'] for f in rule['license-files']] != ['src/lib.rs']:
            continue
        data = (package / 'src/lib.rs').read_bytes()
        value = xxh32(data, len(data), 0)
        sections[index], count = re.subn(r'(path\s*=\s*"src/lib\.rs"\s*,\s*hash\s*=\s*)0x[\da-fA-F]+', lambda m: f'{m[1]}0x{value:08x}', section, count=1)
        if count != 1:
            raise ValueError(f'Unrecognized cargo-deny license hash for {package.name}')
    updated = ''.join(sections)
    if updated != original:
        policy.write_text(updated)
    PYTHON

# Reconstruct the repository lock from the requested roots and their checked SDK outputs.
certified-kernel-lock name directory:
    #!/usr/bin/env bash
    set -euo pipefail
    node - {{quote(name)}} {{quote(directory)}} <<'JS'
    const fs = require('node:fs'), path = require('node:path'), assert = require('node:assert/strict');
    const [name, directory] = process.argv.slice(2);
    const read = file => JSON.parse(fs.readFileSync(file, 'utf8'));
    const result = read(path.join(directory, 'result.json'));
    const wanted = read(`app/${name}-production.json`).roots;
    assert.equal(result.roots.length, wanted.length, 'root count');
    for (const [i, root] of result.roots.entries()) {
      const request = wanted[i];
      assert.equal(root.root, request.root, 'requested root');
      assert.equal(root.outputs.length, 1, root.root);
      const [output] = root.outputs;
      assert.deepEqual(output.target, request.targets[0]);
      assert.equal(output.package.name, request.package.name);
      assert.equal(output.package.version, request.package.version);
      assert.equal(output.certification.status, 'kernel-checked');
    }
    const [primary, packageName] = {
      layout: ['PrismTemplateContract.TensorLayout.tensorByteSpan', 'vtensor_layout'],
      native: ['PrismTemplateContract.NativeBackend.configuration', 'vtensor_native_config'],
      matrix: ['PrismTemplateContract.MatrixGraph.planMatrix', 'vtensor_matrix'],
      index: ['PrismTemplateContract.TensorIndex.elementOffset', 'vtensor_index'],
      fixtures: ['PrismTemplateContract.MatrixFixtures.evaluate', 'vtensor_fixtures'],
      arithmetic: ['PrismTemplateContract.ArithmeticFixtures.evaluate', 'vtensor_arithmetic'],
      spatial: ['PrismTemplateContract.SpatialFixtures.evaluate', 'vtensor_spatial'],
      norm: ['PrismTemplateContract.NormGraph.evaluate', 'vtensor_norm'],
      normfixtures: ['PrismTemplateContract.NormFixtures.evaluate', 'vtensor_normfixtures'],
      groupfixtures: ['PrismTemplateContract.GroupFixtures.evaluate', 'vtensor_groupfixtures'],
      rmsbackfixtures: ['PrismTemplateContract.RmsBackwardFixtures.evaluate', 'vtensor_rmsbackfixtures'],
      probes: ['PrismTemplateContract.FixtureProbes.evaluate', 'vtensor_probes'],
      attention: ['PrismTemplateContract.AttentionGraph.evaluate', 'vtensor_attention'],
      rotary: ['PrismTemplateContract.RotaryGraph.evaluate', 'vtensor_rotary'],
      typed: ['PrismTemplateContract.TypedArithmetic.evaluate', 'vtensor_typed'],
      typedfixtures: ['PrismTemplateContract.TypedBinaryFixtures.evaluate', 'vtensor_typedfixtures'],
      pointwise: ['PrismTemplateContract.PointwiseGraph.evaluate', 'vtensor_pointwise'],
    }[name];
    const output = result.roots.find(r => r.root === primary).outputs[0].package.files.find(f => f.path.endsWith('/src/lib.rs'));
    if (output.path !== `packages/rust-core/${packageName}/src/lib.rs`) throw Error('unexpected exported package');
    const files = result.roots.flatMap(r => r.outputs.flatMap(o => [...o.package.files.filter(f => f.path.endsWith('.rs')), o.crate].map(f => ({path: `generated/${name}/${f.path}`, sha256: f.sha256}))));
    const lock = {spec: 'prism/kernel-lock/1', source: {repository: 'afflom/vTensor', attestation: result.verification.attestation_id, result: result.result_id}, output: {path: `generated/${name}/${output.path}`, sha256: output.sha256}, files};
    process.stdout.write(JSON.stringify(lock) + '\n');
    JS

# Prefetch the same immutable dependencies used by normal Cargo builds.
verdant-prepare:
    cargo fetch --locked

# Reconcile the existing GGML transcripts without executing cases again. Print
# unresolved ordinals on stdout, or request evidence for every factory ordinal.
factory-remaining $view="remaining":
    #!/usr/bin/env python3
    import csv, gzip, hashlib, io, json, os, sys
    from collections import Counter
    from pathlib import Path

    def read(path):
        return json.loads(Path(path).read_bytes())

    def transcript(path, digest):
        raw = gzip.decompress(path.read_bytes())
        if hashlib.sha256(raw).hexdigest() != digest:
            raise ValueError(f"transcript digest mismatch: {path}")
        return io.StringIO(raw.decode())

    view = os.environ["view"]
    if view not in {"remaining", "evidence"}:
        sys.exit("expected remaining or evidence")
    directory = Path("verification")
    inventory = read(directory / "ggml-factory-inventory.json")
    ledger = inventory["factory_verdicts"]
    rows = transcript(directory / inventory["case_transcript"], inventory["case_transcript_sha256"])
    cases = {}
    for tag, ordinal, label, parameters in csv.reader(rows, delimiter="\t"):
        if tag != "CASE" or int(ordinal) != len(cases):
            raise ValueError(f"unexpected factory row: {ordinal}")
        cases[int(ordinal)] = (label, parameters)
    if len(cases) != inventory["cases"]:
        raise ValueError("factory count changed")

    verdicts = {}
    def record(ordinal, verdict, group, source):
        if ordinal not in cases or ordinal in verdicts:
            raise ValueError(f"invalid or duplicate original verdict: {ordinal}")
        verdicts[ordinal] = (verdict, group, source)

    def receipt(entry):
        path = entry if isinstance(entry, str) else entry["evidence"]
        data = read(path)
        if data["authority_revision"] != inventory["authority_revision"]:
            raise ValueError(f"GGML revision mismatch: {path}")
        return data

    standalone = receipt(ledger["standalone_evidence"])
    rows = transcript(directory / standalone["verdict_table"], standalone["uncompressed_verdicts_sha256"])
    for row in csv.DictReader(rows, delimiter="\t"):
        record(int(row["ordinal"]), row["verdict"], "standalone", ledger["standalone_evidence"])

    updates = {**ledger["superseded_standalone_verdicts"], **ledger["superseded_factory_verdicts"]}
    labels = {}
    for requirement, group in ledger["passed_by_requirement"].items():
        if requirement in updates:
            continue
        for label in group:
            if label in labels:
                raise ValueError(f"ambiguous original label: {label}")
            labels[label] = requirement
    for ordinal, (label, parameters) in cases.items():
        if label in labels:
            record(ordinal, "passed", labels[label], ledger["source"])
        for requirement, group in ledger["passed_parameter_groups"].items():
            if group["label"] == label:
                prefixes = group.get("parameters_prefixes", [])
                if "parameters_prefix" in group:
                    prefixes = [group["parameters_prefix"]]
                if ordinal in group.get("ordinals", []) or parameters.startswith(tuple(prefixes)):
                    record(ordinal, "passed", requirement, ledger["source"])
    for ordinal in ledger["refused_ordinals"]:
        record(int(ordinal), "refused", "factory", "verification/ggml-factory-inventory.json")

    for requirement, entry in updates.items():
        for label, ordinals in receipt(entry)["case_ordinals"].items():
            for ordinal in ordinals:
                if ordinal not in verdicts or cases[ordinal][0] != label:
                    raise ValueError(f"receipt disagrees with original verdict: {label} {ordinal}")
                source = entry if isinstance(entry, str) else entry["evidence"]
                verdicts[ordinal] = ("passed", requirement, source)

    counts = Counter(verdict for verdict, _, _ in verdicts.values())
    counts["untested"] += len(cases) - len(verdicts)
    if counts != Counter({name: ledger[name] for name in ("passed", "refused", "failed", "untested")}):
        raise ValueError(f"factory totals disagree with the receipts: {counts}")
    print(f'{counts["passed"]} passed, {counts["refused"]} refused, {counts["failed"]} failed, {counts["untested"]} untested', file=sys.stderr)
    if view == "evidence":
        print("ordinal", "verdict", "group", "evidence", "label", "parameters", sep="\t")
    for ordinal, (label, parameters) in cases.items():
        verdict, group, source = verdicts.get(ordinal, ("untested", "", ""))
        if view == "evidence":
            print(ordinal, verdict, group, source, label, parameters, sep="\t")
        elif verdict != "passed":
            print(ordinal, verdict, label, parameters, sep="\t")
