use std::ops::Deref;
use tauri::{AppHandle, Manager};

use crate::types;

#[tauri::command]
pub async fn update_relative(
    app: AppHandle,
    relative: types::RelativeIndividual,
) -> Result<(), String> {
    let state = app.state::<types::State>();
    let pool = state.pool.clone();
    sqlx::query(
        r#"
            UPDATE 
                relative 
            SET 
                sameness = $1,
                lost_reason = $2,
                sex = $3,
                birthday = $4,
                fname = $5,
                mname = $6,
                lname = $7,
                phone = $8,
                email = $9,
                pinned = $10,
                employable = $11 ,
                swarthy = $12, 
                hotness = $13, 
                crazy = $14
            WHERE 
                id = $15
            ;

    "#,
    )
    .bind(relative.sameness)
    .bind(relative.lost_reason)
    .bind(relative.sex)
    .bind(relative.birthday)
    .bind(relative.first_name)
    .bind(relative.middle_name)
    .bind(relative.last_name)
    .bind(relative.phone)
    .bind(relative.email)
    .bind(relative.pinned)
    .bind(relative.employable)
    .bind(relative.swarthy)
    .bind(relative.hotness)
    .bind(relative.crazy)
    .bind(relative.id)
    .execute(pool.deref())
    .await
    .map_err(|e| {
        println!("error updating: {}", e.to_string());
        e.to_string()
    })?;
    println!("updated");
    Ok(())
}

#[tauri::command]
pub async fn update_note(
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
