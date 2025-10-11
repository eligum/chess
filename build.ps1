# Stop the script if any command fails
$ErrorActionPreference = "Stop"

cargo build --package app

# Check if build succeeded
if ($LASTEXITCODE -eq 0) {
	$binName = "chess"

    $targetExe = Join-Path (Get-Location) "target\debug\$binName.exe"
    $destExe = Join-Path (Get-Location) "$binName.exe"

    if (Test-Path $targetExe) {
        Copy-Item $targetExe $destExe -Force
        Write-Host "Copied '$binName.exe' to project root." -ForegroundColor Green

		# Prompt user if they want to run the executable
        $response = Read-Host "Do you want to run '$binName.exe' now? (y/n)"
        if ($response -match '^(y|yes)$') {
            Write-Host "Running '$binName.exe'..." -ForegroundColor Cyan
            Start-Process "./$binName.exe" -Wait
        } else {
            Write-Host "Skipping execution." -ForegroundColor Green
        }
    } else {
        Write-Host "Could not find compiled executable: $targetExe" -ForegroundColor Red
    }
} else {
    Write-Host "Build failed. Exiting." -ForegroundColor Red
}
