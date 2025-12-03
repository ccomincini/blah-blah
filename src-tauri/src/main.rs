#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::path::{Path, PathBuf};
use std::fs;
use std::io::Write;
use tauri::api::process::{Command, CommandEvent};
use tauri::{AppHandle, Manager, Window};

#[derive(Clone, serde::Serialize)]
struct ProgressPayload {
    percent: u32,
    message: String,
}

fn emit_progress(window: &Window, percent: u32, message: &str) {
    let _ = window.emit(
        "transcription-progress",
        ProgressPayload {
            percent,
            message: message.to_string(),
        },
    );
}

fn get_output_path(input_path: &str, language: &str) -> PathBuf {
    let path = Path::new(input_path);
    let stem = path.file_stem().unwrap_or_default().to_string_lossy();
    let lang_code = language.to_uppercase();
    let new_name = format!("{}_{}.vtt", stem, lang_code);
    path.parent().unwrap_or(Path::new(".")).join(new_name)
}

fn get_temp_wav_path(input_path: &str) -> PathBuf {
    let path = Path::new(input_path);
    let stem = path.file_stem().unwrap_or_default().to_string_lossy();
    path.parent()
        .unwrap_or(Path::new("."))
        .join(format!("{}_temp.wav", stem))
}

fn parse_vtt_time(time_str: &str) -> Option<f64> {
    let parts: Vec<&str> = time_str.trim().split(':').collect();
    if parts.len() != 3 {
        return None;
    }
    let hours: f64 = parts[0].parse().ok()?;
    let minutes: f64 = parts[1].parse().ok()?;
    let seconds: f64 = parts[2].parse().ok()?;
    Some(hours * 3600.0 + minutes * 60.0 + seconds)
}

fn format_vtt_time(seconds: f64) -> String {
    let h = (seconds / 3600.0).floor() as u32;
    let m = ((seconds % 3600.0) / 60.0).floor() as u32;
    let s = seconds % 60.0;
    format!("{:02}:{:02}:{:06.3}", h, m, s)
}

fn post_process_vtt(vtt_path: &Path, music_start: f64, music_end: f64, silence_start: f64, silence_end: f64) -> Result<(), String> {
    let content = fs::read_to_string(vtt_path)
        .map_err(|e| format!("Errore lettura VTT: {}", e))?;
    
    let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    let mut new_lines: Vec<String> = Vec::new();
    let mut added_intro = false;
    
    for line in &lines {
        // Aggiungi silenzio e musica prima del primo cue
        if line.contains(" --> ") && !added_intro {
            // Aggiungi silenzio se specificato
            if silence_end > silence_start && silence_start >= 0.0 {
                new_lines.push(format!("{} --> {}", format_vtt_time(silence_start), format_vtt_time(silence_end)));
                new_lines.push(" ".to_string());  // Sottotitolo vuoto per silenzio
                new_lines.push("".to_string());
            }
            
            // Aggiungi musica se specificata
            if music_end > music_start && music_start >= 0.0 {
                new_lines.push(format!("{} --> {}", format_vtt_time(music_start), format_vtt_time(music_end)));
                new_lines.push("[♪ Musica]".to_string());
                new_lines.push("".to_string());
            }
            
            added_intro = true;
        }
        
        new_lines.push(line.clone());
    }
    
    let mut file = fs::File::create(vtt_path)
        .map_err(|e| format!("Errore scrittura VTT: {}", e))?;
    
    for line in new_lines {
        writeln!(file, "{}", line).map_err(|e| format!("Errore scrittura: {}", e))?;
    }
    
    Ok(())
}

fn get_model_path(app_handle: &AppHandle) -> Result<PathBuf, String> {
    // In dev mode, usa il percorso relativo
    let dev_path = PathBuf::from("models/ggml-medium.bin");
    if dev_path.exists() {
        return Ok(dev_path);
    }
    
    // In produzione, cerca nella cartella resources dell'app
    let resource_path = app_handle
        .path_resolver()
        .resolve_resource("models/ggml-medium.bin")
        .ok_or("Modello non trovato nelle risorse dell'app")?;
    
    if resource_path.exists() {
        return Ok(resource_path);
    }
    
    Err(format!("Modello non trovato. Cercato in: {:?}", resource_path))
}

/// Copia le DLL dalla cartella resources alla cartella dei binaries su Windows
#[cfg(target_os = "windows")]
fn setup_dlls(app_handle: &AppHandle) -> Result<(), String> {
    use std::path::PathBuf;
    
    // Trova la directory dove sono i sidecar (whisper.exe, ffmpeg.exe)
    let sidecar_dir = app_handle
        .path_resolver()
        .resolve_resource("")
        .ok_or("Impossibile trovare la directory risorse")?
        .parent()
        .map(|p| p.to_path_buf())
        .ok_or("Impossibile trovare la directory parent")?;
    
    // In alternativa, trova la directory dell'eseguibile principale
    let exe_dir = std::env::current_exe()
        .map_err(|e| format!("Errore trovando exe: {}", e))?
        .parent()
        .map(|p| p.to_path_buf())
        .ok_or("Impossibile trovare directory exe")?;
    
    // Cerca le DLL nelle risorse
    if let Some(resources_dir) = app_handle.path_resolver().resolve_resource("") {
        if let Ok(entries) = fs::read_dir(&resources_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(ext) = path.extension() {
                    if ext == "dll" {
                        let dll_name = path.file_name().unwrap();
                        let dest_path = exe_dir.join(dll_name);
                        
                        // Copia la DLL se non esiste già
                        if !dest_path.exists() {
                            if let Err(e) = fs::copy(&path, &dest_path) {
                                eprintln!("Warning: impossibile copiare DLL {:?}: {}", dll_name, e);
                            } else {
                                println!("Copiata DLL: {:?}", dll_name);
                            }
                        }
                    }
                }
            }
        }
    }
    
    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn setup_dlls(_app_handle: &AppHandle) -> Result<(), String> {
    // Su macOS/Linux non serve fare nulla
    Ok(())
}

