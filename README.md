# Whisper Subtitles

Applicazione cross-platform per generare sottotitoli VTT da file audio/video usando Whisper.

## Requisiti

- Node.js 18+
- Rust (per Tauri)
- ffmpeg installato
- whisper.cpp compilato

## Setup

1. Installa le dipendenze:
```bash
npm install
```

2. Prepara i binari (solo la prima volta):
```bash
chmod +x setup-binaries.sh
./setup-binaries.sh
```

3. Avvia in modalità sviluppo:
```bash
npm run tauri dev
```

4. Compila per la distribuzione:
```bash
npm run tauri build
```

## Build per altre piattaforme

### Windows
Su una macchina Windows:
1. Installa Visual Studio Build Tools
2. Compila whisper.cpp per Windows
3. Scarica ffmpeg per Windows
4. Rinomina i binari come `whisper-x86_64-pc-windows-msvc.exe` e `ffmpeg-x86_64-pc-windows-msvc.exe`
5. Esegui `npm run tauri build`

### Linux
Su una macchina Linux:
1. Compila whisper.cpp
2. Installa ffmpeg (`apt install ffmpeg`)
3. Esegui `./setup-binaries.sh`
4. Esegui `npm run tauri build`

## Struttura

```
whisper-subtitles/
├── src/                    # Frontend (HTML/JS)
├── src-tauri/
│   ├── src/main.rs        # Backend Rust
│   ├── binaries/          # Binari whisper/ffmpeg per OS
│   └── models/            # Modello ggml-medium.bin
└── setup-binaries.sh      # Script setup
```
