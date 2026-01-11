// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: GPL-3.0-only

use anyhow::Result;
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::Command;

const REG_KEY: &str =
    "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\TinyWiiBackupManager";

fn reg(args: &[&str]) -> Result<()> {
    Command::new("reg")
        .args(args)
        .creation_flags(0x08000000) // CREATE_NO_WINDOW
        .output()?;

    Ok(())
}

pub fn install_reg_keys(version: &str, install_dir: &Path) -> Result<()> {
    // 1. Create the key (implicitly created by adding values) and set DisplayName
    reg(&[
        "add",
        REG_KEY,
        "/v",
        "DisplayName",
        "/t",
        "REG_SZ",
        "/d",
        "TinyWiiBackupManager",
        "/f",
    ])?;

    // 2. Set Version
    reg(&[
        "add",
        REG_KEY,
        "/v",
        "DisplayVersion",
        "/t",
        "REG_SZ",
        "/d",
        &version,
        "/f",
    ])?;

    // 3. Set Publisher
    reg(&[
        "add",
        REG_KEY,
        "/v",
        "Publisher",
        "/t",
        "REG_SZ",
        "/d",
        "Manuel Quarneti",
        "/f",
    ])?;

    // 4. Set Install Location
    reg(&[
        "add",
        REG_KEY,
        "/v",
        "InstallLocation",
        "/t",
        "REG_SZ",
        "/d",
        &install_dir.to_string_lossy(),
        "/f",
    ])?;

    // 5. Set Icon
    reg(&[
        "add",
        REG_KEY,
        "/v",
        "DisplayIcon",
        "/t",
        "REG_SZ",
        "/d",
        &install_dir
            .join("TinyWiiBackupManager.exe")
            .to_string_lossy(),
        "/f",
    ])?;

    // 6. Set Uninstall String
    let uninstall_cmd = format!(
        "powershell.exe -ExecutionPolicy Bypass -WindowStyle Hidden -File \"{}\"",
        &install_dir.join("uninstall.ps1").to_string_lossy()
    );
    reg(&[
        "add",
        REG_KEY,
        "/v",
        "UninstallString",
        "/t",
        "REG_SZ",
        "/d",
        &uninstall_cmd,
        "/f",
    ])?;

    // 7. NoModify / NoRepair
    reg(&[
        "add",
        REG_KEY,
        "/v",
        "NoModify",
        "/t",
        "REG_DWORD",
        "/d",
        "1",
        "/f",
    ])?;
    reg(&[
        "add",
        REG_KEY,
        "/v",
        "NoRepair",
        "/t",
        "REG_DWORD",
        "/d",
        "1",
        "/f",
    ])?;

    Ok(())
}
