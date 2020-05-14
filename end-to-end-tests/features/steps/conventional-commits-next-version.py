import os


from util import execute_command
from subprocess import Popen, PIPE, STDOUT
from behave import *


@when(
    'I call conventional-commits-next-version with the from commit "{commit_hash}" and version "{version}".')
def execute_conventional_commits_next_version(context, commit_hash, version):
    execute_conventional_commits_next_version(context, commit_hash, version, False)


@when(
    'I call conventional-commits-next-version with the from commit "{commit_hash}" and version "{version}" and batch-commits flag.')
def execute_batch_conventional_commits_next_version(context, commit_hash, version):
    execute_conventional_commits_next_version(context, commit_hash, version, True)

def execute_conventional_commits_next_version(context, commit_hash, version, batch_commits):
    current_directory = os.getcwd()
    conventional_commits_next_version_path = current_directory + \
        "/../target/debug/conventional_commits_next_version"

    conventional_commits_next_version_command = conventional_commits_next_version_path + \
        " --from-commit " + commit_hash + " --version " + version

    if batch_commits:
        conventional_commits_next_version_command += " --batch-commits "

    os.chdir(context.temporary_directory.name)
    context.returned = execute_command(
        conventional_commits_next_version_command)

    os.chdir(current_directory)



@then('the returned version should be "{expected_version}".')
def compare_returned_and_expected_versions(context, expected_version):
    print(context.returned + " == " + expected_version)
    assert context.returned == expected_version
