def assert_command_successful(context):
    assert context.exit_code == 0, f"Expected a zero exit code to indicate a successful execution.\nExit code = '{context.exit_code}'.\n"
