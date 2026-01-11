# SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
# SPDX-License-Identifier: GPL-3.0-only

$regKey = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\TinyWiiBackupManager"
$installDir = Join-Path $env:LOCALAPPDATA "TinyWiiBackupManager"
$exePath = Join-Path $installDir "TinyWiiBackupManager.exe"
$uninstallerPath = Join-Path $installDir "uninstall.ps1"
$desktop = [Environment]::GetFolderPath('Desktop')

# --- Desktop shortcut ---
$desktopShortcutPath = Join-Path $desktop 'TinyWiiBackupManager.lnk'
if (Test-Path $desktopShortcutPath) {
    Remove-Item $desktopShortcutPath -Force
}

$WScriptShell = New-Object -ComObject WScript.Shell
$Shortcut = $WScriptShell.CreateShortcut($desktopShortcutPath)
$Shortcut.TargetPath = $exePath
$Shortcut.WorkingDirectory = $installDir
$Shortcut.IconLocation = "$exePath,0"
$Shortcut.Description = "Launch TinyWiiBackupManager"
$Shortcut.Save()

# --- Start menu shortcut ---
$startMenuShortcutDir = Join-Path $env:APPDATA "Microsoft\Windows\Start Menu\Programs\TinyWiiBackupManager"
if (Test-Path $startMenuShortcutDir) {
    Remove-Item $startMenuShortcutDir -Recurse -Force
}
New-Item -Path $startMenuShortcutDir -ItemType Directory -Force | Out-Null
$startMenuShortcutPath = Join-Path $startMenuShortcutDir "TinyWiiBackupManager.lnk"
Copy-Item -Path $desktopShortcutPath -Destination $startMenuShortcutPath

# --- Add registry entry ---
if (Test-Path $regKey) {
    Remove-Item -Path $regKey -Recurse -Force
}
New-Item -Path $regKey -Force | Out-Null
Set-ItemProperty -Path $regKey -Name "DisplayName" -Value "TinyWiiBackupManager" -Type String
Set-ItemProperty -Path $regKey -Name "DisplayVersion" -Value "TWBM_VERSION" -Type String
Set-ItemProperty -Path $regKey -Name "Publisher" -Value "Manuel Quarneti" -Type String
Set-ItemProperty -Path $regKey -Name "InstallLocation" -Value $installDir -Type String
Set-ItemProperty -Path $regKey -Name "DisplayIcon" -Value $exePath -Type String
Set-ItemProperty -Path $regKey -Name "UninstallString" -Value "powershell.exe -ExecutionPolicy Bypass -WindowStyle Hidden -File `"$uninstallerPath`"" -Type String
Set-ItemProperty -Path $regKey -Name "NoModify" -Value 1 -Type DWord
Set-ItemProperty -Path $regKey -Name "NoRepair" -Value 1 -Type DWord

