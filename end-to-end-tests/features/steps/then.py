import os
from behave import then

from utilities import execute_conventional_commits_next_version


@then('the returned version should be "{expected_version}".')
def compare_returned_and_expected_versions(context, expected_version):
    execute_conventional_commits_next_version(context)
    assert context.stderr == ""
    assert int(context.exit_code) == 0
    assert context.stdout == expected_version


@then('the current version assertion passes.')
def current_version_assertion_passes(context):
    execute_conventional_commits_next_version(context)
    assert context.stdout == ""
    assert context.stderr == ""
    assert int(context.exit_code) == 0


@then('the current version assertion fails.')
def current_version_assertion_fails(context):
    execute_conventional_commits_next_version(context)
    assert context.stdout == ""
    assert int(context.exit_code) != 0
    assert starts_with(
        context.stderr,
        "ERROR conventional_commits_next_version > The current version ")


@then('the error message is "{error_message}".')
def then_the_error_message_is(context, error_message):
    execute_conventional_commits_next_version(context)
    assert starts_with(context.stderr, error_message)


@then('the error message is either "{error_message}" or "{error_message2}".')
def then_the_error_message_is_either(context, error_message, error_message2):
    execute_conventional_commits_next_version(context)
    assert starts_with(
        context.stderr,
        error_message) or starts_with(
        context.stderr,
        error_message2)


@then('their is a could not find commit hash "{commit_hash}" error.')
def then_could_not_find_commit_hash(context, commit_hash):
    execute_conventional_commits_next_version(context)
    could_not_find_commit_hash_error = " ERROR conventional_commits_next_version::model::commits > Can not find commit hash '" + \
        commit_hash + "' on the Git revision walker.\n"
    assert context.stdout == ""
    assert int(context.exit_code) != 0
    assert context.stderr == could_not_find_commit_hash_error


def starts_with(searching, searching_for):
    return searching.strip().startswith(searching_for.strip())