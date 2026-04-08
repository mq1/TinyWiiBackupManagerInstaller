; SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
; SPDX-License-Identifier: MIT OR Apache-2.0

!include "MUI2.nsh"
!include "LogicLib.nsh"
!include "x64.nsh"

; General
Name "TinyWiiBackupManager"
OutFile "TinyWiiBackupManagerInstaller.exe"
Unicode True
InstallDir "$LOCALAPPDATA\TinyWiiBackupManager"
InstallDirRegKey HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\TinyWiiBackupManager" "InstallLocation"
RequestExecutionLevel user

; Variables
Var Version
Var Arch
Var Asset

; Interface Settings
!define MUI_ABORTWARNING

; Pages
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!define MUI_FINISHPAGE_TEXT "Successfully installed $Asset"
!define MUI_FINISHPAGE_RUN "$INSTDIR\TinyWiiBackupManager.exe"
!define MUI_FINISHPAGE_RUN_TEXT "Run TinyWiiBackupManager"
!insertmacro MUI_PAGE_FINISH
!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES
!insertmacro MUI_UNPAGE_FINISH

; Languages
!insertmacro MUI_LANGUAGE "English"


; Installer
Section
  ; Initialize Temp directory
  InitPluginsDir

  ; Fetch Version String
  inetc::get /TOSTACK "https://github.com/mq1/TinyWiiBackupManager/releases/latest/download/version.txt" "" /END
  Pop $0

  ${If} $0 != "OK"
    MessageBox MB_OK|MB_ICONSTOP "Failed to fetch version info. Check internet connection."
    Abort
  ${EndIf}

  Pop $Version
  DetailPrint "Latest version: v$Version"

  ; Detect Architecture
  ${If} ${IsNativeARM64}
    StrCpy $Arch "arm64"
  ${ElseIf} ${IsNativeAMD64}
    StrCpy $Arch "x64"
  ${Else}
    StrCpy $Arch "x86"
  ${EndIf}

  ; Construct asset file name
  StrCpy $Asset "TinyWiiBackupManager-v$Version-windows-$Arch.exe"
  DetailPrint "Downloading $Asset"

  ; Ensure install dir exists before unzipping
  CreateDirectory "$INSTDIR"

  ; Download Asset
  inetc::get "https://github.com/mq1/TinyWiiBackupManager/releases/download/v$Version/$Asset" "$INSTDIR\TinyWiiBackupManager.exe" /END
  Pop $0

  ${If} $0 != "OK"
    MessageBox MB_OK|MB_ICONSTOP "Failed to download application asset.$\nServer returned: $0"
    Abort
  ${EndIf}

  ; Create Uninstaller
  WriteUninstaller "$INSTDIR\uninstall.exe"

  ; Create registry entries
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\TinyWiiBackupManager" "DisplayName" "TinyWiiBackupManager"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\TinyWiiBackupManager" "DisplayVersion" "$Version"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\TinyWiiBackupManager" "Publisher" "Manuel Quarneti"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\TinyWiiBackupManager" "UninstallString" "$\"$INSTDIR\uninstall.exe$\""
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\TinyWiiBackupManager" "DisplayIcon" "$INSTDIR\TinyWiiBackupManager.exe"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\TinyWiiBackupManager" "InstallLocation" "$INSTDIR"
  WriteRegDWORD HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\TinyWiiBackupManager" "NoModify" 1
  WriteRegDWORD HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\TinyWiiBackupManager" "NoRepair" 1

  ; Create shortcuts
  CreateDirectory "$SMPROGRAMS\TinyWiiBackupManager"
  CreateShortcut "$SMPROGRAMS\TinyWiiBackupManager\TinyWiiBackupManager.lnk" "$INSTDIR\TinyWiiBackupManager.exe"
  CreateShortcut "$DESKTOP\TinyWiiBackupManager.lnk" "$INSTDIR\TinyWiiBackupManager.exe"
SectionEnd


Section "Uninstall"
  ; Remove registry entries
  DeleteRegKey HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\TinyWiiBackupManager"

  ; Remove shortcuts
  RMDir /r "$SMPROGRAMS\TinyWiiBackupManager"
  Delete "$DESKTOP\TinyWiiBackupManager.lnk"

  ; Remove installation directory
  RMDir /r "$INSTDIR"

  ; Remove data directory
  RMDir /r "$APPDATA\mq1\TinyWiiBackupManager"
SectionEnd
