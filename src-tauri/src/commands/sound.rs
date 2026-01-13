use crate::{
    app::{client::AppState, CustomSound},
    helper,
};
use std::{
    fs,
    path::{Component, Path},
    sync::Mutex,
};
use utils::Error;

const MAX_SOUND_FILE_SIZE_BYTES: u64 = 10 * 1024 * 1024;
const ALLOWED_SOUND_EXTENSIONS: [&str; 3] = ["mp3", "wav", "ogg"];

fn validate_sound_file(file_path: &str) -> Result<String, Error> {
    let path = Path::new(file_path);
    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_ascii_lowercase())
        .filter(|ext| ALLOWED_SOUND_EXTENSIONS.contains(&ext.as_str()))
        .ok_or_else(|| {
            Error::new(
                "Sound",
                "Unsupported sound file type. Allowed: mp3, wav, ogg.",
                utils::get_location!(),
            )
        })?;

    let metadata = fs::metadata(path).map_err(|e| {
        Error::new(
            "Sound",
            &format!("Failed to read sound file metadata: {}", e),
            utils::get_location!(),
        )
    })?;
    if !metadata.is_file() {
        return Err(Error::new(
            "Sound",
            "Sound file path is not a file.",
            utils::get_location!(),
        ));
    }
    if metadata.len() > MAX_SOUND_FILE_SIZE_BYTES {
        return Err(Error::new(
            "Sound",
            "Sound file is too large. Max size is 10 MB.",
            utils::get_location!(),
        ));
    }

    Ok(extension)
}

fn validate_file_name(file_name: &str) -> Result<(), Error> {
    if file_name.trim().is_empty() {
        return Err(Error::new(
            "Sound",
            "Sound file name is required.",
            utils::get_location!(),
        ));
    }
    let path = Path::new(file_name);
    let mut components = path.components();
    match (components.next(), components.next()) {
        (Some(Component::Normal(_)), None) => Ok(()),
        _ => Err(Error::new(
            "Sound",
            "Invalid sound file name.",
            utils::get_location!(),
        )),
    }
}

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

    let extension = validate_sound_file(&file_path)?;

    // Add file to sound dir
    let sounds_path = helper::get_sounds_path();
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

    validate_file_name(&file_name)?;

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

#[tauri::command]
pub async fn sound_get_custom_sounds_path() -> Result<String, Error> {
    let sounds_path = helper::get_sounds_path();
    let path = sounds_path.to_str().ok_or_else(|| {
        Error::new(
            "Sound",
            "Failed to resolve sounds path.",
            utils::get_location!(),
        )
    })?;
    Ok(path.to_string())
}
