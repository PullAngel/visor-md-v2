param(
    [string]$OutputPath = "sbom.cdx.json",
    [switch]$Check
)

$ErrorActionPreference = "Stop"
$repoRoot = Split-Path -Parent $PSScriptRoot
$destination = if ([IO.Path]::IsPathRooted($OutputPath)) {
    $OutputPath
} else {
    Join-Path $repoRoot $OutputPath
}

Push-Location $repoRoot
try {
    $metadataJson = cargo metadata --locked --format-version 1
    if ($LASTEXITCODE -ne 0) {
        throw "cargo metadata fallo con codigo $LASTEXITCODE"
    }
    $metadata = $metadataJson | ConvertFrom-Json
} finally {
    Pop-Location
}

function Get-Purl {
    param($Package)

    $name = [Uri]::EscapeDataString([string]$Package.name)
    $version = [Uri]::EscapeDataString([string]$Package.version)
    "pkg:cargo/$name@$version"
}

function Get-LicenseExpression {
    param([string]$License)

    # Cargo acepto durante anos estas dos formas anteriores a SPDX. Su
    # significado es OR, que se normaliza para que CycloneDX sea validable.
    switch ($License) {
        "MIT/Apache-2.0" { "MIT OR Apache-2.0" }
        "Unlicense/MIT" { "Unlicense OR MIT" }
        default { $License }
    }
}

$packagesById = @{}
foreach ($package in $metadata.packages) {
    $packagesById[[string]$package.id] = $package
}

$rootPackage = $packagesById[[string]$metadata.resolve.root]
if ($null -eq $rootPackage) {
    throw "cargo metadata no identifico el paquete raiz"
}

function New-Component {
    param(
        $Package,
        [string]$Type
    )

    $purl = Get-Purl $Package
    $component = [ordered]@{
        type      = $Type
        'bom-ref' = $purl
        name      = [string]$Package.name
        version   = [string]$Package.version
        purl      = $purl
    }
    if (-not [string]::IsNullOrWhiteSpace([string]$Package.license)) {
        $component.licenses = @(
            [ordered]@{
                expression = Get-LicenseExpression ([string]$Package.license)
            }
        )
    }
    $component
}

$components = @(
    $metadata.packages |
        Where-Object { [string]$_.id -ne [string]$metadata.resolve.root } |
        Sort-Object name, version, source |
        ForEach-Object { New-Component $_ "library" }
)

$dependencies = @(
    $metadata.resolve.nodes |
        ForEach-Object {
            $package = $packagesById[[string]$_.id]
            $dependsOn = @(
                $_.dependencies |
                    ForEach-Object { Get-Purl $packagesById[[string]$_] } |
                    Sort-Object -Unique
            )
            [ordered]@{
                ref       = Get-Purl $package
                dependsOn = $dependsOn
            }
        } |
        Sort-Object { $_.ref }
)

$allRefs = @((Get-Purl $rootPackage)) + @($components | ForEach-Object { $_.'bom-ref' })
$duplicateRefs = @($allRefs | Group-Object | Where-Object { $_.Count -gt 1 })
if ($duplicateRefs.Count -gt 0) {
    throw "el SBOM produciria referencias duplicadas"
}
$knownRefs = [Collections.Generic.HashSet[string]]::new([string[]]$allRefs)
$missingRefs = @(
    $dependencies |
        ForEach-Object { @($_.ref) + @($_.dependsOn) } |
        Where-Object { -not $knownRefs.Contains([string]$_) } |
        Sort-Object -Unique
)
if ($missingRefs.Count -gt 0) {
    throw "el SBOM contiene dependencias sin componente: $($missingRefs -join ', ')"
}

$bom = [ordered]@{
    '$schema'    = "https://cyclonedx.org/schema/bom-1.6.schema.json"
    bomFormat    = "CycloneDX"
    specVersion  = "1.6"
    version      = 1
    metadata     = [ordered]@{
        component = New-Component $rootPackage "application"
    }
    components   = $components
    dependencies = $dependencies
}

$content = ($bom | ConvertTo-Json -Depth 20) + [Environment]::NewLine
$fullDestination = [IO.Path]::GetFullPath($destination)
if ($Check) {
    if (-not [IO.File]::Exists($fullDestination)) {
        throw "falta el SBOM generado: $fullDestination"
    }
    $current = [IO.File]::ReadAllText($fullDestination)
    if (-not [string]::Equals($current, $content, [StringComparison]::Ordinal)) {
        throw "el SBOM esta desactualizado; ejecute scripts/generate-sbom.ps1"
    }
    Write-Host "SBOM CycloneDX vigente: $fullDestination"
} else {
    [IO.File]::WriteAllText(
        $fullDestination,
        $content,
        [Text.UTF8Encoding]::new($false)
    )
    Write-Host "SBOM CycloneDX generado: $fullDestination ($($components.Count) componentes externos)"
}
