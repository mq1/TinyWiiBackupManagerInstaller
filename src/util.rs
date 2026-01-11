// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: GPL-3.0-only

use anyhow::{Result, anyhow};
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use std::fs::File;
use std::io::{self, Write};
use std::os::windows::process::CommandExt;
use std::process::Stdio;
use std::{env, fs, io::Cursor, path::Path, process::Command};
use zip::ZipArchive;

const POSTINSTALL_PS1: &str = include_str!("../postinstall.ps1");
const UNINSTALL_PS1: &[u8] = include_bytes!("../uninstall.ps1");

pub async fn install(version: String, bytes: Vec<u8>) -> Result<String> {
    // Open the archive
    let cursor = Cursor::new(bytes);
    let mut archive = ZipArchive::new(cursor)?;

    let localappdata = env::var("LOCALAPPDATA")?;
    let install_dir = Path::new(&localappdata).join("TinyWiiBackupManager");

    // Remove existing install
    if install_dir.exists() {
        fs::remove_dir_all(&install_dir)?;
        fs::create_dir(&install_dir)?;
    } else {
        fs::create_dir_all(&install_dir)?;
    }

    // Extract the dist .zip into the install dir
    let mut archived_exe = archive.by_name("TinyWiiBackupManager.exe")?;
    let mut file = File::create(install_dir.join("TinyWiiBackupManager.exe"))?;
    io::copy(&mut archived_exe, &mut file)?;

    // Write the uninstaller script
    fs::write(install_dir.join("uninstall.ps1"), UNINSTALL_PS1)?;

    // Run postinstall script
    let postinstall = POSTINSTALL_PS1.replace("TWBM_VERSION", &version);

    let mut child = Command::new("powershell")
        .arg("-NoProfile")
        .arg("-NonInteractive")
        .arg("-Command")
        .arg("-")
        .creation_flags(0x08000000)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    {
        let stdin = child.stdin.as_mut().ok_or(anyhow!("failed to get stdin"))?;
        stdin.write_all(postinstall.as_bytes())?;
    }
    child.wait()?;

    Ok(version)
}

pub fn is_installed() -> bool {
    let localappdata = match env::var("LOCALAPPDATA") {
        Ok(localappdata) => localappdata,
        Err(_) => return false,
    };

    Path::new(&localappdata)
        .join("TinyWiiBackupManager")
        .exists()
}

pub async fn download(version: String, os: Os, arch: Arch) -> Result<(String, Vec<u8>)> {
    let url = format!(
        "https://github.com/mq1/TinyWiiBackupManager/releases/download/v{}/TinyWiiBackupManager-v{}-{}-{}.zip",
        &version,
        &version,
        os.as_str(),
        arch.as_str()
    );

    let bytes = minreq::get(&url).send()?.into_bytes();

    Ok((version, bytes))
}

pub async fn get_latest_version() -> Result<String> {
    let version = minreq::get(
        "https://github.com/mq1/TinyWiiBackupManager/releases/latest/download/version.txt",
    )
    .send()?
    .as_str()?
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
        .creation_flags(0x08000000)
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
    X86_64v3,
    Aarch64,
}

impl Arch {
    pub fn as_str(&self) -> &'static str {
        match self {
            Arch::I686 => "x86",
            Arch::X86_64 => "x86_64",
            Arch::X86_64v3 => "x86_64-v3",
            Arch::Aarch64 => "arm64",
        }
    }

    pub fn as_display_str(&self) -> &'static str {
        match self {
            Arch::I686 => "x86 (32-bit)",
            Arch::X86_64 => "x86 (64-bit)",
            Arch::X86_64v3 => "x86 (64-bit with AVX2 instructions)",
            Arch::Aarch64 => "ARM64",
        }
    }
}

pub fn get_arch() -> Arch {
    match env::var("PROCESSOR_ARCHITEW6432").as_deref() {
        Ok("AMD64") => {
            if std::is_x86_feature_detected!("avx2")
                && std::is_x86_feature_detected!("fma")
                && std::is_x86_feature_detected!("bmi2")
            {
                Arch::X86_64v3
            } else {
                Arch::X86_64
            }
        }
        Ok("ARM64") => Arch::Aarch64,
        _ => Arch::I686,
    }
}

pub fn launch_twbm() -> Result<()> {
    let install_dir = Path::new(&env::var("LOCALAPPDATA")?).join("TinyWiiBackupManager");
    let exe_path = install_dir.join("TinyWiiBackupManager.exe");

    Command::new("cmd")
        .args(["/C", "start", "/B", &exe_path.to_string_lossy()])
        .current_dir(install_dir)
        .creation_flags(0x08000000) // CREATE_NO_WINDOW (run invisibly)
        .spawn()?;

    Ok(())
}
