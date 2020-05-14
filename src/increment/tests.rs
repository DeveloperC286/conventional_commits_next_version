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
)]
fn test_is_major_increment(commit_message: &str, expected: bool) {
    assert_eq!(expected, is_major_increment(commit_message));
}

#[rstest(
    commit_message,
    expected,
    case("feat: add usage for single-digit boolean aliases (#1580)\n\n", true),
    case("FEAT: add usage for single-digit boolean aliases (#1580)\n\n", true),
    case("feat(completion): takes negated flags into account when", true)
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
    )
)]
fn test_is_patch_increment(commit_message: &str, expected: bool) {
    assert_eq!(expected, is_patch_increment(commit_message));
}

#[rstest(
    commit_messages,
    semantic_version,
    expected_next_version,
    case(vec![
        "feat(completion): takes negated flags into account when".to_string(),
        "fix: __proto__ will now be replaced with ___proto___ in parse (#1591)".to_string(),
    ], "0.3.2", "0.4.1"),
)]
fn test_get_next_version_from_commits_consecutive_consecutive_increments(
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
