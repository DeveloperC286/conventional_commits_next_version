import tempfile
from behave import when


@when('linting the "{commit_message}".')
def set_linting_the(context, commit_message):
    context.commit_message = commit_message.strip()
    context.pre_command = f"echo -e {context.commit_message} | "
    context.from_ref = " \"-\""
    # Testing we can use stdin when not in a Git repository.
    # https://gitlab.com/DeveloperC/conventional_commits_linter/-/issues/3
    context.remote_repository_cache = tempfile.mkdtemp()


@when('linting from the "{git}".')
def set_linting_from_the(context, git):
    context.from_ref = f" \"{git}\""


@when('the argument --monorepo is provided as "{monorepo}".')
def set_monorepo(context, monorepo):
    context.arguments += f" --monorepo {monorepo} "


@when('the argument --history-mode is provided as "{history_mode}".')
def set_history_mode(context, history_mode):
    context.arguments += f" --history-mode {history_mode} "


@when('the argument --from-version is provided as "{from_version}".')
def set_from_version(context, from_version):
    context.arguments += f" --from-version {from_version} "


@when('the argument --calculation-mode is provided as "{calculation_mode}".')
def set_calculation_mode(context, calculation_mode):
    context.arguments += f" --calculation-mode {calculation_mode} "


@when('the argument --current-version is provided as "{current_version}".')
def set_current_version(context, current_version):
    context.arguments += f" --current-version {current_version} "
