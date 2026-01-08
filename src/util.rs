// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: GPL-3.0-only

use std::{env, io, process::Command};

pub fn get_download_url(version: &str, os: Os, arch: Arch) -> io::Result<String> {
    let url = format!(
        "https://github.com/mq1/TinyWiiBackupManager/releases/download/v{}/TinyWiiBackupManager-v{}-{}-{}.zip",
        version,
        version,
        os.as_str(),
        arch.as_str()
    );

    Ok(url)
}

pub async fn get_latest_version() -> Result<String, String> {
    let version = minreq::get(
        "https://github.com/mq1/TinyWiiBackupManager/releases/latest/download/version.txt",
    )
    .send()
    .map_err(|e| e.to_string())?
    .as_str()
    .map_err(|e| e.to_string())?
    .to_string();

    Ok(version)
}

#[derive(Clone, Debug)]
pub enum Os {
    Windows,
    Windows7,
}

impl Os {
    pub fn as_str(&self) -> &'static str {
        match self {
            Os::Windows => "windows",
            Os::Windows7 => "windows7",
        }
    }

    pub fn as_display_str(&self) -> &'static str {
        match self {
            Os::Windows => "Windows 10+",
            Os::Windows7 => "Windows 7/8/8.1",
        }
    }
}

pub fn get_os() -> Os {
    if Command::new("cmd")
        .args(["/c", "ver"])
        .output()
        .is_ok_and(|o| String::from_utf8_lossy(&o.stdout).contains("Version 10"))
    {
        Os::Windows
    } else {
        Os::Windows7
    }
}

#[derive(Clone, Debug)]
pub enum Arch {
    I686,
    X86_64,
    AARCH64,
}

impl Arch {
    pub fn as_str(&self) -> &'static str {
        match self {
            Arch::I686 => "x86",
            Arch::X86_64 => "x86_64",
            Arch::AARCH64 => "arm64",
        }
    }

    pub fn as_display_str(&self) -> &'static str {
        match self {
            Arch::I686 => "x86 (32-bit)",
            Arch::X86_64 => "x86 (64-bit)",
            Arch::AARCH64 => "ARM64",
        }
    }
}

pub fn get_arch() -> Arch {
    match env::var("PROCESSOR_ARCHITEW6432").as_deref() {
        Ok("AMD64") => Arch::X86_64,
        Ok("ARM64") => Arch::AARCH64,
        _ => Arch::I686,
    }
}
