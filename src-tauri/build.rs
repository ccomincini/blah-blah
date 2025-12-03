fn main() {
    // Su Windows, copia le DLL nella stessa directory dei binaries
    #[cfg(target_os = "windows")]
    {
        use std::fs;
        use std::path::Path;

        let binaries_dir = Path::new("binaries");
        
        if binaries_dir.exists() {
            // Trova tutte le DLL nella directory binaries
            if let Ok(entries) = fs::read_dir(binaries_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if let Some(ext) = path.extension() {
                        if ext == "dll" {
                            // Marca le DLL come risorse da includere
                            println!("cargo:rerun-if-changed={}", path.display());
                        }
                    }
                }
            }
        }
        
        println!("cargo:rerun-if-changed=binaries/");
    }

    tauri_build::build()
}
