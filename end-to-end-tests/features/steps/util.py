from subprocess import Popen, PIPE, STDOUT


def execute_command(command):
    process = Popen(
        command,
        shell=True,
        stdin=PIPE,
        stdout=PIPE,
        stderr=STDOUT,
        close_fds=True)
    process.wait()
    return process.stdout.read().decode('utf-8')
