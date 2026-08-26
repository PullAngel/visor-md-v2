$ErrorActionPreference = "Stop"
$repoRoot = Split-Path -Parent $PSScriptRoot
$failures = [Collections.Generic.List[string]]::new()

Push-Location $repoRoot
try {
    $documents = @(git ls-files -- "*.md")
    if ($LASTEXITCODE -ne 0) {
        throw "git ls-files fallo con codigo $LASTEXITCODE"
    }

    foreach ($relativeDocument in $documents) {
        $document = Join-Path $repoRoot $relativeDocument
        $lineNumber = 0
        foreach ($line in [IO.File]::ReadLines($document)) {
            $lineNumber++
            foreach ($match in [regex]::Matches($line, '!?(?:\[[^\]]*\])\(([^)]+)\)')) {
                $target = $match.Groups[1].Value.Trim()
                if ($target.StartsWith('<') -and $target.EndsWith('>')) {
                    $target = $target.Substring(1, $target.Length - 2)
                }
                if ($target -match '^(?:[a-z][a-z0-9+.-]*:|#)') {
                    continue
                }

                $pathPart = ($target -split '[?#]', 2)[0]
                if ([string]::IsNullOrWhiteSpace($pathPart)) {
                    continue
                }
                $pathPart = [Uri]::UnescapeDataString($pathPart)
                $base = Split-Path -Parent $document
                $resolved = if ($pathPart.StartsWith('/')) {
                    Join-Path $repoRoot $pathPart.TrimStart('/')
                } else {
                    Join-Path $base $pathPart
                }
                if (-not (Test-Path -LiteralPath $resolved)) {
                    $failures.Add("${relativeDocument}:${lineNumber} -> $target")
                }
            }
        }
    }
} finally {
    Pop-Location
}

if ($failures.Count -gt 0) {
    $detail = $failures -join [Environment]::NewLine
    throw "enlaces Markdown locales rotos:$([Environment]::NewLine)$detail"
}

Write-Host "$($documents.Count) documentos Markdown sin enlaces locales rotos"
