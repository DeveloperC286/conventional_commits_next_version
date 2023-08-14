import re
from behave import then

from utilities import execute_conventional_commits_next_version
from assertions import *


@then('the returned version should be "{expected_version}".')
def compare_returned_and_expected_versions(context, expected_version):
    # When
    execute_conventional_commits_next_version(context)

    # Then
    assert_no_errors(context)
    assert_command_successful(context)
    assert context.stdout == expected_version


@then('the current version assertion passes.')
def current_version_assertion_passes(context):
    # When
    execute_conventional_commits_next_version(context)

    # Then
    assert_no_output(context)
    assert_no_errors(context)
    assert_command_successful(context)


@then('the current version assertion fails.')
def current_version_assertion_fails(context):
    # When
    execute_conventional_commits_next_version(context)

    # Then
    assert_no_output(context)
    assert_command_unsuccessful(context)


@then('their is a could not find commit hash "{commit_hash}" error.')
def then_could_not_find_commit_hash_error(context, commit_hash):
    # Given
    could_not_find_commit_hash_error = f" ERROR conventional_commits_next_version_lib::commits > Can not find a commit with the hash '{commit_hash}'.\n"

    # When
    execute_conventional_commits_next_version(context)

    # Then
    assert_no_output(context)
    assert_command_unsuccessful(context)
    assert_error_equals(context, could_not_find_commit_hash_error)


@then('their is a could not find reference "{reference}" error.')
def then_could_not_find_reference_error(context, reference):
    # Given
    could_not_find_reference_error = f" ERROR conventional_commits_next_version_lib::commits > Could not find a reference with the name \"{reference}\".\n"

    # When/Then
    current_version_assertion_fails(context)

    # Then
    assert_error_equals(context, could_not_find_reference_error)


@then(
    'their is a could not find shortened commit hash "{shortened_commit_hash}" error.')
def then_could_not_find_shortened_commit_hash_error(
        context, shortened_commit_hash):
    # Given
    could_not_find_shortened_commit_hash_error = f" ERROR conventional_commits_next_version_lib::commits > No commit hashes start with the provided short commit hash \"{shortened_commit_hash}\".\n"

    # When/Then
    current_version_assertion_fails(context)

    # Then
    assert_error_equals(context, could_not_find_shortened_commit_hash_error)


@then(
    'their is a ambiguous shortened commit hash "{shortened_commit_hash}" error.')
def then_ambiguous_shortened_commit_hash_error(context, shortened_commit_hash):
    # Given
    ambiguous_shortened_commit_hash_error = re.compile(
        f"^ ERROR conventional_commits_next_version_lib::commits > Ambiguous short commit hash, the commit hashes [[]({shortened_commit_hash}[a-f0-9]*(, )?)*[]] all start with the provided short commit hash \"{shortened_commit_hash}\".\n$")

    # When/Then
    current_version_assertion_fails(context)

    # Then
    assert_error_matches_regex(context, ambiguous_shortened_commit_hash_error)


@then('their is a missing from argument error.')
def then_missing_from_argument_error(context):
    # Given
    missing_from_argument_error = "error: The following required arguments were not provided:\n" + \
                                  "    <--from-stdin|--from-reference <from-reference>|--from-commit-hash <from-commit-hash>>\n" + \
                                  "\n" + \
                                  "USAGE:\n" + \
                                  "    conventional_commits_next_version --calculation-mode <calculation-mode> --from-version <from-version> --git-history-mode <git-history-mode> <--from-stdin|--from-reference <from-reference>|--from-commit-hash <from-commit-hash>>\n" + \
                                  "\n" + \
                                  "For more information try --help\n"

    # When/Then
    current_version_assertion_fails(context)

    # Then
    assert_error_equals(context, missing_from_argument_error)


@then('their is a conflicting from arguments error.')
def then_conflicting_from_arguments_error(context):
    # Given
    conflicting_arguments_end = "\n" + \
        "USAGE:\n" + \
        "    conventional_commits_next_version --calculation-mode <calculation-mode> --from-version <from-version> --git-history-mode <git-history-mode> <--from-stdin|--from-reference <from-reference>|--from-commit-hash <from-commit-hash>>\n" + \
        "\n" + \
        "For more information try --help\n"

    conflicting_from_commit_hash_error = f"error: The argument '--from-commit-hash <from-commit-hash>' cannot be used with one or more of the other specified arguments\n{conflicting_arguments_end}"
    conflicting_from_reference_error = f"error: The argument '--from-reference <from-reference>' cannot be used with one or more of the other specified arguments\n{conflicting_arguments_end}"
    conflicting_from_stdin_error = f"error: The argument '--from-stdin' cannot be used with one or more of the other specified arguments\n{conflicting_arguments_end}"

    # When/Then
    current_version_assertion_fails(context)

    # Then
    assert_error_is_one_of(context,
                           [conflicting_from_commit_hash_error,
                            conflicting_from_reference_error,
                            conflicting_from_stdin_error])
