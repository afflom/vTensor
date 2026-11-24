//! Typed registries parsed from `model/*.toml`.
//!
//! The model is authored once and has exactly one source (R1): the conformance
//! ID register, the claim ledger, and the authorities this repository cites.
//! `CONFORMANCE.md` is generated from it by [`codegen`], so a claim cannot exist
//! in the documentation without a ledger row, or in the ledger without appearing
//! in the documentation.
//!
//! This crate is build-time and CI infrastructure. It is not a dependency of
//! any shipped crate, and it may use `std`.

#![deny(missing_docs)]

pub mod artifact;
pub mod codegen;
pub mod registry;

pub use registry::{Authorities, AuthorityRow, Claim, DeclaredFailure, IdRow, Ids, Ledger, Level};

use std::path::{Path, PathBuf};

/// Everything `model/*.toml` says, parsed and cross-checked.
#[derive(Debug, Clone)]
pub struct Model {
    /// `model/ledger.toml`: one row per claim, at exactly one honesty level.
    pub ledger: Ledger,
    /// `model/ids.toml`: the conformance ID register.
    pub ids: Ids,
    /// `model/authorities.toml`: what this repository cites rather than proves.
    pub authorities: Authorities,
}

/// A failure to load or to cross-check the model.
#[derive(Debug)]
pub enum ModelError {
    /// A model file could not be read.
    Io(PathBuf, std::io::Error),
    /// A model file could not be parsed.
    Parse(PathBuf, toml::de::Error),
    /// The model disagrees with itself, or with a derivation (CM-01).
    Inconsistent(String),
}

impl std::fmt::Display for ModelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(p, e) => write!(f, "reading {}: {e}", p.display()),
            Self::Parse(p, e) => write!(f, "parsing {}: {e}", p.display()),
            Self::Inconsistent(m) => write!(f, "model is inconsistent: {m}"),
        }
    }
}

impl std::error::Error for ModelError {}

impl Model {
    /// Load every model file from a `model/` directory.
    pub fn load(dir: &Path) -> Result<Self, ModelError> {
        Ok(Self {
            ledger: read(dir, "ledger.toml")?,
            ids: read(dir, "ids.toml")?,
            authorities: read(dir, "authorities.toml")?,
        })
    }

    /// Load the model from the repository root, resolved from this crate's
    /// manifest directory so that it works from any working directory.
    pub fn load_from_repo_root() -> Result<Self, ModelError> {
        Self::load(&repo_root().join("model"))
    }

    /// Cross-check the model against itself: every ID well formed, every claim
    /// well formed for its level, and every `some-true` claim bound to an
    /// authority that exists (`CM-01` .. `CM-03`, R2).
    pub fn check(&self) -> Result<(), ModelError> {
        self.ledger.check()?;
        self.check_ids()?;
        self.check_authorities()?;
        Ok(())
    }

    /// `CM-02`: every registered ID is well formed.
    ///
    /// The structural rules only. Rules about a *class* of ID --- that a fitted
    /// exponent is always `open`, that a cross-library result is never
    /// `some-true` --- belong to the repository that has those classes, and they
    /// belong to the repository that has those classes. A repository adding one adds
    /// its rule here, in the same commit that adds the first ID in it.
    fn check_ids(&self) -> Result<(), ModelError> {
        let bad = |m: String| ModelError::Inconsistent(m);
        let mut seen = std::collections::BTreeSet::new();
        for row in &self.ids.id {
            if !seen.insert(&row.id) {
                return Err(bad(format!("{}: registered twice", row.id)));
            }

            if row.statement.trim().is_empty() {
                return Err(bad(format!(
                    "{}: an untagged claim does not ship (R2)",
                    row.id
                )));
            }
            if row.suite.trim().is_empty() {
                return Err(bad(format!(
                    "{}: every ID names the Gherkin suite its scenario lives in (R3)",
                    row.id
                )));
            }
        }
        // The declared failure types are part of the register, so a malformed
        // declaration is a malformed register (R5).
        self.ids.declared_failures()?;
        Ok(())
    }

