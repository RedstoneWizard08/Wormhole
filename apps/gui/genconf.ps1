pip3 install yq

$scriptDirectory = Split-Path -Parent $MyInvocation.MyCommand.Definition

Set-Location $scriptDirectory

$version = (Get-Content ../../Cargo.toml | tomlq -r '.workspace.package.version')
$env:CARGO_MANIFEST_DIR = $scriptDirectory
$env:CARGO_PKG_VERSION = $version

rustc "conf.rs" -o conf.exe

.\conf.exe
Remove-Item conf.exe
