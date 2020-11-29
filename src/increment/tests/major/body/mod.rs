use rstest::rstest;

use crate::increment::*;

#[rstest(
    commit_message,
    case("refactor(ts)!: ship yargs.d.ts (#1671)\n\nBREAKING CHANGE: yargs now ships with its own types"),
    case("chore: upgrade yargs-parser (#633)\n\n* chore: upgrade yargs-parser\r\nBREAKING CHANGE: coerce is now applied as a final step after other parsing is complete\r\n\r\n* add documentation for breaking changes in yargs-parser@4\r\n\r\n* fix: a few small editing nits\r\n\r\n* fix: bump yargs-parser again\r\n"),
    case("fix!: calling parse multiple times now appropriately maintains state (#1137) (#1369)\n\nBREAKING CHANGE: previously to this fix methods like `yargs.getOptions()` contained the state of the last command to execute."),
    case("fix(default): Remove undocumented alias of default() (#469)\n\nBREAKING CHANGE:\r\n\r\nremoved undocumented `defaults` alias for `default`.\r\n\r\n"),
    case("feat: apply default builder to command() and apply fail() handlers globally (#583)\n\nBREAKING CHANGE: fail is now applied globally.\r\nBREAKING CHANGE: we now default to an empty builder function when command is executed with no builder."),
    case("feat: tweaks to API surface based on user feedback (#1726)\n\nBREAKING CHANGE: tweaks to ESM/Deno API surface: now exports yargs function by default; getProcessArgvWithoutBin becomes hidBin; types now exported for Deno."),
)]
fn test_is_major_footer_increment(commit_message: &str) {
    assert!(is_major_footer_increment(commit_message));
}
