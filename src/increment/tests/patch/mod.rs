use crate::increment::*;
use rstest::rstest;

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
    )
)]
fn test_is_patch_increment(commit_message: &str, expected: bool) {
    assert_eq!(expected, is_patch_increment(commit_message));
}
