# setup-windows.ps1
# Esegui come Amministratore

$ErrorActionPreference = "Stop"

Write-Host "=== Blah-Blah Windows Setup ===" -ForegroundColor Cyan

$BINARIES_DIR = "src-tauri\binaries"
$MODELS_DIR = "src-tauri\models"

# Crea cartelle
New-Item -ItemType Directory -Force -Path $BINARIES_DIR | Out-Null
New-Item -ItemType Directory -Force -Path $MODELS_DIR | Out-Null

Write-Host ""
Write-Host "1. Scaricando whisper.cpp..." -ForegroundColor Yellow

$whisperUrl = "https://github.com/ggerganov/whisper.cpp/releases/download/v1.7.2/whisper-bin-x64.zip"
$whisperZip = "$env:TEMP\whisper-bin-x64.zip"
$whisperExtract = "$env:TEMP\whisper-bin"

Invoke-WebRequest -Uri $whisperUrl -OutFile $whisperZip
Expand-Archive -Path $whisperZip -DestinationPath $whisperExtract -Force

# Cerca l'eseguibile whisper
$whisperExe = Get-ChildItem -Path $whisperExtract -Recurse -Filter "*.exe" | Where-Object { $_.Name -match "whisper|main" } | Select-Object -First 1
if ($whisperExe) {
    Copy-Item $whisperExe.FullName "$BINARIES_DIR\whisper-x86_64-pc-windows-msvc.exe"
    Write-Host "   OK: whisper copiato" -ForegroundColor Green
} else {
    Write-Host "   ERRORE: whisper.exe non trovato" -ForegroundColor Red
}

Write-Host ""
Write-Host "2. Scaricando ffmpeg..." -ForegroundColor Yellow

$ffmpegUrl = "https://www.gyan.dev/ffmpeg/builds/ffmpeg-release-essentials.zip"
$ffmpegZip = "$env:TEMP\ffmpeg.zip"
$ffmpegExtract = "$env:TEMP\ffmpeg"

Invoke-WebRequest -Uri $ffmpegUrl -OutFile $ffmpegZip
Expand-Archive -Path $ffmpegZip -DestinationPath $ffmpegExtract -Force

$ffmpegExe = Get-ChildItem -Path $ffmpegExtract -Recurse -Filter "ffmpeg.exe" | Select-Object -First 1
if ($ffmpegExe) {
    Copy-Item $ffmpegExe.FullName "$BINARIES_DIR\ffmpeg-x86_64-pc-windows-msvc.exe"
    Write-Host "   OK: ffmpeg copiato" -ForegroundColor Green
} else {
    Write-Host "   ERRORE: ffmpeg.exe non trovato" -ForegroundColor Red
}

Write-Host ""
Write-Host "3. Scaricando modello ggml-medium.bin (~1.5GB)..." -ForegroundColor Yellow

$modelUrl = "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-medium.bin"
$modelPath = "$MODELS_DIR\ggml-medium.bin"

if (!(Test-Path $modelPath)) {
    Invoke-WebRequest -Uri $modelUrl -OutFile $modelPath
    Write-Host "   OK: modello scaricato" -ForegroundColor Green
} else {
    Write-Host "   OK: modello gia presente" -ForegroundColor Green
}

Write-Host ""
Write-Host "=== Setup completato ===" -ForegroundColor Cyan
Write-Host ""
Write-Host "Contenuto $BINARIES_DIR`:" -ForegroundColor Yellow
Get-ChildItem $BINARIES_DIR

Write-Host ""
Write-Host "Per compilare:" -ForegroundColor Yellow
Write-Host "  npm install"
Write-Host "  npm run tauri build"
