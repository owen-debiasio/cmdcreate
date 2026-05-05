// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Owen Debiasio
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use crate::{
    logger::{Severity, log},
    meta::project_information::PROJECT,
    output,
    utils::colors::COLORS,
    version::CURRENT_PROJECT_VERSION,
};
use std::process::exit;

pub fn cmdcreate_usage() {
    let (blue, cyan, yellow, reset) = (COLORS.blue, COLORS.cyan, COLORS.yellow, COLORS.reset);

    log(
        "usage::cmdcreate_usage(): Displaying usage information...",
        Severity::Normal,
    );

    let project_name = PROJECT.name;

    output!(
        "
{project_name} {reset}{CURRENT_PROJECT_VERSION}
Usage: {blue}{project_name} {cyan}<flag(s)>{reset} [{blue}command{reset}, {yellow}argument(s){reset}]

Commands:
  {blue}create  {reset}   Create a command
  {blue}remove  {reset}   Remove a command
  {blue}edit    {reset}   Modify contents of a command
  {blue}list    {reset}   Display installed commands
  {blue}search  {reset}   Searches for matched command
  {blue}display {reset}   Display contents of a command
  {blue}rename  {reset}   Renames a command
  {blue}favorite{reset}   Adds or removes a command from favorites
  {blue}config  {reset}   Manage your configurations for cmdcreate
  {blue}doc     {reset}   View various documentation references

  Update:
    {blue}check {reset}   Checks for updates
    {blue}update{reset}   Updates cmdcreate

  Backup:
    {blue}export{reset}   Exports your installed commands
    {blue}import{reset}   Imports your exported commands

Flags:
  {cyan}-v{reset}, {cyan}--version    {reset}   Displays version
  {cyan}-o{reset}, {cyan}--offline    {reset}   Runs cmdcreate in offline mode
  {cyan}-m{reset}, {cyan}--monochrome {reset}   Disables colorized output
  {cyan}-V{reset}, {cyan}--verbose    {reset}   Print logs to output
  {cyan}-s{reset}, {cyan}--silent     {reset}   Suppress any non-error output
  {cyan}-f{reset}, {cyan}--force      {reset}   Force commands to be ran
  {cyan}-b{reset}, {cyan}--bypass-root{reset}   Bypass root requirement {yellow}(USE WITH CAUTION){reset}

About:
   {project_name} allows you to create custom commands for your Linux terminal
   without needing to enter the same \"complex\" commands over and over.",
   false
    );

    exit(1)
}
