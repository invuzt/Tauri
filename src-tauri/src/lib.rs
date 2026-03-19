#[tauri::command]
fn proses_ke_rust(input: String) -> String {
    // Di sini Rust bekerja sebagai mesin utama
    // Kita balik teksnya dan ubah ke Base64 sebagai contoh pemrosesan
    let reversed = input.chars().rev().collect::<String>();
    let encoded = format!("RUST_POWER:{}", b64_encode(&reversed));
    encoded
}

fn b64_encode(input: &str) -> String {
    use base64::{Engine as _, engine::general_purpose};
    general_purpose::STANDARD.encode(input)
}

#[cfg_ some_os_filter] // Tetap biarkan bagian run() bawaan di bawah
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![proses_ke_rust])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
