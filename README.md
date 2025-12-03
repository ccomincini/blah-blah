# Blah-Blah - Generatore di Sottotitoli

Applicazione desktop cross-platform per generare sottotitoli VTT da file audio/video usando OpenAI Whisper.

## Funzionalità

- Trascrizione automatica audio/video in sottotitoli VTT
- Supporto multilingua (italiano, inglese, spagnolo, francese, tedesco, auto-detect)
- Aggiunta manuale di marcatori musica e silenzio
- Interfaccia semplice con selezione file click-based

## Formati Supportati

**Input**: MP4, MP3, WAV, M4A, AVI, MKV, MOV  
**Output**: VTT (WebVTT)

---

## Compilazione macOS

### Prerequisiti

```bash
# Installa Homebrew (se non presente)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Installa Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Installa Node.js
brew install node

# Installa ffmpeg e compila whisper.cpp (o scarica binari precompilati)
brew install ffmpeg
```

### Setup Binaries

Nella cartella `src-tauri/binaries/` servono:
- `whisper-aarch64-apple-darwin` (per Apple Silicon) o `whisper-x86_64-apple-darwin` (per Intel)
- `ffmpeg-aarch64-apple-darwin` o `ffmpeg-x86_64-apple-darwin`

Nella cartella `src-tauri/models/` serve:
- `ggml-medium.bin` (scaricabile da [huggingface.co/ggerganov/whisper.cpp](https://huggingface.co/ggerganov/whisper.cpp))

### Compilazione

```bash
# Clona il repository
git clone https://github.com/ccomincini/blah-blah.git
cd blah-blah

# Installa dipendenze Node
npm install

# Sviluppo (hot reload)
npm run tauri dev

# Build produzione
npm run tauri build
```

L'installer `.dmg` viene generato in `src-tauri/target/release/bundle/dmg/`

---

## Compilazione Windows

### Prerequisiti

```powershell
# Installa Rust
winget install Rustlang.Rustup

# Installa Node.js
winget install OpenJS.NodeJS.LTS

# Installa Visual Studio Build Tools (con "Desktop development with C++")
winget install Microsoft.VisualStudio.2022.BuildTools
```

### Setup Binaries

Nella cartella `src-tauri\binaries\` servono:
- `whisper-x86_64-pc-windows-msvc.exe`
- `ffmpeg-x86_64-pc-windows-msvc.exe`
- `ggml.dll`
- `ggml-base.dll`
- `ggml-cpu.dll`
- `whisper.dll`
- `SDL2.dll`

Nella cartella `src-tauri\models\` serve:
- `ggml-medium.bin`

### Compilazione

```powershell
# Clona il repository
git clone https://github.com/ccomincini/blah-blah.git
cd blah-blah

# Installa dipendenze Node
npm install

# Sviluppo
npm run tauri dev

# Build produzione (solo installer NSIS)
$env:TAURI_NSIS_INSTALLER_HOOKS = "nsis-hooks.nsi"
npm run tauri build -- --bundles nsis
```

**Importante**: La variabile `TAURI_NSIS_INSTALLER_HOOKS` è necessaria per copiare le DLL nella cartella corretta durante l'installazione.

L'installer `.exe` viene generato in `src-tauri\target\release\bundle\nsis\`

---

## Compilazione Linux

### Prerequisiti (Ubuntu/Debian)

```bash
# Dipendenze di sistema
sudo apt update
sudo apt install -y build-essential libwebkit2gtk-4.0-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev

# Installa Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Installa Node.js
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt install -y nodejs

# Installa ffmpeg
sudo apt install ffmpeg
```

### Setup Binaries

Nella cartella `src-tauri/binaries/` servono:
- `whisper-x86_64-unknown-linux-gnu`
- `ffmpeg-x86_64-unknown-linux-gnu`

Nella cartella `src-tauri/models/` serve:
- `ggml-medium.bin`

### Compilazione

```bash
# Clona il repository
git clone https://github.com/ccomincini/blah-blah.git
cd blah-blah

# Installa dipendenze Node
npm install

# Build produzione
npm run tauri build
```

I pacchetti `.deb` e `.AppImage` vengono generati in `src-tauri/target/release/bundle/`

---

## Struttura Progetto

```
blah-blah/
├── src/                      # Frontend HTML/JS
├── src-tauri/
│   ├── src/main.rs          # Backend Rust (logica trascrizione)
│   ├── binaries/            # Eseguibili whisper/ffmpeg per ogni OS
│   ├── models/              # Modello Whisper (ggml-medium.bin)
│   ├── icons/               # Icone app
│   ├── nsis-hooks.nsi       # Hook per installer Windows
│   ├── tauri.conf.json      # Configurazione Tauri
│   └── Cargo.toml           # Dipendenze Rust
├── package.json
└── README.md
```

---

## Dove Trovare i Binaries

### Whisper.cpp
Compila da sorgente: [github.com/ggerganov/whisper.cpp](https://github.com/ggerganov/whisper.cpp)

```bash
# macOS/Linux
git clone https://github.com/ggerganov/whisper.cpp
cd whisper.cpp
make
# L'eseguibile 'main' va rinominato in whisper-{target}
```

### FFmpeg
- **macOS**: `brew install ffmpeg` poi copia da `/opt/homebrew/bin/ffmpeg`
- **Windows**: Scarica da [gyan.dev/ffmpeg](https://www.gyan.dev/ffmpeg/builds/)
- **Linux**: `apt install ffmpeg` poi copia da `/usr/bin/ffmpeg`

### Modello Whisper
Scarica `ggml-medium.bin` da:  
[huggingface.co/ggerganov/whisper.cpp/tree/main](https://huggingface.co/ggerganov/whisper.cpp/tree/main)

---

## Licenza

MIT
