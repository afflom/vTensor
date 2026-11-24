//! The proof `just app` keeps, and the check a run makes before it reuses one.
//!
//! The cache key only finds a candidate. It is not trusted: a kept proof is
//! reused only when its attestation ID resolves to the attestation it names,
//! that attestation's evidence is the evidence it hashes, and the attestation
//! binds the inputs this checkout would prove now. Anything else proves in full.

use std::path::Path;
use std::process::Command;

use crate::Fail;
use serde_json::Value;

/// Where `just app` keeps a proof, and where bootstrap.yml caches it from.
const KEPT: &str = ".prism-proof";
/// The three files `prismpm verify` publishes under `verified/<attestation-id>/`.
const EVIDENCE: [&str; 3] = [
    "manifest.json",
    "lexlean-attestation.json",
    "application-acceptance.json",
];
/// Every checked-in byte the proof reads, as bootstrap.yml's cache key names it.
const INPUT_PATHS: &str = "app :(exclude)app/specs prismpm.lock Justfile rust-toolchain.toml";

/// A kept proof, as read back from `.prism-proof`.
#[derive(Clone)]
struct Kept {
    attestation: String,
    manifest: Vec<u8>,
    lexlean: Vec<u8>,
    acceptance: Vec<u8>,
    inputs: String,
}

/// What this checkout and this SDK image would prove now.
#[derive(Clone)]
#[cfg_attr(test, derive(serde::Deserialize))]
struct Observed {
    inputs: String,
    source_id: String,
    semantic_id: String,
    lean_sha256: String,
    lake_sha256: String,
    arch: String,
    /// `(path, sha256)` of each Lake workspace file the attestation names, read from `app/`.
    workspace: Vec<(String, String)>,
}

use repo_model::artifact::{file_sha256, sha256_hex as hex};
use std::fmt::Write as _;

fn text<'a>(value: &'a Value, pointer: &str) -> Result<&'a str, String> {
    value
        .pointer(pointer)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("the attestation has no {pointer}"))
}

fn same(what: &str, kept: &str, observed: &str) -> Result<(), String> {
    if kept == observed {
        Ok(())
    } else {
        Err(format!(
            "{what} differs: the kept proof has {kept}, this run has {observed}"
        ))
    }
}

fn workspace_rows(lexlean: &Value) -> Result<&Vec<Value>, String> {
    lexlean
        .get("lake_workspace")
        .and_then(Value::as_array)
        .ok_or_else(|| "the attestation names no Lake workspace".into())
}

fn workspace_path(row: &Value) -> Result<&str, String> {
    let path = text(row, "/path")?;
    if path.is_empty() || path.starts_with('/') || path.split('/').any(|part| part == "..") {
        return Err(format!(
            "the attestation names a Lake file outside app/: {path}"
        ));
    }
    Ok(path)
}

/// The digest the manifest binds for the acceptance document. PrismPM 0.4.0
/// keeps `browser_timing`, a per-run measurement, out of it: the manifest
/// hashes the canonical JSON of the acceptance without that field (sorted
/// keys, no whitespace), so two runs of one build share an attestation ID.
fn acceptance_digest(acceptance: &mut Value) -> Result<String, String> {
    if let Some(object) = acceptance.as_object_mut() {
        object.remove("browser_timing");
    }
    serde_json::to_vec(acceptance)
        .map(|bytes| hex(&bytes))
        .map_err(|error| format!("application-acceptance.json: {error}"))
}

