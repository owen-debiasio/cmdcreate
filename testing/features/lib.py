import subprocess
import os
import sys


def command(cmd):
    try:
        subprocess.run(cmd, shell=True, check=True)
    except subprocess.CalledProcessError:
        print(f"Error: Command {cmd} failed... exiting...")
        sys.exit(1)


home = os.path.expanduser("~")
