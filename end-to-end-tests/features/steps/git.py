import os


from util import execute_command
from tempfile import TemporaryDirectory
from behave import *


@given('the repository "{remote_repository}" is cloned and checked out at the commit "{commit_hash}".')
def clone_remote_repository_and_checkout_commit(context, remote_repository, commit_hash):
    current_directory = os.getcwd()

    context.temporary_directory = TemporaryDirectory()
    os.chdir(context.temporary_directory.name)

    (returncode, returned_version) = execute_command("git clone " + remote_repository + " .")
    assert returncode == 0
    (returncode, returned_version) = execute_command("git checkout " + commit_hash)
    assert returncode == 0

    os.chdir(current_directory)
