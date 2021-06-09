use super::*;

mod variations;

pub fn test_major_title_commits(
    should_generate_preceding_whitespace: bool,
    should_generate_empty_scope: bool,
    should_generate_space_after_type: bool,
    should_generate_description: bool,
    should_generate_description_termination: bool,
    should_generate_body: bool,
) {
    for commit_message in generate_commit_messages(
        variations::get_preceding_whitespace_variations(should_generate_preceding_whitespace),
        variations::get_major_commit_type_variations(),
        variations::get_scope_variations(should_generate_empty_scope),
        should_generate_space_after_type,
        variations::get_description_variations(should_generate_description),
        variations::get_description_termination_variations(
            should_generate_description_termination || should_generate_body,
        ),
        variations::get_body_variations(should_generate_body),
    ) {
        is_major_title_increment(&commit_message);
    }
}

pub fn test_major_footer_commits(
    should_generate_preceding_whitespace: bool,
    should_generate_empty_scope: bool,
    should_generate_space_after_type: bool,
    should_generate_description: bool,
) {
    for commit_message in generate_commit_messages(
        variations::get_preceding_whitespace_variations(should_generate_preceding_whitespace),
        variations::get_commit_types_variations(),
        variations::get_scope_variations(should_generate_empty_scope),
        should_generate_space_after_type,
        variations::get_description_variations(should_generate_description),
        variations::get_description_termination_variations(true),
        variations::get_major_footer_variations(),
    ) {
        is_major_footer_increment(&commit_message);
    }
}

pub fn test_minor_commits(
    should_generate_preceding_whitespace: bool,
    should_generate_empty_scope: bool,
    should_generate_space_after_type: bool,
    should_generate_description: bool,
    should_generate_description_termination: bool,
    should_generate_body: bool,
) {
    for commit_message in generate_commit_messages(
        variations::get_preceding_whitespace_variations(should_generate_preceding_whitespace),
        variations::get_minor_commit_type_variations(),
        variations::get_scope_variations(should_generate_empty_scope),
        should_generate_space_after_type,
        variations::get_description_variations(should_generate_description),
        variations::get_description_termination_variations(
            should_generate_description_termination || should_generate_body,
        ),
        variations::get_body_variations(should_generate_body),
    ) {
        is_minor_increment(&commit_message);
    }
}

pub fn test_patch_commits(
    should_generate_preceding_whitespace: bool,
    should_generate_empty_scope: bool,
    should_generate_space_after_type: bool,
    should_generate_description: bool,
    should_generate_description_termination: bool,
    should_generate_body: bool,
) {
    for commit_message in generate_commit_messages(
        variations::get_preceding_whitespace_variations(should_generate_preceding_whitespace),
        variations::get_patch_commit_type_variations(),
        variations::get_scope_variations(should_generate_empty_scope),
        should_generate_space_after_type,
        variations::get_description_variations(should_generate_description),
        variations::get_description_termination_variations(
            should_generate_description_termination || should_generate_body,
        ),
        variations::get_body_variations(should_generate_body),
    ) {
        is_patch_increment(&commit_message);
    }
}

fn generate_commit_messages(
    preceding_whitespace_variations: Vec<String>,
    commit_type_variations: Vec<String>,
    scope_variations: Vec<String>,
    should_generate_space_after_type: bool,
    description_variations: Vec<String>,
    description_termination_variations: Vec<String>,
    body_variations: Vec<String>,
) -> Vec<String> {
    let mut commit_messages = vec![];

    let space_after_type = match should_generate_space_after_type {
        true => " ",
        false => "",
    };

    for preceding_whitespace in &preceding_whitespace_variations {
        for commit_type in &commit_type_variations {
            for scope in &scope_variations {
                for description in &description_variations {
                    for description_termination in &description_termination_variations {
                        for body in &body_variations {
                            commit_messages.push(format!(
                                "{}{}{}:{}{}{}{}",
                                preceding_whitespace,
                                commit_type,
                                scope,
                                space_after_type,
                                description,
                                description_termination,
                                body
                            ));
                        }
                    }
                }
            }
        }
    }

    commit_messages
}
