#!/usr/bin/env bash
# uninstall_cmdcreate.sh - Uninstall cmdcreate binary
#
# This script removes the cmdcreate binary from /usr/bin.
# It provides a safe way to uninstall, printing a message if removal fails.

# Attempt to remove the binary; suppress errors if it doesn't exist
sudo rm -f /usr/bin/cmdcreate || echo "Failed to uninstall cmdcreate"

# Exit with a non-zero status to indicate the script ran
exit 1