#[tauri::command]
async fn transcribe(
    app_handle: AppHandle,
    window: Window,
    file_path: String,
    language: String,
    music_start: f64,
    music_end: f64,
    silence_start: f64,
    silence_end: f64,
) -> Result<String, String> {
    let input_path = Path::new(&file_path);
    let extension = input_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    emit_progress(&window, 5, "Analisi file...");

    // Trova il percorso del modello
    let model_path = get_model_path(&app_handle)?;
    let model_path_str = model_path.to_str().ok_or("Percorso modello non valido")?;

    // Elimina VTT esistente per permettere sovrascrittura
    let output_vtt = get_output_path(&file_path, &language);
    if output_vtt.exists() {
        let _ = fs::remove_file(&output_vtt);
    }

    // Determina se serve conversione audio
    let audio_path: PathBuf;
    let needs_cleanup: bool;

    if extension == "wav" {
        audio_path = input_path.to_path_buf();
        needs_cleanup = false;
    } else {
        emit_progress(&window, 10, "Estrazione audio...");
        
        let temp_wav = get_temp_wav_path(&file_path);
        
        if temp_wav.exists() {
            let _ = fs::remove_file(&temp_wav);
        }
        
        let (mut rx, _child) = Command::new_sidecar("ffmpeg")
            .map_err(|e| format!("FFmpeg non trovato: {}", e))?
            .args([
                "-y",
                "-i", &file_path,
                "-ar", "16000",
                "-ac", "1",
                "-c:a", "pcm_s16le",
                temp_wav.to_str().unwrap(),
            ])
            .spawn()
            .map_err(|e| format!("Errore avvio ffmpeg: {}", e))?;

        let mut ffmpeg_stderr = String::new();
        
        while let Some(event) = rx.recv().await {
            match event {
                CommandEvent::Terminated(payload) => {
                    if payload.code != Some(0) {
                        return Err(format!("FFmpeg fallito (code {:?}): {}", payload.code, ffmpeg_stderr));
                    }
                    break;
                }
                CommandEvent::Stderr(line) => {
                    ffmpeg_stderr.push_str(&line);
                    ffmpeg_stderr.push('\n');
                    if line.contains("time=") {
                        emit_progress(&window, 20, "Estrazione audio...");
                    }
                }
                _ => {}
            }
        }

        audio_path = temp_wav;
        needs_cleanup = true;
    }

    emit_progress(&window, 30, "Avvio trascrizione...");

    let output_base = output_vtt.with_extension("");
    let lang_arg = if language == "auto" { "auto" } else { &language };

    let (mut rx, _child) = Command::new_sidecar("whisper")
        .map_err(|e| format!("Whisper non trovato: {}", e))?
        .args([
            "-m", model_path_str,
            "-l", lang_arg,
            "-ovtt",
            "-of", output_base.to_str().unwrap(),
            audio_path.to_str().unwrap(),
        ])
        .spawn()
        .map_err(|e| format!("Errore avvio whisper: {}", e))?;

    let mut current_progress = 30u32;
    let mut whisper_output = String::new();

    while let Some(event) = rx.recv().await {
        match event {
            CommandEvent::Stdout(line) => {
                whisper_output.push_str(&line);
                whisper_output.push('\n');
                if line.contains("-->") {
                    current_progress = (current_progress + 2).min(95);
                    emit_progress(&window, current_progress, "Trascrizione in corso...");
                }
            }
            CommandEvent::Stderr(line) => {
                whisper_output.push_str("[ERR] ");
                whisper_output.push_str(&line);
                whisper_output.push('\n');
            }
            CommandEvent::Terminated(payload) => {
                if payload.code != Some(0) {
                    if needs_cleanup {
                        let _ = std::fs::remove_file(&audio_path);
                    }
                    return Err(format!("Whisper fallito (code {:?}): {}", payload.code, whisper_output));
                }
                break;
            }
            _ => {}
        }
    }

    // Cleanup file WAV temporaneo
    if needs_cleanup {
        let _ = std::fs::remove_file(&audio_path);
    }

    // Verifica che il VTT sia stato creato
    if !output_vtt.exists() {
        return Err(format!("VTT non creato. Output whisper: {}", whisper_output));
    }

    emit_progress(&window, 95, "Finalizzazione...");

    if let Err(e) = post_process_vtt(&output_vtt, music_start, music_end, silence_start, silence_end) {
        eprintln!("Warning: post-processing failed: {}", e);
    }

    emit_progress(&window, 100, "Completato!");

    Ok(output_vtt.file_name().unwrap().to_string_lossy().to_string())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Setup DLL su Windows all'avvio
            if let Err(e) = setup_dlls(&app.handle()) {
                eprintln!("Warning: setup DLL fallito: {}", e);
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![transcribe])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
