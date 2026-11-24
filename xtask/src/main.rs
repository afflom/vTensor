//! Repository gates.
//!
//! `cargo xtask <task>`; `just vv` runs the whole normative acceptance gate.
//! Each task below enforces one of the rules `AGENTS.md` sets out, and each
//! names the rule it enforces when it fails, so that a red gate says *which
//! promise* was broken rather than merely that something is wrong.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use repo_model::{codegen, Model};

// A compiled fixture carries its reviewed digest, including in focused gate runs.
macro_rules! pinned_fixture {
    ($name:literal, $digest:literal) => {{
        let bytes = include_bytes!(concat!("../../verification/", $name, ".json"));
        assert_eq!(repo_model::artifact::sha256_hex(bytes), $digest, $name);
        serde_json::from_slice(bytes).expect($name)
    }};
}

mod audit;
mod bootstrap;
mod proof;
mod scaffold;

fn with_model(root: &Path, gate: fn(&Path, &Model) -> Result<(), Fail>) -> Result<(), Fail> {
    gate(root, &Model::load(&root.join("model"))?)
}

fn main() -> ExitCode {
    let task = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "help".to_string());
    let write = std::env::args().nth(2).is_some_and(|a| a == "--write");
    let root = repo_model::repo_root();

    let result = match task.as_str() {
        "check-model" => check_model(&root, write).map(drop),
        "audit-limits" => with_model(&root, audit::check_limits),
        "audit-deferral" => audit::audit_deferral(&root),
        "audit-bootstrap" => with_model(&root, bootstrap::audit),
        "audit-dependency-updates" => with_model(&root, bootstrap::audit_dependency_updates),
        "audit-prism" => audit::prism::audit(&root),
        "validate" => validate(&root),
        "proof-reuse" => proof::reuse(&root),
        "proof-keep" => proof::keep(&root, &std::env::args().nth(2).unwrap_or_default()),
        "new-capability" => {
            let args: Vec<String> = std::env::args().skip(2).collect();
            scaffold::new_capability(&root, &args)
        }
        _ => {
            eprintln!(
                "cargo xtask <task>\n\
                 \n\
                 check-model       R1: model/*.toml is the single source; regenerate and diff\n\
                 audit-limits      R5: a shipped crate names only error types the register declares, each with a negative test\n\
                 audit-deferral    R4: no deferral marker, no stub, no capability behind a flag\n\
                 audit-bootstrap   immutable SDK and least-privilege workflow trust root\n\
                 audit-dependency-updates   SDK ownership and ordinary dependency maintenance\n\
                 audit-prism       only Prism is implemented: non-Prism code may only shrink\n\
                 validate          run every gate above\n\
                 proof-reuse       succeed only if the kept proof binds these exact proof inputs\n\
                 proof-keep <ID>   keep the proof prismpm verify just published, with its inputs\n\
                 new-capability <ID> <level> <suite> <statement>\n\
                                   R3: write the register row, scenario and failing test\n\
                 \n\
                 --write           check-model only: rewrite the generated file"
            );
            return ExitCode::from(2);
        }
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("gate failed: {e}");
            ExitCode::FAILURE
        }
    }
}

/// A gate failure, reported with the rule it broke.
type Fail = Box<dyn std::error::Error>;

/// R1, `CM-01`: the generated Rust consts equal the model, numeral for
/// numeral.
fn check_model(root: &Path, write: bool) -> Result<Model, Fail> {
    let model = Model::load(&root.join("model"))?;
    model.check()?;
    model.check_artifacts(root)?;

    let conformance = codegen::render_conformance(&model);
    let conformance_path: PathBuf = root.join(codegen::CONFORMANCE_PATH);

    if write {
        std::fs::write(&conformance_path, &conformance)?;
        println!("wrote {}", conformance_path.display());
        return Ok(model);
    }

    let committed = std::fs::read_to_string(&conformance_path).map_err(|e| {
        format!(
            "{}: {e}\nrun `cargo xtask check-model --write`",
            conformance_path.display()
        )
    })?;
    if committed != conformance {
        return Err(format!(
            "{} is stale: it disagrees with model/ids.toml.\n\
             R2: a claim cannot exist in the documentation without a ledger row. \
             Run `cargo xtask check-model --write`.",
            conformance_path.display()
        )
        .into());
    }
    println!(
        "check-model: CONFORMANCE.md equals the model, {} ids (CM-01)",
        model.ids.id.len()
    );
    Ok(model)
}

/// The whole normative acceptance gate, in one place.
fn validate(root: &Path) -> Result<(), Fail> {
    let model = check_model(root, false)?;
    audit::check_limits(root, &model)?;
    audit::audit_deferral(root)?;
    bootstrap::audit(root, &model)?;
    audit::prism::audit(root)?;
    println!("validate: every gate passed");
    Ok(())
}

#[cfg(test)]
mod test_repo {
    use std::{
        fs,
        io::ErrorKind,
        path::{Path, PathBuf},
        sync::atomic::{AtomicU64, Ordering},
    };