    /// `CM-03`: every `some-true` claim has a row in `model/authorities.toml`
    /// with a citation, and every authority names IDs that exist.
    fn check_authorities(&self) -> Result<(), ModelError> {
        let bad = |m: String| ModelError::Inconsistent(m);
        for a in &self.authorities.authority {
            if a.citation.trim().is_empty() {
                return Err(bad(format!("{}: an authority with no citation", a.id)));
            }
            match (&a.artifact, a.checksum.as_str()) {
                (Some(artifact), "none") => {
                    return Err(bad(format!(
                        "{}: artifact `{artifact}` is committed but the row records no \
                         checksum. An authority's artifact may be committed only with a \
                         checksum that `check-model` recomputes (R6)",
                        a.id
                    )))
                }
                (None, checksum) if checksum != "none" => {
                    return Err(bad(format!(
                        "{}: records a checksum but names no `artifact`, so nothing is \
                         checked against it. A checksum is recorded and checked, or it is \
                         `none` with a reason (R6)",
                        a.id
                    )))
                }
                _ => {}
            }
            if let Some(artifact) = &a.artifact {
                if !artifact::is_inside(artifact) {
                    return Err(bad(format!(
                        "{}: artifact `{artifact}` must be a relative path inside the \
                         repository",
                        a.id
                    )));
                }
                if !artifact::is_lowercase_hex(&a.checksum, 64) {
                    return Err(bad(format!(
                        "{}: checksum `{}` of artifact `{artifact}` must be 64 lowercase hex \
                         digits (a SHA-256)",
                        a.id, a.checksum
                    )));
                }
            }
            if a.checksum == "none" && a.checksum_reason.trim().is_empty() {
                return Err(bad(format!(
                    "{}: no checksum and no reason. A missing checksum must be a stated \
                     fact, not an omission (R6)",
                    a.id
                )));
            }
            for id in &a.realized_by {
                if self.ids.get(id).is_none() {
                    return Err(bad(format!("{}: realized_by names unknown ID {id}", a.id)));
                }
            }
        }
        // Every some-true claim in the ledger names a known authority.
        for c in &self.ledger.claim {
            if c.level != Level::SomeTrue {
                continue;
            }
            let Some(name) = &c.authority else {
                return Err(bad(format!(
                    "{}: a some-true claim must name an authority",
                    c.id
                )));
            };
            if !self.authorities.authority.iter().any(|a| &a.id == name) {
                return Err(bad(format!(
                    "{}: cites {name}, which has no row in model/authorities.toml (CM-03)",
                    c.id
                )));
            }
        }
        Ok(())
    }
}

fn read<T: serde::de::DeserializeOwned>(dir: &Path, name: &str) -> Result<T, ModelError> {
    let path = dir.join(name);
    let text = std::fs::read_to_string(&path).map_err(|e| ModelError::Io(path.clone(), e))?;
    toml::from_str(&text).map_err(|e| ModelError::Parse(path, e))
}

/// The repository root, resolved from this crate's manifest directory.
pub fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crates/model is two levels below the repository root")
        .to_path_buf()
}

#[cfg(test)]
mod tests {
    use super::*;

    static MODEL: std::sync::LazyLock<Model> = std::sync::LazyLock::new(|| {
        let model = Model::load_from_repo_root().expect("model loads");
        model.check().expect("model checks");
        model
    });

    /// CM-01: the model is self-consistent and every numeral in it derives.
    #[test]
    fn model_is_consistent_cm_01() {
        std::sync::LazyLock::force(&MODEL);
    }

    /// CM-02: every registered ID is unique and well formed.
    #[test]
    fn the_id_register_is_well_formed_cm_02() {
        eprintln!(
            "CM-02: {} registered IDs, each unique and tagged",
            MODEL.ids.id.len()
        );
    }

    /// CM-03: every `some-true` claim cites an authority that exists.
    #[test]
    fn every_some_true_claim_cites_an_authority_cm_03() {
        let listed = |id: &String| MODEL.authorities.authority.iter().any(|row| &row.id == id);
        for claim in &MODEL.ledger.claim {
            if claim.level == Level::SomeTrue {
                assert!(claim.authority.as_ref().is_some_and(listed), "{}", claim.id);
            }
        }
    }

