use tauri_plugin_dialog::DialogExt;

#[tauri::command]
fn save_scene_file(app: tauri::AppHandle, contents: String) -> Result<bool, String> {
    let result = app
        .dialog()
        .file()
        .add_filter("Scene files", &["dat"])
        .blocking_save_file();

    match result {
        Some(fp) => {
            let path = fp.as_path().ok_or_else(|| "Not a file path".to_string())?;
            std::fs::write(path, &contents).map_err(|e| e.to_string())?;
            Ok(true)
        }
        None => Ok(false), // user cancelled
    }
}

#[tauri::command]
fn open_scene_file(app: tauri::AppHandle) -> Result<Option<String>, String> {
    let result = app
        .dialog()
        .file()
        .add_filter("Scene files", &["dat", "txt"])
        .blocking_pick_file();

    match result {
        Some(fp) => {
            let path = fp.as_path().ok_or_else(|| "Not a file path".to_string())?;
            let contents = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
            Ok(Some(contents))
        }
        None => Ok(None), // user cancelled
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![save_scene_file, open_scene_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
