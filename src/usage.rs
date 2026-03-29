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
    let (blue, cyan, yellow, magenta, reset) = (
        COLORS.blue,
        COLORS.cyan,
        COLORS.yellow,
        COLORS.magenta,
        COLORS.reset,
    );

    let project_name = PROJECT.name;

    log(
        "usage::cmdcreate_usage(): Displaying usage information...",
        Severity::Normal,
    );

    for line in vec![
        // Version
        format!("{project_name} {CURRENT_PROJECT_VERSION}"),
        // Usage identifier
        format!(
            "Usage: {project_name} {magenta}(flags){reset} [{blue}command{reset}, {cyan}argument{reset}] {yellow}<args> {magenta}(flags){reset}"
        ),
        // Commands
        "\nCommands:".into(),
        format!("  {blue}create{yellow}   <command>    <contents>{reset}  Create a command"),
        format!("  {blue}remove {yellow}  <command>{reset}                Remove a command"),
        format!(
            "  {blue}edit   {yellow}  <command>{reset}                Modify contents of a command"
        ),
        format!("  {blue}list{reset}                              Display installed commands"),
        format!(
            "  {blue}search {yellow}  <command>{reset}                Searches for matched command"
        ),
        format!(
            "  {blue}display {yellow} <command>{reset}                Display contents of a command"
        ),
        format!("  {blue}rename {yellow}  <command>    <renamed>{reset}   Renames a command"),
        format!(
            "  {blue}favorite {yellow}<add/remove> <command>{reset}   Adds or removes a command from favorites"
        ),
        "\n  Update:".into(),
        format!("    {blue}check{reset}                           Checks for updates"),
        format!("    {blue}update{reset}                          Updates cmdcreate"),
        "\n  Backup:".into(),
        format!(
            "    {blue}export{reset} {yellow}<output directory>{reset}       Exports your installed commands"
        ),
        format!(
            "    {blue}import{reset} {yellow}<file input>{reset}             Imports your exported commands"
        ),
        // Arguments and flags
        "\nArguments and flags:".into(),
        format!("  {cyan}-v{reset},{cyan} --version {reset}                    Displays version"),
        format!("  {cyan}-c{reset},{cyan} --changelog {reset}                  Displays changelog"),
        format!("  {cyan}-l{reset},{cyan} --license {reset}                    Displays license"),
        format!(
            "  {cyan}-o{reset},{cyan} --offline {reset}                    Runs cmdcreate in offline mode"
        ),
        format!(
            "  {cyan}-m{reset},{cyan} --monochrome {reset}                 Disables colorized output"
        ),
        format!(
            "  {cyan}-V{reset},{cyan} --verbose {reset}                    Print logs to output"
        ),
        format!(
            "  {cyan}-f{reset},{cyan} --force {reset}                      Force commands to be ran"
        ),
        format!(
            "  {cyan}-b{reset},{cyan} --bypass-root {reset}                Bypass root requirement {yellow}(USE WITH CAUTION){reset}"
        ),
        // About section
        "\nAbout:".into(),
        format!("   {project_name} allows you to create custom commands for your Linux terminal"),
        "   without needing to enter the same \"complex\" commands over and over.".into(),
    ] {
        println!("{line}");
    }

    exit(1)
}
