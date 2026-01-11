// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: GPL-3.0-only

use anyhow::{Result, anyhow};
use directories::{BaseDirs, UserDirs};
use mslnk::ShellLink;
use std::fs::File;
use std::io;
use std::os::windows::process::CommandExt;
use std::path::PathBuf;
use std::{env, fs, io::Cursor, process::Command};
use windows_registry::{CURRENT_USER, LOCAL_MACHINE};
use zip::ZipArchive;

const UNINSTALL_PS1: &[u8] = include_bytes!("../uninstall.ps1");

pub async fn install(version: String, bytes: Vec<u8>) -> Result<String> {
    let base_dirs = BaseDirs::new().ok_or(anyhow!("Failed to get base dirs"))?;
    let user_dirs = UserDirs::new().ok_or(anyhow!("Failed to get user dirs"))?;
    let install_dir = base_dirs.data_local_dir().join("TinyWiiBackupManager");
    let install_dir_str = install_dir
        .to_str()
        .ok_or(anyhow!("Failed to get install dir"))?;
    let exe_path = install_dir.join("TinyWiiBackupManager.exe");
    let exe_path_str = exe_path.to_str().ok_or(anyhow!("Failed to get exe path"))?;
    let uninstaller_path = install_dir.join("uninstall.ps1");
    let uninstaller_path_str = uninstaller_path
        .to_str()
        .ok_or(anyhow!("Failed to get uninstaller path"))?;
    let desktop_dir = user_dirs
        .desktop_dir()
        .ok_or(anyhow!("Failed to get desktop dir"))?;

    // Open the archive
    let cursor = Cursor::new(bytes);
    let mut archive = ZipArchive::new(cursor)?;

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
    fs::write(&uninstaller_path, UNINSTALL_PS1)?;

    // Create desktop shortcut
    let desktop_shortcut_path = desktop_dir.join("TinyWiiBackupManager.lnk");
    if desktop_shortcut_path.exists() {
        fs::remove_file(&desktop_shortcut_path)?;
    }
    let mut sl = ShellLink::new(&exe_path)?;
    sl.set_working_dir(install_dir.to_str().map(String::from));
    sl.set_icon_location(exe_path.to_str().map(String::from));
    sl.set_name(Some("TinyWiiBackupManager".to_string()));
    sl.create_lnk(&desktop_shortcut_path)?;

    // Create start menu shortcut
    let start_menu_dir = base_dirs
        .data_dir()
        .join("Microsoft\\Windows\\Start Menu\\Programs\\TinyWiiBackupManager");
    if start_menu_dir.exists() {
        fs::remove_dir_all(&start_menu_dir)?;
    }
    fs::create_dir_all(&start_menu_dir)?;
    let start_menu_shortcut_path = start_menu_dir.join("TinyWiiBackupManager.lnk");
    fs::copy(&desktop_shortcut_path, &start_menu_shortcut_path)?;

    // Write windows registry keys
    let key = CURRENT_USER
        .create("Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\TinyWiiBackupManager")?;

    let uninstall_cmd = format!(
        "powershell.exe -ExecutionPolicy Bypass -WindowStyle Hidden -File \"{}\"",
        uninstaller_path_str
    );

    key.set_string("DisplayName", "TinyWiiBackupManager")?;
    key.set_string("DisplayVersion", &version)?;
    key.set_string("Publisher", "Manuel Quarneti")?;
    key.set_string("InstallLocation", install_dir_str)?;
    key.set_string("DisplayIcon", exe_path_str)?;
    key.set_string("UninstallString", &uninstall_cmd)?;
    key.set_u32("NoModify", 1)?;
    key.set_u32("NoRepair", 1)?;

    Ok(version)
}

pub fn is_installed() -> Result<bool> {
    let base_dirs = BaseDirs::new().ok_or(anyhow!("Failed to get base dirs"))?;
    let install_dir = base_dirs.data_local_dir().join("TinyWiiBackupManager");

    Ok(install_dir.exists())
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

pub async fn download_to_dir(
    version: String,
    os: Os,
    arch: Arch,
    dest_dir: PathBuf,
) -> Result<(String, PathBuf)> {
    let (version, bytes) = download(version, os, arch).await?;
    let dest_path = dest_dir.join(format!("TinyWiiBackupManager-v{}-portable.exe", version));

    let cursor = Cursor::new(bytes);
    let mut archive = ZipArchive::new(cursor)?;

    if dest_path.exists() {
        fs::remove_file(&dest_path)?;
    }

    let mut file = File::create(&dest_path)?;
    let mut archived_exe = archive.by_name("TinyWiiBackupManager.exe")?;
    io::copy(&mut archived_exe, &mut file)?;

    Ok((version, dest_path))
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

#[derive(Clone, Copy, Debug, Default)]
pub enum Os {
    #[default]
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

pub fn get_os() -> Result<Os> {
    let key = LOCAL_MACHINE.open("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion")?;
    let product_name = key.get_string("ProductName")?;

    if product_name.contains("Windows 10") {
        Ok(Os::Windows)
    } else {
        Ok(Os::Windows7)
    }
}

#[derive(Clone, Copy, Debug)]
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
    let base_dirs = BaseDirs::new().ok_or(anyhow!("Failed to get base dirs"))?;
    let install_dir = base_dirs.data_local_dir().join("TinyWiiBackupManager");
    let exe_path = install_dir.join("TinyWiiBackupManager.exe");
    let exe_path_str = exe_path.to_str().ok_or(anyhow!("Failed to get exe path"))?;

    Command::new("cmd")
        .args(["/C", "start", "/B", exe_path_str])
        .current_dir(install_dir)
        .creation_flags(0x08000000) // CREATE_NO_WINDOW (run invisibly)
        .spawn()?;

    Ok(())
}

pub fn launch_twbm_portable(exe_path: PathBuf) -> Result<()> {
    let parent = exe_path.parent().ok_or(anyhow!("Failed to get parent"))?;
    let exe_path_str = exe_path.to_str().ok_or(anyhow!("Failed to get exe path"))?;

    Command::new("cmd")
        .args(["/C", "start", "/B", exe_path_str])
        .current_dir(parent)
        .creation_flags(0x08000000) // CREATE_NO_WINDOW (run invisibly)
        .spawn()?;

    Ok(())
}
