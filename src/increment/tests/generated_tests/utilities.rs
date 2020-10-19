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
    should_generate_no_description: bool,
    should_generate_no_space_after_type: bool,
) -> Vec<String> {
    generate_commits(
        get_major_commit_types(),
        should_generate_preceding_whitespace,
        should_generate_empty_scope,
        should_generate_no_description,
        should_generate_no_space_after_type,
    )
}

pub fn generate_minor_commits(
    should_generate_preceding_whitespace: bool,
    should_generate_empty_scope: bool,
    should_generate_no_description: bool,
    should_generate_no_space_after_type: bool,
) -> Vec<String> {
    generate_commits(
        get_minor_commit_types(),
        should_generate_preceding_whitespace,
        should_generate_empty_scope,
        should_generate_no_description,
        should_generate_no_space_after_type,
    )
}

pub fn generate_patch_commits(
    should_generate_preceding_whitespace: bool,
    should_generate_empty_scope: bool,
    should_generate_no_description: bool,
    should_generate_no_space_after_type: bool,
) -> Vec<String> {
    generate_commits(
        get_patch_commit_types(),
        should_generate_preceding_whitespace,
        should_generate_empty_scope,
        should_generate_no_description,
        should_generate_no_space_after_type,
    )
}

fn generate_commits(
    commit_types: Vec<String>,
    should_generate_preceding_whitespace: bool,
    should_generate_empty_scope: bool,
    should_generate_no_description: bool,
    should_generate_no_space_after_type: bool,
) -> Vec<String> {
    let mut commits = vec![];

    let scope_variations = match should_generate_empty_scope {
        true => {
            get_empty_scope_variations()
        }
        false => vec!["".to_string()],
    };

    let preceding_variations = match should_generate_preceding_whitespace {
        true => {
            get_preceding_whitespace_variations()
        }
        false => vec!["".to_string()],
    };

    let description_variations = match should_generate_no_description {
        true => {
            get_no_description_variations()
        }
        false => get_description_variations(),
    };

    let space_after_type = match should_generate_no_space_after_type {
        true => {
            ""
        }
        false => " ",
    };

    for preceding in &preceding_variations {
        for commit_type in &commit_types {
            for scope in &scope_variations {
                for description in &description_variations {
                    let generated_commit = format!(
                        "{}{}{}:{}{}",
                        preceding, commit_type, scope, space_after_type, description
                    );
                    commits.push(generated_commit);
                }
            }
        }
    }

    commits
}

fn get_no_description_variations() -> Vec<String> {
    return vec![
        "".to_string(),
        "\t".to_string(),
        "\n\n".to_string(),
        "\n\r".to_string(),
    ];
}

fn get_description_variations() -> Vec<String> {
    return vec![
        "this is a description".to_string(),
        "this is a description\n\n".to_string(),
    ];
}

fn get_preceding_whitespace_variations() -> Vec<String> {
    return vec![
        "  ".to_string(),
        " ".to_string(),
        "\t".to_string(),
        "\n\r".to_string(),
    ];
}

fn get_empty_scope_variations() -> Vec<String> {
    return vec![
        "()".to_string(),
        "( )".to_string(),
    ];
}

fn get_major_commit_types() -> Vec<String> {
    return vec![
        "feat!".to_string(),
        "fix!".to_string(),
        "FIX!".to_string(),
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