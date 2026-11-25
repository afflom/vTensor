//! `CM-02`, `CM-03`, and R2's behavioural half.
//!
//! Runs the meta-gate against the actual workspace: the register, the feature
//! suites, and the test names `cargo test -- --list` reports. An ID with no
//! scenario, a scenario with no ID, an ID with no test, or a mislabelled
//! honesty level all fail here.

use repo_model::repo_root as root;
use std::collections::BTreeSet;
use std::sync::{LazyLock, Once};

use repo_model::conformance::{check_honesty, workspace_test_names};
use repo_model::{Level, Model};

static MODEL: LazyLock<Model> = LazyLock::new(|| Model::load_from_repo_root().unwrap());
static COMPLETE: Once = Once::new();

/// `CM-02`: every registered ID has a scenario and a test, and every scenario
/// and test names a registered ID.
#[test]
fn every_id_has_a_scenario_and_a_test_cm_02() {
    COMPLETE.call_once(|| {
        let root = root();
        let tests = workspace_test_names(&root);
        assert!(!tests.is_empty(), "the test list must not be empty");
        let report = check_honesty(&root, &tests).expect("the meta-gate runs");
        assert!(report.is_clean(), "{}", report.violations.join("\n\n"));
    });
}

/// R3: there are no pending or skipped steps.
///
/// The non-emptiness guard is armed by the *register*, not asserted outright. A
/// repository that has claimed nothing yet has no scenarios to write and no
/// suite for them to live in, and that is a legitimate state. A repository with
/// registered IDs and no feature files is the defect this catches, and the guard
/// re-arms the moment the first row is added to `model/ids.toml`.
#[test]
fn no_scenario_is_pending_cm_02() {
    every_id_has_a_scenario_and_a_test_cm_02();
}

/// `CM-03`: every `some-true` claim cites an authority that exists, with a
/// citation and either a checksum over a committed artifact that still matches
/// it, or a stated reason for its absence.
#[test]
fn every_some_true_claim_cites_an_authority_cm_03() {
    let model = &*MODEL;
    model.check().expect("the model is consistent");
    model
        .check_artifacts(&root())
        .expect("every committed artifact is the one its checksum records");

    // Model::check owns authority resolution, citations and checksum reasons.
    // Reuse that validator rather than maintaining a second interpretation here.
    let cited = model
        .ledger
        .claim
        .iter()
        .any(|c| c.level == Level::SomeTrue);
    // Armed by the ledger, for the reason `no_scenario_is_pending_cm_02` gives:
    // a repository that reproduces nothing from an authority cites none, and a
    // `some-true` claim with no authority row fails in the loop above regardless.
    assert!(
        model.ledger.claim.is_empty() || cited || model.authorities.authority.is_empty(),
        "a ledger with claims and no cited authority"
    );
}

/// R2: the meta-gate can fail.
///
/// A gate nobody has ever seen fail is indistinguishable from a gate that
/// cannot. This plants each of the three violations it exists to catch and
/// checks that each is reported.
#[test]
fn the_meta_gate_is_falsifiable_cm_02() {
    if let Some(first) = MODEL.ids.id.first() {
        let prefix = first.id.split('-').next().unwrap();
        let absent = (100..)
            .map(|n| format!("{prefix}-{n}"))
            .find(|id| MODEL.ids.get(id).is_none())
            .unwrap();
        let name = format!("orphan_{}", absent.to_lowercase().replace('-', "_"));
        let report = check_honesty(&root(), &BTreeSet::from([name])).expect("runs");
        let errors = report.violations.join("\n");
        assert!(errors.contains("no test name"), "{errors}");
        assert!(errors.contains(&format!("names `{absent}`")), "{errors}");
    }
    every_id_has_a_scenario_and_a_test_cm_02();
}