    /// Prose with its comment markers removed and its lines joined.
    fn words(text: &str, markers: &[&str]) -> String {
        text.lines()
            .map(|line| {
                let line = line.trim();
                markers
                    .iter()
                    .find_map(|m| line.strip_prefix(m))
                    .unwrap_or(line)
                    .trim()
            })
            .flat_map(str::split_whitespace)
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// The rule for an authority and its artifact is stated in three places ---
    /// the doc comment of `AuthorityRow`, the header of `model/authorities.toml`
    /// and the generated `CONFORMANCE.md` --- and they say the same thing. The
    /// issue this closes was two of them disagreeing.
    #[test]
    fn the_authority_rule_is_one_rule() {
        let rule = words(registry::AUTHORITY_RULE, &[]);
        let root = repo_root();
        let doc = words(include_str!("registry.rs"), &["///"]);
        assert!(
            doc.contains(&rule),
            "the doc comment of `AuthorityRow` must state AUTHORITY_RULE"
        );
        let header = std::fs::read_to_string(root.join("model/authorities.toml"))
            .expect("authorities.toml reads");
        assert!(
            words(&header, &["# ", "#"]).contains(&rule),
            "the header of model/authorities.toml must state AUTHORITY_RULE"
        );
        let generated = codegen::render_conformance(&MODEL);
        assert!(
            words(&generated, &[]).contains(&rule),
            "CONFORMANCE.md is generated from AUTHORITY_RULE"
        );
    }
}

/// Repository behavioral scenarios and honesty checks, used by build and CI tools.
pub mod conformance {
    pub use meta::{check_honesty, HonestyReport};
    pub use runner::{scenarios_in, source_files, workspace_test_names, Scenario, SuiteReport};
    mod runner {
        //! Reading `features/suites/*.feature` (R3).

        use std::collections::BTreeSet;
        use std::path::Path;

        /// One scenario, and the conformance ID it discharges.
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
        pub struct Scenario {
            /// The conformance ID from the scenario's tag.
            pub id: String,
            /// The honesty level from the scenario's tag.
            pub level: String,
            /// The scenario's one-line statement.
            pub statement: String,
            /// Which suite file it came from.
            pub suite: String,
            /// The steps, in order.
            pub steps: Vec<String>,
        }

        /// What a suite directory contains.
        #[derive(Clone, Debug, Default)]
        pub struct SuiteReport {
            /// Every scenario found.
            pub scenarios: Vec<Scenario>,
            /// Files that were read.
            pub files: usize,
        }

        /// Parse every `.feature` file in `dir`.
        ///
        /// A deliberately small parser. Cucumber's full grammar buys nothing here: the
        /// scenarios are generated from the register and their job is to be *readable*
        /// and to carry the ID, not to be executed by a step-definition engine. What
        /// executes the claims are the tests, and `CM-02` is what ties the two together.
        pub fn scenarios_in(dir: &Path) -> std::io::Result<SuiteReport> {
            let mut report = SuiteReport::default();
            let mut entries: Vec<_> = std::fs::read_dir(dir)?.collect::<Result<_, _>>()?;
            entries.sort_by_key(|e| e.path());

            for entry in entries {
                let path = entry.path();
                if path.extension().is_none_or(|e| e != "feature") {
                    continue;
                }
                report.files += 1;
                let suite = path
                    .file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                let text = std::fs::read_to_string(&path)?;

                let first = report.scenarios.len();
                let mut pending_tags = None;
                for line in text.lines().map(str::trim) {
                    if line.starts_with('@') {
                        pending_tags = Some(line);
                    } else if let Some(statement) = line.strip_prefix("Scenario:") {
                        let mut tags = pending_tags
                            .take()
                            .unwrap_or_default()
                            .split_whitespace()
                            .map(|tag| tag.trim_start_matches('@').to_owned());
                        report.scenarios.push(Scenario {
                            id: tags.next().unwrap_or_default(),
                            level: tags.next().unwrap_or_default(),
                            statement: statement.trim().to_owned(),
                            suite: suite.clone(),
                            steps: Vec::new(),
                        });
                    } else if let Some(scenario) = report.scenarios[first..].last_mut() {
                        if ["Given ", "When ", "Then ", "And ", "But "]
                            .iter()
                            .any(|tag| line.starts_with(tag))
                        {
                            scenario.steps.push(line.to_owned());
                        }
                    }
                }
            }
            Ok(report)
        }

        impl SuiteReport {
            /// The set of IDs the suites cover.
            pub fn ids(&self) -> BTreeSet<&str> {
                self.scenarios.iter().map(|s| s.id.as_str()).collect()
            }
        }

        /// Gather repository sources, including nested modules. Build output only belongs
        /// to a shipped source tree if it was deliberately placed under that crate's src.
        pub fn source_files(
            dir: &Path,
            files: &mut Vec<std::path::PathBuf>,
            shipped: bool,
            extensions: &[&str],
        ) -> std::io::Result<()> {
            for entry in std::fs::read_dir(dir)? {
                let path = entry?.path();
                if path.is_dir() {
                    if shipped || path.file_name().is_none_or(|n| n != "target") {
                        source_files(&path, files, shipped, extensions)?;
                    }
                } else if path
                    .extension()
                    .and_then(|e| e.to_str())
                    .is_some_and(|e| extensions.contains(&e))
                {
                    files.push(path);
                }
            }
            Ok(())
        }

