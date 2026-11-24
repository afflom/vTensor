//! Checks for the hand-reviewed bootstrap policy.

use std::path::{Path, PathBuf};

use crate::Fail;

const UNIVERSAL_POLICY_PATHS: [&str; 7] = [
    ".devcontainer/devcontainer.json",
    ".github/workflows/bootstrap.yml",
    "AGENTS.md",
    "VERIFICATION.md",
    "prismpm.lock",
    "template-contract.json",
    "template.lock",
];
const PROJECT_CONTENT_PATHS: [&str; 1] = ["CONFORMANCE.md"];

fn read(root: &Path, path: &str) -> Result<String, Fail> {
    Ok(std::fs::read_to_string(root.join(path))
        .map_err(|error| format!("reading {path}: {error}"))?)
}

fn immutable_image(value: &str) -> bool {
    value.rsplit_once("@sha256:").is_some_and(|(name, digest)| {
        let valid_registry = name.split_once('/').is_some_and(|(registry, path)| {
            let (host, port) = registry
                .rsplit_once(':')
                .map_or((registry, None), |(host, port)| (host, Some(port)));
            !host.is_empty()
                && host.bytes().all(|byte| {
                    byte.is_ascii_lowercase() || byte.is_ascii_digit() || b".-".contains(&byte)
                })
                && port.is_none_or(|port| port.parse::<u16>().is_ok_and(|port| port > 0))
                && !path.is_empty()
                && path.bytes().all(|byte| {
                    byte.is_ascii_lowercase() || byte.is_ascii_digit() || b"./_-".contains(&byte)
                })
        });
        valid_registry && repo_model::artifact::is_lowercase_hex(digest, 64)
    })
}

fn canonical_json(root: &Path, path: &str) -> Result<(serde_json::Value, Vec<u8>), Fail> {
    let bytes =
        std::fs::read(root.join(path)).map_err(|error| format!("{path} is required: {error}"))?;
    let parsed = serde_json::from_slice(&bytes)?;
    if serde_json::to_vec(&parsed)? != bytes {
        return Err(format!("{path} is not canonical JSON").into());
    }
    Ok((parsed, bytes))
}

fn sha256(bytes: &[u8]) -> String {
    format!("sha256:{}", repo_model::artifact::sha256_hex(bytes))
}

fn content_matches(bytes: &[u8], expected: &str) -> bool {
    sha256(bytes) == expected
}

fn selected_lines<'a>(lines: &[&'a str], select: impl Fn(&str) -> bool) -> Vec<&'a str> {
    lines.iter().copied().filter(|line| select(line)).collect()
}

