use std::{fs, ops::Deref, path::Path};
use tauri::{AppHandle, Manager};

use crate::types;
use crate::utils;

#[tauri::command]
pub async fn create_relative(
    app: AppHandle,
    new_relative: types::CreateRelativeParams,
) -> Result<types::CreatedRelative, String> {
    let state = app.state::<types::State>();
    let pool = state.pool.clone();

    if !utils::is_valid_email(&new_relative.email.clone().unwrap_or_default()) {
        return Err("Invalid Email Adress".to_string());
    }

    if !utils::is_valid_phone(&new_relative.phone.clone().unwrap_or_default()) {
        return Err("Invalid Phone Number".to_string());
    }

    if !utils::is_valid_state(&new_relative.state.clone().unwrap_or_default()) {
        return Err("Invalid State".to_string());
    }

    if !utils::is_valid_date(&new_relative.birthday.clone().unwrap_or_default()) {
        return Err("Date must be in format MMDDYYYY".to_string());
    }
    if !utils::is_valid_date(&new_relative.died_at.clone().unwrap_or_default()) {
        return Err("Date must be in format MMDDYYYY".to_string());
    }

    let sqlite_birthday = utils::sqlite_date(new_relative.birthday.clone().unwrap_or_default())
        .map_err(|e| e.to_string())?;
    let sqlite_diedat = utils::sqlite_date(new_relative.birthday.clone().unwrap_or_default())
        .map_err(|e| e.to_string())?;
    let rel: types::CreatedRelative = sqlx::query_as(
        r#"
    INSERT INTO relative (
        sameness, lost_reason, sex, birthday, died_at, fname, mname, lname, maiden_name, 
        phone, email, pinned, hotness, crazy, swarthy, employable, 
        mother_id, father_id, state, address, city, zipcode
    ) 
    VALUES (
        $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
        $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22
    ) RETURNING id;
    "#,
    )
    .bind(new_relative.sameness)
    .bind(new_relative.lost_reason)
    .bind(new_relative.sex)
    .bind(sqlite_birthday)
    .bind(sqlite_diedat)
    .bind(new_relative.first_name)
    .bind(new_relative.middle_name)
    .bind(new_relative.last_name)
    .bind(new_relative.maiden_name)
    .bind(new_relative.phone)
    .bind(new_relative.email)
    .bind(new_relative.pinned)
    .bind(new_relative.hotness)
    .bind(new_relative.crazy)
    .bind(new_relative.swarthy)
    .bind(new_relative.employable)
    .bind(new_relative.mother_id)
    .bind(new_relative.father_id)
    .bind(new_relative.state)
    .bind(new_relative.address)
    .bind(new_relative.city)
    .bind(new_relative.zipcode)
    .fetch_one(pool.deref())
    .await
    .map_err(|e| {
        println!("error adding: {}", e.to_string());
        "Error Creating Relative".to_string()
    })?;
    Ok(rel)
}

#[tauri::command]
pub async fn create_note(
    app: AppHandle,
    params: types::note::CreateNoteParams,
) -> Result<(), String> {
    let state = app.state::<types::State>();
    let pool = state.pool.clone();
    sqlx::query(
        r#"
        INSERT INTO note
            (text, pinned, relative_id)
        VALUES
            ($1, $2, $3)
    "#,
    )
    .bind(params.text)
    .bind(params.pinned)
    .bind(params.relative_id)
    .execute(pool.deref())
    .await
    .map_err(|e| {
        println!("{}", e.to_string());
        e.to_string()
    })?;
    Ok(())
}

#[tauri::command]
pub async fn create_file(
    app: AppHandle,
    params: types::file::CreateFileParams,
) -> Result<(), String> {
    let state = app.state::<types::State>();
    let pool = state.pool.clone();
    let file = Path::new(&params.file_path);

    let mut file_size: u32 = 0;
    match fs::copy(
        Path::new(&params.file_path),
        Path::new(&std::format!(
            "{}/{}-{}",
            state.files_dir,
            params.relative_id,
            file.file_name().unwrap().to_str().unwrap().to_string()
        )),
    ) {
        Ok(n) => {
            file_size += n as u32;
            sqlx::query(
                r#"
                    INSERT INTO 
                        file (file_name, file_path, relative_id, type, size)
                    VALUES
                        ($1, $2, $3, $4, $5)
                "#,
            )
            .bind(file.file_name().unwrap().to_str().unwrap().to_string())
            .bind(std::format!(
                "{}/{}-{}",
                state.files_dir,
                params.relative_id,
                file.file_name().unwrap().to_str().unwrap().to_string()
            ))
            .bind(params.relative_id)
            .bind(file.extension().unwrap().to_str().unwrap().to_string())
            .bind(file_size)
            .execute(pool.deref())
            .await
            .map_err(|e| {
                println!("error adding file: {}", e.to_string());
                e.to_string()
            })?;
        }
        Err(e) => {
            println!("error : {}", e.to_string());
            return Err(e.to_string());
        }
    }
    Ok(())
}

#[tauri::command]
pub async fn create_image(app: AppHandle, params: types::CreateImageParams) -> Result<(), String> {
    let state = app.state::<types::State>();
    let pool = state.pool.clone();
    let file = Path::new(&params.image_path);

    match fs::copy(
        Path::new(&params.image_path),
        Path::new(&std::format!(
            "{}/{}-{}",
            state.images_dir,
            params.relative_id,
            file.file_name().unwrap().to_str().unwrap().to_string()
        )),
    ) {
        Ok(_) => {
            sqlx::query(
                r#"
                    INSERT INTO 
                        image (filename, relative_id)
                    VALUES
                        ($1, $2)
                "#,
            )
            .bind(std::format!(
                "{}/{}-{}",
                state.images_dir,
                params.relative_id,
                file.file_name().unwrap().to_str().unwrap().to_string()
            ))
            .bind(params.relative_id)
            .execute(pool.deref())
            .await
            .map_err(|e| {
                println!("error adding file: {}", e.to_string());
                e.to_string()
            })?;
        }
        Err(e) => {
            println!("error : {}", e.to_string());
            return Err(e.to_string());
        }
    }
    Ok(())
}
