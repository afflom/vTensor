//! Repository gates.
//!
//! `cargo xtask <task>`; `just vv` runs the whole normative acceptance gate.
//! Each task below enforces one of the rules `AGENTS.md` sets out, and each
//! names the rule it enforces when it fails, so that a red gate says *which
//! promise* was broken rather than merely that something is wrong.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use repo_model::{codegen, Model};

mod audit;
mod bootstrap;
mod proof;
mod scaffold;

fn main() -> ExitCode {
    let task = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "help".to_string());
    let write = std::env::args().nth(2).is_some_and(|a| a == "--write");
    let root = repo_model::repo_root();

    let result = match task.as_str() {
        "check-model" => check_model(&root, write).map(drop),
        "audit-limits" => audit::audit_limits(&root),
        "audit-deferral" => audit::audit_deferral(&root),
        "audit-bootstrap" => bootstrap::audit(&root),
        "audit-dependency-updates" => bootstrap::audit_dependency_updates(&root),
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
    bootstrap::audit(root)?;
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

    use repo_model::artifact::sha256_hex as hex;

    use super::*;

    /// The checksum a directory artifact records: SHA-256 over one
    /// `<sha256 of file>  <path>` line per file, in byte order of the paths.
    fn manifest(files: &[(&str, &str)]) -> String {
        let mut files = files.to_vec();
        files.sort();
        let lines: String = files
            .iter()
            .map(|(path, text)| format!("{}  {path}\n", hex(text.as_bytes())))
            .collect();
        hex(lines.as_bytes())
    }

    const CORPUS: [(&str, &str); 3] = [("a.json", "[]"), ("n_b.json", "[1,"), ("sub/c.json", "{}")];

    use super::test_repo::TestRepo;

    impl TestRepo {
        /// A repository whose one authority names `artifact` and records
        /// `checksum`, with `CORPUS` committed under `data/corpus`.
        fn artifact(name: &str, artifact: Option<&str>, checksum: &str) -> Self {
            let fixture = Self::new(name);
            for (path, text) in CORPUS {
                fixture.write(format!("data/corpus/{path}"), text);
            }
            fixture.write("data/single.txt", "one file");
            let artifact_line = artifact
                .map(|a| format!("artifact = \"{a}\"\n"))
                .unwrap_or_default();
            let reason = if checksum == "none" {
                "checksum_reason = \"nothing is committed\"\n"
            } else {
                ""
            };
            fixture.registers(
                "id = []",
                &format!(
                    "[[authority]]\nid = \"A-CORPUS\"\nname = \"A corpus\"\n\
                     citation = \"Somebody, https://example.invalid/corpus\"\n\
                     checksum = \"{checksum}\"\n{reason}{artifact_line}\
                     statement = \"What the corpus says.\"\n"
                ),
            );
            let model = Model::load(&fixture.root.join("model")).expect("the fixture model loads");
            fixture.write("CONFORMANCE.md", codegen::render_conformance(&model));
            fixture
        }

        fn check(&self) -> Result<(), String> {
            check_model(&self.root, false)
                .map(drop)
                .map_err(|e| e.to_string())
        }
    }

    #[test]
    fn a_committed_directory_with_its_recorded_checksum_passes() {
        let f = TestRepo::artifact("dir-ok", Some("data/corpus"), &manifest(&CORPUS));
        f.check()
            .expect("the recorded checksum is the computed one");
    }

    #[test]
    fn a_committed_file_with_its_recorded_checksum_passes() {
        let f = TestRepo::artifact("file-ok", Some("data/single.txt"), &hex(b"one file"));
        f.check()
            .expect("a file's checksum is the SHA-256 of its bytes");
    }

    #[test]
    fn a_committed_artifact_with_no_checksum_fails() {
        let f = TestRepo::artifact("no-checksum", Some("data/corpus"), "none");
        test_repo::rejected(
            f.check(),
            &["A-CORPUS", "data/corpus", "records no checksum"],
        );
    }

    #[test]
    fn a_committed_artifact_whose_checksum_does_not_match_fails() {
        let f = TestRepo::artifact("altered", Some("data/corpus"), &manifest(&CORPUS));
        std::fs::write(f.root.join("data/corpus/a.json"), "[ ]").expect("alter a file");
        test_repo::rejected(
            f.check(),
            &["A-CORPUS", "does not match", "recorded", "computed"],
        );
    }

    #[test]
    fn a_file_added_to_or_removed_from_the_artifact_changes_its_checksum() {
        let f = TestRepo::artifact("added", Some("data/corpus"), &manifest(&CORPUS));
        std::fs::write(f.root.join("data/corpus/extra.json"), "1").expect("add a file");
        test_repo::rejected(f.check(), &["A-CORPUS", "does not match"]);
        let g = TestRepo::artifact("removed", Some("data/corpus"), &manifest(&CORPUS));
        std::fs::remove_file(g.root.join("data/corpus/sub/c.json")).expect("remove a file");
        test_repo::rejected(g.check(), &["A-CORPUS", "does not match"]);
    }

    #[test]
    fn a_checksum_with_no_artifact_to_check_fails() {
        let f = TestRepo::artifact("unchecked", None, &manifest(&CORPUS));
        test_repo::rejected(f.check(), &["A-CORPUS", "names no `artifact`"]);
    }

    #[test]
    fn an_artifact_that_is_not_committed_fails() {
        let f = TestRepo::artifact("missing", Some("data/absent"), &manifest(&CORPUS));
        test_repo::rejected(f.check(), &["A-CORPUS", "data/absent", "is not committed"]);
    }

    #[test]
    fn an_artifact_outside_the_repository_fails() {
        for path in ["../outside", "/etc/hostname", "data/../../outside"] {
            let f = TestRepo::artifact("outside", Some(path), &manifest(&CORPUS));
            test_repo::rejected(
                f.check(),
                &["A-CORPUS", "relative path inside the repository"],
            );
        }
    }

    #[test]
    fn a_checksum_that_is_not_a_sha256_fails() {
        for bad in ["abc", &manifest(&CORPUS).to_uppercase(), "sha256:deadbeef"] {
            let f = TestRepo::artifact("not-hex", Some("data/corpus"), bad);
            test_repo::rejected(f.check(), &["A-CORPUS", "64 lowercase hex digits"]);
        }
    }

    #[test]
    fn an_artifact_with_nothing_in_it_fails() {
        let f = TestRepo::artifact("empty", Some("data/empty"), &hex(b""));
        std::fs::create_dir_all(f.root.join("data/empty")).expect("mkdir");
        test_repo::rejected(f.check(), &["A-CORPUS", "data/empty", "no files"]);
    }

    #[test]
    fn the_template_itself_passes() {
        check_model(&repo_model::repo_root(), false).expect("the template cites no artifact");
    }
}