/// The whole read-time check. Pure, so each planted defect is a unit test.
fn binds(
    kept: &Kept,
    observe: impl FnOnce(&Value) -> Result<Observed, String>,
) -> Result<(), String> {
    if !repo_model::artifact::is_lowercase_hex(&kept.attestation, 64) {
        return Err("the kept attestation ID is not a SHA-256".into());
    }
    // The ID resolves: prismpm defines the attestation ID as the SHA-256 of
    // the manifest bytes.
    let address = hex(&kept.manifest);
    same("the manifest's address", &kept.attestation, &address)?;
    let parse = |bytes: &[u8], name: &str| {
        serde_json::from_slice::<Value>(bytes).map_err(|error| format!("{name}: {error}"))
    };
    let manifest = parse(&kept.manifest, "manifest.json")?;
    let lexlean = parse(&kept.lexlean, "lexlean-attestation.json")?;
    let mut acceptance = parse(&kept.acceptance, "application-acceptance.json")?;
    let check_field =
        |doc: &Value, path: &str, expected: &str| same(path, text(doc, path)?, expected);
    let schema = "prismpm/application-verification-manifest/2";
    check_field(&manifest, "/schema", schema)?;
    let lexlean_digest = hex(&kept.lexlean);
    check_field(&manifest, "/lexlean_attestation_sha256", &lexlean_digest)?;
    // PrismPM's own verifier reads the acceptance only in canonical framing.
    if serde_json::to_vec(&acceptance).ok().as_deref() != Some(kept.acceptance.as_slice()) {
        return Err("the application acceptance is not canonical JSON".into());
    }
    let sealed = text(&manifest, "/acceptance_sha256")?;
    let measured = acceptance_digest(&mut acceptance)?;
    same("the application acceptance", sealed, &measured)?;
    for (label, document) in [
        ("the LexLean status", &lexlean),
        ("the acceptance status", &acceptance),
    ] {
        same(label, text(document, "/status")?, "verified")?;
    }
    let model_id = text(&lexlean, "/attestation_id")?;
    check_field(&acceptance, "/lexlean_attestation_id", model_id)?;
    check_field(&acceptance, "/build_id", text(&manifest, "/build_id")?)?;
    let recorded = kept
        .inputs
        .strip_prefix(&format!("attestation {}\n", kept.attestation))
        .ok_or("the kept inputs name another attestation")?;
    // Check the live model and toolchain only once the sealed documents agree.
    let observed = observe(&lexlean)?;
    for (path, expected) in [
        ("/source_id", &observed.source_id),
        ("/semantic_id", &observed.semantic_id),
        ("/toolchain/lean/executable_sha256", &observed.lean_sha256),
        ("/toolchain/lake/executable_sha256", &observed.lake_sha256),
        ("/host/arch", &observed.arch),
    ] {
        check_field(&lexlean, path, expected)?;
    }
    let rows = workspace_rows(&lexlean)?;
    if rows.len() != observed.workspace.len() {
        return Err("the Lake workspace differs".into());
    }
    for (row, (path, sha256)) in rows.iter().zip(&observed.workspace) {
        same("a Lake workspace path", text(row, "/path")?, path)?;
        same(&format!("app/{path}"), text(row, "/sha256")?, sha256)?;
    }
    // Every other proof input: the SDK image, the tools it runs, and every
    // checked-in byte the cache key names.
    if recorded != observed.inputs {
        return Err("the proof inputs differ from the kept proof's".into());
    }
    Ok(())
}

