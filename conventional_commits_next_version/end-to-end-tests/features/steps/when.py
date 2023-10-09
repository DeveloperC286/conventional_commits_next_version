import tempfile
from behave import when


@when('the flag --from-stdin is set and the standard input is "{standard_input}".')
def set_from_stdin(context, standard_input):
    context.standard_input = standard_input.strip()
    context.pre_command = f"echo {context.standard_input} | "
    context.arguments += " --from-stdin "
    # Testing we can use stdin when not in a Git repository.
    # https://gitlab.com/DeveloperC/conventional_commits_linter/-/issues/3
    context.remote_repository_cache = tempfile.mkdtemp()


@when('the argument --from-reference is provided as "{from_reference}".')
def set_from_reference(context, from_reference):
    context.arguments += f" --from-reference {from_reference} "


@when('the argument --from-commit-hash is provided as "{from_commit_hash}".')
def set_from_commit_hash(context, from_commit_hash):
    context.arguments += f" --from-commit-hash {from_commit_hash} "


@when('the argument --monorepo is provided as "{monorepo}".')
def set_monorepo(context, monorepo):
    context.arguments += f" --monorepo {monorepo} "


@when('the argument --git-history-mode is provided as "{git_history_mode}".')
def set_batch_commits_flag(context, git_history_mode):
    context.arguments += f" --git-history-mode {git_history_mode} "


@when('the argument --from-version is provided as "{from_version}".')
def set_from_version(context, from_version):
    context.arguments += f" --from-version {from_version} "


@when('the argument --calculation-mode is provided as "{calculation_mode}".')
def set_batch_commits_flag(context, calculation_mode):
    context.arguments += f" --calculation-mode {calculation_mode} "


@when('the argument --current-version is provided as "{current_version}".')
def set_current_version(context, current_version):
    context.arguments += f" --current-version {current_version} "
