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

use crate::utils::fs::read_file_to_string;

#[derive(Debug)]
pub enum InstallMethod {
    Aur,
    Dpkg,
    Other,
    Rpm,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DistroBase {
    Arch,
    Debian,
    Fedora,
    Unknown,
}

/// This function is kind of fucked so bear with me here
pub fn get_distro_base() -> DistroBase {
    let (mut distro_id, mut distro_id_alt) = ("", "");

    let os_release = read_file_to_string("/etc/os-release").to_lowercase();

    for line in os_release.lines() {
        if let Some(distro_release_version) = line.strip_prefix("id=") {
            distro_id = distro_release_version.trim_matches('"');
        } else if let Some(distro_release_version_alt) = line.strip_prefix("id_like=") {
            distro_id_alt = distro_release_version_alt.trim_matches('"');
        }
    }

    let distro_base = format!("{distro_id} {distro_id_alt}");

    if distro_base.contains("arch") || distro_base.contains("manjaro") {
        DistroBase::Arch
    } else if distro_base.contains("fedora")
        || distro_base.contains("rhel")
        || distro_base.contains("centos")
    {
        DistroBase::Fedora
    } else if distro_base.contains("debian")
        || distro_base.contains("ubuntu")
        || distro_base.contains("linuxmint")
    {
        DistroBase::Debian
    } else {
        DistroBase::Unknown
    }
}

pub fn installation_method() -> InstallMethod {
    match get_distro_base() {
        DistroBase::Arch => InstallMethod::Aur,
        DistroBase::Fedora => InstallMethod::Rpm,
        DistroBase::Debian => InstallMethod::Dpkg,
        DistroBase::Unknown => InstallMethod::Other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distro_detection_returns_known_or_unknown() {
        let distro_base = get_distro_base();
        assert!(
            distro_base == DistroBase::Arch
                || distro_base == DistroBase::Fedora
                || distro_base == DistroBase::Debian
                || distro_base == DistroBase::Unknown
        );
    }
}
