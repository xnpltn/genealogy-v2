use crate::types;
use std::ops::Deref;
use tauri::{AppHandle, Manager};

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
    println!("{note_id}");
    Ok(())
}