fn run(root: &Path, dir: &str, program: &str, args: &[&str]) -> Result<String, Fail> {
    let output = Command::new(program)
        .args(args)
        .current_dir(root.join(dir))
        .output()
        .map_err(|error| format!("{program}: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "{program} {}: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }
    Ok(String::from_utf8(output.stdout)?)
}

/// The inputs line by line: the SDK image this run is in, the versions of the
/// tools it proves with, the architecture, the SHA-256 of every tracked input
/// file, and any untracked or modified input.
fn inputs(root: &Path) -> Result<String, Fail> {
    let mut out = format!(
        "sdk {}\nprismpm {}lexlean {}arch {}\n",
        std::env::var("SDK_IMAGE").unwrap_or_default(),
        run(root, ".", "prismpm", &["--version"])?,
        run(root, ".", "lexlean", &["--version"])?,
        std::env::consts::ARCH,
    );
    let mut args = vec!["ls-files", "-z", "--"];
    args.extend(INPUT_PATHS.split(' '));
    let files = run(root, ".", "git", &args)?;
    for path in files.split('\0').filter(|path| !path.is_empty()) {
        writeln!(out, "{}  {path}", file_sha256(&root.join(path))?)?;
    }
    let mut args = vec!["status", "--porcelain=v1", "--untracked-files=all", "--"];
    args.extend(INPUT_PATHS.split(' '));
    out.push_str(&run(root, ".", "git", &args)?);
    Ok(out)
}

fn observe(root: &Path, lexlean: &Value) -> Result<Observed, Fail> {
    let check: Value = serde_json::from_str(&run(
        root,
        "app",
        "lexlean",
        &["--diagnostic-format", "json", "check"],
    )?)?;
    if check.get("success") != Some(&Value::Bool(true)) {
        return Err("lexlean check did not succeed".into());
    }
    let prefix = run(root, "app", "lean", &["--print-prefix"])?;
    let bin = Path::new(prefix.trim()).join("bin");
    let mut workspace = Vec::new();
    for row in workspace_rows(lexlean)? {
        let path = workspace_path(row)?;
        let sha256 = file_sha256(&root.join("app").join(path))?;
        workspace.push((path.to_owned(), sha256));
    }
    Ok(Observed {
        inputs: inputs(root)?,
        source_id: text(&check, "/source_id")?.to_owned(),
        semantic_id: text(&check, "/semantic_id")?.to_owned(),
        lean_sha256: file_sha256(&bin.join("lean"))?,
        lake_sha256: file_sha256(&bin.join("lake"))?,
        arch: std::env::consts::ARCH.to_owned(),
        workspace,
    })
}

fn read_kept(root: &Path) -> Result<Kept, Fail> {
    let read = |path: &str| {
        std::fs::read(root.join(KEPT).join(path)).map_err(|error| format!("{KEPT}/{path}: {error}"))
    };
    Ok(Kept {
        attestation: String::from_utf8(read("attestation")?)?
            .trim_end()
            .to_owned(),
        manifest: read("verified/manifest.json")?,
        lexlean: read("verified/lexlean-attestation.json")?,
        acceptance: read("verified/application-acceptance.json")?,
        inputs: String::from_utf8(read("inputs")?)?,
    })
}

fn check(root: &Path) -> Result<String, Fail> {
    let kept = read_kept(root)?;
    binds(&kept, |lexlean| {
        observe(root, lexlean).map_err(|e| e.to_string())
    })?;
    Ok(kept.attestation)
}

/// `cargo xtask proof-reuse`: succeeds only when the kept proof binds these
/// exact inputs. Any failure, including no kept proof, means: prove in full.
pub fn reuse(root: &Path) -> Result<(), Fail> {
    match check(root) {
        Ok(attestation) => {
            println!(
                "app: reused attestation {attestation}: it resolves and binds these exact proof inputs"
            );
            Ok(())
        }
        Err(error) => {
            println!("app: proving in full, the kept proof is not reusable: {error}");
            Err(error)
        }
    }
}

/// `cargo xtask proof-keep <attestation-id>`: keep the proof `prismpm verify`
/// just published, with the inputs it was proved from, then read it back
/// through the same check a later run makes.
pub fn keep(root: &Path, attestation: &str) -> Result<(), Fail> {
    if !repo_model::artifact::is_lowercase_hex(attestation, 64) {
        return Err(format!("prismpm verify printed no attestation ID: {attestation:?}").into());
    }
    let published = root.join("app/.prism/verified").join(attestation);
    let kept = root.join(KEPT);
    if kept.exists() {
        std::fs::remove_dir_all(&kept)?;
    }
    std::fs::create_dir_all(kept.join("verified"))?;
    for name in EVIDENCE {
        std::fs::copy(published.join(name), kept.join("verified").join(name))
            .map_err(|error| format!("{}: {error}", published.join(name).display()))?;
    }
    std::fs::write(
        kept.join("inputs"),
        format!("attestation {attestation}\n{}", inputs(root)?),
    )?;
    std::fs::write(kept.join("attestation"), format!("{attestation}\n"))?;
    check(root).map_err(|error| format!("the proof just kept does not read back: {error}"))?;
    println!("app: kept attestation {attestation} with the inputs it binds");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{acceptance_digest, hex, Kept, Observed};

    fn binds(kept: &Kept, observed: &Observed) -> Result<(), String> {
        super::binds(kept, |_| Ok(observed.clone()))
    }
    use serde_json::json;

    const INPUTS: &str =
        "sdk ghcr.io/uor-foundation/prismpm-sdk-candidate@sha256:cd3b\nprismpm 0.3.0\n";

    fn proof() -> serde_json::Value {
        pinned_fixture!("proof-reuse-documents")
    }

    /// Seals a proof the way `prismpm verify` does: the manifest hashes the
    /// evidence, and the attestation ID is the manifest's SHA-256.
    fn seal(proof: &serde_json::Value) -> Kept {
        let lexlean = serde_json::to_vec(&proof["lexlean"]).unwrap();
        let acceptance = serde_json::to_vec(&proof["acceptance"]).unwrap();
        let manifest = serde_json::to_vec(&json!({
            "acceptance_sha256": acceptance_digest(&mut proof["acceptance"].clone()).unwrap(),
            "build_id": "2".repeat(64),
            "lexlean_attestation_sha256": hex(&lexlean),
            "schema": "prismpm/application-verification-manifest/2"
        }))
        .unwrap();
        let attestation = hex(&manifest);
        Kept {
            inputs: format!("attestation {attestation}\n{INPUTS}"),
            attestation,
            manifest,
            lexlean,
            acceptance,
        }
    }

    static OBSERVATIONS: std::sync::LazyLock<serde_json::Value> =
        std::sync::LazyLock::new(|| pinned_fixture!("proof-reuse-observations"));
    fn observed() -> Observed {
        serde_json::from_value(OBSERVATIONS["baseline"].clone()).unwrap()
    }

    #[test]
    fn a_proof_of_these_exact_inputs_is_reused() {
        assert_eq!(binds(&seal(&proof()), &observed()), Ok(()));
    }

    /// PrismPM 0.4.0 publishes per-run `browser_timing` in the acceptance and
    /// leaves it out of the manifest's digest: such a proof still reads back,
    /// and any other change to the acceptance still does not.
    #[test]
    fn browser_timing_is_outside_the_acceptance_digest() {
        let mut timed = proof();
        timed["acceptance"]["browser_timing"] = json!({"engine": "chromium", "vectors": []});
        let kept = seal(&timed);
        assert_ne!(
            hex(&kept.acceptance),
            acceptance_digest(&mut timed["acceptance"].clone()).unwrap()
        );
        assert_eq!(binds(&kept, &observed()), Ok(()));

        let mut changed = seal(&timed);
        let mut acceptance: serde_json::Value =
            serde_json::from_slice(&changed.acceptance).unwrap();
        acceptance["build_id"] = json!("3".repeat(64));
        changed.acceptance = serde_json::to_vec(&acceptance).unwrap();
        assert!(binds(&changed, &observed())
            .unwrap_err()
            .starts_with("the application acceptance differs"));
    }

    #[test]
    fn a_kept_proof_that_does_not_bind_these_inputs_is_proved_again() {
        let baseline = seal(&proof());
        let now = observed();

        // Damage sealed evidence without updating the manifest.
        type Mutation<T> = (&'static str, fn(&mut T));
        let corrupt_kept: &[Mutation<Kept>] = &[
            ("attestation ID", |k| k.attestation = "f".repeat(64)),
            ("inputs name another attestation", |k| {
                k.inputs.insert(0, '!')
            }),
            ("manifest", |k| k.manifest.push(b' ')),
            ("LexLean attestation", |k| k.lexlean.push(b' ')),
            ("application acceptance", |k| k.acceptance.push(b' ')),
        ];
        for (defect, corrupt) in corrupt_kept {
            let mut kept = baseline.clone();
            corrupt(&mut kept);
            assert!(super::binds(&kept, |_| panic!(
                "observed checkout after invalid {defect}"
            ))
            .is_err());
        }

        // Seal a consistent hash over documents that disagree about their verdict or identity.
        for (document, key, value) in [
            ("lexlean", "status", "failed"),
            ("acceptance", "status", "failed"),
            ("acceptance", "lexlean_attestation_id", &"3".repeat(64)),
        ] {
            let mut changed = proof();
            changed[document][key] = json!(value);
            assert!(binds(&seal(&changed), &now).is_err(), "{document}/{key}");
        }

        let changes = OBSERVATIONS["changes"].as_object().unwrap();
        assert_eq!(changes.len(), 8);
        for (defect, fields) in changes {
            let mut changed = OBSERVATIONS["baseline"].as_object().unwrap().clone();
            changed.extend(fields.as_object().unwrap().clone());
            let observation = serde_json::from_value(changed.into()).unwrap();
            assert!(
                binds(&baseline, &observation).is_err(),
                "{defect} was reused"
            );
        }
    }

    #[test]
    fn a_lake_file_outside_the_application_is_never_read() {
        assert!(super::workspace_path(&proof()["lexlean"]["lake_workspace"][0]).is_ok());
        for path in ["../Justfile", "/etc/passwd", "a/../../b", ""] {
            assert!(
                super::workspace_path(&json!({"path":path})).is_err(),
                "{path}"
            );
        }
    }
}