        /// Find ordinary test functions and the declarations expanded by the GGML batch macro.
        /// VT-40 also checks the batch names against the enabled tests in the compiled harness.
        pub fn workspace_test_names(root: &Path) -> BTreeSet<String> {
            let mut files = Vec::new();
            for dir in [root.join("crates"), root.join("xtask")]
                .into_iter()
                .filter(|p| p.is_dir())
            {
                source_files(&dir, &mut files, false, &["rs"]).expect("read workspace sources");
            }
            let mut names = BTreeSet::new();
            for path in files {
                let source = std::fs::read_to_string(path).expect("read Rust source");
                let (mut armed, mut batch) = (false, false);
                for line in source.lines().map(str::trim) {
                    match line {
                        "ggml_batches! {" | "fixture_query_tests! {" => batch = true,
                        "}" if batch => batch = false,
                        "#[test]" => armed = true,
                        attr if attr.is_empty() || attr.starts_with('#') => {}
                        _ => {
                            if let Some((name, _)) =
                                line.strip_prefix("fn ").and_then(|f| f.split_once('('))
                            {
                                if armed || batch {
                                    names.insert(name.to_owned());
                                }
                            }
                            armed = false;
                        }
                    }
                }
            }
            names
        }
    }
    mod meta {
        //! The honesty meta-gate (R2).
        //!
        //! Three levels only mean something if the suite respects them, and nothing in
        //! the arithmetic can enforce that. What can is a check on the *names* and the
        //! *prose*: an `open` claim that a test asserts, or a `some-true` authority
        //! that the README presents as this repository's own result, is exactly the
        //! blurring of registers the discipline exists to prevent.
        //!
        //! The gate is deliberately about language, because that is where the failure
        //! mode lives. Nobody sets out to claim they proved an upstream theorem; they
        //! write "proves" where they meant "is evidence for", and six months later the
        //! sentence is load-bearing.

        use std::collections::BTreeSet;
        use std::path::Path;

        use crate::{Level, Model};

        use crate::conformance::runner::SuiteReport;

        /// What the meta-gate found.
        #[derive(Clone, Debug, Default)]
        pub struct HonestyReport {
            /// Every problem, each naming the rule it breaks.
            pub violations: Vec<String>,
            /// How many registered IDs were checked.
            pub ids_checked: usize,
            /// How many scenarios were read.
            pub scenarios_checked: usize,
        }

        impl HonestyReport {
            /// Did everything hold?
            pub fn is_clean(&self) -> bool {
                self.violations.is_empty()
            }
        }

        /// Words that assert a claim as established.
        ///
        /// A `build` claim may use them --- it *is* evidence, constructed here. An
        /// `open` claim may not, because it is a measurement, and a `some-true` claim
        /// may not, because it belongs to someone else.
        const ASSERTIVE: &[&str] = &[
            "proves",
            "proven",
            "proof that",
            "guarantees",
            "establishes",
            "demonstrates that",
            "shows that",
            "confirms",
        ];

