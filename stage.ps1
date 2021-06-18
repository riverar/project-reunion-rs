if(!(Test-Path tmp)) {
    New-Item -ItemType Directory -Path tmp | Out-Null
}

$ProjectReunionUrl = [uri]"https://github.com/microsoft/ProjectReunion/releases/download/v0.8.0-rc/ProjectReunion.NuGetPackages.0.8.0-rc.zip"
Invoke-WebRequest -UseBasicParsing $ProjectReunionUrl -OutFile "tmp/bundle.zip"

Expand-Archive .\tmp\bundle.zip -DestinationPath .\tmp -Force 
Expand-Archive .\tmp\Microsoft.ProjectReunion.Foundation.0.8.0-rc.nupkg .\tmp\foundation -Force
Expand-Archive .\tmp\Microsoft.ProjectReunion.WinUI.0.8.0-rc.nupkg .\tmp\winui -Force

@(
    [tuple]::Create("x86", "i686"),
    [tuple]::Create("x64", "x86_64"),
    [tuple]::Create("arm64", "aarch64")
) | ForEach-Object {
    $srcArch = $_.item1;
    $dstArch = $_.item2;
    $libPath = ".\.windows\lib\$dstArch\"
    $winmdPath = ".\.windows\winmd"

    New-Item -Path $libPath -ItemType Directory -Force | Out-Null
    New-Item -Path $winmdPath -ItemType Directory -Force | Out-Null
    Move-Item .\tmp\foundation\lib\win10-$srcArch\* $libPath -Force
    Move-Item .\tmp\foundation\runtimes\lib\native\$srcArch\* $libPath -Force
    Move-Item .\tmp\foundation\lib\native\*.winmd $winmdPath -Force
    Move-Item .\tmp\winui\lib\uap10.0\*.winmd $winmdPath -Force
}

Remove-Item .\tmp -Recurse -Force
