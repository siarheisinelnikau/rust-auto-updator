param([switch]$Offline)
$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest
$projectRoot = Split-Path -Parent $PSScriptRoot
Push-Location -LiteralPath $projectRoot
try {
    $manifest = Get-Content -Raw -LiteralPath 'Cargo.toml'
    $releaseSection = [regex]::Match($manifest, '(?ms)^\[workspace\.metadata\.release\]\s*(.*?)(?=^\[|\z)').Groups[1].Value
    $version = [regex]::Match($releaseSection, '(?m)^version = "([^"]+)"').Groups[1].Value
    if ($version -notmatch '^\d+\.\d+\.\d+$') { throw 'Expected a stable workspace.metadata.release version' }
    if (git status --porcelain) { throw 'Commit the release sources before packaging' }
    New-Item -ItemType Directory -Force -Path 'dist' | Out-Null
    $asset = "RustAutoUpdator-v$version-source.zip"
    & git archive --format=zip "--prefix=rust-auto-updator-v$version/" "--output=dist/$asset" HEAD
    if ($LASTEXITCODE -ne 0) { throw 'Cannot archive committed library sources' }
    Copy-Item -LiteralPath 'LICENSE', 'RELEASE_NOTES.md' -Destination 'dist'
    $digest = (Get-FileHash -LiteralPath (Join-Path 'dist' $asset) -Algorithm SHA256).Hash.ToLowerInvariant()
    [IO.File]::WriteAllText((Join-Path $projectRoot 'dist/SHA256SUMS.txt'), "$digest  $asset`n", [Text.UTF8Encoding]::new($false))
    Write-Output "Prepared dist/$asset and SHA256SUMS.txt. Nothing was published."
} finally { Pop-Location }
