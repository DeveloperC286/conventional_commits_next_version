use crate::increment::*;
use rstest::rstest;

#[rstest(
    commit_message,
    expected,
    case("fix(default): Remove undocumented alias of default() (#469)\n\nBREAKING CHANGE:\r\n\r\nremoved undocumented `defaults` alias for `default`.\r\n\r\n", true),
    case("feat: apply default builder to command() and apply fail() handlers globally (#583)\n\nBREAKING CHANGE: fail is now applied globally.\r\nBREAKING CHANGE: we now default to an empty builder function when command is executed with no builder.", true),
    case("refactor!: drop support for Node 6", true),
    case("feat(deps)!: pull in yargs-parser@17.0.0 (#1553)\n\n", true),
    case("build!(deps): updating build deps to latest versions", true),
    case("build: switch to action for release-please (#1657)", false),
    case("chore(ts): tsify lib/middleware (#1636)\n\n", false),
    case("fix(i18n): Japanese translation phrasing (#1619)\n\n", false),
    case("feat: add usage for single-digit boolean aliases (#1580)\n\n", false),
    case("feat(yargs-parser)!: introduce single-digit boolean aliases (#1576)\n\n", true),
    case("feat(yargs parser)!: introduce single-digit boolean aliases (#1576)\n\n", true),
    case("feat(yargs_parser)!: introduce single-digit boolean aliases (#1576)\n\n", true),
)]
fn test_is_major_increment(commit_message: &str, expected: bool) {
    assert_eq!(expected, is_major_increment(commit_message));
}
