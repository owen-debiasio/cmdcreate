import subprocess, os


def command(command):
    subprocess.run(command, shell=True)


home = os.path.expanduser("~")
