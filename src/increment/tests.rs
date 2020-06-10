use super::*;

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

#[rstest(
    commit_message,
    expected,
    case("feat: add usage for single-digit boolean aliases (#1580)\n\n", true),
    case("FEAT: add usage for single-digit boolean aliases (#1580)\n\n", true),
    case("feat(completion): takes negated flags into account when", true),
    case(
        "fix(strict mode): report default command unknown arguments (#1626)",
        false
    ),
    case("chore: release 15.2.0 (#1558)", false),
    case("feat(deps)!: pull in yargs-parser@17.0.0 (#1553)", false),
    case("feat(yargs-parser): introduce single-digit boolean aliases (#1576)\n\n", true),
    case("feat(yargs parser): introduce single-digit boolean aliases (#1576)\n\n", true),
    case("feat(yargs_parser): introduce single-digit boolean aliases (#1576)\n\n", true),
)]
fn test_is_minor_increment(commit_message: &str, expected: bool) {
    assert_eq!(expected, is_minor_increment(commit_message));
}

#[rstest(
    commit_message,
    expected,
    case(
        "fix: __proto__ will now be replaced with ___proto___ in parse (#1591)",
        true
    ),
    case(
        "FIX: __proto__ will now be replaced with ___proto___ in parse (#1591)",
        true
    ),
    case(
        "fix(deps): fix enumeration for normalized path arguments (#1567)\n\n",
        true
    ),
    case("feat(deps)!: pull in yargs-parser@17.0.0 (#1553)\n\n", false),
    case("chore(deps): update dependency which to v2\n\n", false),
    case(
        "docs: state limitations of using command handlers returning promises\n\n",
        false
    ),
    case(
        "fix(deps-updated): fix enumeration for normalized path arguments (#1567)\n\n",
        true
    ),
    case(
        "fix(deps updated): fix enumeration for normalized path arguments (#1567)\n\n",
        true
    ),
    case(
        "fix(deps_updated): fix enumeration for normalized path arguments (#1567)\n\n",
        true
    ),

)]
fn test_is_patch_increment(commit_message: &str, expected: bool) {
    assert_eq!(expected, is_patch_increment(commit_message));
}
