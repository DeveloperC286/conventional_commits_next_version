import os
import tempfile
from behave import given

from util import execute_command


@given('the arguments are reset.')
def reset_arguments(context):
    context.arguments = ""


@given('the context and environment are reset.')
def reset_context(context):
    context.behave_directory = os.getcwd()
    context.temporary_directory = tempfile.TemporaryDirectory()

    context.conventional_commits_next_version_path = context.behave_directory + \
        "/../target/debug/conventional_commits_next_version"
    reset_arguments(context)

    if "GIT_DIR" in os.environ:
        del os.environ["GIT_DIR"]


@given('the repository "{remote_repository}" is cloned and checked out at the commit "{commit_hash}".')
def clone_remote_repository_and_checkout_commit(
        context, remote_repository, commit_hash):
    reset_context(context)

    os.chdir(context.temporary_directory.name)
    (exit_code, _) = execute_command(
        "git clone " + remote_repository + " .")
    assert exit_code == 0
    (exit_code, _) = execute_command("git checkout " + commit_hash)
    assert exit_code == 0

    os.chdir(context.behave_directory)


@given('the directory is changed to the cloned repository.')
def change_into_git_dir(context):
    os.chdir(context.temporary_directory.name)


@given('the GIT_DIR environment variable is set to the cloned repository.')
def set_git_dir(context):
    os.environ["GIT_DIR"] = str(context.temporary_directory.name + "/.git")
