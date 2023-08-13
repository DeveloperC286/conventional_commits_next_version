def assert_command_successful(context):
    assert context.exit_code == 0, f"Expected a zero exit code to indicate a successful execution.\nExit code = '{context.exit_code}'.\n"


def assert_command_unsuccessful(context):
    assert context.exit_code != 0, f"Expected a non-zero exit code to indicate a unsuccessful execution\nExit code = '{context.exit_code}'.\n"


def assert_no_output(context):
    assert context.stdout == "", f"Expected standard output to be empty.\nStandard output = {context.stdout.encode()}.\n"


def assert_no_errors(context):
    assert context.stderr == "", f"Expected standard error to be empty.\nStandard error = {context.stderr.encode()}.\n"
