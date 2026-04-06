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
    utils::colors::COLORS,
    version::CURRENT_PROJECT_VERSION,
};
use std::process::exit;

pub fn cmdcreate_usage() {
    let (blue, cyan, yellow, magenta, red) = (
        COLORS.blue,
        COLORS.cyan,
        COLORS.yellow,
        COLORS.magenta,
        COLORS.reset,
    );

    log(
        "usage::cmdcreate_usage(): Displaying usage information...",
        Severity::Normal,
    );

    let project_name = PROJECT.name;
    let version = CURRENT_PROJECT_VERSION;

    println!(
        "
{project_name} {version}
Usage: {project_name} {magenta}(flags){red} [{blue}command{red}, {cyan}argument{red}] {yellow}<args> {magenta}(flags){red}

Commands:
  {blue}create{yellow}   <command>    <contents>{red}  Create a command
  {blue}remove{yellow}   <command>{red}                Remove a command
  {blue}edit{yellow}     <command>{red}                Modify contents of a command
  {blue}list{red}                              Display installed commands
  {blue}search{yellow}   <command>{red}                Searches for matched command
  {blue}display{yellow}  <command>{red}                Display contents of a command
  {blue}rename{yellow}   <command>    <renamed>{red}   Renames a command
  {blue}favorite{yellow} <add/remove> <command>{red}   Adds or removes a command from favorites

  Update:
    {blue}check{red}                           Checks for updates
    {blue}update{red}                          Updates cmdcreate

  Backup:
    {blue}export{red} {yellow}<output directory>{red}       Exports your installed commands
    {blue}import{red} {yellow}<file input>{red}             Imports your exported commands

Arguments and flags:
  {cyan}-v{red}, {cyan}--version{red}                     Displays version
  {cyan}-c{red}, {cyan}--changelog{red}                   Displays changelog
  {cyan}-l{red}, {cyan}--license{red}                     Displays license
  {cyan}-o{red}, {cyan}--offline{red}                     Runs cmdcreate in offline mode
  {cyan}-m{red}, {cyan}--monochrome{red}                  Disables colorized output
  {cyan}-V{red}, {cyan}--verbose{red}                     Print logs to output
  {cyan}-f{red}, {cyan}--force{red}                       Force commands to be ran
  {cyan}-b{red}, {cyan}--bypass-root{red}                 Bypass root requirement {yellow}(USE WITH CAUTION){red}

About:
   {project_name} allows you to create custom commands for your Linux terminal
   without needing to enter the same \"complex\" commands over and over.",
    );

    exit(1)
}
