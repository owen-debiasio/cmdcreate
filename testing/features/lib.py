"""Stores system utilities used by the script"""

import subprocess
import os


def command(cmd):
    """Function to run shell commands"""
    subprocess.run(cmd, shell=True, check=True)


"""Retrieve the home directory"""
home = os.path.expanduser("~")
