

#define MyAppName "towerview"
#define MyAppVersion "0.0.1"


[Setup]
AppName={#MyAppName}
AppVersion={#MyAppVersion}
WizardStyle=modern
DefaultDirName={autopf}\TowerView
DefaultGroupName=TowerView
UninstallDisplayIcon={app}\uninstall.exe
Compression=lzma2
SolidCompression=yes
OutputDir=C:\Users\Administrator
OutputBaseFilename={#MyAppName}-{#MyAppVersion}-x64
; "ArchitecturesAllowed=x64" specifies that Setup cannot run on
; anything but x64.
ArchitecturesAllowed=x64
; "ArchitecturesInstallIn64BitMode=x64" requests that the install be
; done in "64-bit mode" on x64, meaning it should use the native
; 64-bit Program Files directory and the 64-bit view of the registry.
ArchitecturesInstallIn64BitMode=x64

[Files]
Source: "..\..\target\release\{#MyAppName}.exe"; DestDir: "{app}"; DestName: "{#MyAppName}.exe"
Source: "..\..\config\*"; DestDir: "{app}\config"

[Registry]
Root: HKLM; Subkey: "Software\{#MyAppName}"; Flags: uninsdeletekeyifempty
Root: HKLM; Subkey: "Software\{#MyAppName}\{#MyAppName}"; Flags: uninsdeletekey
Root: HKLM; Subkey: "Software\{#MyAppName}\{#MyAppName}\Settings"; ValueType: string; ValueName: "InstallPath"; ValueData: "{app}"
Root: HKLM; SubKey: "SYSTEM\CurrentControlSet\Control\Session Manager\Environment\"; ValueType: expandsz; ValueName: "Path"; ValueData: "{reg:HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Environment\,Path};{app}"; Tasks: "addtopath"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; \
    GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked
Name: "addtopath"; Description: "Add to PATH environment variable"; \
    GroupDescription: "Settings"; Flags: checked

[Icons]
Name: "{userdesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppName}.exe"; Tasks: "desktopicon"
