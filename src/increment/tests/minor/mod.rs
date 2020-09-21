use crate::increment::*;
use rstest::rstest;

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
    case(
        "feat(yargs-parser): introduce single-digit boolean aliases (#1576)\n\n",
        true
    ),
    case(
        "feat(yargs parser): introduce single-digit boolean aliases (#1576)\n\n",
        true
    ),
    case(
        "feat(yargs_parser): introduce single-digit boolean aliases (#1576)\n\n",
        true
    )
)]
fn test_is_minor_increment(commit_message: &str, expected: bool) {
    assert_eq!(expected, is_minor_increment(commit_message));
}
