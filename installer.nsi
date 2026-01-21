; SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
; SPDX-License-Identifier: GPL-3.0-only

!include "MUI2.nsh"
!include "WinVer.nsh"
!include "x64.nsh"
!include "LogicLib.nsh"

;--------------------------------
; General

  Name "TinyWiiBackupManager"
  BrandingText "TinyWiiBackupManager Installer"
  OutFile "TinyWiiBackupManagerInstaller.exe"
  
  ; User Mode Install (No Admin)
  RequestExecutionLevel user
  InstallDir "$LOCALAPPDATA\TinyWiiBackupManager"

;--------------------------------
; Interface

  !define MUI_ABORTWARNING
  !define MUI_ICON "${NSISDIR}\Contrib\Graphics\Icons\modern-install.ico"
  
  !insertmacro MUI_PAGE_WELCOME
  !insertmacro MUI_PAGE_INSTFILES
  !define MUI_FINISHPAGE_NOAUTOCLOSE ; Keep the install log visible until user clicks next
  !define MUI_FINISHPAGE_RUN "$INSTDIR\TinyWiiBackupManager.exe"
  !define MUI_FINISHPAGE_RUN_TEXT "Launch TinyWiiBackupManager"
  !insertmacro MUI_PAGE_FINISH
  
  !insertmacro MUI_UNPAGE_CONFIRM
  !insertmacro MUI_UNPAGE_INSTFILES
  
  !insertmacro MUI_LANGUAGE "English"

;--------------------------------
; Variables

  Var VersionString
  Var OsTag
  Var ArchTag
  Var DownloadUrl
  Var ZipPath

;--------------------------------
; Installer Section

Section "Install" SecInstall
  ; Initialize Temp directory
  InitPluginsDir
  
  ; Fetch Version String
  inetc::get /QUIET "https://github.com/mq1/TinyWiiBackupManager/releases/latest/download/version.txt" "$PLUGINSDIR\version.txt"
  Pop $0
  StrCmp $0 "OK" version_ok
    MessageBox MB_OK|MB_ICONSTOP "Failed to fetch version info. Check internet connection."
    Abort
  version_ok:
  
  ; Read version from file
  FileOpen $0 "$PLUGINSDIR\version.txt" r
  FileRead $0 $VersionString
  FileClose $0
  
  DetailPrint "Latest Version: $VersionString"

  ; Detect OS and Architecture Logic
  
  ${If} ${AtLeastWin10}
    ; --- Windows 10+ Logic ---
    StrCpy $OsTag "windows"
    DetailPrint "Detected Windows 10+"

    ${If} ${IsNativeARM64}
        StrCpy $ArchTag "arm64"
    ${ElseIf} ${IsNativeAMD64}
        ; Check for AVX2 support (PF_AVX2_INSTRUCTIONS_AVAILABLE = 40)
        System::Call "kernel32::IsProcessorFeaturePresent(i 40) i .r0"
        
        ${If} $0 != 0
            StrCpy $ArchTag "x86_64-v3"
        ${Else}
            StrCpy $ArchTag "x86_64"
        ${EndIf}
    ${Else}
        StrCpy $ArchTag "x86"
    ${EndIf}

  ${Else}
    ; --- Windows 7/8/8.1 Logic ---
    StrCpy $OsTag "windows7"
    DetailPrint "Detected Windows 7/8/8.1"
    
    ; Win7 builds in your list are only x86 or x86_64 (no v3, no arm64)
    ${If} ${IsNativeAMD64}
        StrCpy $ArchTag "x86_64"
    ${Else}
        StrCpy $ArchTag "x86"
    ${EndIf}
  ${EndIf}
  
  DetailPrint "Detected Arch: $ArchTag"

  ; Construct Download URL
  StrCpy $DownloadUrl "https://github.com/mq1/TinyWiiBackupManager/releases/download/v$VersionString/TinyWiiBackupManager-v$VersionString-$OsTag-$ArchTag.zip"
  StrCpy $ZipPath "$PLUGINSDIR\app.zip"
  
  ; Download Asset
  DetailPrint "Downloading: TinyWiiBackupManager-v$VersionString-$OsTag-$ArchTag.zip"
  inetc::get "$DownloadUrl" "$ZipPath" /END
  Pop $0
  StrCmp $0 "OK" download_ok
    MessageBox MB_OK|MB_ICONSTOP "Failed to download application asset.$\nServer returned: $0"
    Abort
  download_ok:

  ; Ensure install dir exists before unzipping
  CreateDirectory "$INSTDIR"
  
  ; Syntax: nsisunz::Unzip "SourceFile" "DestinationDir"
  nsisunz::Unzip "$ZipPath" "$INSTDIR"
  Pop $0
  StrCmp $0 "success" extract_ok
    MessageBox MB_OK|MB_ICONSTOP "Failed to unzip application.$\nError: $0"
    Abort
  extract_ok:

  ; Create Uninstaller
  WriteUninstaller "$INSTDIR\uninstall.exe"
  
  ; Create shortcuts
  CreateDirectory "$SMPROGRAMS\TinyWiiBackupManager"
  CreateShortcut "$SMPROGRAMS\TinyWiiBackupManager\TinyWiiBackupManager.lnk" "$INSTDIR\TinyWiiBackupManager.exe"
  CreateShortcut "$SMPROGRAMS\TinyWiiBackupManager\Uninstall.lnk" "$INSTDIR\uninstall.exe"
  CreateShortcut "$DESKTOP\TinyWiiBackupManager.lnk" "$INSTDIR\TinyWiiBackupManager.exe"

SectionEnd

;--------------------------------
; Uninstaller Section

Section "Uninstall"
  ; Remove shortcuts
  RMDir /r "$SMPROGRAMS\TinyWiiBackupManager"
  Delete "$DESKTOP\TinyWiiBackupManager.lnk"

  ; Remove installation directory
  RMDir /r "$INSTDIR"

  ; Remove data directory
  RMDir /r "$APPDATA\TinyWiiBackupManager"

SectionEnd
