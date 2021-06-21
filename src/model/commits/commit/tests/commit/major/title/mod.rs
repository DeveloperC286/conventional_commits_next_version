use super::*;

#[rstest]
#[case("refactor!: drop support for Node 6")]
#[case("feat(deps)!: pull in yargs-parser@17.0.0 (#1553)\n\n")]
fn test_is_major_title_increment(#[case] commit_message: &str) {
    is_only_major_title_increment(commit_message);
}

#[rstest]
#[case("build!(deps): updating build deps to latest versions")]
fn test_is_major_title_increment_invalid_preceding_exclamation(#[case] commit_message: &str) {
    is_only_major_title_increment(commit_message);
}

#[rstest]
#[case("feat(yargs-parser)!: introduce single-digit boolean aliases (#1576)\n\n")]
#[case("feat(yargs parser)!: introduce single-digit boolean aliases (#1576)\n\n")]
#[case("feat(yargs_parser)!: introduce single-digit boolean aliases (#1576)\n\n")]
fn test_is_major_title_increment_invalid_scope(#[case] commit_message: &str) {
    is_only_major_title_increment(commit_message);
}

#[rstest]
#[case(" refactor!: drop support for Node 6")]
#[case("\tfeat(deps)!: pull in yargs-parser@17.0.0 (#1553)\n\n")]
fn test_is_major_title_increment_preceding_whitespace(#[case] commit_message: &str) {
    is_only_major_title_increment(commit_message);
}

#[rstest]
#[case("build: switch to action for release-please (#1657)")]
#[case("chore(ts): tsify lib/middleware (#1636)\n\n")]
#[case("fix(i18n): Japanese translation phrasing (#1619)\n\n")]
#[case("feat: add usage for single-digit boolean aliases (#1580)\n\n")]
fn test_is_not_major_title_increment(#[case] commit_message: &str) {
    is_not_major_title_increment(commit_message);
}

#[rstest]
#[case("\n\rbuild: switch to action for release-please (#1657)")]
#[case(" chore(ts): tsify lib/middleware (#1636)\n\n")]
#[case("  fix(i18n): Japanese translation phrasing (#1619)\n\n")]
#[case("\tfeat: add usage for single-digit boolean aliases (#1580)\n\n")]
fn test_is_not_major_title_increment_preceding_whitespace(#[case] commit_message: &str) {
    is_not_major_title_increment(commit_message);
}

#[rstest]
#[case("refactor(  )!: drop support for Node 6")]
#[case("feat()!: pull in yargs-parser@17.0.0 (#1553)\n\n")]
fn test_is_major_title_increment_empty_scope(#[case] commit_message: &str) {
    is_only_major_title_increment(commit_message);
}

#[rstest]
#[case("build( ): switch to action for release-please (#1657)")]
#[case("chore(): tsify lib/middleware (#1636)\n\n")]
#[case("fix(  ): Japanese translation phrasing (#1619)\n\n")]
fn test_is_not_major_title_increment_empty_scope(#[case] commit_message: &str) {
    is_not_major_title_increment(commit_message);
}
