import os


from util import execute_command
from tempfile import TemporaryDirectory
from behave import *


@given('the repository "{remote_repository}" is cloned and checked out at the commit "{commit_hash}".')
def clone_remote_repository_and_checkout_commit(context, remote_repository, commit_hash):
    current_directory = os.getcwd()

    context.temporary_directory = TemporaryDirectory()
    os.chdir(context.temporary_directory.name)

    if "GIT_DIR" in os.environ:
        del os.environ["GIT_DIR"]

    (exit_code, stdout) = execute_command("git clone " + remote_repository + " .")
    assert exit_code == 0
    (exit_code, stdout) = execute_command("git checkout " + commit_hash)
    assert exit_code == 0

    os.chdir(current_directory)


@given('the GIT_DIR environment variable is set to the cloned repository and the working directory is changed.')
def set_git_dir(context):
    os.environ["GIT_DIR"] = str(context.temporary_directory.name + "/.git")

    context.temporary_directory.name = os.getcwd()
