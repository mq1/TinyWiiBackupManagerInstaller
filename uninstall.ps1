# SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
# SPDX-License-Identifier: GPL-3.0-only

Add-Type -AssemblyName System.Windows.Forms


# Confirmation msb box
$result = [System.Windows.Forms.MessageBox]::Show(
    "Do you want to uninstall TinyWiiBackupManager?",
    "Confirm Uninstall",
    [System.Windows.Forms.MessageBoxButtons]::YesNo,
    [System.Windows.Forms.MessageBoxIcon]::Warning
)

# If user clicked 'No', exit the script immediately
if ($result -ne [System.Windows.Forms.DialogResult]::Yes) {
    Exit
}

# Desktop shortcut
$shortcutPath = Join-Path $env:USERPROFILE "Desktop\TinyWiiBackupManager.lnk"
if (Test-Path $shortcutPath) {
    Remove-Item $shortcutPath -Force
}

# Start Menu folder
$startMenuDir = Join-Path $env:APPDATA "Microsoft\Windows\Start Menu\Programs\TinyWiiBackupManager"
if (Test-Path $startMenuDir) {
    Remove-Item $startMenuDir -Recurse -Force
}

# App data directory
$dataDir = Join-Path $env:APPDATA "mq1\TinyWiiBackupManager"
if (Test-Path $dataDir) {
    Remove-Item $dataDir -Recurse -Force
}

# Registry
$regKey = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\TinyWiiBackupManager"
if (Test-Path $regKey) {
    Remove-Item -Path $regKey -Recurse -Force
}

# Install dir
$installDir = Join-Path $env:LOCALAPPDATA "TinyWiiBackupManager"
if (Test-Path $installDir) {
    Set-Location $env:TEMP
    Remove-Item $installDir -Recurse -Force
}

# Uninstallation notice
[System.Windows.Forms.MessageBox]::Show(
    "TinyWiiBackupManager has been successfully uninstalled.",
    "Uninstall Complete",
    [System.Windows.Forms.MessageBoxButtons]::OK,
    [System.Windows.Forms.MessageBoxIcon]::Information
)

