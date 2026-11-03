$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest
Set-Location -LiteralPath (Split-Path -Parent $PSScriptRoot)
$cargo = Get-Content -LiteralPath Cargo.toml -Raw
$section = [regex]::Match($cargo, '(?ms)^\[workspace\.metadata\.release\]\s*\r?\n(?<body>.*?)(?=^\[|\z)').Groups['body'].Value
$version = [regex]::Match($section, '(?m)^version\s*=\s*"(\d+\.\d+\.\d+)"\s*$').Groups[1].Value
if (-not $version) { throw 'Expected a stable source distribution version in workspace.metadata.release.' }
$event = Get-Content -LiteralPath $env:GITHUB_EVENT_PATH -Raw | ConvertFrom-Json
$subject = ($event.head_commit.message -split "`n")[0].TrimEnd("`r")
if ($env:GITHUB_EVENT_NAME -ne 'push' -or $env:GITHUB_REF -ne 'refs/heads/main' -or $subject -cne "Release v$version") {
    throw 'The source release requires an explicit final release commit matching its version.'
}
$configuration = Get-Content -LiteralPath scripts/localization-release.json -Raw | ConvertFrom-Json
$notes = Get-Content -LiteralPath $configuration.notes -Raw -Encoding utf8
if ($notes.Length -lt 100 -or -not $notes.Contains($version)) { throw 'Readable notes for this source distribution are required.' }
& ./scripts/package.ps1
if ($LASTEXITCODE -ne 0) { throw 'Source packaging failed.' }
if (Test-Path -LiteralPath '.release-stage') { throw 'Use a fresh release staging directory.' }
New-Item -ItemType Directory -Path '.release-stage' | Out-Null
$sourceName = "RustAutoUpdator-v$version-source.zip"
foreach ($name in @($sourceName, 'SHA256SUMS.txt', 'LICENSE')) {
    Copy-Item -LiteralPath (Join-Path dist $name) -Destination (Join-Path '.release-stage' $name)
}
$archiveDigest = (Get-FileHash -LiteralPath (Join-Path '.release-stage' $sourceName) -Algorithm SHA256).Hash.ToLowerInvariant()
if ((Get-Content -LiteralPath '.release-stage/SHA256SUMS.txt' -Raw).Trim() -cne "$archiveDigest  $sourceName") {
    throw 'The source archive checksum did not match.'
}
$assets = @()
foreach ($name in @($sourceName, 'SHA256SUMS.txt', 'LICENSE')) {
    $asset = Join-Path '.release-stage' $name
    $assets += [ordered]@{name=$name; size=(Get-Item -LiteralPath $asset).Length; digest=('sha256:' + (Get-FileHash -LiteralPath $asset -Algorithm SHA256).Hash.ToLowerInvariant())}
}
$manifest = [ordered]@{kind='source'; product='RustAutoUpdator'; version=$version; tag="v$version"; commit=$env:GITHUB_SHA; notes=$configuration.notes; assets=@($assets)}
[IO.File]::WriteAllText((Join-Path (Get-Location) '.release-stage/manifest.json'), ($manifest | ConvertTo-Json -Depth 6), [Text.UTF8Encoding]::new($false))
