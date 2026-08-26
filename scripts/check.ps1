param(
    [switch]$SkipRelease
)

$ErrorActionPreference = "Stop"

function Invoke-Checked {
    param(
        [Parameter(Mandatory)]
        [string]$Description,
        [Parameter(Mandatory)]
        [scriptblock]$Command
    )

    Write-Host "==> $Description"
    & $Command
    if ($LASTEXITCODE -ne 0) {
        throw "$Description fallo con codigo $LASTEXITCODE"
    }
}

Push-Location (Split-Path -Parent $PSScriptRoot)
try {
    Invoke-Checked "Formato" { cargo fmt -- --check }
    Invoke-Checked "Clippy" { cargo clippy --all-targets --all-features -- -D warnings }
    Invoke-Checked "Pruebas" { cargo test }

    Write-Host "==> SBOM"
    & "$PSScriptRoot\generate-sbom.ps1" -Check

    if (-not $SkipRelease) {
        Invoke-Checked "Build release" { cargo build --release }
    }
} finally {
    Pop-Location
}
