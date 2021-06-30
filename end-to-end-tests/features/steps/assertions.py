import os
from behave import then

from util import execute_command


@then('the returned version should be "{expected_version}".')
def compare_returned_and_expected_versions(context, expected_version):
    execute_conventional_commits_next_version(context)
    assert int(context.exit_code) == 0
    assert context.stdout == expected_version


@then('the current version assertion passes.')
def current_version_assertion_passes(context):
    execute_conventional_commits_next_version(context)
    assert int(context.exit_code) == 0
    assert context.stdout == ""


@then('the current version assertion fails.')
def current_version_assertion_fails(context):
    execute_conventional_commits_next_version(context)
    assert int(context.exit_code) != 0
    assert starts_with(
        context.stdout,
        "ERROR conventional_commits_next_version > The current version ")


@then('the error message is "{error_message}".')
def then_the_error_message_is(context, error_message):
    execute_conventional_commits_next_version(context)
    assert starts_with(context.stdout, error_message)


@then('the error message is either "{error_message}" or "{error_message2}".')
def then_the_error_message_is_either(context, error_message, error_message2):
    execute_conventional_commits_next_version(context)
    assert starts_with(
        context.stdout,
        error_message) or starts_with(
        context.stdout,
        error_message2)


def execute_conventional_commits_next_version(context):
    os.chdir(context.temporary_directory.name)
    (context.exit_code, context.stdout) = execute_command(context.pre_command +
                                                          context.conventional_commits_next_version_path + context.arguments)
    os.chdir(context.behave_directory)


def starts_with(stdout, error_message):
    return stdout.strip().startswith(error_message.strip())