    pub struct TestRepo {
        pub root: PathBuf,
    }

    impl TestRepo {
        pub fn new(name: &str) -> Self {
            static NEXT: AtomicU64 = AtomicU64::new(0);
            loop {
                let id = NEXT.fetch_add(1, Ordering::Relaxed);
                let root = std::env::temp_dir()
                    .join(format!("vtensor-xtask-{}-{name}-{id}", std::process::id()));
                match fs::create_dir(&root) {
                    Ok(()) => return Self { root },
                    Err(e) if e.kind() == ErrorKind::AlreadyExists => continue,
                    Err(e) => panic!("{}: {e}", root.display()),
                }
            }
        }

        pub fn write(&self, relative: impl AsRef<Path>, contents: impl AsRef<[u8]>) {
            let path = self.root.join(relative);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, contents).unwrap();
        }

        pub fn registers(&self, ids: &str, authorities: &str) {
            for (name, rows) in [
                ("ids", ids),
                ("ledger", "claim = []"),
                ("authorities", authorities),
            ] {
                self.write(
                    format!("model/{name}.toml"),
                    format!("spec = \"template/1\"\n{rows}\n"),
                );
            }
        }
    }

    impl Drop for TestRepo {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }

    #[derive(serde::Deserialize)]
    struct RepositoryCase {
        files: std::collections::BTreeMap<String, String>,
        #[serde(default)]
        dirs: Vec<String>,
        expect: Vec<String>,
    }

    #[derive(serde::Deserialize)]
    struct Suite {
        base: std::collections::BTreeMap<String, String>,
        cases: std::collections::BTreeMap<String, Vec<RepositoryCase>>,
    }

    pub fn replay(
        suite: &str,
        name: &str,
        count: usize,
        check: impl Fn(&TestRepo) -> Result<(), super::Fail>,
    ) {
        static SUITES: std::sync::LazyLock<std::collections::BTreeMap<String, Suite>> =
            std::sync::LazyLock::new(|| {
                pinned_fixture!(
                    "repository-gate-fixtures",
                    "c72231192e8d2e034efef570ddd3f719d54a0562815a58b924e202f5034ba8d5"
                )
            });
        let suite = &SUITES[suite];
        let cases = &suite.cases[name];
        assert_eq!(cases.len(), count);
        for case in cases {
            let repo = TestRepo::new(name);
            for (path, text) in suite.base.iter().chain(&case.files) {
                repo.write(path, text);
            }
            for path in &case.dirs {
                fs::create_dir_all(repo.root.join(path)).unwrap();
            }
            let observed = check(&repo);
            if case.expect.is_empty() {
                observed.unwrap();
            } else {
                let errors: Vec<_> = case.expect.iter().map(String::as_str).collect();
                rejected(observed.map_err(|e| e.to_string()), &errors);
            }
        }
    }

    macro_rules! repository_cases {
        ($suite:literal, $gate:expr; $( $name:ident => $count:literal ),+ $(,)?) => { $(
            #[test]
            fn $name() { crate::test_repo::replay($suite,stringify!($name),$count,$gate); }
        )+ };
    }
    pub(crate) use repository_cases;

    pub fn rejected(result: Result<(), String>, needles: &[&str]) {
        let message = result.expect_err("the gate must reject this fixture");
        for needle in needles {
            assert!(
                message.contains(needle),
                "missing `{needle}` in diagnostic:\n{message}"
            );
        }
    }
}

#[cfg(test)]
mod tests {
    //! `check-model` and an authority's committed artifact. Each fixture is a
    //! throwaway repository root with its own register and one cited authority;
    //! the template's `authorities.toml` is empty, and a row in it would be
    //! copied into every repository cut from the template.

    use super::*;

    fn check_authority_artifact(repo: &test_repo::TestRepo) -> Result<(), Fail> {
        let model = Model::load(&repo.root.join("model"))?;
        repo.write("CONFORMANCE.md", codegen::render_conformance(&model));
        check_model(&repo.root, false).map(drop)
    }

    test_repo::repository_cases! {
        "artifacts", check_authority_artifact;
        a_checksum_that_is_not_a_sha256_fails => 3,
        a_checksum_with_no_artifact_to_check_fails => 1,
        a_committed_artifact_whose_checksum_does_not_match_fails => 1,
        a_committed_artifact_with_no_checksum_fails => 1,
        a_committed_directory_with_its_recorded_checksum_passes => 1,
        a_committed_file_with_its_recorded_checksum_passes => 1,
        a_file_added_to_or_removed_from_the_artifact_changes_its_checksum => 2,
        an_artifact_outside_the_repository_fails => 3,
        an_artifact_that_is_not_committed_fails => 1,
        an_artifact_with_nothing_in_it_fails => 1,
    }

    #[test]
    fn the_template_itself_passes() {
        check_model(&repo_model::repo_root(), false).expect("the template cites no artifact");
    }
}
