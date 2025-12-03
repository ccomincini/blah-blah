# Build per Windows

## Prerequisiti da installare su Windows

1. **Node.js** (v18+): https://nodejs.org/
2. **Rust**: https://rustup.rs/
3. **Visual Studio Build Tools**: https://visualstudio.microsoft.com/visual-cpp-build-tools/
   - Seleziona "Desktop development with C++"

## Passaggi

### 1. Copia il progetto su Windows
Copia l'intera cartella `whisper-subtitles` su Windows.

### 2. Scarica i binari Windows

Scarica whisper.cpp precompilato per Windows:
- Vai su https://github.com/ggerganov/whisper.cpp/releases
- Scarica `whisper-bin-x64.zip`
- Estrai `whisper-cli.exe` (o `main.exe` rinominalo in `whisper.exe`)

Scarica ffmpeg per Windows:
- Vai su https://www.gyan.dev/ffmpeg/builds/
- Scarica `ffmpeg-release-essentials.zip`
- Estrai `ffmpeg.exe`

### 3. Posiziona i binari
Copia i file nella cartella `src-tauri/binaries/`:
```
src-tauri/binaries/whisper-x86_64-pc-windows-msvc.exe
src-tauri/binaries/ffmpeg-x86_64-pc-windows-msvc.exe
```

### 4. Copia il modello
Copia `ggml-medium.bin` in `src-tauri/models/`

### 5. Installa dipendenze e compila
```powershell
cd whisper-subtitles
npm install
npm run tauri build
```

### 6. Output
L'installer sarà in:
```
src-tauri/target/release/bundle/msi/Blah-Blah_1.0.0_x64.msi
```

Oppure l'eseguibile standalone:
```
src-tauri/target/release/Blah-Blah.exe
```
