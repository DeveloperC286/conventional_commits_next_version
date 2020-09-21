import os


from util import execute_command
from subprocess import Popen, PIPE, STDOUT
from behave import *


@when('the argument --from-commit-hash is provided as "{from_commit_hash}".')
def set_from_commit_hash(context, from_commit_hash):
    context.arguments += " --from-commit-hash " + from_commit_hash


@when('the argument --from-version is provided as "{from_version}".')
def set_from_version(context, from_version):
    context.arguments += " --from-version " + from_version


@when('the --batch-commits flag is set.')
def set_batch_commits_flag(context):
    context.arguments += " --batch-commits "


@when('the argument --current-version is provided as "{current_version}".')
def set_current_version(context, current_version):
    context.arguments += " --current-version " + current_version + " "


@when('the argument --from-tag is provided as "{from_tag}".')
def set_from_tag(context, from_tag):
    context.arguments += " --from-tag " + from_tag


def execute_conventional_commits_next_version(context):
    current_directory = os.getcwd()

    conventional_commits_next_version_path = current_directory + \
        "/../target/debug/conventional_commits_next_version"
    conventional_commits_next_version_command = conventional_commits_next_version_path + context.arguments

    os.chdir(context.temporary_directory.name)
    (context.exit_code, context.stdout) = execute_command(conventional_commits_next_version_command)

    os.chdir(current_directory)


@then('the returned version should be "{expected_version}".')
def compare_returned_and_expected_versions(context, expected_version):
    execute_conventional_commits_next_version(context)
    assert context.exit_code == 0
    assert context.stdout == expected_version


@then('the conventional_commits_next_version assertion passes.')
def compare_returned_and_expected_versions(context):
    execute_conventional_commits_next_version(context)
    assert int(context.exit_code) == 0


@then('the conventional_commits_next_version assertion fails.')
def compare_returned_and_expected_versions(context):
    execute_conventional_commits_next_version(context)
    assert int(context.exit_code) != 0


@then('the error message is "{error_message}".')
def then_the_error_message_is(context, error_message):
    execute_conventional_commits_next_version(context)
    assert starts_with(context.stdout, error_message)


@then('the error message is either "{error_message}" or "{error_message2}".')
def then_the_error_message_is_either(context, error_message, error_message2):
    execute_conventional_commits_next_version(context)
    assert starts_with(context.stdout, error_message) or starts_with(context.stdout, error_message2)


def starts_with(stdout, error_message):
    return stdout.strip().startswith(error_message.strip())
