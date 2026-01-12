use crate::{
    app::{client::AppState, CustomSound},
    helper,
};
use std::fs;
use std::sync::Mutex;
use utils::Error;

#[tauri::command]
pub async fn sound_get_custom_sounds(
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<Vec<CustomSound>, Error> {
    let app = app.lock()?;
    Ok(app.settings.custom_sounds.clone())
}

#[tauri::command]
pub async fn sound_add_custom_sound(
    name: String,
    file_path: String,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<Vec<CustomSound>, Error> {
    let mut app = app.lock()?;
    
    // Add file to sound dir
    let sounds_path = helper::get_sounds_path();
    let extension = std::path::Path::new(&file_path)
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap_or("mp3"); // Default to mp3 if no extension, though validate on frontend
    
    let file_name = format!("{}.{}", uuid::Uuid::new_v4(), extension);
    let destination = sounds_path.join(&file_name);
    
    fs::copy(&file_path, &destination).map_err(|e| {
        Error::new(
            "Sound",
            &format!("Failed to copy sound file: {}", e),
            utils::get_location!(),
        )
    })?;
    
    // Add to settings
    let new_sound = CustomSound::new(name, file_name);
    app.settings.custom_sounds.push(new_sound);
    app.settings.save()?;
    
    Ok(app.settings.custom_sounds.clone())
}

#[tauri::command]
pub async fn sound_delete_custom_sound(
    file_name: String,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<Vec<CustomSound>, Error> {
    let mut app = app.lock()?;
    
    // Remove file from sounds dir
    let sounds_path = helper::get_sounds_path();
    let file_path = sounds_path.join(&file_name);
    
    if file_path.exists() {
        fs::remove_file(&file_path).map_err(|e| {
            Error::new(
                "Sound",
                &format!("Failed to delete sound file: {}", e),
                utils::get_location!(),
            )
        })?;
    }
    
    // Rem from settings
    app.settings.custom_sounds.retain(|s| s.file_name != file_name);
    app.settings.save()?;
    
    Ok(app.settings.custom_sounds.clone())
}
