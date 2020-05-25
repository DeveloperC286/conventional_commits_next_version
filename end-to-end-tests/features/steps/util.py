from subprocess import Popen, PIPE, STDOUT


def execute_command(command):
    process = Popen(
        command,
        shell=True,
        stdin=PIPE,
        stdout=PIPE,
        stderr=STDOUT)
    process.wait()

    if process.returncode != 0:
        raise RuntimeError('The command "'+command+'" failed.')
    else:
        return process.stdout.read().decode('utf-8')
