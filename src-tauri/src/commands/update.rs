use std::ops::Deref;
use tauri::{AppHandle, Manager};

use crate::types;

#[tauri::command]
pub async fn update_relative(
    app: AppHandle,
    relative: types::CreateRelativeParams,
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
            died_at = $5,
            fname = $6,
            mname = $7,
            lname = $8,
            phone = $9,
            email = $10,
            pinned = $11,
            hotness = $12,
            crazy = $13,
            swarthy = $14,
            employable = $15,
            mother_id = $16,
            father_id = $17,
            state = $18,
            address = $19
        WHERE 
            id = $20
        ;
    "#,
    )
    .bind(relative.sameness)
    .bind(relative.lost_reason)
    .bind(relative.sex)
    .bind(relative.birthday)
    .bind(relative.died_at)
    .bind(relative.first_name)
    .bind(relative.middle_name)
    .bind(relative.last_name)
    .bind(relative.phone)
    .bind(relative.email)
    .bind(relative.pinned)
    .bind(relative.hotness)
    .bind(relative.crazy)
    .bind(relative.swarthy)
    .bind(relative.employable)
    .bind(relative.mother_id)
    .bind(relative.father_id)
    .bind(relative.state)
    .bind(relative.address)
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
