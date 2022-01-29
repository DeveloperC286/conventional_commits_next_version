import os

from subprocess import Popen, PIPE


def execute_command(command):
    process = Popen(
        command,
        shell=True,
        stdin=PIPE,
        stdout=PIPE,
        stderr=PIPE)
    process.wait()

    stdout, stderr = process.communicate()
    return process.returncode, stdout.decode("utf-8"), stderr.decode("utf-8")


def execute_conventional_commits_next_version(context):
    os.chdir(context.remote_repository_cache)
    (context.exit_code, context.stdout, context.stderr) = execute_command(
        context.pre_command + context.conventional_commits_next_version_path + context.arguments)
    os.chdir(context.behave_directory)
