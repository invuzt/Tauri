use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use zip::write::FileOptions;
use walkdir::WalkDir;

#[tauri::command]
async fn kompres_file_asli(input_path: String) -> Result<String, String> {
    let path = Path::new(&input_path);
    if !path.exists() { return Err("File tidak ditemukan!".into()); }
    let output_path = format!("{}.zip", input_path);
    let file = File::create(&output_path).map_err(|e| e.to_string())?;
    let mut zip = zip::ZipWriter::new(file);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated).unix_permissions(0o755);
    zip.start_file(path.file_name().unwrap().to_string_lossy(), options).map_err(|e| e.to_string())?;
    let mut f = File::open(path).map_err(|e| e.to_string())?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
    zip.write_all(&buffer).map_err(|e| e.to_string())?;
    zip.finish().map_err(|e| e.to_string())?;
    Ok(format!("File dikompres ke: {}", output_path))
}

#[tauri::command]
async fn kompres_folder_asli(input_path: String) -> Result<String, String> {
    let path = Path::new(&input_path);
    if !path.exists() { return Err("Folder tidak ditemukan!".into()); }
    let output_path = format!("{}.zip", input_path);
    let file = File::create(&output_path).map_err(|e| e.to_string())?;
    let mut zip = zip::ZipWriter::new(file);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated).unix_permissions(0o755);

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let entry_path = entry.path();
        let name = entry_path.strip_prefix(path).map_err(|e| e.to_string())?;
        if entry_path.is_file() {
            zip.start_file(name.to_string_lossy(), options).map_err(|e| e.to_string())?;
            let mut f = File::open(entry_path).map_err(|e| e.to_string())?;
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
            zip.write_all(&buffer).map_err(|e| e.to_string())?;
        } else if !name.as_os_str().is_empty() {
            zip.add_directory(name.to_string_lossy(), options).map_err(|e| e.to_string())?;
        }
    }
    zip.finish().map_err(|e| e.to_string())?;
    Ok(format!("Folder dikompres ke: {}", output_path))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![kompres_file_asli, kompres_folder_asli])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
