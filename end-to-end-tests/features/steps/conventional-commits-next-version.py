import os


from util import execute_command
from subprocess import Popen, PIPE, STDOUT
from behave import *


@when(
    'the --from-commit-hash and --from-version arguments are set as "{from_commit_hash}" and "{from_version}".')
def set_from_commit_hash_and_from_version(context, from_commit_hash, from_version):
    context.arguments = " --from-commit-hash " + from_commit_hash + " --from-version " + from_version + " "


@when(
    'the --batch-commits flag is set.')
def set_batch_commits_flag(context):
    context.arguments += " --batch-commits "


@when(
    'the --current-version argument is set as "{current_version}".')
def set_current_version(context, current_version):
    context.arguments += " --current-version " + current_version + " "


@when(
    'conventional_commits_next_version is executed with the set arguments.')
def execute_conventional_commits_next_version(context):
    current_directory = os.getcwd()

    conventional_commits_next_version_path = current_directory + \
        "/../target/debug/conventional_commits_next_version"
    conventional_commits_next_version_command = conventional_commits_next_version_path + context.arguments

    os.chdir(context.temporary_directory.name)
    (context.exit_code, context.returned_version) = execute_command(
        conventional_commits_next_version_command)

    os.chdir(current_directory)


@then('the returned version should be "{expected_version}".')
def compare_returned_and_expected_versions(context, expected_version):
    assert context.exit_code == 0
    assert context.returned_version == expected_version


@then('the conventional_commits_next_version assertion passes.')
def compare_returned_and_expected_versions(context):
    assert int(context.exit_code) == 0

@then('the conventional_commits_next_version assertion fails.')
def compare_returned_and_expected_versions(context):
    assert int(context.exit_code) != 0
