const PRECEDING_WHITESPACE_VARIATIONS: &[&str] = &["  ", " ", "\t", "\n", "\n\r"];
const NON_PRECEDING_WHITESPACE_VARIATIONS: &[&str] = &[""];

pub fn get_preceding_whitespace_variations(
    should_generate_preceding_whitespace: bool,
) -> &'static [&'static str] {
    match should_generate_preceding_whitespace {
        true => PRECEDING_WHITESPACE_VARIATIONS,
        false => NON_PRECEDING_WHITESPACE_VARIATIONS,
    }
}

pub const MAJOR_TITLE_COMMIT_TYPE_VARIATIONS: &[&str] =
    &["feat!", "fix!", "FIX!", "Build!", "Feat!"];

pub const MINOR_COMMIT_TYPE_VARIATIONS: &[&str] = &["feat", "FEAT", "Feat"];

pub const PATCH_COMMIT_TYPE_VARIATIONS: &[&str] = &["fix", "FIX", "Fix"];

pub const COMMIT_TYPE_VARIATIONS: &[&str] =
    &["Lint", "bug", "fix", "feat", "ci", "chore", "docs", "CI"];

const EMPTY_SCOPE_VARIATIONS: &[&str] = &["()", "(  )"];
const NON_EMPTY_SCOPE_VARIATIONS: &[&str] = &["", "(i18n)", "(parser)", "(strict mode)"];

pub fn get_scope_variations(should_generate_empty_scope: bool) -> &'static [&'static str] {
    match should_generate_empty_scope {
        true => EMPTY_SCOPE_VARIATIONS,
        false => NON_EMPTY_SCOPE_VARIATIONS,
    }
}

pub fn get_after_type_variation(should_generate_space_after_type: bool) -> &'static str {
    match should_generate_space_after_type {
        true => " ",
        false => "",
    }
}

const DESCRIPTION_VARIATIONS: &[&str] = &[
    "expose hideBin helper for CJS ",
    "release 16.1.0 (#1779)",
    "update types for deno ^1.4.0",
    "Japanese translation phrasing (#1619)",
];
const NON_DESCRIPTION_VARIATIONS: &[&str] = &["", "\t", "      "];

pub fn get_description_variations(should_generate_description: bool) -> &'static [&'static str] {
    match should_generate_description {
        true => DESCRIPTION_VARIATIONS,
        false => NON_DESCRIPTION_VARIATIONS,
    }
}

const DESCRIPTION_TERMINATION_VARIATIONS: &[&str] = &["\n\n"];
const NON_DESCRIPTION_TERMINATION_VARIATIONS: &[&str] = &[""];

pub fn get_description_termination_variations(
    should_generate_description_termination: bool,
) -> &'static [&'static str] {
    match should_generate_description_termination {
        true => DESCRIPTION_TERMINATION_VARIATIONS,
        false => NON_DESCRIPTION_TERMINATION_VARIATIONS,
    }
}

const BODY_VARIATIONS: &[&str] = &[
    "Helps license scanning tools like https://github.com/licensee/licensee\r\nto successfully detect that this is an MIT licensed project.",
    "* Group all type definitions and helpers in using modules\r\n* Move .d.ts to typings directory\r\n* Get rid of types directory",
    "closes #706\n",
    "Co-authored-by: Renovate Bot <bot@renovateapp.com>",
    "Co-authored-by: github-actions[bot] <41898282+github-actions[bot]@users.noreply.github.com>\r\nCo-authored-by: Benjamin E. Coe <bencoe@google.com>",
];
const NON_BODY_VARIATIONS: &[&str] = &["", "\n", "\n\n"];

pub fn get_body_variations(should_generate_body: bool) -> &'static [&'static str] {
    match should_generate_body {
        true => BODY_VARIATIONS,
        false => NON_BODY_VARIATIONS,
    }
}

pub const MAJOR_FOOTER_VARIATIONS: &[&str] = &[
    "BREAKING CHANGE:\r\n\r\nremoved undocumented `defaults` alias for `default`.",
    "BREAKING CHANGE: find-up replaced with escalade; export map added (limits importable files in Node >= 12); yarser-parser@19.x.x (new decamelize/camelcase implementation).",
    "* chore: upgrade yargs-parser\r\nBREAKING-CHANGE: coerce is now applied as a final step after other parsing is complete\r\n\r\n* add documentation for breaking changes in yargs-parser@4\r\n\r\n* fix: a few small editing nits\r\n\r\n* fix: bump yargs-parser again\r\n",
    "BREAKING-CHANGE: fail is now applied globally.\r\nBREAKING-CHANGE: we now default to an empty builder function when command is executed with no builder.",
];
