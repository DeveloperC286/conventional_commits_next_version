mod variations;

pub(super) fn generate_major_title_commits() -> Vec<Vec<String>> {
    generate_commit_messages(variations::MAJOR_TITLE_COMMIT_TYPE_VARIATIONS, None)
}

pub(super) fn generate_major_footer_commits() -> Vec<Vec<String>> {
    generate_commit_messages(
        variations::COMMIT_TYPE_VARIATIONS,
        Some(variations::MAJOR_FOOTER_VARIATIONS),
    )
}

pub(super) fn generate_minor_commits() -> Vec<Vec<String>> {
    generate_commit_messages(variations::MINOR_COMMIT_TYPE_VARIATIONS, None)
}

pub(super) fn generate_patch_commits() -> Vec<Vec<String>> {
    generate_commit_messages(variations::PATCH_COMMIT_TYPE_VARIATIONS, None)
}

fn generate_commit_messages(
    commit_type_variations: &[&str],
    body_variations: Option<&[&str]>,
) -> Vec<Vec<String>> {
    let mut commit_messages = vec![];
    let mut number_of_variants: usize = 6;

    if body_variations.is_some() {
        number_of_variants = 4;
    }

    for i in 1..2_usize.pow(number_of_variants as u32) {
        let binary_string = format!("{:0number_of_variants$b}", i);

        let preceding_whitespace_variations = variations::get_preceding_whitespace_variations(
            is_position_in_binary_string_true(&binary_string, 0),
        );
        let scope_variations =
            variations::get_scope_variations(is_position_in_binary_string_true(&binary_string, 1));
        let after_type_variation = variations::get_after_type_variation(
            is_position_in_binary_string_true(&binary_string, 2),
        );
        let description_variations = variations::get_description_variations(
            is_position_in_binary_string_true(&binary_string, 3),
        );
        let (description_termination_variations, body_variations) = match body_variations {
            Some(body_variations) => {
                let description_termination_variations =
                    variations::get_description_termination_variations(true);

                (description_termination_variations, body_variations)
            }
            None => {
                let should_generate_description_termination =
                    is_position_in_binary_string_true(&binary_string, 4);
                let should_generate_body = is_position_in_binary_string_true(&binary_string, 5);
                let description_termination_variations =
                    variations::get_description_termination_variations(
                        should_generate_description_termination || should_generate_body,
                    );
                let body_variations = variations::get_body_variations(should_generate_body);

                (description_termination_variations, body_variations)
            }
        };

        commit_messages.push(generate_commit_messages_from(
            preceding_whitespace_variations,
            commit_type_variations,
            scope_variations,
            after_type_variation,
            description_variations,
            description_termination_variations,
            body_variations,
        ));
    }

    commit_messages
}

fn generate_commit_messages_from(
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
                            commit_messages.push(format!("{preceding_whitespace}{commit_type}{scope}:{after_type_variation}{description}{description_termination}{body}"));
                        }
                    }
                }
            }
        }
    }

    commit_messages
}

fn is_position_in_binary_string_true(binary_string: &str, position: usize) -> bool {
    match binary_string.chars().nth(position).unwrap() {
        '0' => false,
        '1' => true,
        _ => {
            panic!("Should be either 0 or 1.");
        }
    }
}
