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
        variations::MAJOR_TITLE_COMMIT_TYPE_VARIATIONS,
        variations::get_scope_variations(should_generate_empty_scope),
        variations::get_after_type_variation(should_generate_space_after_type),
        variations::get_description_variations(should_generate_description),
        variations::get_description_termination_variations(
            should_generate_description_termination || should_generate_body,
        ),
        variations::get_body_variations(should_generate_body),
    ) {
        is_only_major_title_increment(&commit_message);
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
        variations::COMMIT_TYPE_VARIATIONS,
        variations::get_scope_variations(should_generate_empty_scope),
        variations::get_after_type_variation(should_generate_space_after_type),
        variations::get_description_variations(should_generate_description),
        variations::get_description_termination_variations(true),
        variations::MAJOR_FOOTER_VARIATIONS,
    ) {
        is_only_major_footer_increment(&commit_message);
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
        variations::MINOR_COMMIT_TYPE_VARIATIONS,
        variations::get_scope_variations(should_generate_empty_scope),
        variations::get_after_type_variation(should_generate_space_after_type),
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
        variations::PATCH_COMMIT_TYPE_VARIATIONS,
        variations::get_scope_variations(should_generate_empty_scope),
        variations::get_after_type_variation(should_generate_space_after_type),
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
    preceding_whitespace_variations: &[&str],
    commit_type_variations: &[&str],
    scope_variations: &[&str],
    after_type_variation: &str,
    description_variations: &[&str],
    description_termination_variations: &[&str],
    body_variations: &[&str],
) -> Vec<String> {
    let mut commit_messages = vec![];

    for preceding_whitespace in preceding_whitespace_variations {
        for commit_type in commit_type_variations {
            for scope in scope_variations {
                for description in description_variations {
                    for description_termination in description_termination_variations {
                        for body in body_variations {
                            commit_messages.push(format!(
                                "{}{}{}:{}{}{}{}",
                                preceding_whitespace,
                                commit_type,
                                scope,
                                after_type_variation,
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
