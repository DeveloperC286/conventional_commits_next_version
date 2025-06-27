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
    opt_body_variations: Option<&[&str]>,
) -> Vec<Vec<String>> {
    let mut commit_messages = vec![];

    let description_termination_variations = if opt_body_variations.is_some() {
        &variations::DESCRIPTION_TERMINATION_VARIATIONS_LIST[0..1]
    } else {
        variations::DESCRIPTION_TERMINATION_VARIATIONS_LIST
    };

    let body_variations = match opt_body_variations {
        Some(body_variations) => &[body_variations],
        None => variations::BODY_VARIATIONS_LIST,
    };

    for preceding_whitespace_variations in variations::PRECEDING_VARIATIONS {
        for scope_variations in variations::SCOPE_VARIATIONS {
            for after_type_variations in variations::AFTER_TYPE_VARIATIONS {
                for description_variations in variations::DESCRIPTION_VARIATIONS_LIST {
                    for description_termination_variations_inner in
                        description_termination_variations
                    {
                        for body_variations_inner in body_variations {
                            commit_messages.push(generate_commit_messages_from(
                                preceding_whitespace_variations,
                                commit_type_variations,
                                scope_variations,
                                after_type_variations,
                                description_variations,
                                description_termination_variations_inner,
                                body_variations_inner,
                            ));
                        }
                    }
                }
            }
        }
    }

    commit_messages
}

fn generate_commit_messages_from(
    preceding_whitespace_variations: &[&str],
    commit_type_variations: &[&str],
    scope_variations: &[&str],
    after_type_variations: &[&str],
    description_variations: &[&str],
    description_termination_variations: &[&str],
    body_variations: &[&str],
) -> Vec<String> {
    let mut commit_messages = vec![];

    for preceding_whitespace in preceding_whitespace_variations {
        for commit_type in commit_type_variations {
            for scope in scope_variations {
                for after_type in after_type_variations {
                    for description in description_variations {
                        for description_termination in description_termination_variations {
                            for body in body_variations {
                                commit_messages.push(format!("{preceding_whitespace}{commit_type}{scope}:{after_type}{description}{description_termination}{body}"));
                            }
                        }
                    }
                }
            }
        }
    }

    commit_messages
}
