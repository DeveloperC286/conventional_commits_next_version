import os


from util import execute_command
from tempfile import TemporaryDirectory
from behave import *


@given('I clone the repository "{remote_repository}".')
def clone_remote_repository(context, remote_repository):
    current_directory = os.getcwd()

    context.temporary_directory = TemporaryDirectory()
    os.chdir(context.temporary_directory.name)
    clone_command = "git clone "+remote_repository+" ."
    execute_command(clone_command)

    os.chdir(current_directory)


@given('I checkout the commit "{commit_hash}".')
def checkout_commit(context, commit_hash):
    current_directory = os.getcwd()

    os.chdir(context.temporary_directory.name)
    checkout_command = "git checkout "+commit_hash
    execute_command(checkout_command)

    os.chdir(current_directory)
