# -*- coding: utf-8 -*-

# SPDX-License-Identifier: GPL-3.0-or-later
# Copyright (C) 2026 Owen Debiasio
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

from testing.lib import command
from testing.shared import enter_to_continue, simple_init, simple_cleanup


def test():
    command("clear")

    print("\nRunning tests: Command creation\n")

    simple_init()

    print("\nRunning created command...\n\n")
    command("test_command")

    enter_to_continue()

    simple_cleanup()


if __name__ == "__main__":
    test()
