use super::*;

mod generation;
mod utilities;

#[test]
fn test_major_title_commits() {
    let number_of_variants: usize = 6;

    for i in 1..2_usize.pow(number_of_variants as u32) {
        //Given
        let binary_string = format!(
            "{:0desired_length$b}",
            i,
            desired_length = number_of_variants
        );

        let commit_messages = generation::generate_major_title_commits(
            utilities::is_position_in_binary_string_true(&binary_string, 0),
            utilities::is_position_in_binary_string_true(&binary_string, 1),
            utilities::is_position_in_binary_string_true(&binary_string, 2),
            utilities::is_position_in_binary_string_true(&binary_string, 3),
            utilities::is_position_in_binary_string_true(&binary_string, 4),
            utilities::is_position_in_binary_string_true(&binary_string, 5),
        );

        for commit_message in commit_messages {
            is_only_major_title_increment(&commit_message);
        }
    }
}

#[test]
fn test_major_footer_commits() {
    let number_of_variants: usize = 4;

    for i in 1..2_usize.pow(number_of_variants as u32) {
        //Given
        let binary_string = format!(
            "{:0desired_length$b}",
            i,
            desired_length = number_of_variants
        );

        let commit_messages = generation::generate_major_footer_commits(
            utilities::is_position_in_binary_string_true(&binary_string, 0),
            utilities::is_position_in_binary_string_true(&binary_string, 1),
            utilities::is_position_in_binary_string_true(&binary_string, 2),
            utilities::is_position_in_binary_string_true(&binary_string, 3),
        );

        for commit_message in commit_messages {
            is_only_major_footer_increment(&commit_message);
        }
    }
}

#[test]
fn test_minor_commits() {
    let number_of_variants: usize = 6;

    for i in 1..2_usize.pow(number_of_variants as u32) {
        //Given
        let binary_string = format!(
            "{:0desired_length$b}",
            i,
            desired_length = number_of_variants
        );

        let commit_messages = generation::generate_minor_commits(
            utilities::is_position_in_binary_string_true(&binary_string, 0),
            utilities::is_position_in_binary_string_true(&binary_string, 1),
            utilities::is_position_in_binary_string_true(&binary_string, 2),
            utilities::is_position_in_binary_string_true(&binary_string, 3),
            utilities::is_position_in_binary_string_true(&binary_string, 4),
            utilities::is_position_in_binary_string_true(&binary_string, 5),
        );

        for commit_message in commit_messages {
            is_minor_increment(&commit_message);
        }
    }
}

#[test]
fn test_patch_commits() {
    let number_of_variants: usize = 6;

    for i in 1..2_usize.pow(number_of_variants as u32) {
        //Given
        let binary_string = format!(
            "{:0desired_length$b}",
            i,
            desired_length = number_of_variants
        );

        let commit_messages = generation::generate_patch_commits(
            utilities::is_position_in_binary_string_true(&binary_string, 0),
            utilities::is_position_in_binary_string_true(&binary_string, 1),
            utilities::is_position_in_binary_string_true(&binary_string, 2),
            utilities::is_position_in_binary_string_true(&binary_string, 3),
            utilities::is_position_in_binary_string_true(&binary_string, 4),
            utilities::is_position_in_binary_string_true(&binary_string, 5),
        );

        for commit_message in commit_messages {
            is_patch_increment(&commit_message);
        }
    }
}
