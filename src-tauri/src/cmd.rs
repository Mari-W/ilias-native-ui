use std::path::{PathBuf};
use crate::ilias::api::{IliasApi, IliasError};
use crate::ilias::tree::{IliasContainer, sync_tree};

#[tauri::command]
pub async fn login(
    state: tauri::State<'_, IliasApi>,
    username: String,
    password: String,
) -> Result<(), String> {
    state.login(username, password).await.map_err(|e| e.to_string())
}


#[tauri::command]
pub async fn sync(
    state: tauri::State<'_, IliasApi>,
    root: String,
) -> Result<(), String> {
    let mut path = PathBuf::new();
    path.push(root);
    if !path.exists() {
        Err("invalid local ilias root".to_string())
    } else {
        sync_tree(IliasContainer::new_root(), &state, path).await.map_err(|e| e.to_string())
    }
}

#[tauri::command]
pub async fn open(
    path: String,
) -> Result<(), String> {
    open::that(path).map_err(|_| IliasError::IOOperationFailed.to_string())?;
    Ok(())
}
