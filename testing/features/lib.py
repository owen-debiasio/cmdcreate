import subprocess
import os


def command(cmd):
    subprocess.run(cmd, shell=True, check=True)


home = os.path.expanduser("~")
