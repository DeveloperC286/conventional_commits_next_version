use crate::increment::*;

mod utilities;

#[test]
fn test_generate_major_commits() {
    let number_of_variants = 5;
    let upper_bound = 2_u32.pow(number_of_variants) as u32;

    for i in 1..upper_bound {
        //Given
        let binary_string = format!("{:05b}", i); // Update when number_of_variants changes.
        let commits = utilities::generate_major_commits(
            utilities::is_position_in_binary_string_true(&binary_string, 0),
            utilities::is_position_in_binary_string_true(&binary_string, 1),
            utilities::is_position_in_binary_string_true(&binary_string, 2),
            utilities::is_position_in_binary_string_true(&binary_string, 3),
            utilities::is_position_in_binary_string_true(&binary_string, 4),
        );

        for commit_message in commits {
            //When/Then
            println!("{}", commit_message);
            assert_eq!(true, is_major_increment(&*commit_message));
        }
    }
}

#[test]
fn test_generate_minor_commits() {
    let number_of_variants = 5;
    let upper_bound = 2_u32.pow(number_of_variants) as u32;

    for i in 1..upper_bound {
        //Given
        let binary_string = format!("{:05b}", i); // Update when number_of_variants changes.
        let commits = utilities::generate_minor_commits(
            utilities::is_position_in_binary_string_true(&binary_string, 0),
            utilities::is_position_in_binary_string_true(&binary_string, 1),
            utilities::is_position_in_binary_string_true(&binary_string, 2),
            utilities::is_position_in_binary_string_true(&binary_string, 3),
            utilities::is_position_in_binary_string_true(&binary_string, 4),
        );

        for commit_message in commits {
            //When/Then
            println!("{}", commit_message);
            assert_eq!(true, is_minor_increment(&*commit_message));
        }
    }
}

#[test]
fn test_generate_patch_commits() {
    let number_of_variants = 5;
    let upper_bound = 2_u32.pow(number_of_variants) as u32;

    for i in 1..upper_bound {
        //Given
        let binary_string = format!("{:05b}", i); // Update when number_of_variants changes.
        let commits = utilities::generate_patch_commits(
            utilities::is_position_in_binary_string_true(&binary_string, 0),
            utilities::is_position_in_binary_string_true(&binary_string, 1),
            utilities::is_position_in_binary_string_true(&binary_string, 2),
            utilities::is_position_in_binary_string_true(&binary_string, 3),
            utilities::is_position_in_binary_string_true(&binary_string, 4),
        );

        for commit_message in commits {
            //When/Then
            println!("{}", commit_message);
            assert_eq!(true, is_patch_increment(&*commit_message));
        }
    }
}