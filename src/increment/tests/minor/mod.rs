use crate::increment::*;
use rstest::rstest;

#[rstest(
    commit_message,
    case("feat: add usage for single-digit boolean aliases (#1580)\n\n"),
    case("feat(completion): takes negated flags into account when")
)]
fn test_is_minor_increment(commit_message: &str) {
    assert_eq!(true, is_minor_increment(commit_message));
}

#[rstest(
    commit_message,
    case("FEAT: add usage for single-digit boolean aliases (#1580)\n\n")
)]
fn test_is_minor_increment_case_sensitivity(commit_message: &str) {
    assert_eq!(true, is_minor_increment(commit_message));
}

#[rstest(
    commit_message,
    case("feat(yargs-parser): introduce single-digit boolean aliases (#1576)\n\n"),
    case("feat(yargs parser): introduce single-digit boolean aliases (#1576)\n\n"),
    case("feat(yargs_parser): introduce single-digit boolean aliases (#1576)\n\n")
)]
fn test_is_minor_increment_invalid_scope(commit_message: &str) {
    assert_eq!(true, is_minor_increment(commit_message));
}

#[rstest(
    commit_message,
    case("fix(strict mode): report default command unknown arguments (#1626)"),
    case("chore: release 15.2.0 (#1558)"),
    case("feat(deps)!: pull in yargs-parser@17.0.0 (#1553)")
)]
fn test_is_not_minor_increment(commit_message: &str) {
    assert_eq!(false, is_minor_increment(commit_message));
}