use rstest::rstest;

use crate::increment::*;

#[rstest(
    commit_message,
    case("fix: __proto__ will now be replaced with ___proto___ in parse (#1591)"),
    case("fix(deps): fix enumeration for normalized path arguments (#1567)\n\n")
)]
fn test_is_patch_increment(commit_message: &str) {
    assert_eq!(true, is_patch_increment(commit_message));
}

#[rstest(
    commit_message,
    case("FIX: __proto__ will now be replaced with ___proto___ in parse (#1591)")
)]
fn test_is_patch_increment_case_sensitivity(commit_message: &str) {
    assert_eq!(true, is_patch_increment(commit_message));
}

#[rstest(
    commit_message,
    case("fix(deps-updated): fix enumeration for normalized path arguments (#1567)\n\n",),
    case("fix(deps updated): fix enumeration for normalized path arguments (#1567)\n\n"),
    case("fix(deps_updated): fix enumeration for normalized path arguments (#1567)\n\n")
)]
fn test_is_patch_increment_invalid_scope(commit_message: &str) {
    assert_eq!(true, is_patch_increment(commit_message));
}

#[rstest(
    commit_message,
    case("feat(deps)!: pull in yargs-parser@17.0.0 (#1553)\n\n"),
    case("chore(deps): update dependency which to v2\n\n"),
    case("docs: state limitations of using command handlers returning promises\n\n")
)]
fn test_is_not_patch_increment(commit_message: &str) {
    assert_eq!(false, is_patch_increment(commit_message));
}

#[rstest(
    commit_message,
    case("  fix: __proto__ will now be replaced with ___proto___ in parse (#1591)"),
    case("\tfix(deps): fix enumeration for normalized path arguments (#1567)\n\n")
)]
fn test_is_patch_increment_preceding_whitespace(commit_message: &str) {
    assert_eq!(true, is_patch_increment(commit_message));
}

#[rstest(
    commit_message,
    case(" feat(deps)!: pull in yargs-parser@17.0.0 (#1553)\n\n"),
    case("   chore(deps): update dependency which to v2\n\n"),
    case("\tdocs: state limitations of using command handlers returning promises\n\n")
)]
fn test_is_not_patch_increment_preceding_whitespace(commit_message: &str) {
    assert_eq!(false, is_patch_increment(commit_message));
}

#[rstest(
    commit_message,
    case("fix(  ): __proto__ will now be replaced with ___proto___ in parse (#1591)"),
    case("fix(): fix enumeration for normalized path arguments (#1567)\n\n")
)]
fn test_is_patch_increment_empty_scope(commit_message: &str) {
    assert_eq!(true, is_patch_increment(commit_message));
}

#[rstest(
    commit_message,
    case("feat()!: pull in yargs-parser@17.0.0 (#1553)\n\n"),
    case("chore(\t): update dependency which to v2\n\n"),
    case("docs(  ): state limitations of using command handlers returning promises\n\n")
)]
fn test_is_not_patch_increment_empty_scope(commit_message: &str) {
    assert_eq!(false, is_patch_increment(commit_message));
}
