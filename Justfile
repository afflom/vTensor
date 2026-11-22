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
    cargo metadata --locked --no-deps --format-version 1 | node -e 'const fs = require("node:fs"), cp = require("node:child_process"); const m = JSON.parse(fs.readFileSync(0, "utf8")); const p = m.packages.filter(p => m.workspace_members.includes(p.id) && !["vtensor_layout", "vtensor_types", "vtensor_transfer", "vtensor_allocation", "vtensor_native_config", "vtensor_native_predicates", "vtensor_native_memory", "vtensor_matrix", "vtensor_matrix_execution", "vtensor_fixtures", "vtensor_arithmetic", "vtensor_spatial", "vtensor_norm", "vtensor_norm_plan", "vtensor_rms_backward", "vtensor_l2", "vtensor_normfixtures", "vtensor_groupfixtures", "vtensor_rmsbackfixtures", "vtensor_typed", "vtensor_index", "vtensor_pointwise", "vtensor_repeat_offset"].includes(p.name)); const a = ["fmt", ...p.flatMap(p => ["--package", p.name]), "--", ...process.argv.slice(1)]; const r = cp.spawnSync("cargo", a, {stdio: "inherit"}); if (r.error) throw r.error; process.exit(r.status ?? 1);' -- {{options}}

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
      native) project=app/native.lexlean.toml ;;
      matrix) project=app/matrix.lexlean.toml ;;
      index) project=app/index.lexlean.toml ;;
      fixtures) project=app/fixtures.lexlean.toml ;;
      arithmetic) project=app/arithmetic.lexlean.toml ;;
      spatial) project=app/spatial.lexlean.toml ;;
      norm) project=app/norm.lexlean.toml ;;
      normfixtures) project=app/normfixtures.lexlean.toml ;;
      groupfixtures) project=app/groupfixtures.lexlean.toml ;;
      rmsbackfixtures) project=app/rmsbackfixtures.lexlean.toml ;;
      pointwise) project=app/pointwise.lexlean.toml ;;
      typed) project=app/typed.lexlean.toml ;;
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
      typed: ['PrismTemplateContract.TypedArithmetic.evaluate', 'vtensor_typed'],
      pointwise: ['PrismTemplateContract.PointwiseGraph.evaluate', 'vtensor_pointwise'],
    }[name];
    const output = result.roots.find(r => r.root === primary).outputs[0].package.files.find(f => f.path.endsWith('/src/lib.rs'));
    if (output.path !== `packages/rust-core/${packageName}/src/lib.rs`) throw Error('unexpected exported package');
    fs.mkdirSync('generated', {recursive: true});
    const staging = path.join('generated', `.${name}-${process.pid}`);
    fs.cpSync(source, staging, {recursive: true});
    fs.rmSync(`generated/${name}`, {recursive: true, force: true});
    fs.renameSync(staging, `generated/${name}`);
    const files = result.roots.flatMap(r => r.outputs.flatMap(o => [...o.package.files.filter(f => f.path.endsWith('.rs')), o.crate].map(f => ({path: `generated/${name}/${f.path}`, sha256: f.sha256}))));
    const lock = {spec: 'prism/kernel-lock/1', source: {repository: 'afflom/vTensor', attestation: result.verification.attestation_id, result: result.result_id}, output: {path: `generated/${name}/${output.path}`, sha256: output.sha256}, files};
    fs.writeFileSync(`${name}-kernel.lock`, JSON.stringify(lock) + '\n');
    JS

# Reproduce the development dependency without resetting an existing checkout.
verdant-prepare:
    #!/usr/bin/env node
    const fs = require('node:fs'), path = require('node:path');
    const cp = require('node:child_process'), crypto = require('node:crypto');
    const root = process.cwd();
    const m = JSON.parse(fs.readFileSync('model/verdant-development.json', 'utf8'));
    if (m.spec !== 'vtensor/verdant-development/1' || !/^[0-9a-f]{40}$/.test(m.commit) || !/^[0-9a-f]{40}$/.test(m.base)) throw Error('invalid development source record');
    if (m.directory !== '.verdant-engine' || m.bundle !== 'vendor/verdant-development.bundle') throw Error('unexpected development source path');
    const bundle = path.join(root, m.bundle);
    if (crypto.createHash('sha256').update(fs.readFileSync(bundle)).digest('hex') !== m.sha256) throw Error('development source bundle hash differs');
    const target = path.join(root, m.directory);
    const git = (cwd, ...args) => cp.execFileSync('git', ['-C', cwd, ...args], {encoding:'utf8'}).trim();
    const check = directory => {
      if (git(directory, 'rev-parse', 'HEAD') !== m.commit) throw Error('development checkout has a different commit; preserve it before preparing the recorded source');
      if (git(directory, 'status', '--porcelain', '--untracked-files=all')) throw Error('development checkout has local changes; preserve them before preparing the recorded source');
      git(directory, 'merge-base', '--is-ancestor', m.base, m.commit);
    };
    if (fs.existsSync(target)) {
      check(target);
      process.stdout.write(`Verdant development source already matches ${m.commit}\n`);
    } else {
      const staging = path.join(root, `.verdant-engine.prepare-${process.pid}`);
      cp.execFileSync('git', ['clone', '--no-checkout', m.upstream, staging], {stdio:'inherit'});
      git(staging, 'bundle', 'verify', bundle);
      git(staging, 'fetch', bundle, m.ref);
      git(staging, 'checkout', '--detach', m.commit);
      check(staging);
      fs.renameSync(staging, target);
      process.stdout.write(`Prepared Verdant development source ${m.commit}\n`);
    }
