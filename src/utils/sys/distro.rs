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

use std::collections::HashMap;

use crate::utils::fs::{core::read_write::read_file_to_string, paths::path_exists};

/// This function is kind of fucked so bear with me here
pub fn get_distro_base() -> &'static str {
    let (mut distro_id, mut distro_id_alt) = ("", "");

    let os_release = read_file_to_string("/etc/os-release").to_lowercase();

    for line in os_release.lines() {
        if let Some(distro_release_version) = line.strip_prefix("id=") {
            distro_id = distro_release_version.trim_matches('"');
        } else if let Some(distro_release_version_alt) = line.strip_prefix("id_like=") {
            distro_id_alt = distro_release_version_alt.trim_matches('"');
        }
    }

    let distro_base = &format!("{distro_id} {distro_id_alt}");

    return if matches!(distro_base.as_str(), "arch" | "manjaro" | "endeavouros") {
        "Arch"
    } else if matches!(distro_base.as_str(), "fedora" | "rhel" | "centos" | "amzn") {
        "Fedora"
    } else if matches!(distro_base.as_str(), "debian" | "ubuntu" | "linuxmint" | "kali" | "pop") {
        "Debian"
    } else {
        "Unknown"
    }
}

pub fn is_immutable_distro() -> bool {
    if is_usr_directory_readonly() || path_exists("/run/ostree-booted") {
        return true;
    }

    let os_release_contents = read_file_to_string("/etc/os-release");
    check_os_release_immutability(&os_release_contents)
}

fn check_os_release_immutability(contents: &str) -> bool {
    if contents.is_empty() {
        return false;
    }

    let os_properties: HashMap<String, String> = contents
        .lines()
        .filter_map(|line| line.split_once('='))
        .map(|(key, value)| {
            (
                key.trim().to_lowercase(),
                value.trim_matches('"').to_lowercase(),
            )
        })
        .collect();

    let variant_id = os_properties.get("variant_id").map(String::as_str);
    let distro_id = os_properties.get("id").map(String::as_str);

    matches!(
        variant_id,
        Some("silverblue" | "kinoite" | "sericea" | "carbon" | "steamdeck")
    ) || matches!(
        distro_id,
        Some("opensuse-microos" | "opensuse-tumbleweed-microos")
    )
}

fn check_mounts_immutability(contents: &str) -> bool {
    if contents.is_empty() {
        return false;
    }

    for mount_line in contents.lines() {
        let mount_segments: Vec<&str> = mount_line.split_whitespace().collect();

        if mount_segments.len() >= 4 {
            let mount_point = mount_segments[1];

            if mount_point == "/usr" || mount_point == "/" {
                let mount_options = mount_segments[3];

                if mount_options == "ro"
                    || mount_options.starts_with("ro,")
                    || mount_options.contains(",ro,")
                    || mount_options.ends_with(",ro")
                {
                    return true;
                }
            }
        }
    }

    false
}

fn is_usr_directory_readonly() -> bool {
    let active_mounts_contents = read_file_to_string("/proc/mounts");
    check_mounts_immutability(&active_mounts_contents)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_os_release_returns_false() {
        assert!(!check_os_release_immutability(""));
    }

    #[test]
    fn standard_ubuntu_os_release_returns_false() {
        let ubuntu_mock = r#"
            NAME="Ubuntu"
            VERSION="24.04 LTS (Noble Numbat)"
            ID=ubuntu
            ID_LIKE=debian
            PRETTY_NAME="Ubuntu 24.04 LTS"
        "#;
        assert!(!check_os_release_immutability(ubuntu_mock));
    }

    #[test]
    fn steamdeck_os_release_returns_true() {
        let steamdeck_mock = r#"
            NAME="SteamOS"
            ID=steamos
            ID_LIKE="arch"
            PRETTY_NAME="SteamOS"
            VARIANT_ID=steamdeck
        "#;
        assert!(check_os_release_immutability(steamdeck_mock));
    }

    #[test]
    fn fedora_silverblue_os_release_returns_true() {
        let silverblue_mock = r#"
            NAME="Fedora Linux"
            VERSION="40 (Silverblue)"
            ID=fedora
            VARIANT_ID="silverblue"
        "#;
        assert!(check_os_release_immutability(silverblue_mock));
    }

    #[test]
    fn opensuse_microos_os_release_returns_true() {
        let microos_mock = r#"
            NAME="openSUSE MicroOS"
            ID="opensuse-microos"
            PRETTY_NAME="openSUSE MicroOS"
        "#;
        assert!(check_os_release_immutability(microos_mock));
    }

    #[test]
    fn standard_rw_mounts_returns_false() {
        let rw_mounts_mock = r"
            sysfs /sys sysfs rw,nosuid,nodev,noexec,relatime 0 0
            proc /proc proc rw,nosuid,nodev,noexec,relatime 0 0
            /dev/nvme0n1p2 / ext4 rw,relatime,errors=remount-ro 0 0
            /dev/nvme0n1p2 /usr ext4 rw,relatime 0 0
        ";
        assert!(!check_mounts_immutability(rw_mounts_mock));
    }

    #[test]
    fn readonly_root_mount_returns_true() {
        let ro_root_mock = r"
            sysfs /sys sysfs rw,nosuid,nodev,noexec 0 0
            /dev/loop0 / squashfs ro,relatime,errors=continue 0 0
        ";
        assert!(check_mounts_immutability(ro_root_mock));
    }

    #[test]
    fn test_readonly_usr_mount_variants_returns_true() {
        // Checks mid-string options flag containing 'ro'
        let ro_usr_middle = "some_dev /usr ext4 rw,ro,nosuid 0 0";
        // Checks end-string options flag ending with 'ro'
        let ro_usr_end = "some_dev /usr ext4 rw,nosuid,ro 0 0";
        // Checks plain single 'ro' flag
        let ro_usr_plain = "some_dev /usr ext4 ro 0 0";

        assert!(check_mounts_immutability(ro_usr_middle));
        assert!(check_mounts_immutability(ro_usr_end));
        assert!(check_mounts_immutability(ro_usr_plain));
    }

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
