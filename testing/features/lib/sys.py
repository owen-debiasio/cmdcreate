import subprocess


def command(command):
    subprocess.run(command, shell=True)