/// The SDK's `/tmp` is a fresh mode-0700 directory on the runner's disk,
/// removed with the credentials when the step ends. Its space is the disk the
/// runner has, so no fixed size caps how large a model's build may grow, and
/// no `/tmp` mount of another kind or size may stand in for it.
fn private_scratch(lines: &[&str]) -> bool {
    let tmp_mounts = selected_lines(lines, |line| {
        line.starts_with("--tmpfs /tmp") || line.contains(":/tmp\"")
    });
    lines.contains(&"scratch=$(mktemp -d)")
        && lines.contains(&"trap 'rm -rf \"$docker_config\" \"$scratch\"' EXIT")
        && tmp_mounts == [r#"--volume "$scratch:/tmp" \"#]
}

fn docker_credentials_are_confined(devcontainer: &str, bootstrap: &str) -> bool {
    let lines: Vec<_> = bootstrap.lines().map(str::trim).collect();
    let buildx_config = selected_lines(&lines, |line| line.starts_with("--env BUILDX_CONFIG="));
    let Some(start) = lines
        .iter()
        .position(|line| *line == "\"$image\" bash -c '")
    else {
        return false;
    };
    let Some(end) = lines[start + 1..].iter().position(|line| *line == "'") else {
        return false;
    };
    let script = &lines[start + 1..start + 1 + end];
    let Some(first_buildx) = script
        .iter()
        .position(|line| line.starts_with("docker buildx "))
    else {
        return false;
    };
    let mut umasks = script
        .iter()
        .copied()
        .enumerate()
        .filter(|(_, line)| line.starts_with("umask "));
    let private_umask = matches!(umasks.next(), Some((index, "umask 077")) if index < first_buildx)
        && umasks.next().is_none();
    WORKFLOW.docker_mounts.matches(devcontainer)
        && WORKFLOW.docker_workflow.matches(bootstrap)
        && buildx_config == [r"--env BUILDX_CONFIG=/tmp/prismpm-buildx \"]
        && private_scratch(&lines)
        && private_umask
}

fn policy_boundary_is_canonical(universal: &[&str], project: &[&str], required: &[&str]) -> bool {
    let mut union = universal.to_vec();
    union.extend(project.iter().copied());
    union.sort_by(|left, right| left.as_bytes().cmp(right.as_bytes()));
    universal == UNIVERSAL_POLICY_PATHS
        && project == PROJECT_CONTENT_PATHS
        && union == required
        && union
            .windows(2)
            .all(|pair| pair[0].as_bytes() < pair[1].as_bytes())
}

/// A pull request may run the complete gate once, on amd64, because its merge
/// commit is verified again on main. That saving is only sound while every main
/// push and tag still runs the complete gate on both native architectures, and
/// while nothing except a newer push to the same pull request can cancel or
/// replace a run: GitHub keeps one pending run per concurrency group, so a
/// shared main group would silently drop the verdict of a queued commit.
fn native_coverage_is_complete(bootstrap: &str) -> bool {
    const TRIGGERS: &str = "on:\n  push:\n    branches: [main]\n    tags: ['**']\n  pull_request:\n  merge_group:\n  workflow_dispatch:\n\n";
    const BOTH: &str = "[ubuntu-24.04, ubuntu-24.04-arm]";
    const PULL_REQUEST_AMD64: &str = r#"${{ fromJSON((github.event_name == 'pull_request' || github.event_name == 'merge_group') && '["ubuntu-24.04"]' || '["ubuntu-24.04", "ubuntu-24.04-arm"]') }}"#;
    const GROUP: &str = "${{ github.event_name == 'pull_request' && format('bootstrap-pr-{0}', github.event.pull_request.number) || format('bootstrap-run-{0}', github.run_id) }}";
    const CANCEL: &str = "${{ github.event_name == 'pull_request' }}";
    let values = |key: &str| {
        bootstrap
            .lines()
            .filter_map(|line| line.trim().strip_prefix(key))
            .map(str::trim)
            .collect::<Vec<_>>()
    };
    let runners = values("runner:");
    let groups = values("group:");
    let cancels = values("cancel-in-progress:");
    let skips_runs = bootstrap.lines().map(str::trim).any(|line| {
        [
            "if:",
            "continue-on-error:",
            "include:",
            "exclude:",
            "paths:",
            "paths-ignore:",
            "branches-ignore:",
            "tags-ignore:",
        ]
        .iter()
        .any(|key| line.starts_with(key))
    });
    bootstrap.contains(TRIGGERS)
        && (runners == [BOTH] || runners == [PULL_REQUEST_AMD64])
        && values("runs-on:") == ["${{ matrix.runner }}"]
        && ((groups.is_empty() && cancels.is_empty()) || (groups == [GROUP] && cancels == [CANCEL]))
        && !skips_runs
}

/// The cache key: every proof input, the SDK image digest and the Lean
/// toolchain by value. A key that omits one would find a proof of other inputs.
const PROOF_KEY: &str = "key: prism-proof-${{ runner.os }}-${{ runner.arch }}-sdk-${{ steps.proof-inputs.outputs.sdk }}-lean-${{ steps.proof-inputs.outputs.lean }}-${{ hashFiles('app/**', '!app/specs/**', 'prismpm.lock', 'Justfile', 'rust-toolchain.toml') }}";

#[derive(serde::Deserialize)]
struct WorkflowTerms {
    required: Vec<String>,
    forbidden: Vec<String>,
    counts: std::collections::BTreeMap<String, usize>,
}
impl WorkflowTerms {
    fn matches(&self, source: &str) -> bool {
        self.required.iter().all(|term| source.contains(term))
            && self.forbidden.iter().all(|term| !source.contains(term))
            && self
                .counts
                .iter()
                .all(|(term, n)| source.matches(term).count() == *n)
    }
}
#[derive(serde::Deserialize)]
struct WorkflowContract {
    proof_inputs: Vec<String>,
    proof_recipe: Vec<String>,
    update: WorkflowTerms,
    deployment: WorkflowTerms,
    docker_mounts: WorkflowTerms,
    docker_workflow: WorkflowTerms,
}
static WORKFLOW: std::sync::LazyLock<WorkflowContract> = std::sync::LazyLock::new(|| {
    pinned_fixture!(
        "workflow-contract-fixtures",
        "2c368019d757aac0e7e4fa3e2019e446b4d61f7a958b9a977a499b8b90205860"
    )
});

/// A run may reuse a kept proof only when nothing it would prove has changed.
/// The cache key finds a candidate and names every proof input; the event and
/// ref reach `just app` through the environment; `just app` reuses only on a
/// pull request, a merge queue run or a main push, and only after
/// `xtask proof-reuse` has checked that the kept attestation resolves and binds
/// this checkout's inputs. A tag, a manual run and a local run always prove.
/// Main's cache scope holds only proofs main made, so main reuses its own.
fn proof_reuse_is_confined(bootstrap: &str, justfile: &str) -> bool {
    let lines: Vec<_> = bootstrap.lines().map(str::trim).collect();
    let keys = selected_lines(&lines, |line| line.starts_with("key: prism-proof-"));
    let events = selected_lines(&lines, |line| line.contains("PRISM_PROOF_EVENT"));
    let names_inputs = lines
        .windows(WORKFLOW.proof_inputs.len())
        .any(|window| window == WORKFLOW.proof_inputs)
        && lines.contains(&r#"printf 'sdk=%s\nlean=%s\n' "$sdk" "$lean" >>"$GITHUB_OUTPUT""#)
        && bootstrap.matches("steps.proof-inputs.outputs.").count() == 2;
    let Some(start) = justfile.find("\napp:\n") else {
        return false;
    };
    let recipe: Vec<_> = justfile[start + 1..]
        .lines()
        .skip(1)
        .take_while(|line| line.starts_with("    ") || line.is_empty())
        .map(str::trim)
        .collect();
    let exits = recipe
        .iter()
        .filter(|line| line.contains("exit") || line.contains("return"))
        .count();
    keys == [PROOF_KEY]
        && names_inputs
        && events
            == [
                "PRISM_PROOF_EVENT: ${{ github.event_name }}:${{ github.ref }}",
                r"--env PRISM_PROOF_EVENT \",
            ]
        && lines.contains(&"path: .prism-proof")
        && recipe
            .windows(WORKFLOW.proof_recipe.len())
            .any(|window| window == WORKFLOW.proof_recipe)
        && exits == 1
        && justfile.matches("PRISM_PROOF_EVENT").count() == 2
        && justfile.matches("proof-reuse").count() == 2
}

fn workflow_history_is_complete(source: &str) -> bool {
    source.split("\n      - ").all(|step| {
        !step.contains("uses: actions/checkout@")
            || ["fetch-depth: 0", "persist-credentials: false"]
                .iter()
                .all(|required| step.lines().map(str::trim).any(|line| line == *required))
    })
}

fn update_preserves_project_content(source: &str) -> bool {
    source.lines().find_map(|line| {
        line.trim()
            .strip_prefix("for path in ")
            .and_then(|line| line.strip_suffix("; do"))
    }) == Some("AGENTS.md VERIFICATION.md template-contract.json .github/workflows/bootstrap.yml")
        && WORKFLOW.update.matches(source)
}

fn audit_policy_files(root: &Path, lock: &serde_json::Value, sdk_lock: &[u8]) -> Result<(), Fail> {
    let (contract, contract_bytes) = canonical_json(root, "template-contract.json")?;
    // The two identity fields and three path arrays below account for every allowed key.
    if contract.as_object().is_none_or(|object| object.len() != 5)
        || contract["schema"] != "uor/template-contract/1"
        || contract["version"] != "1.0.0"
    {
        return Err("template contract schema or version is unsupported".into());
    }
    let paths = |key: &str| -> Result<Vec<&str>, Fail> {
        contract[key]
            .as_array()
            .ok_or_else(|| format!("template contract has no {key}"))?
            .iter()
            .map(|value| {
                value
                    .as_str()
                    .ok_or_else(|| format!("{key} path is not a string").into())
            })
            .collect()
    };
    let universal = paths("universal_policy_paths")?;
    let project = paths("project_content_paths")?;
    let required = paths("required_paths")?;
    if !policy_boundary_is_canonical(&universal, &project, &required) {
        return Err("template contract path boundary is not canonical or disjoint".into());
    }
    if lock["contract_digest"] != sha256(&contract_bytes) {
        return Err("template.lock contract digest is stale".into());
    }
    // The registered universal paths are already in the canonical byte order.
    let expected = universal
        .into_iter()
        .filter(|&path| path != "template.lock");
    let policy_files = lock["policy_files"]
        .as_array()
        .ok_or("template.lock has no policy_files")?;
    if policy_files.len() != UNIVERSAL_POLICY_PATHS.len() - 1 {
        return Err("template.lock policy file set differs from the contract".into());
    }
    for (row, path) in policy_files.iter().zip(expected) {
        if row.as_object().is_none_or(|object| object.len() != 2)
            || row["path"].as_str() != Some(path)
        {
            return Err("template.lock policy file set differs from the contract".into());
        }
        let digest = row["sha256"]
            .as_str()
            .ok_or("policy_files sha256 is not a string")?;
        let on_disk;
        let bytes = match path {
            "template-contract.json" => &contract_bytes[..],
            "prismpm.lock" => sdk_lock,
            _ => {
                on_disk = std::fs::read(root.join(path))?;
                &on_disk
            }
        };
        if !content_matches(bytes, digest) {
            return Err(format!("universal policy drift: {path}").into());
        }
    }
    if lock["policy_tree_sha256"] != sha256(&serde_json::to_vec(policy_files)?) {
        return Err("template.lock policy tree digest is stale".into());
    }
    Ok(())
}

fn audit_sdk_inventory(lock: &serde_json::Value) -> Result<(), Fail> {
    let inventory_path = std::env::var_os("PRISMPM_SDK_INVENTORY")
        .ok_or("audit-bootstrap must run inside the digest-selected PrismPM SDK")?;
    let inventory_bytes = std::fs::read(inventory_path)?;
    let architecture = match std::env::consts::ARCH {
        "x86_64" => "amd64",
        "aarch64" => "arm64",
        value => value,
    };
    compare_sdk_inventory(
        lock,
        &inventory_bytes,
        &format!("{}/{architecture}", std::env::consts::OS),
    )
}

fn compare_sdk_inventory(
    lock: &serde_json::Value,
    inventory_bytes: &[u8],
    platform: &str,
) -> Result<(), Fail> {
    let inventory: serde_json::Value = serde_json::from_slice(inventory_bytes)?;
    let artifacts = inventory["artifacts"]
        .as_array()
        .ok_or("running SDK has no artifact inventory")?;
    if lock["schema"] == "prismpm/sdk-lock/2" {
        let platforms = lock["platforms"]
            .as_array()
            .ok_or("SDK platform inventory is absent")?;
        let mut matching = platforms.iter().filter(|row| row["platform"] == platform);
        let selected = matching.next().ok_or("SDK platform inventory is missing")?;
        if platforms.len() != 2 || matching.next().is_some() {
            return Err("SDK platform inventory is missing or duplicated".into());
        }
        if selected["inventory"] != inventory["artifacts"]
            || selected["inventory_digest"] != sha256(inventory_bytes)
        {
            return Err(
                "prismpm.lock native artifact inventory disagrees with the running SDK".into(),
            );
        }
        // The complete template-check also invokes the SDK's closed /2
        // validator for the index and child bindings. This independent gate
        // compares the actual executing native inventory without a fallback.
        return Ok(());
    }
    if lock["schema"] != "prismpm/sdk-lock/1" {
        return Err("unsupported SDK lock schema".into());
    }
    let locked = lock["inventory"]
        .as_array()
        .ok_or("prismpm.lock has no SDK inventory")?;
    let observed = locked.iter().filter(|row| row["id"] != "sdk-manifest");
    if !observed.eq(artifacts) {
        return Err("prismpm.lock artifact inventory disagrees with the running SDK".into());
    }
    let image_digest = lock["sdk_image"]
        .as_str()
        .and_then(|image| image.rsplit_once('@').map(|(_, digest)| digest))
        .ok_or("prismpm.lock SDK image is malformed")?;
    let manifest = locked
        .iter()
        .find(|row| row["id"] == "sdk-manifest")
        .ok_or("prismpm.lock has no SDK manifest inventory row")?;
    if manifest["kind"] != "image" || manifest["digest"] != image_digest {
        return Err("SDK manifest inventory row disagrees with sdk_image".into());
    }
    Ok(())
}

fn action_reference_is_pinned(line: &str) -> bool {
    let Some(reference) = line.trim().strip_prefix("uses:") else {
        return true;
    };
    let reference = reference.split('#').next().unwrap_or_default().trim();
    if reference.starts_with("./") {
        return false;
    }
    let Some((name, revision)) = reference.rsplit_once('@') else {
        return false;
    };
    !name.is_empty() && repo_model::artifact::is_lowercase_hex(revision, 40)
}

fn pipeline_lifecycle_is_complete(source: &str) -> bool {
    let conformance = source.find("command: conformance");
    let evidence_signature = source.find("command: sign-evidence");
    let deploy_has_signing_identity = source
        .split_once("\n  deploy:\n")
        .and_then(|(_, rest)| rest.split_once("\n  verify-deployment:\n"))
        .is_some_and(|(deploy, _)| deploy.contains("id-token: write"));
    WORKFLOW.deployment.matches(source)
        && conformance
            .is_some_and(|offset| evidence_signature.is_some_and(|signature| offset < signature))
        && !deploy_has_signing_identity
}

fn workflow_files(root: &Path) -> Result<Vec<PathBuf>, Fail> {
    let directory = root.join(".github/workflows");
    let mut paths = std::fs::read_dir(&directory)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "yml" || extension == "yaml")
        })
        .collect::<Vec<_>>();
    paths.sort();
    Ok(paths)
}

