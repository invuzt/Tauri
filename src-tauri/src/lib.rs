use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use zip::write::FileOptions;

#[tauri::command]
async fn kompres_file_asli(input_path: String) -> Result<String, String> {
    let path = Path::new(&input_path);
    if !path.exists() {
        return Err("File tidak ditemukan!".into());
    }

    // Nama file hasil zip
    let output_path = format!("{}.zip", input_path);
    let zip_file = File::create(&output_path).map_err(|e| e.to_string())?;
    let mut zip = zip::ZipWriter::new(zip_file);

    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    // Ambil nama file saja dari path lengkap
    let file_name = path.file_name()
        .and_then(|n| n.to_string_lossy().into())
        .unwrap_or_else(|| "file".to_string());

    zip.start_file(file_name, options).map_err(|e| e.to_string())?;
    
    let mut f = File::open(path).map_err(|e| e.to_string())?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
    
    zip.write_all(&buffer).map_err(|e| e.to_string())?;
    zip.finish().map_err(|e| e.to_string())?;

    Ok(format!("Berhasil! File tersimpan di: {}", output_path))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![kompres_file_asli])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
