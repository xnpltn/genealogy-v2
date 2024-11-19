use crate::types;
use std::ops::Deref;
use tauri::{AppHandle, Manager};

#[tauri::command]
pub async fn delete_relative(app: AppHandle, relative_id: u32) -> Result<(), String> {
    let state = app.state::<types::State>();
    let pool = state.pool.clone();
    sqlx::query(
        r#"
        DELETE FROM
            relative 
        WHERE 
            id = $1
    
    "#,
    )
    .bind(relative_id)
    .execute(pool.deref())
    .await
    .map_err(|e| {
        println!("error deleting: {}", e.to_string());
        e.to_string()
    })?;
    Ok(())
}

#[tauri::command]
pub async fn delete_note(app: AppHandle, note_id: u32) -> Result<(), String> {
    let state = app.state::<types::State>();
    let pool = state.pool.clone();
    sqlx::query(
        r#"
        DELETE FROM
            note 
        WHERE 
            id = $1
    
    "#,
    )
    .bind(note_id)
    .execute(pool.deref())
    .await
    .map_err(|e| {
        println!("error deleting: {}", e.to_string());
        e.to_string()
    })?;
    Ok(())
}

#[tauri::command]
pub async fn delete_file(app: AppHandle, file_id: u32) -> Result<(), String> {
    let state = app.state::<types::State>();
    let pool = state.pool.clone();
    sqlx::query(
        r#"
        DELETE FROM 
            file 
        WHERE
            id= $1
    "#,
    )
    .bind(file_id)
    .execute(pool.deref())
    .await
    .map_err(|e| {
        println!("error deleting: {}", e.to_string());
        e.to_string()
    })?;
    Ok(())
}
