def assert_command_successful(result):
    assert result.exit_code == 0, f"Expected a zero exit code to indicate a successful execution.\nExit code = '{result.exit_code}'.\n"


def assert_command_unsuccessful(result):
    assert result.exit_code != 0, f"Expected a non-zero exit code to indicate a unsuccessful execution\nExit code = '{result.exit_code}'.\n"


def assert_no_output(result):
    assert result.stdout == "", f"Expected standard output to be empty.\nStandard output = {result.stdout.encode()}.\n"


def assert_no_errors(result):
    assert result.stderr == "", f"Expected standard error to be empty.\nStandard error = {result.stderr.encode()}.\n"


def assert_error_equals(result, error):
    assert result.stderr == error, f"Expected standard error to equal the error.\nStandard error = {result.stderr.encode()}.\nError          = {error.encode()}.\n"


def assert_error_matches_regex(result, regex):
    assert regex.match(
        result.stderr) is not None, f"Expected standard errors to match the regex.\nStandard error = {result.stderr.encode()}.\nRegex          = {regex.pattern.encode()}.\n"


def assert_error_is_one_of(result, errors):
    assert result.stderr in errors, f"Expected standard error to equal one of these errors.\nStandard error = {result.stderr.encode()}.\nErrors         = {errors}.\n"


def assert_next_version(result, expected_next_version):
    assert result.stdout == expected_next_version, f"The next version was not what was expected.\nExpected = {expected_next_version}.\nActual   = {result.stdout}\n"
