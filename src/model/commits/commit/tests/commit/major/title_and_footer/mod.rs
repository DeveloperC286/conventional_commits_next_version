use super::*;

#[rstest(
    commit_message,
    case("refactor(ts)!: ship yargs.d.ts (#1671)\n\nBREAKING-CHANGE: yargs now ships with its own types"),
    case("fix!: calling parse multiple times now appropriately maintains state (#1137) (#1369)\n\nBREAKING-CHANGE: previously to this fix methods like `yargs.getOptions()` contained the state of the last command to execute."),
)]
fn test_is_major_title_and_footer_increment(commit_message: &str) {
    is_major_title_and_footer_increment(commit_message);
}

#[rstest(
    commit_message,
    case("refactor(ts)!: ship yargs.d.ts (#1671)\n\nBREAKING CHANGE: yargs now ships with its own types"),
    case("fix!: calling parse multiple times now appropriately maintains state (#1137) (#1369)\n\nBREAKING CHANGE: previously to this fix methods like `yargs.getOptions()` contained the state of the last command to execute."),
)]
fn test_is_major_title_and_footer_increment_synonymous(commit_message: &str) {
    is_major_title_and_footer_increment(commit_message);
}
