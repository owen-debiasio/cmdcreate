import subprocess
import os


def command(cmd):
    try:
        subprocess.run(cmd, shell=True, check=True)
    except subprocess.CalledProcessError:
        print(f"Error: Command {cmd} failed... exiting...")
        exit(1)

home = os.path.expanduser("~")
