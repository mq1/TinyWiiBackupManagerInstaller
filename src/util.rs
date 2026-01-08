// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: GPL-3.0-only

use std::{env, io, process::Command};

pub fn get_download_url(version: &str) -> io::Result<String> {
    let os = get_os()?;
    let arch = get_arch();

    let url = format!(
        "https://github.com/mq1/TinyWiiBackupManager/releases/download/v{}/TinyWiiBackupManager-v{}-{}-{}.zip",
        version, version, os, arch
    );

    Ok(url)
}

pub fn get_latest_version() -> Result<String, minreq::Error> {
    let version = minreq::get(
        "https://github.com/mq1/TinyWiiBackupManager/releases/latest/download/version.txt",
    )
    .send()?
    .as_str()?
    .to_string();

    Ok(version)
}

fn get_os() -> io::Result<&'static str> {
    let output = Command::new("cmd").args(["/c", "ver"]).output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);

    if stdout.contains("Version 10") {
        Ok("Windows")
    } else {
        Ok("Windows7")
    }
}

fn get_arch() -> &'static str {
    let arch_var = env::var("PROCESSOR_ARCHITECTURE").unwrap_or_default();
    let arch_wow = env::var("PROCESSOR_ARCHITEW6432").unwrap_or_default();

    if arch_var == "ARM64" || arch_wow == "ARM64" {
        return "ARM64";
    }

    if arch_var == "AMD64" || arch_wow == "AMD64" {
        return "x64";
    }

    "x86"
}
