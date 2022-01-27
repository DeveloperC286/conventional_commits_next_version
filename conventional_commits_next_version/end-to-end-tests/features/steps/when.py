from behave import when


@when(
    'the flag --from-stdin is set and the standard input is "{standard_input}".')
def set_from_stdin(context, standard_input):
    context.standard_input = standard_input.strip()
    context.pre_command = "echo " + context.standard_input + " | "
    context.arguments += " --from-stdin "


@when('the argument --from-commit-hash is provided as "{from_commit_hash}".')
def set_from_commit_hash(context, from_commit_hash):
    context.arguments += " --from-commit-hash " + from_commit_hash + " "


@when('the argument --from-version is provided as "{from_version}".')
def set_from_version(context, from_version):
    context.arguments += " --from-version " + from_version + " "


@when('the --batch-commits flag is set.')
def set_batch_commits_flag(context):
    context.arguments += " --batch-commits "


@when('the argument --monorepo is provided as "{monorepo}".')
def set_monorepo(context, monorepo):
    context.arguments += " --monorepo " + monorepo + " "


@when('the argument --current-version is provided as "{current_version}".')
def set_current_version(context, current_version):
    context.arguments += " --current-version " + current_version + " "


@when('the argument --from-reference is provided as "{from_reference}".')
def set_from_reference(context, from_reference):
    context.arguments += " --from-reference " + from_reference + " "
