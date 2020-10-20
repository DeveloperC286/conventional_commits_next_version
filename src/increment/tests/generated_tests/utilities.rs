pub fn is_position_in_binary_string_true(binary_string: &str, position: usize) -> bool {
    match binary_string.chars().nth(position).unwrap() {
        '0' => false,
        '1' => true,
        _ => {
            panic!("Should be either 0 or 1.");
        }
    }
}

pub fn generate_major_commits(
    should_generate_preceding_whitespace: bool,
    should_generate_empty_scope: bool,
    should_generate_no_space_after_type: bool,
    should_generate_no_description: bool,
    should_generate_no_description_termination: bool,
) -> Vec<String> {
    generate_commits(
        get_preceding_whitespace_variations(should_generate_preceding_whitespace),
        get_major_commit_types(),
        get_scope_variations(should_generate_empty_scope),
        should_generate_no_space_after_type,
        get_description_variations(should_generate_no_description),
        get_description_termination_variations(should_generate_no_description_termination),
    )
}

pub fn generate_minor_commits(
    should_generate_preceding_whitespace: bool,
    should_generate_empty_scope: bool,
    should_generate_no_space_after_type: bool,
    should_generate_no_description: bool,
    should_generate_no_description_termination: bool,
) -> Vec<String> {
    generate_commits(
        get_preceding_whitespace_variations(should_generate_preceding_whitespace),
        get_minor_commit_types(),
        get_scope_variations(should_generate_empty_scope),
        should_generate_no_space_after_type,
        get_description_variations(should_generate_no_description),
        get_description_termination_variations(should_generate_no_description_termination),
    )
}

pub fn generate_patch_commits(
    should_generate_preceding_whitespace: bool,
    should_generate_empty_scope: bool,
    should_generate_no_space_after_type: bool,
    should_generate_no_description: bool,
    should_generate_no_description_termination: bool,
) -> Vec<String> {
    generate_commits(
        get_preceding_whitespace_variations(should_generate_preceding_whitespace),
        get_patch_commit_types(),
        get_scope_variations(should_generate_empty_scope),
        should_generate_no_space_after_type,
        get_description_variations(should_generate_no_description),
        get_description_termination_variations(should_generate_no_description_termination),
    )
}

fn generate_commits(
    preceding_whitespace_variations: Vec<String>,
    commit_types: Vec<String>,
    scope_variations: Vec<String>,
    should_generate_no_space_after_type: bool,
    description_variations: Vec<String>,
    description_termination_variations: Vec<String>,
) -> Vec<String> {
    let mut commits = vec![];

    let space_after_type = match should_generate_no_space_after_type {
        true => {
            ""
        }
        false => " ",
    };

    for preceding in &preceding_whitespace_variations {
        for commit_type in &commit_types {
            for scope in &scope_variations {
                for description in &description_variations {
                    for description_termination in &description_termination_variations {
                        let generated_commit = format!(
                            "{}{}{}:{}{}{}",
                            preceding, commit_type, scope, space_after_type, description, description_termination
                        );
                        commits.push(generated_commit);
                    }
                }
            }
        }
    }

    commits
}

fn get_preceding_whitespace_variations(should_generate_preceding_whitespace: bool) -> Vec<String> {
    match should_generate_preceding_whitespace {
        true => vec![
            "  ".to_string(),
            " ".to_string(),
            "\t".to_string(),
            "\n\r".to_string(),
        ],
        false => vec!["".to_string()],
    }
}

fn get_scope_variations(should_generate_empty_scope: bool) -> Vec<String> {
    match should_generate_empty_scope {
        true => vec![
            "()".to_string(),
            "( )".to_string(),
        ],
        false => vec!["".to_string()],
    }
}

fn get_description_variations(should_generate_no_description: bool) -> Vec<String> {
    match should_generate_no_description {
        true => vec![
            "".to_string(),
            "\t".to_string(),
            "      ".to_string(),
        ],
        false => vec![
            "expose hideBin helper for CJS ".to_string(),
            "release 16.1.0 (#1779)".to_string(),
            "update types for deno ^1.4.0".to_string(),
        ]
    }
}

fn get_description_termination_variations(should_generate_no_description_termination: bool) -> Vec<String> {
    match should_generate_no_description_termination {
        true => vec![
            "".to_string(),
        ],
        false => vec![
            "\n\n".to_string(),
        ]
    }
}

fn get_major_commit_types() -> Vec<String> {
    return vec![
        "feat!".to_string(),
        "fix!".to_string(),
        "FIX!".to_string(),
        "Build!".to_string(),
    ];
}

fn get_minor_commit_types() -> Vec<String> {
    return vec![
        "feat".to_string(),
        "FEAT".to_string(),
        "Feat".to_string(),
    ];
}

fn get_patch_commit_types() -> Vec<String> {
    return vec![
        "fix".to_string(),
        "FIX".to_string(),
        "Fix".to_string(),
    ];
}