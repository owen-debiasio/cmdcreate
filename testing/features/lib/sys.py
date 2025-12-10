import subprocess


# Run a shell command
def command(command):
    subprocess.run(command, shell=True)
