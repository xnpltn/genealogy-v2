use crate::types;
use std::ops::Deref;
use tauri::{AppHandle, Manager};

#[tauri::command]
pub async fn delete_relative(app: AppHandle, relative_id: u32) -> Result<(), String> {
    let state = app.state::<types::State>();
    let pool = state.pool.clone();

    sqlx::query(
        r#"
        UPDATE relative
        SET mother_id = CASE 
                WHEN mother_id = $1 THEN NULL 
                ELSE mother_id 
            END,
            father_id = CASE 
                WHEN father_id = $1 THEN NULL 
                ELSE father_id 
            END
        WHERE mother_id = $1 OR father_id = $1;
        "#,
    )
    .bind(relative_id)
    .execute(pool.deref())
    .await
    .map_err(|e| {
        println!("error updating references: {}", e.to_string());
        "Error Deleting Relative".to_string()
    })?;

    sqlx::query(
        r#"
        DELETE FROM relative 
        WHERE id = $1;
        "#,
    )
    .bind(relative_id)
    .execute(pool.deref())
    .await
    .map_err(|e| {
        println!("error deleting: {}", e.to_string());
        "Error Deleting Relative".to_string()
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
        "Error Deleting Note".to_string()
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
        "Error Deleting File".to_string()
    })?;
    Ok(())
}

#[tauri::command]
pub async fn delete_image(app: AppHandle, image_id: u32) -> Result<(), String> {
    let state = app.state::<types::State>();
    let pool = state.pool.clone();
    sqlx::query(
        r#"
        DELETE FROM 
            image 
        WHERE
            id= $1
    "#,
    )
    .bind(image_id)
    .execute(pool.deref())
    .await
    .map_err(|e| {
        println!("error deleting: {}", e.to_string());
        "Error Deleting Image".to_string()
    })?;
    Ok(())
}
