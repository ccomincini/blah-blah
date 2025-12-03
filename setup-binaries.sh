#!/bin/bash
# Script per preparare i binari per ogni piattaforma

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
BINARIES_DIR="$SCRIPT_DIR/src-tauri/binaries"
MODELS_DIR="$SCRIPT_DIR/src-tauri/models"

mkdir -p "$BINARIES_DIR"
mkdir -p "$MODELS_DIR"

echo "=== Whisper Subtitles - Setup Binari ==="
echo ""

# Rileva OS
OS=$(uname -s)
ARCH=$(uname -m)

case "$OS" in
    Darwin)
        if [ "$ARCH" = "arm64" ]; then
            TARGET="aarch64-apple-darwin"
        else
            TARGET="x86_64-apple-darwin"
        fi
        WHISPER_BIN="whisper-cli"
        FFMPEG_BIN="ffmpeg"
        ;;
    Linux)
        TARGET="x86_64-unknown-linux-gnu"
        WHISPER_BIN="whisper-cli"
        FFMPEG_BIN="ffmpeg"
        ;;
    MINGW*|MSYS*|CYGWIN*)
        TARGET="x86_64-pc-windows-msvc"
        WHISPER_BIN="whisper-cli.exe"
        FFMPEG_BIN="ffmpeg.exe"
        ;;
    *)
        echo "OS non supportato: $OS"
        exit 1
        ;;
esac

echo "Target: $TARGET"
echo ""

# 1. Copia whisper-cli dal build locale (se su Mac)
if [ "$OS" = "Darwin" ]; then
    echo "1. Copiando whisper-cli dal build locale..."
    if [ -f "$HOME/Projects/whisper.cpp/build/bin/whisper-cli" ]; then
        cp "$HOME/Projects/whisper.cpp/build/bin/whisper-cli" "$BINARIES_DIR/whisper-$TARGET"
        chmod +x "$BINARIES_DIR/whisper-$TARGET"
        echo "   ✓ whisper-cli copiato"
    else
        echo "   ✗ whisper-cli non trovato in ~/Projects/whisper.cpp/build/bin/"
        echo "   Compilalo prima con: cd ~/Projects/whisper.cpp && make"
    fi
fi

# 2. Copia ffmpeg (deve essere installato)
echo ""
echo "2. Cercando ffmpeg..."
FFMPEG_PATH=$(which ffmpeg 2>/dev/null || true)
if [ -n "$FFMPEG_PATH" ]; then
    cp "$FFMPEG_PATH" "$BINARIES_DIR/ffmpeg-$TARGET"
    chmod +x "$BINARIES_DIR/ffmpeg-$TARGET"
    echo "   ✓ ffmpeg copiato da $FFMPEG_PATH"
else
    echo "   ✗ ffmpeg non trovato. Installalo con:"
    echo "     Mac: brew install ffmpeg"
    echo "     Linux: sudo apt install ffmpeg"
    echo "     Windows: scarica da https://ffmpeg.org/download.html"
fi

# 3. Copia modello whisper
echo ""
echo "3. Cercando modello ggml-medium.bin..."
if [ -f "$HOME/Projects/whisper.cpp/models/ggml-medium.bin" ]; then
    cp "$HOME/Projects/whisper.cpp/models/ggml-medium.bin" "$MODELS_DIR/"
    echo "   ✓ Modello copiato (~1.5GB)"
else
    echo "   ✗ Modello non trovato. Scaricalo con:"
    echo "   curl -L -o $MODELS_DIR/ggml-medium.bin https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-medium.bin"
fi

echo ""
echo "=== Setup completato ==="
echo ""
echo "Contenuto $BINARIES_DIR:"
ls -la "$BINARIES_DIR"
echo ""
echo "Contenuto $MODELS_DIR:"
ls -lh "$MODELS_DIR" 2>/dev/null || echo "(vuoto)"
echo ""
echo "Per compilare l'app:"
echo "  cd $SCRIPT_DIR"
echo "  npm install"
echo "  npm run tauri build"
