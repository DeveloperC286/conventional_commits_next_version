import os


from util import execute_command
from tempfile import TemporaryDirectory
from behave import *


@given('I clone the repository "{remote_repository}" and checkout the commit "{commit_hash}".')
def clone_remote_repository(context, remote_repository, commit_hash):
    current_directory = os.getcwd()

    context.temporary_directory = TemporaryDirectory()
    os.chdir(context.temporary_directory.name)

    execute_command("git clone " + remote_repository + " .")
    execute_command("git checkout " + commit_hash)

    os.chdir(current_directory)
