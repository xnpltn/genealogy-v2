use std::ops::Deref;
use tauri::{AppHandle, Manager};

use crate::types;

#[tauri::command]
pub async fn edit_note(
    app: AppHandle,
    edited_note: types::note::UpdateNoteParams,
) -> Result<(), String> {
    let state = app.state::<types::State>();
    let pool = state.pool.clone();
    println!("to update: {}", edited_note.text);
    sqlx::query(
        r#"
        UPDATE 
            note 
        SET 
            text = $1, pinned= $2 
        WHERE 
            id = $3;
    "#,
    )
    .bind(edited_note.text)
    .bind(edited_note.pinned)
    .bind(edited_note.id)
    .execute(pool.deref())
    .await
    .map_err(|e| {
        println!(
            "error deleting note: {}, err: {}",
            edited_note.id,
            e.to_string()
        );
        e.to_string()
    })?;
    Ok(())
}
