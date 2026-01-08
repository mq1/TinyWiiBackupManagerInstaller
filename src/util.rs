// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: GPL-3.0-only

use std::os::windows::process::CommandExt;
use std::{env, fs, io::Cursor, path::Path, process::Command};
use zip::ZipArchive;

pub async fn install(version: String, bytes: Vec<u8>) -> Result<String, String> {
    // Open the archive
    let cursor = Cursor::new(bytes);
    let mut archive = ZipArchive::new(cursor).map_err(|e| e.to_string())?;

    let appdata = env::var("APPDATA").map_err(|e| e.to_string())?;
    let localappdata = env::var("LOCALAPPDATA").map_err(|e| e.to_string())?;
    let userprofile = env::var("USERPROFILE").map_err(|e| e.to_string())?;
    let install_dir = Path::new(&localappdata).join("TinyWiiBackupManager");

    // Remove existing install
    if install_dir.exists() {
        fs::remove_dir_all(&install_dir).map_err(|e| e.to_string())?;
        fs::create_dir(&install_dir).map_err(|e| e.to_string())?;
    } else {
        fs::create_dir_all(&install_dir).map_err(|e| e.to_string())?;
    }

    // Extract the dist .zip into the install dir
    archive.extract(&install_dir).map_err(|e| e.to_string())?;

    // Find the executable
    let exe_path = install_dir.join("TinyWiiBackupManager.exe");
    if !exe_path.exists() {
        return Err("Executable not found in extracted archive".to_string());
    }

    // Create shortcut on the desktop
    let shortcut_path = Path::new(&userprofile)
        .join("Desktop")
        .join("TinyWiiBackupManager.lnk");

    if shortcut_path.exists() {
        fs::remove_file(&shortcut_path).map_err(|e| e.to_string())?;
    }

    let mut shortcut = mslnk::ShellLink::new(&exe_path).map_err(|e| e.to_string())?;
    shortcut.set_working_dir(Some(install_dir.to_string_lossy().to_string()));
    shortcut
        .create_lnk(&shortcut_path)
        .map_err(|e| e.to_string())?;

    // Create Start Menu shortcut
    let start_menu_dir = Path::new(&appdata)
        .join("Microsoft")
        .join("Windows")
        .join("Start Menu")
        .join("Programs")
        .join("TinyWiiBackupManager");

    let start_menu_shortcut = start_menu_dir.join("TinyWiiBackupManager.lnk");

    if start_menu_shortcut.exists() {
        fs::remove_file(&start_menu_shortcut).map_err(|e| e.to_string())?;
    } else {
        fs::create_dir_all(&start_menu_dir).map_err(|e| e.to_string())?;
    }

    let mut shortcut = mslnk::ShellLink::new(&exe_path).map_err(|e| e.to_string())?;
    shortcut.set_working_dir(Some(install_dir.to_string_lossy().to_string()));
    shortcut
        .create_lnk(&start_menu_shortcut)
        .map_err(|e| e.to_string())?;

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

pub async fn uninstall() -> Result<(), String> {
    let appdata = env::var("APPDATA").map_err(|e| e.to_string())?;
    let localappdata = env::var("LOCALAPPDATA").map_err(|e| e.to_string())?;
    let userprofile = env::var("USERPROFILE").map_err(|e| e.to_string())?;

    let install_dir = Path::new(&localappdata).join("TinyWiiBackupManager");
    if install_dir.exists() {
        fs::remove_dir_all(&install_dir).map_err(|e| e.to_string())?;
    }

    let shortcut_path = Path::new(&userprofile)
        .join("Desktop")
        .join("TinyWiiBackupManager.lnk");
    if shortcut_path.exists() {
        fs::remove_file(&shortcut_path).map_err(|e| e.to_string())?;
    }

    let start_menu_dir = Path::new(&appdata)
        .join("Microsoft")
        .join("Windows")
        .join("Start Menu")
        .join("Programs")
        .join("TinyWiiBackupManager");
    if start_menu_dir.exists() {
        fs::remove_dir_all(&start_menu_dir).map_err(|e| e.to_string())?;
    }

    let data_dir = Path::new(&appdata).join("mq1").join("TinyWiiBackupManager");
    if data_dir.exists() {
        fs::remove_dir_all(&data_dir).map_err(|e| e.to_string())?;
    }

    Ok(())
}

pub async fn download(version: String, os: Os, arch: Arch) -> Result<(String, Vec<u8>), String> {
    let url = format!(
        "https://github.com/mq1/TinyWiiBackupManager/releases/download/v{}/TinyWiiBackupManager-v{}-{}-{}.zip",
        &version,
        &version,
        os.as_str(),
        arch.as_str()
    );

    let bytes = minreq::get(&url)
        .send()
        .map_err(|e| e.to_string())?
        .into_bytes();

    Ok((version, bytes))
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