/// The Cargo dependencies the model's authorities pin (`AuthorityRow::pins`),
/// sorted; `None` when a name is empty, a pattern, or pinned twice.
fn upstream_pins(authorities: &repo_model::Authorities) -> Option<Vec<String>> {
    let mut pins: Vec<String> = authorities
        .authority
        .iter()
        .flat_map(|row| row.pins.iter().cloned())
        .collect();
    let count = pins.len();
    pins.sort();
    pins.dedup();
    let named = pins.iter().all(|name| {
        !name.is_empty()
            && name
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_')
    });
    (pins.len() == count && named).then_some(pins)
}

fn policy_json(value: &serde_json::Value) -> String {
    format!("{}\n", serde_json::to_string_pretty(value).unwrap())
}

fn dependency_updates_are_confined(source: &str, pins: &[String]) -> bool {
    const UNPINNED: &str = include_str!("../../verification/dependency-update-policy.json");
    if pins.is_empty() {
        return source == UNPINNED;
    }
    let mut expected: serde_json::Value =
        serde_json::from_str(UNPINNED).expect("canonical update policy");
    expected["updates"][1]["ignore"] = pins
        .iter()
        .map(|name| serde_json::json!({"dependency-name": name}))
        .collect();
    source == policy_json(&expected)
}

