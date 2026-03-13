; SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
; SPDX-License-Identifier: GPL-3.0-only

[Setup]
AppName=TinyWiiBackupManager
AppVersion=1.0
AppPublisher=Manuel Quarneti
AppPublisherURL=https://mq1.it
AppSupportURL=https://github.com/mq1/TinyWiiBackupManager/issues
AppUpdatesURL=https://github.com/mq1/TinyWiiBackupManager/releases
DefaultDirName={localappdata}\TinyWiiBackupManager
Compression=none
DefaultGroupName=TinyWiiBackupManager
UninstallDisplayIcon={app}\TinyWiiBackupManager.exe
PrivilegesRequired=lowest
OutputBaseFilename=TinyWiiBackupManagerInstaller

[Code]
var
  Version: AnsiString;
  DownloadUrl: AnsiString;

procedure CurStepChanged(CurStep: TSetupStep);
begin
  if CurStep = ssInstall then
  begin
    DownloadTemporaryFile('https://github.com/mq1/TinyWiiBackupManager/releases/latest/download/version.txt', 'version.txt', '', nil);
    LoadStringFromFile(ExpandConstant('{tmp}\version.txt'), Version);

    DownloadUrl := 'https://github.com/mq1/TinyWiiBackupManager/releases/download/v' + Version + '/TinyWiiBackupManager-v' + Version + '-windows-x86_64.zip';
    DownloadTemporaryFile(DownloadUrl, 'TinyWiiBackupManager.zip', '', nil);
    ExtractArchive(ExpandConstant('{tmp}\TinyWiiBackupManager.zip'), ExpandConstant('{app}'), '', true, nil);
  end;
end;
