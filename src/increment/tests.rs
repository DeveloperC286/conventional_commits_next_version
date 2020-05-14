use super::*;

use rstest::rstest;

#[rstest(
    commit_messages,
    semantic_version,
    expected_next_version,
    case(vec!["fix(default): Remove undocumented alias of default() (#469)\n\nBREAKING CHANGE:\r\n\r\nremoved undocumented `defaults` alias for `default`.\r\n\r\n".to_string()], "1.2.4", "2.0.0"),
    case(vec!["feat: apply default builder to command() and apply fail() handlers globally (#583)\n\nBREAKING CHANGE: fail is now applied globally.\r\nBREAKING CHANGE: we now default to an empty builder function when command is executed with no builder.".to_string()], "13.1.22", "14.0.0"),
)]
fn test_get_next_version_from_commits_consecutive_footer_major_increment(
    commit_messages: Vec<String>,
    semantic_version: &str,
    expected_next_version: &str,
) {
    assert_expected_returned(commit_messages, semantic_version, expected_next_version);
}

#[rstest(
    commit_messages,
    semantic_version,
    expected_next_version,
    case(vec!["refactor!: drop support for Node 6".to_string()], "0.2.4", "1.0.0"),
    case(vec!["feat(deps)!: pull in yargs-parser@17.0.0 (#1553)\n\n".to_string()], "2.1.10", "3.0.0"),
    case(vec!["build!(deps): updating build deps to latest versions".to_string()], "2.1.10", "3.0.0"),
)]
fn test_get_next_version_from_commits_consecutive_major_increment(
    commit_messages: Vec<String>,
    semantic_version: &str,
    expected_next_version: &str,
) {
    assert_expected_returned(commit_messages, semantic_version, expected_next_version);
}

#[rstest(
    commit_messages,
    semantic_version,
    expected_next_version,
    case(vec!["feat: add usage for single-digit boolean aliases (#1580)\n\n".to_string()], "1.0.0", "1.1.0"),
    case(vec!["FEAT: add usage for single-digit boolean aliases (#1580)\n\n".to_string()], "1.0.0", "1.1.0"),
    case(vec!["feat(completion): takes negated flags into account when".to_string()], "2.2.4", "2.3.0"),
)]
fn test_get_next_version_from_commits_consecutive_minor_increment(
    commit_messages: Vec<String>,
    semantic_version: &str,
    expected_next_version: &str,
) {
    assert_expected_returned(commit_messages, semantic_version, expected_next_version);
}

#[rstest(
    commit_messages,
    semantic_version,
    expected_next_version,
    case(vec!["fix: __proto__ will now be replaced with ___proto___ in parse (#1591)".to_string()], "0.3.2", "0.3.3"),
    case(vec!["FIX: __proto__ will now be replaced with ___proto___ in parse (#1591)".to_string()], "1.1.0", "1.1.1"),
    case(vec!["fix(deps): fix enumeration for normalized path arguments (#1567)\n\n".to_string()], "2.1.2", "2.1.3"),
)]
fn test_get_next_version_from_commits_consecutive_patch_increment(
    commit_messages: Vec<String>,
    semantic_version: &str,
    expected_next_version: &str,
) {
    assert_expected_returned(commit_messages, semantic_version, expected_next_version);
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
    assert_expected_returned(commit_messages, semantic_version, expected_next_version);
}

fn assert_expected_returned(
    commit_messages: Vec<String>,
    semantic_version: &str,
    expected_next_version: &str,
) {
    //when
    let returned_next_version = get_next_version_from_commits_consecutive(
        commit_messages,
        Version::parse(semantic_version).unwrap(),
    )
    .to_string();

    //then
    assert_eq!(expected_next_version, returned_next_version);
}