/// Keep SDK/template updates owned by their release flow, without disabling dependency maintenance.
pub fn audit_dependency_updates(root: &Path, model: &repo_model::Model) -> Result<(), Fail> {
    let Some(pins) = upstream_pins(&model.authorities) else {
        return Err("an authority's pins must name each Cargo dependency once, exactly".into());
    };
    if !dependency_updates_are_confined(&read(root, ".github/dependabot.yml")?, &pins) {
        return Err("Dependabot policy must use canonical JSON-form YAML, exclude only the locked bootstrap and SDK action, ignore exactly the Cargo dependencies an authority pins, and retain weekly Actions/Cargo maintenance".into());
    }
    println!("audit-dependency-updates: SDK ownership and ordinary maintenance are preserved");
    Ok(())
}

/// Validate immutable SDK selection and the independent workflow trust root.
pub fn audit(root: &Path, model: &repo_model::Model) -> Result<(), Fail> {
    audit_dependency_updates(root, model)?;
    if root.join(".github/actions/prismpm").exists() {
        return Err(
            "the template must consume the shared PrismPM action, not a copied wrapper".into(),
        );
    }
    let (lock, sdk_lock_bytes) = canonical_json(root, "prismpm.lock")?;
    let image = lock["sdk_image"]
        .as_str()
        .filter(|value| immutable_image(value))
        .ok_or("prismpm.lock does not select an immutable lowercase SDK manifest digest")?;
    audit_sdk_inventory(&lock)?;
    let (template_lock, _) = canonical_json(root, "template.lock")?;
    if template_lock["schema"] != "uor/template-lock/1"
        || template_lock["sdk_image"] != image
        || template_lock["template_repository"] != "https://github.com/UOR-Foundation/template"
        || !template_lock["template_revision"]
            .as_str()
            .is_some_and(|revision| repo_model::artifact::is_lowercase_hex(revision, 40))
    {
        return Err("template.lock identity is stale or mutable".into());
    }
    audit_policy_files(root, &template_lock, &sdk_lock_bytes)?;
    let devcontainer = read(root, ".devcontainer/devcontainer.json")?;
    let devcontainer_value: serde_json::Value = serde_json::from_str(&devcontainer)?;
    let contains = |key: &str, item: &str| {
        devcontainer_value[key]
            .as_array()
            .is_some_and(|items| items.iter().any(|value| value == item))
    };
    if devcontainer_value["image"] != image
        || devcontainer_value.get("build").is_some()
        || devcontainer_value.get("dockerFile").is_some()
        || devcontainer_value.get("features").is_some()
        || devcontainer_value["containerUser"] != "root"
        || devcontainer_value["remoteUser"] != "vscode"
        || devcontainer_value["overrideCommand"] != false
        || !contains("mounts", "source=/var/run/docker.sock,target=/var/run/docker.sock,type=bind")
        || !contains("mounts", "source=${localEnv:HOME}/.docker/config.json,target=/home/vscode/.docker/config.json,type=bind,readonly")
        || !devcontainer_value["initializeCommand"]
            .as_str()
            .is_some_and(|command| command.contains(".docker/config.json"))
        || !contains("runArgs", "prismpm-devcontainer-init")
        || devcontainer.contains("curl")
        || devcontainer.contains("stable")
    {
        return Err("the devcontainer is not a direct use of the locked SDK image".into());
    }

    let bootstrap = read(root, ".github/workflows/bootstrap.yml")?;
    if bootstrap.contains("pull_request_target")
        || !bootstrap.contains("permissions:\n  contents: read")
        || !bootstrap.contains("persist-credentials: false")
        || !docker_credentials_are_confined(&devcontainer, &bootstrap)
    {
        return Err(
            "bootstrap.yml is not a read-only pull-request trust root with confined credentials and private Buildx state"
                .into(),
        );
    }
    if !proof_reuse_is_confined(&bootstrap, &read(root, "Justfile")?) {
        return Err(
            "bootstrap.yml and the Justfile may reuse a proof only on a pull request, a merge queue run or a main push, keyed by every proof input and checked on read"
                .into(),
        );
    }
    if !native_coverage_is_complete(&bootstrap) {
        return Err(
            "bootstrap.yml must run the complete gate on both native architectures for every main push and tag, and never cancel or replace those runs"
                .into(),
        );
    }

    for path in workflow_files(root)? {
        let source = std::fs::read_to_string(&path)?;
        if !workflow_history_is_complete(&source) {
            return Err(format!(
                "{} omits complete history or retains checkout credentials",
                path.display()
            )
            .into());
        }
        if source.contains("ubuntu-latest")
            || source.contains("@main")
            || source.contains("@master")
            || source
                .lines()
                .any(|line| line.trim().starts_with("uses:") && !action_reference_is_pinned(line))
        {
            return Err(format!("{} contains a floating workflow input", path.display()).into());
        }
    }
    if !update_preserves_project_content(&read(root, ".github/workflows/template-update.yml")?) {
        return Err(
            "template updates overwrite project-owned inputs or omit a derived lock".into(),
        );
    }

    let reusable = read(root, ".github/workflows/prismpm.yml")?;
    let action_lines = reusable
        .lines()
        .filter(|line| line.contains("# prismpm-action-input"))
        .collect::<Vec<_>>();
    if action_lines.is_empty()
        || action_lines
            .iter()
            .any(|line| !action_reference_is_pinned(line))
    {
        return Err(
            "reusable pipeline does not use a full-commit-pinned shared PrismPM action".into(),
        );
    }
    let action = action_lines[0]
        .split_once("uses:")
        .and_then(|(_, value)| value.split('#').next())
        .map(str::trim)
        .ok_or("reusable pipeline action reference is malformed")?;
    if !action.starts_with("UOR-Foundation/PrismPM/action@")
        || action_lines.iter().any(|line| !line.contains(action))
    {
        return Err("reusable pipeline action references are inconsistent".into());
    }
    if !pipeline_lifecycle_is_complete(&reusable) {
        return Err(
            "reusable pipeline does not preserve the build/plan/state artifacts or complete the SDK-owned signing lifecycle"
                .into(),
        );
    }
    // The lifecycle predicate above has already checked the command counts and trust paths.
    for required in include_str!("../../verification/release-workflow-requirements.txt").lines() {
        if !reusable.contains(required) {
            return Err(format!("reusable pipeline is missing policy: {required}").into());
        }
    }

    println!("audit-bootstrap: SDK and workflow trust roots are immutable and separated");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        action_reference_is_pinned, content_matches, dependency_updates_are_confined,
        docker_credentials_are_confined, immutable_image, native_coverage_is_complete,
        pipeline_lifecycle_is_complete, policy_boundary_is_canonical, policy_json,
        proof_reuse_is_confined, sha256, update_preserves_project_content, upstream_pins,
        workflow_history_is_complete, PROJECT_CONTENT_PATHS, UNIVERSAL_POLICY_PATHS,
    };

    #[test]
    fn hosted_sdk_disk_is_prepared_before_image_pull() {
        // Both workflows that pull the SDK image on a hosted runner (#49).
        for workflow in [
            include_str!("../../.github/workflows/bootstrap.yml"),
            include_str!("../../.github/workflows/template-update.yml"),
        ] {
            assert_disk_reserved_before_sdk(workflow);
        }
    }

    #[test]
    fn sdk_pull_without_disk_reservation_is_rejected() {
        let update = include_str!("../../.github/workflows/template-update.yml");
        let start = update
            .find("      - name: Reserve disk for the locked SDK")
            .expect("reservation step");
        let end = update
            .find("      - name: Render the complete proposal from immutable inputs")
            .expect("render step");
        let plant = format!("{}{}", &update[..start], &update[end..]);
        assert!(std::panic::catch_unwind(|| assert_disk_reserved_before_sdk(&plant)).is_err());
    }

    fn assert_disk_reserved_before_sdk(workflow: &str) {
        let preparation = workflow
            .find("name: Reserve disk for the locked SDK")
            .expect("prepare hosted runner storage before pulling the large SDK");
        let pull = ["docker run --rm", "docker buildx imagetools inspect"]
            .iter()
            .filter_map(|needle| workflow.find(needle))
            .min()
            .expect("SDK invocation");
        assert!(preparation < pull);
        let script = &workflow[preparation..pull];
        assert!(script.contains("${RUNNER_ENVIRONMENT:-} == github-hosted"));
        assert!(script.contains("${RUNNER_OS:-} == Linux"));
        assert!(script.contains("sudo rm -rf -- /usr/local/lib/android /usr/share/dotnet /opt/ghc /opt/hostedtoolcache/CodeQL"));
        assert!(script.contains("df -h / /var/lib/docker"));
        assert!(!script.contains("rm -rf -- /var/lib/docker"));
        assert!(!script.contains("rm -rf -- \"$HOME\""));
    }

    fn check_workflow_controls(
        group: &str,
        count: usize,
        source: &str,
        accepts: impl Fn(&str) -> bool,
    ) {
        type Controls = std::collections::BTreeMap<String, Vec<Vec<[String; 2]>>>;
        static CONTROLS: std::sync::LazyLock<Controls> = std::sync::LazyLock::new(|| {
            pinned_fixture!(
                "native-workflow-controls",
                "d7901e2ebf9354e15b8be41f29e781b4996ca635f7f49870f7352d43d56785fb"
            )
        });
        let cases = &CONTROLS[group];
        assert_eq!(cases.len(), count);
        for edits in cases {
            let mut changed = source.to_string();
            for [before, after] in edits {
                if before.is_empty() {
                    changed.push_str(after);
                } else {
                    assert!(changed.contains(before), "missing workflow input {before}");
                    changed = changed.replace(before, after);
                }
            }
            assert_ne!(changed, source);
            assert!(!accepts(&changed), "{group} accepted {edits:?}");
        }
    }

    #[test]
    fn dropped_native_coverage_plant_is_rejected() {
        let bootstrap = include_str!("../../.github/workflows/bootstrap.yml");
        assert!(native_coverage_is_complete(bootstrap));
        check_workflow_controls("native", 8, bootstrap, native_coverage_is_complete);
        // Running both architectures on every event, without concurrency, is
        // stronger than the policy and stays admissible.
        let mut unconditional = bootstrap.replace(
            r#"${{ fromJSON((github.event_name == 'pull_request' || github.event_name == 'merge_group') && '["ubuntu-24.04"]' || '["ubuntu-24.04", "ubuntu-24.04-arm"]') }}"#,
            "[ubuntu-24.04, ubuntu-24.04-arm]",
        );
        let concurrency = unconditional
            .find("concurrency:\n")
            .expect("the workflow declares concurrency");
        let jobs = unconditional
            .find("jobs:\n")
            .expect("the workflow has jobs");
        unconditional.replace_range(concurrency..jobs, "");
        assert!(native_coverage_is_complete(&unconditional));
    }

    #[test]
    fn widened_proof_reuse_plant_is_rejected() {
        let bootstrap = include_str!("../../.github/workflows/bootstrap.yml");
        let justfile = include_str!("../../Justfile");
        assert!(proof_reuse_is_confined(bootstrap, justfile));
        check_workflow_controls("proof_workflow", 13, bootstrap, |changed| {
            proof_reuse_is_confined(changed, justfile)
        });
        check_workflow_controls("proof_recipe", 8, justfile, |changed| {
            proof_reuse_is_confined(bootstrap, changed)
        });
        // The merge queue trigger is part of the fixed trigger block: dropping it is refused.
        assert!(!native_coverage_is_complete(
            &bootstrap.replace("  merge_group:\n", "")
        ));
    }

    #[test]
    fn dependency_updates_preserve_sdk_ownership_and_ordinary_maintenance() {
        let source = include_str!("../../.github/dependabot.yml");
        assert!(dependency_updates_are_confined(source, &[]));
        assert_eq!(
            repo_model::artifact::sha256_hex(include_bytes!(
                "../../verification/dependency-update-policy.json"
            )),
            "12fcd70628c6c9d51f89e52d312432a87a84214511fbf6c22b9128907840a583"
        );
        check_workflow_controls("dependency_updates", 25, source, |changed| {
            dependency_updates_are_confined(changed, &[])
        });
        for malformed in ["{}\n", "version: 2\n"] {
            assert!(!dependency_updates_are_confined(malformed, &[]));
        }
    }

    /// #22: an instance ignores a Cargo dependency that an authority pins, and
    /// nothing else.
    #[test]
    fn upstream_pinned_dependency_is_ignored_and_nothing_else() {
        let source = include_str!("../../.github/dependabot.yml");
        let original: serde_json::Value = serde_json::from_str(source).unwrap();
        let with_ignore = |names: &[&str]| {
            let mut changed = original.clone();
            changed["updates"][1]["ignore"] = names
                .iter()
                .map(|name| serde_json::json!({"dependency-name": name}))
                .collect();
            policy_json(&changed)
        };
        let pins = vec!["unicode-normalization".to_string()];
        let pinned_policy = with_ignore(&["unicode-normalization"]);
        assert!(dependency_updates_are_confined(&pinned_policy, &pins));
        assert!(!dependency_updates_are_confined(&pinned_policy, &[]));
        assert!(!dependency_updates_are_confined(source, &pins));
        let undeclared_ignore = with_ignore(&["serde", "unicode-normalization"]);
        assert!(!dependency_updates_are_confined(&undeclared_ignore, &pins));

        let mut changed: serde_json::Value = serde_json::from_str(&pinned_policy).unwrap();
        changed["updates"][1]["schedule"]["interval"] = serde_json::json!("never");
        assert!(!dependency_updates_are_confined(
            &policy_json(&changed),
            &pins
        ));
    }

    #[test]
    fn authority_pins_name_each_dependency_once() {
        let cases: [(repo_model::Authorities, Option<Vec<String>>); 5] = pinned_fixture!(
            "authority-pin-controls",
            "c584ef665bea4bdc6a3e2c159d700791259656e80e35bfddd944ce6fa2dd241e"
        );
        for (authorities, expected) in cases {
            assert_eq!(upstream_pins(&authorities), expected);
        }
    }

    #[test]
    fn sha256_identity_retains_canonical_lowercase_bytes() {
        assert_eq!(
            sha256(b""),
            "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(
            sha256(b"abc"),
            "sha256:ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    fn run_node_suite(suite: &str) {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap();
        let result = std::process::Command::new("node")
            .args(["--test", suite])
            .current_dir(root)
            .status()
            .expect("the locked SDK supplies Node");
        assert!(result.success(), "{suite} failed");
    }

    #[test]
    fn initial_standards_binding_refuses_project_drift() {
        run_node_suite("bootstrap/standards-lock.test.mjs");
    }

    #[test]
    fn holo_cache_refuses_every_unbound_holo() {
        run_node_suite("ci/kappa-cache.test.mjs");
    }

    #[test]
    fn native_platform_inventory_rejects_swaps_missing_platforms_and_legacy_drift() {
        let fixture: serde_json::Value = pinned_fixture!(
            "sdk-platform-inventory-controls",
            "78410957e2a7e8e8b2d0ef1d80d658559873788b9752b6d884ce5c29854d4eb2"
        );
        let amd64 = fixture["amd64"].as_str().unwrap().as_bytes();
        let arm64 = fixture["arm64"].as_str().unwrap().as_bytes();
        let lock = &fixture["lock"];
        let legacy = &fixture["legacy"];
        for (platform, actual, other) in
            [("linux/amd64", amd64, arm64), ("linux/arm64", arm64, amd64)]
        {
            super::compare_sdk_inventory(lock, actual, platform).unwrap();
            assert!(super::compare_sdk_inventory(lock, other, platform).is_err());
        }
        let mut missing = lock.clone();
        missing["platforms"].as_array_mut().unwrap().pop();
        assert!(super::compare_sdk_inventory(&missing, amd64, "linux/amd64").is_err());
        super::compare_sdk_inventory(legacy, amd64, "linux/amd64").unwrap();
        assert!(super::compare_sdk_inventory(legacy, arm64, "linux/arm64").is_err());
    }

    #[test]
    fn floating_action_plant_is_rejected() {
        assert!(!action_reference_is_pinned("uses: actions/checkout@v4"));
        assert!(action_reference_is_pinned(
            "uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683"
        ));
        assert!(!action_reference_is_pinned(
            "uses: ./.github/actions/prismpm"
        ));
    }

    macro_rules! workflow_tests {
        ($(fn $name:ident() { $file:literal, $controls:literal, $count:literal, $accept:ident })+) => { $(
            #[test]
            fn $name() {
                let source = include_str!(concat!("../../.github/workflows/", $file));
                assert!($accept(source));
                check_workflow_controls($controls, $count, source, $accept);
            }
        )+ };
    }
    workflow_tests! {
        fn shallow_checkout_plant_is_rejected() { "prismpm.yml", "history", 2, workflow_history_is_complete }
        fn project_overwrite_plant_is_rejected() { "template-update.yml", "template_update", 5, update_preserves_project_content }
        fn incomplete_release_lifecycle_plant_is_rejected() { "prismpm.yml", "lifecycle", 5, pipeline_lifecycle_is_complete }
    }

    #[test]
    fn mutable_sdk_plant_is_rejected() {
        let digest = sha256(b"test SDK");
        let github = "ghcr.io/uor-foundation/prismpm-sdk";
        assert!(!immutable_image(&format!("{github}:latest")));
        assert!(immutable_image(&format!("{github}@{digest}")));
        assert!(immutable_image(&format!(
            "localhost:5000/prismpm-sdk@{digest}"
        )));
        assert!(!immutable_image(&format!(
            "localhost:65536/prismpm-sdk@{digest}"
        )));
    }

    #[test]
    fn universal_policy_drift_plant_is_rejected() {
        let recorded = sha256(b"reviewed policy\n");
        assert!(content_matches(b"reviewed policy\n", &recorded));
        assert!(!content_matches(b"changed policy\n", &recorded));
    }

    #[test]
    fn docker_credentials_and_buildx_state_are_confined() {
        let devcontainer = include_str!("../../.devcontainer/devcontainer.json");
        let bootstrap = include_str!("../../.github/workflows/bootstrap.yml");
        assert!(docker_credentials_are_confined(devcontainer, bootstrap));
        assert!(!docker_credentials_are_confined(
            &devcontainer.replace("/.docker/config.json,target", "/.docker,target"),
            bootstrap,
        ));
        check_workflow_controls("docker", 13, bootstrap, |changed| {
            docker_credentials_are_confined(devcontainer, changed)
        });
    }

    #[test]
    fn narrowed_policy_boundary_plant_is_rejected() {
        let mut required = UNIVERSAL_POLICY_PATHS.to_vec();
        required.extend(PROJECT_CONTENT_PATHS);
        required.sort_by(|left, right| left.as_bytes().cmp(right.as_bytes()));
        let accepts =
            |paths: &[&str]| policy_boundary_is_canonical(paths, &PROJECT_CONTENT_PATHS, &required);
        assert!(accepts(&UNIVERSAL_POLICY_PATHS));
        assert!(!accepts(&UNIVERSAL_POLICY_PATHS[..6]));
    }
}
