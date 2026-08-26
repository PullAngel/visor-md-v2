param(
    [Parameter(Mandatory)]
    [string]$Document,
    [ValidateRange(3, 100)]
    [int]$Runs = 10,
    [ValidateRange(1, 10000)]
    [int]$ScrollFrames = 240,
    [string]$OutputPath,
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"
$repoRoot = Split-Path -Parent $PSScriptRoot

function Get-Metric {
    param(
        [string]$Text,
        [string]$Pattern,
        [string]$Name
    )

    $match = [regex]::Match($Text, $Pattern)
    if (-not $match.Success) {
        throw "no se encontro la medida $Name en la salida del ejecutable"
    }
    [double]::Parse(
        $match.Groups[1].Value.Replace(',', '.'),
        [Globalization.CultureInfo]::InvariantCulture
    )
}

function Get-Percentile {
    param(
        [double[]]$Values,
        [double]$Percentile
    )

    $sorted = @($Values | Sort-Object)
    $rank = [Math]::Ceiling($Percentile * $sorted.Count) - 1
    $sorted[[Math]::Max(0, $rank)]
}

Push-Location $repoRoot
try {
    if (-not $SkipBuild) {
        cargo build --release
        if ($LASTEXITCODE -ne 0) {
            throw "cargo build --release fallo con codigo $LASTEXITCODE"
        }
    }

    $exe = Get-Item -LiteralPath ".\target\release\visor-md.exe"
    $documentPath = (Resolve-Path -LiteralPath $Document).Path
    $samples = @()

    for ($run = 1; $run -le $Runs; $run++) {
        $lines = @(& $exe.FullName $documentPath --bench=0 2>&1)
        if ($LASTEXITCODE -ne 0) {
            throw "la ejecucion $run termino con codigo $LASTEXITCODE"
        }
        $text = $lines -join [Environment]::NewLine
        $samples += [ordered]@{
            run              = $run
            parseMs          = Get-Metric $text 'parseo de .*?: ([0-9]+(?:[.,][0-9]+)?) ms' 'parseo'
            windowVisibleMs  = Get-Metric $text 'ventana visible: ([0-9]+(?:[.,][0-9]+)?) ms' 'ventana visible'
            firstContentMs   = Get-Metric $text 'primer pintado: ([0-9]+(?:[.,][0-9]+)?) ms' 'primer contenido'
        }
    }

    $scrollLines = @(& $exe.FullName $documentPath "--bench=$ScrollFrames" 2>&1)
    if ($LASTEXITCODE -ne 0) {
        throw "la medicion de scroll termino con codigo $LASTEXITCODE"
    }
    $scrollText = $scrollLines -join [Environment]::NewLine
    $scrollMs = Get-Metric $scrollText 'promedio ([0-9]+(?:[.,][0-9]+)?) ms' 'scroll'

    $parseValues = [double[]]@($samples | ForEach-Object { $_.parseMs })
    $windowValues = [double[]]@($samples | ForEach-Object { $_.windowVisibleMs })
    $contentValues = [double[]]@($samples | ForEach-Object { $_.firstContentMs })
    $trackedChanges = @(git status --porcelain --untracked-files=no)

    $report = [ordered]@{
        schemaVersion = 1
        measuredAtUtc = [DateTime]::UtcNow.ToString('o')
        commit = (git rev-parse HEAD).Trim()
        trackedWorkingTreeClean = ($trackedChanges.Count -eq 0)
        platform = [ordered]@{
            os = [Environment]::OSVersion.VersionString
            architecture = [Runtime.InteropServices.RuntimeInformation]::OSArchitecture.ToString()
            rustc = (rustc --version).Trim()
            cargo = (cargo --version).Trim()
        }
        executable = [ordered]@{
            bytes = $exe.Length
            sha256 = (Get-FileHash -LiteralPath $exe.FullName -Algorithm SHA256).Hash
        }
        corpus = [ordered]@{
            path = [IO.Path]::GetRelativePath($repoRoot, $documentPath)
            bytes = (Get-Item -LiteralPath $documentPath).Length
        }
        protocol = [ordered]@{
            runs = $Runs
            cacheState = "sequential-uncontrolled"
            systemLoad = "uncontrolled"
            scrollFrames = $ScrollFrames
        }
        summaryMs = [ordered]@{
            parseMedian = Get-Percentile $parseValues 0.5
            parseP95 = Get-Percentile $parseValues 0.95
            windowVisibleMedian = Get-Percentile $windowValues 0.5
            windowVisibleP95 = Get-Percentile $windowValues 0.95
            firstContentMedian = Get-Percentile $contentValues 0.5
            firstContentP95 = Get-Percentile $contentValues 0.95
            scrollAverage = $scrollMs
        }
        samples = $samples
    }

    $json = $report | ConvertTo-Json -Depth 10
    if (-not [string]::IsNullOrWhiteSpace($OutputPath)) {
        $destination = if ([IO.Path]::IsPathRooted($OutputPath)) {
            $OutputPath
        } else {
            Join-Path $repoRoot $OutputPath
        }
        [IO.File]::WriteAllText(
            [IO.Path]::GetFullPath($destination),
            $json + [Environment]::NewLine,
            [Text.UTF8Encoding]::new($false)
        )
        Write-Host "Benchmark guardado en $destination"
    }
    $json
} finally {
    Pop-Location
}
