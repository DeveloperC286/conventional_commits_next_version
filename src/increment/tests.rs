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
    case("feat(deps)!: pull in yargs-parser@17.0.0 (#1553)", false)
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
    )
)]
fn test_is_patch_increment(commit_message: &str, expected: bool) {
    assert_eq!(expected, is_patch_increment(commit_message));
}

#[rstest(
    commit_messages,
    semantic_version,
    expected_next_version,
    case(
        vec![
            "feat(completion): takes negated flags into account when".to_string(),
            "fix: __proto__ will now be replaced with ___proto___ in parse (#1591)".to_string(),
        ],
        "0.3.2",
        "0.4.1"
    ),
    case(
        vec![
            "feat: complete short options with a single dash (#1507)\n\n".to_string(),
            "fix(docs): use recommended cjs import syntax for ts examples (#1513)".to_string(),
            "feat: display appropriate $0 for electron apps (#1536)\n\n".to_string(),
        ],
        "1.5.9",
        "1.7.0"
    ),
    case(
        vec![
            "fix: temporary fix for libraries that call Object.freeze() (#1483)\n\n".to_string(),
            "fix(docs): update boolean description and examples in docs (#1474)\n\n".to_string(),
            "feat: expose `Parser` from `require('yargs/yargs')` (#1477)\n\n".to_string(),
            "feat(deps)!: yargs-parser now throws on invalid combinations of config (#1470)\n\n".to_string(),
        ],
        "3.1.4",
        "4.0.0"
    ),
)]
fn test_get_next_version_from_commits_consecutive(
    commit_messages: Vec<String>,
    semantic_version: &str,
    expected_next_version: &str,
) {
    //when
    let returned_next_version = get_next_version_from_commits(
        commit_messages,
        Version::parse(semantic_version).unwrap(),
        false,
    )
    .to_string();

    //then
    assert_eq!(expected_next_version, returned_next_version);
}
