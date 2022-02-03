import re
from behave import then

from utilities import execute_conventional_commits_next_version


@then('the returned version should be "{expected_version}".')
def compare_returned_and_expected_versions(context, expected_version):
    # When
    execute_conventional_commits_next_version(context)

    # Then
    assert context.stderr == ""
    assert int(context.exit_code) == 0
    assert context.stdout == expected_version


@then('the current version assertion passes.')
def current_version_assertion_passes(context):
    # When
    execute_conventional_commits_next_version(context)

    # Then
    assert context.stdout == ""
    assert context.stderr == ""
    assert int(context.exit_code) == 0


@then('the current version assertion fails.')
def current_version_assertion_fails(context):
    # When
    execute_conventional_commits_next_version(context)

    # Then
    assert context.stdout == ""
    assert int(context.exit_code) != 0


@then('their is a could not find commit hash "{commit_hash}" error.')
def then_could_not_find_commit_hash_error(context, commit_hash):
    # Given
    could_not_find_commit_hash_error = " ERROR conventional_commits_next_version::model::commits > Can not find commit hash '" + \
        commit_hash + "' on the Git revision walker.\n"

    # When
    execute_conventional_commits_next_version(context)

    # Then
    assert context.stdout == ""
    assert int(context.exit_code) != 0
    assert context.stderr == could_not_find_commit_hash_error


@then('their is a could not find reference "{reference}" error.')
def then_could_not_find_reference_error(context, reference):
    # Given
    could_not_find_reference_error = " ERROR conventional_commits_next_version::model::commits > Could not find a reference with the name \"" + \
                                     reference + "\".\n"

    # When/Then
    current_version_assertion_fails(context)

    # Then
    assert context.stderr == could_not_find_reference_error


@then(
    'their is a could not find shortened commit hash "{shortened_commit_hash}" error.')
def then_could_not_find_shortened_commit_hash_error(
        context, shortened_commit_hash):
    # Given
    could_not_find_shortened_commit_hash_error = " ERROR conventional_commits_next_version::model::commits > No actual commit hashes start with the provided short commit hash \"" + \
        shortened_commit_hash + "\".\n"

    # When/Then
    current_version_assertion_fails(context)

    # Then
    assert context.stderr == could_not_find_shortened_commit_hash_error


@then(
    'their is a ambiguous shortened commit hash "{shortened_commit_hash}" error.')
def then_ambiguous_shortened_commit_hash_error(context, shortened_commit_hash):
    # Given
    ambiguous_shortened_commit_hash_error = re.compile(
        '^ ERROR conventional_commits_next_version::model::commits > Ambiguous short commit hash, the commit hashes [[](' +
        shortened_commit_hash +
        '[a-f0-9]*(, )?)*[]] all start with the provided short commit hash "' +
        shortened_commit_hash +
        '".\n$')

    # When/Then
    current_version_assertion_fails(context)

    # Then
    assert ambiguous_shortened_commit_hash_error.match(
        context.stderr) is not None


@then('their is a missing from argument error.')
def then_missing_from_argument_error(context):
    # Given
    missing_from_argument_error = "error: The following required arguments were not provided:\n" + \
                                  "    <--from-stdin|--from-commit-hash <from-commit-hash>|--from-reference <from-reference>>\n" + \
                                  "\n" + \
                                  "USAGE:\n" + \
                                  "    conventional_commits_next_version [FLAGS] [OPTIONS] --from-version <from-version> <--from-stdin|--from-commit-hash <from-commit-hash>|--from-reference <from-reference>>\n" + \
                                  "\n" + \
                                  "For more information try --help\n"

    # When/Then
    current_version_assertion_fails(context)

    # Then
    assert context.stderr == missing_from_argument_error


@then('their is a conflicting from arguments error.')
def then_conflicting_from_arguments_error(context):
    # Given
    conflicting_arguments_end = "\n" + \
        "USAGE:\n" + \
        "    conventional_commits_next_version --from-version <from-version> <--from-stdin|--from-commit-hash <from-commit-hash>|--from-reference <from-reference>>\n" + \
        "\n" + \
        "For more information try --help\n"

    conflicting_from_commit_hash_error = "error: The argument '--from-commit-hash <from-commit-hash>' cannot be used with one or more of the other specified arguments\n" + conflicting_arguments_end
    conflicting_from_reference_error = "error: The argument '--from-reference <from-reference>' cannot be used with one or more of the other specified arguments\n" + conflicting_arguments_end
    conflicting_from_stdin_error = "error: The argument '--from-stdin' cannot be used with one or more of the other specified arguments\n" + conflicting_arguments_end

    # When/Then
    current_version_assertion_fails(context)

    # Then
    assert context.stderr in [
        conflicting_from_commit_hash_error,
        conflicting_from_reference_error,
        conflicting_from_stdin_error]
