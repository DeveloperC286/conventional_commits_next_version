use crate::increment::*;
use rstest::rstest;

#[rstest(
    commit_message,
    case("refactor!: drop support for Node 6"),
    case("feat(deps)!: pull in yargs-parser@17.0.0 (#1553)\n\n")
)]
fn test_is_major_increment(commit_message: &str) {
    assert_eq!(true, is_major_increment(commit_message));
}

#[rstest(
    commit_message,
    case("build!(deps): updating build deps to latest versions")
)]
fn test_is_major_increment_invalid_preceding_exclamation(commit_message: &str) {
    assert_eq!(true, is_major_increment(commit_message));
}

#[rstest(
    commit_message,
    case("feat(yargs-parser)!: introduce single-digit boolean aliases (#1576)\n\n"),
    case("feat(yargs parser)!: introduce single-digit boolean aliases (#1576)\n\n"),
    case("feat(yargs_parser)!: introduce single-digit boolean aliases (#1576)\n\n")
)]
fn test_is_major_increment_invalid_scope(commit_message: &str) {
    assert_eq!(true, is_major_increment(commit_message));
}

#[rstest(
    commit_message,
    case("fix(default): Remove undocumented alias of default() (#469)\n\nBREAKING CHANGE:\r\n\r\nremoved undocumented `defaults` alias for `default`.\r\n\r\n"),
    case("feat: apply default builder to command() and apply fail() handlers globally (#583)\n\nBREAKING CHANGE: fail is now applied globally.\r\nBREAKING CHANGE: we now default to an empty builder function when command is executed with no builder."),
)]
fn test_is_major_increment_footer(commit_message: &str) {
    assert_eq!(true, is_major_increment(commit_message));
}

#[rstest(
    commit_message,
    case("build: switch to action for release-please (#1657)"),
    case("chore(ts): tsify lib/middleware (#1636)\n\n"),
    case("fix(i18n): Japanese translation phrasing (#1619)\n\n"),
    case("feat: add usage for single-digit boolean aliases (#1580)\n\n")
)]
fn test_is_not_major_increment(commit_message: &str) {
    assert_eq!(false, is_major_increment(commit_message));
}

#[rstest(
    commit_message,
    case(" refactor!: drop support for Node 6"),
    case("\tfeat(deps)!: pull in yargs-parser@17.0.0 (#1553)\n\n")
)]
fn test_is_major_increment_preceding_whitespace(commit_message: &str) {
    assert_eq!(true, is_major_increment(commit_message));
}

#[rstest(
    commit_message,
    case("\n\rbuild: switch to action for release-please (#1657)"),
    case(" chore(ts): tsify lib/middleware (#1636)\n\n"),
    case("  fix(i18n): Japanese translation phrasing (#1619)\n\n"),
    case("\tfeat: add usage for single-digit boolean aliases (#1580)\n\n")
)]
fn test_is_not_major_increment_preceding_whitespace(commit_message: &str) {
    assert_eq!(false, is_major_increment(commit_message));
}

#[rstest(
    commit_message,
    case("refactor(  )!: drop support for Node 6"),
    case("feat()!: pull in yargs-parser@17.0.0 (#1553)\n\n")
)]
fn test_is_major_increment_empty_scope(commit_message: &str) {
    assert_eq!(true, is_major_increment(commit_message));
}

#[rstest(
    commit_message,
    case("build( ): switch to action for release-please (#1657)"),
    case("chore(): tsify lib/middleware (#1636)\n\n"),
    case("fix(  ): Japanese translation phrasing (#1619)\n\n")
)]
fn test_is_not_major_increment_empty_scope(commit_message: &str) {
    assert_eq!(false, is_major_increment(commit_message));
}
