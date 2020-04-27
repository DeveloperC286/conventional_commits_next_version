use super::*;

use rstest::rstest;

#[rstest(
    commit_messages,
    semantic_version,
    expected_next_version,
    case(vec!["feat: add usage for single-digit boolean aliases (#1580)\n\n".to_string()], "1.0.0", "1.1.0"),
    case(vec![" fix: __proto__ will now be replaced with ___proto___ in parse (#1591)".to_string()], "0.3.0", "0.3.1"),
    case(vec![" fix : __proto__ will now be replaced with ___proto___ in parse (#1591)".to_string()], "0.3.2", "0.3.3"),
    case(vec!["FIX: __proto__ will now be replaced with ___proto___ in parse (#1591)".to_string()], "1.1.0", "1.1.1"),
    case(vec!["fix(deps): fix enumeration for normalized path arguments (#1567)\n\n".to_string()], "2.1.2", "2.1.3")
)]
fn test_get_next_version_from_commits(
    commit_messages: Vec<String>,
    semantic_version: &str,
    expected_next_version: &str,
) {
    //when
    let returned_next_version =
        get_next_version_from_commits(commit_messages, Version::parse(semantic_version).unwrap())
            .to_string();

    //then
    assert_eq!(expected_next_version, returned_next_version);
}
