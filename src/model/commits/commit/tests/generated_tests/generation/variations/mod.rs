pub fn get_preceding_whitespace_variations(
    should_generate_preceding_whitespace: bool,
) -> Vec<String> {
    match should_generate_preceding_whitespace {
        true => vec![
            "  ".to_string(),
            " ".to_string(),
            "\t".to_string(),
            "\n".to_string(),
            "\n\r".to_string(),
        ],
        false => vec!["".to_string()],
    }
}

pub fn get_major_commit_type_variations() -> Vec<String> {
    vec![
        "feat!".to_string(),
        "fix!".to_string(),
        "FIX!".to_string(),
        "Build!".to_string(),
        "Feat!".to_string(),
    ]
}

pub fn get_minor_commit_type_variations() -> Vec<String> {
    vec!["feat".to_string(), "FEAT".to_string(), "Feat".to_string()]
}

pub fn get_patch_commit_type_variations() -> Vec<String> {
    vec!["fix".to_string(), "FIX".to_string(), "Fix".to_string()]
}

pub fn get_commit_types_variations() -> Vec<String> {
    vec![
        "Lint".to_string(),
        "bug".to_string(),
        "fix".to_string(),
        "feat".to_string(),
        "ci".to_string(),
        "chore".to_string(),
        "docs".to_string(),
        "CI".to_string(),
    ]
}

pub fn get_scope_variations(should_generate_empty_scope: bool) -> Vec<String> {
    match should_generate_empty_scope {
        true => vec!["()".to_string(), "(  )".to_string()],
        false => vec![
            "".to_string(),
            "(i18n)".to_string(),
            "(parser)".to_string(),
            "(strict mode)".to_string(),
        ],
    }
}

pub fn get_description_variations(should_generate_description: bool) -> Vec<String> {
    match should_generate_description {
        true => vec![
            "expose hideBin helper for CJS ".to_string(),
            "release 16.1.0 (#1779)".to_string(),
            "update types for deno ^1.4.0".to_string(),
            "Japanese translation phrasing (#1619)".to_string(),
        ],
        false => vec!["".to_string(), "\t".to_string(), "      ".to_string()],
    }
}

pub fn get_description_termination_variations(
    should_generate_description_termination: bool,
) -> Vec<String> {
    match should_generate_description_termination {
        true => vec!["\n\n".to_string()],
        false => vec!["".to_string()],
    }
}

pub fn get_body_variations(should_generate_body: bool) -> Vec<String> {
    match should_generate_body {
        true => vec![
            "Helps license scanning tools like https://github.com/licensee/licensee\r\nto successfully detect that this is an MIT licensed project.".to_string(),
            "* Group all type definitions and helpers in using modules\r\n* Move .d.ts to typings directory\r\n* Get rid of types directory".to_string(),
            "closes #706\n".to_string(),
            "Co-authored-by: Renovate Bot <bot@renovateapp.com>".to_string(),
            "Co-authored-by: github-actions[bot] <41898282+github-actions[bot]@users.noreply.github.com>\r\nCo-authored-by: Benjamin E. Coe <bencoe@google.com>".to_string(),
        ],
        false => vec![
            "".to_string(),
            "\n".to_string(),
            "\n\n".to_string(),
        ]
    }
}

pub fn get_major_footer_variations() -> Vec<String> {
    vec![
        "BREAKING CHANGE:\r\n\r\nremoved undocumented `defaults` alias for `default`.".to_string(),
        "BREAKING CHANGE: find-up replaced with escalade; export map added (limits importable files in Node >= 12); yarser-parser@19.x.x (new decamelize/camelcase implementation).".to_string(),
        "* chore: upgrade yargs-parser\r\nBREAKING-CHANGE: coerce is now applied as a final step after other parsing is complete\r\n\r\n* add documentation for breaking changes in yargs-parser@4\r\n\r\n* fix: a few small editing nits\r\n\r\n* fix: bump yargs-parser again\r\n".to_string(),
        "BREAKING-CHANGE: fail is now applied globally.\r\nBREAKING-CHANGE: we now default to an empty builder function when command is executed with no builder.".to_string(),
    ]
}
