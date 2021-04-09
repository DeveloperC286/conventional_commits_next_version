mod variations;

pub fn generate_major_commits(
    should_generate_preceding_whitespace: bool,
    should_generate_empty_scope: bool,
    should_generate_space_after_type: bool,
    should_generate_description: bool,
    should_generate_description_termination: bool,
    should_generate_body: bool,
) -> Vec<String> {
    generate_commits(
        variations::get_preceding_whitespace_variations(should_generate_preceding_whitespace),
        variations::get_major_commit_types(),
        variations::get_scope_variations(should_generate_empty_scope),
        should_generate_space_after_type,
        variations::get_description_variations(should_generate_description),
        variations::get_description_termination_variations(
            should_generate_description_termination || should_generate_body,
        ),
        variations::get_body_variations(should_generate_body),
    )
}

pub fn generate_major_body_commits(
    should_generate_preceding_whitespace: bool,
    should_generate_empty_scope: bool,
    should_generate_space_after_type: bool,
    should_generate_description: bool,
) -> Vec<String> {
    generate_commits(
        variations::get_preceding_whitespace_variations(should_generate_preceding_whitespace),
        variations::get_commit_types_variations(),
        variations::get_scope_variations(should_generate_empty_scope),
        should_generate_space_after_type,
        variations::get_description_variations(should_generate_description),
        variations::get_description_termination_variations(true),
        variations::get_major_body_variations(),
    )
}

pub fn generate_minor_commits(
    should_generate_preceding_whitespace: bool,
    should_generate_empty_scope: bool,
    should_generate_space_after_type: bool,
    should_generate_description: bool,
    should_generate_description_termination: bool,
    should_generate_body: bool,
) -> Vec<String> {
    generate_commits(
        variations::get_preceding_whitespace_variations(should_generate_preceding_whitespace),
        variations::get_minor_commit_types(),
        variations::get_scope_variations(should_generate_empty_scope),
        should_generate_space_after_type,
        variations::get_description_variations(should_generate_description),
        variations::get_description_termination_variations(
            should_generate_description_termination || should_generate_body,
        ),
        variations::get_body_variations(should_generate_body),
    )
}

pub fn generate_patch_commits(
    should_generate_preceding_whitespace: bool,
    should_generate_empty_scope: bool,
    should_generate_space_after_type: bool,
    should_generate_description: bool,
    should_generate_description_termination: bool,
    should_generate_body: bool,
) -> Vec<String> {
    generate_commits(
        variations::get_preceding_whitespace_variations(should_generate_preceding_whitespace),
        variations::get_patch_commit_types(),
        variations::get_scope_variations(should_generate_empty_scope),
        should_generate_space_after_type,
        variations::get_description_variations(should_generate_description),
        variations::get_description_termination_variations(
            should_generate_description_termination || should_generate_body,
        ),
        variations::get_body_variations(should_generate_body),
    )
}

fn generate_commits(
    preceding_whitespace_variations: Vec<String>,
    commit_types: Vec<String>,
    scope_variations: Vec<String>,
    should_generate_space_after_type: bool,
    description_variations: Vec<String>,
    description_termination_variations: Vec<String>,
    body_variations: Vec<String>,
) -> Vec<String> {
    let mut commits = vec![];

    let space_after_type = match should_generate_space_after_type {
        true => " ",
        false => "",
    };

    for preceding in &preceding_whitespace_variations {
        for commit_type in &commit_types {
            for scope in &scope_variations {
                for description in &description_variations {
                    for description_termination in &description_termination_variations {
                        for body in &body_variations {
                            let generated_commit = format!(
                                "{}{}{}:{}{}{}{}",
                                preceding,
                                commit_type,
                                scope,
                                space_after_type,
                                description,
                                description_termination,
                                body
                            );
                            commits.push(generated_commit);
                        }
                    }
                }
            }
        }
    }

    commits
}
