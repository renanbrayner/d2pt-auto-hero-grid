use std::env;
use std::fs;
use std::path::PathBuf;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_hero_grid_path() -> Result<String, String> {
    let base_path =
        env::var("ProgramFiles(x86)").unwrap_or_else(|_| "C:\\Program Files (x86)".to_string());
    let mut path = PathBuf::from(base_path);
    path.push("Steam");
    path.push("userdata");

    // List all folders in the userdata directory
    let entries = match fs::read_dir(&path) {
        Ok(entries) => entries,
        Err(_) => return Err("Could not read userdata directory".to_string()),
    };

    // Filter for directories only
    let mut steam_ids: Vec<PathBuf> = entries
        .filter_map(|entry| {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    Some(path)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    if steam_ids.is_empty() {
        return Err("No Steam ID directories found".to_string());
    } else if steam_ids.len() == 1 {
        // If there's only one folder, automatically select it
        path = steam_ids.pop().unwrap();
    } else {
        // If there are multiple folders, you can return an error or handle it differently
        return Err(
            "Multiple Steam ID directories found. Please specify the Steam ID.".to_string(),
        );
    }

    // Continue building the path
    path.push("570");
    path.push("remote");
    path.push("cfg");

    // Convert the final path to a String
    Ok(path.to_string_lossy().to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![get_hero_grid_path])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
