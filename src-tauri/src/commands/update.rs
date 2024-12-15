use std::ops::Deref;
use tauri::{AppHandle, Manager};

use crate::types;
use crate::utils;

#[tauri::command]
pub async fn update_relative(
    app: AppHandle,
    relative: types::UpdateRelativeParams,
) -> Result<(), String> {
    let state = app.state::<types::State>();
    let pool = state.pool.clone();
    if !utils::is_valid_email(&relative.email.clone().unwrap_or("".to_string())) {
        return Err("Invalid Email Address".to_string());
    }

    if !utils::is_valid_phone(&relative.phone.clone().unwrap_or("".to_string())) {
        return Err("Invalid Phone Number".to_string());
    }

    if !utils::is_valid_state(&relative.state.clone().unwrap_or("".to_string())) {
        return Err("Invalid State".to_string());
    }

    if !utils::is_valid_state(&relative.state.clone().unwrap_or_default()) {
        return Err("Invalid State".to_string());
    }

    if !utils::is_valid_date(&relative.birthday.clone().unwrap_or_default()) {
        return Err("Date must be in format MMDDYYYY".to_string());
    }
    if !utils::is_valid_date(&relative.died_at.clone().unwrap_or_default()) {
        return Err("Date must be in format MMDDYYYY".to_string());
    }

    let sqlite_birthday = utils::sqlite_date(relative.birthday.clone().unwrap_or_default())
        .map_err(|e| e.to_string())?;
    let sqlite_diedat = utils::sqlite_date(relative.birthday.clone().unwrap_or_default())
        .map_err(|e| e.to_string())?;

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
            address = $19,
            city = $20,
            zipcode = $21,
            maiden_name= $22
        WHERE 
            id = $23
        ;
    "#,
    )
    .bind(relative.sameness)
    .bind(relative.lost_reason)
    .bind(relative.sex)
    .bind(sqlite_birthday)
    .bind(sqlite_diedat)
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
    .bind(relative.city)
    .bind(relative.zipcode)
    .bind(relative.maiden_name)
    .bind(relative.id)
    .execute(pool.deref())
    .await
    .map_err(|e| {
        println!("error updating: {}", e.to_string());
        "Error Updating Relative".to_string()
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

#[tauri::command]
pub async fn pin_note(app: AppHandle, note_id: u32) -> Result<(), String> {
    let state = app.state::<types::State>();
    let pool = state.pool.clone();
    sqlx::query(
        r#"
        UPDATE 
            note 
        SET 
            pinned = 1 
        WHERE 
            id = $1;
    "#,
    )
    .bind(note_id)
    .execute(pool.deref())
    .await
    .map_err(|e| {
        println!("error pinnign note:  err: {}", e.to_string());
        e.to_string()
    })?;
    Ok(())
}

#[tauri::command]
pub async fn unpin_note(app: AppHandle, note_id: u32) -> Result<(), String> {
    let state = app.state::<types::State>();
    let pool = state.pool.clone();
    sqlx::query(
        r#"
        UPDATE 
            note 
        SET 
            pinned = 0 
        WHERE 
            id = $1;
    "#,
    )
    .bind(note_id)
    .execute(pool.deref())
    .await
    .map_err(|e| {
        println!("error pining note: err: {}", e.to_string());
        e.to_string()
    })?;
    Ok(())
}

#[tauri::command]
pub async fn pin_file(app: AppHandle, file_id: u32) -> Result<(), String> {
    let state = app.state::<types::State>();
    let pool = state.pool.clone();
    sqlx::query(
        r#"
        UPDATE 
            file 
        SET 
            pinned = 1 
        WHERE 
            id = $1;
    "#,
    )
    .bind(file_id)
    .execute(pool.deref())
    .await
    .map_err(|e| {
        println!("error pinnign file:  err: {}", e.to_string());
        e.to_string()
    })?;
    Ok(())
}

#[tauri::command]
pub async fn unpin_file(app: AppHandle, file_id: u32) -> Result<(), String> {
    let state = app.state::<types::State>();
    let pool = state.pool.clone();
    sqlx::query(
        r#"
        UPDATE 
            file 
        SET 
            pinned = 0 
        WHERE 
            id = $1;
    "#,
    )
    .bind(file_id)
    .execute(pool.deref())
    .await
    .map_err(|e| {
        println!("error pining file: err: {}", e.to_string());
        e.to_string()
    })?;
    Ok(())
}

#[tauri::command]
pub async fn pin_image(app: AppHandle, image_id: u16) -> Result<(), String> {
    let state = app.state::<types::State>();
    let pool = state.pool.clone();
    sqlx::query(
        r#"
        UPDATE 
            image 
        SET 
            pinned = 1 
        WHERE 
            id = $1;
    "#,
    )
    .bind(image_id)
    .execute(pool.deref())
    .await
    .map_err(|e| {
        println!("error pining file: err: {}", e.to_string());
        e.to_string()
    })?;
    Ok(())
}