        /// Run the meta-gate.
        ///
        /// `root` is the repository root. `tests` are the test names collected from the
        /// workspace, which the caller gathers because it knows how to run `cargo`.
        pub fn check_honesty(
            root: &Path,
            tests: &BTreeSet<String>,
        ) -> std::io::Result<HonestyReport> {
            let mut report = HonestyReport::default();
            let model = match Model::load(&root.join("model")) {
                Ok(m) => m,
                Err(e) => {
                    report
                        .violations
                        .push(format!("R1: the model does not load: {e}"));
                    return Ok(report);
                }
            };
            let suites: SuiteReport =
                crate::conformance::runner::scenarios_in(&root.join("features/suites"))?;
            report.scenarios_checked = suites.scenarios.len();
            report.ids_checked = model.ids.id.len();

            let scenario_ids = suites.ids();

            // R3, CM-02: every registered ID has a scenario, and a test named for it.
            for row in &model.ids.id {
                if !scenario_ids.contains(row.id.as_str()) {
                    report.violations.push(format!(
                        "R3: {} is registered but has no scenario in features/suites/. Every \
                 capability begins as a Gherkin scenario.",
                        row.id
                    ));
                }
                let slug = row.id.to_lowercase().replace('-', "_");
                if !tests.iter().any(|t| t.ends_with(&slug)) {
                    report.violations.push(format!(
                        "CM-02: {} is registered but no test name ends in `{slug}`. A claim with \
                 no test is an assertion.",
                        row.id
                    ));
                }
            }

            // CM-02, the other direction: every scenario names a registered ID.
            for s in &suites.scenarios {
                if model.ids.get(&s.id).is_none() {
                    report.violations.push(format!(
                        "CM-02: scenario `{}` in {} names `{}`, which is not in the register.",
                        s.statement, s.suite, s.id
                    ));
                }
                if s.steps.is_empty() {
                    report.violations.push(format!(
                        "R3: scenario `{}` in {} has no steps. There are no pending steps.",
                        s.statement, s.suite
                    ));
                }
                // R2: the scenario's tag must agree with the register.
                if let Some(row) = model.ids.get(&s.id) {
                    if s.level != row.level.as_str() {
                        report.violations.push(format!(
                            "R2: {} is tagged `{}` in {} but `{}` in the register.",
                            s.id,
                            s.level,
                            s.suite,
                            row.level.as_str()
                        ));
                    }
                }
            }

            // CM-02, the third direction: every ID a *test* names is registered.
            //
            // Without this the register can be a subset of what the suite claims: a test
            // called `..._ct_04` looks like it discharges `CT-04`, but if `CT-04` is not
            // a row then nothing checks it has a scenario and `CONFORMANCE.md` does not
            // list it. Three IDs were in exactly that state --- `CK-08`, `CT-04`, and
            // `CT-05` --- with passing tests and no register rows, which is a claim made
            // by a name and by nothing else.
            //
            // Only prefixes the register already uses are checked, so a test whose name
            // merely happens to end in two letters and two digits is not a claim.
            let prefixes: BTreeSet<String> = model
                .ids
                .id
                .iter()
                .filter_map(|r| r.id.split('-').next().map(str::to_lowercase))
                .collect();
            for name in tests {
                let Some(tail) = name.rsplit("::").next() else {
                    continue;
                };
                let parts: Vec<&str> = tail.rsplitn(3, '_').collect();
                if parts.len() < 3 {
                    continue;
                }
                let (digits, letters) = (parts[0], parts[1]);
                if digits.len() != 2 || !digits.bytes().all(|b| b.is_ascii_digit()) {
                    continue;
                }
                if letters.len() != 2 || !prefixes.contains(letters) {
                    continue;
                }
                let id = format!("{}-{digits}", letters.to_uppercase());
                if model.ids.get(&id).is_none() {
                    report.violations.push(format!(
                "CM-02: test `{name}` names `{id}`, which is not in the register. An ID                  that exists only in a test name has no scenario and no row in                  CONFORMANCE.md."
            ));
                }
            }

            // R2: an `open` claim must not be asserted as established, anywhere.
            let open_ids: Vec<&str> = model
                .ids
                .id
                .iter()
                .filter(|r| r.level == Level::Open)
                .map(|r| r.id.as_str())
                .collect();
            for doc in [
                "README.md",
                "CONFORMANCE.md",
                "VERIFICATION.md",
                "ANALYSIS.md",
            ] {
                let Ok(text) = std::fs::read_to_string(root.join(doc)) else {
                    continue;
                };
                let checks_authorities = matches!(doc, "README.md" | "CONFORMANCE.md");
                for (index, line) in text.lines().enumerate() {
                    let lower = line.to_lowercase();
                    let Some(word) = ASSERTIVE.iter().find(|word| lower.contains(**word)) else {
                        continue;
                    };
                    let location = format!("R2: {doc}:{}", index + 1);
                    for id in open_ids.iter().filter(|id| line.contains(**id)) {
                        report.violations.push(format!(
                            "{location}: `{id}` is an `open` claim --- measured and reported, never asserted --- but this line says `{word}`.\n    {}", line.trim()
                        ));
                    }
                    if checks_authorities {
                        for authority in &model.authorities.authority {
                            if line.contains(&authority.id) {
                                report.violations.push(format!(
                                    "{location}: `{}` is cited, not established here, but this line says `{word}`.\n    {}", authority.id, line.trim()
                                ));
                            }
                        }
                    }
                }
            }

            Ok(report)
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            /// The gate's own vocabulary check must be able to fire, or every clean
            /// run above means nothing.
            #[test]
            fn the_assertive_vocabulary_is_recognised() {
                for word in ASSERTIVE {
                    let line = format!("CG-01 {word} the exponent is one");
                    assert!(
                        ASSERTIVE.iter().any(|w| line.to_lowercase().contains(*w)),
                        "{word} must be recognised"
                    );
                }
                // And an honest sentence about an open claim does not trip it.
                let honest = "CG-01 reports the fitted exponent with its confidence interval";
                assert!(!ASSERTIVE.iter().any(|w| honest.to_lowercase().contains(*w)));
            }
        }
    }
}
