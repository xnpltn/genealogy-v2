use std::{ops::Deref, path::Path};
use tauri::{AppHandle, Manager};

use crate::types;

#[tauri::command]
pub async fn create_relative(
    app: AppHandle,
    new_relative: types::CreateRelativeParams,
) -> Result<(), String> {
    let state = app.state::<types::State>();
    let pool = state.pool.clone();
    sqlx::query(
        r#"
    INSERT INTO relative (
        sameness, lost_reason,  sex, birthday, fname, mname, lname, 
        phone, email, pinned, hotness, 
        crazy, swarthy, employable
    ) VALUES (
        $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
        $11, $12, $13, $14
    );
    "#,
    )
    .bind(new_relative.sameness)
    .bind(new_relative.lost_reason)
    .bind(new_relative.sex)
    .bind(new_relative.birthday)
    .bind(new_relative.first_name)
    .bind(new_relative.middle_name)
    .bind(new_relative.last_name)
    .bind(new_relative.phone)
    .bind(new_relative.email)
    .bind(new_relative.pinned)
    .bind(new_relative.hotness)
    .bind(new_relative.crazy)
    .bind(new_relative.swarthy)
    .bind(new_relative.employable)
    .execute(pool.deref())
    .await
    .map_err(|e| {
        println!("error adding: {}", e.to_string());
        e.to_string()
    })?;
    Ok(())
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

/*


CREATE TABLE IF NOT EXISTS file (
    file_name           TEXT NOT NULL,
    file_path           TEXT NOT NULL,
    relative_id         INTEGER  NOT NULL,
    type                TEXT NOT NULL,
    size                TEXT NOT NULL,
    pinned              BOOLEAN DEFAULT 0,
    FOREIGN KEY         (relative_id) REFERENCES relative(id) ON DELETE CASCADE
);

*/
#[tauri::command]
pub async fn create_file(
    app: AppHandle,
    params: types::file::CreateFileParams,
) -> Result<(), String> {
    let state = app.state::<types::State>();
    let pool = state.pool.clone();
    let file = Path::new(&params.file_path);
    let mut file_size = 0;
    sqlx::query(
        r#"
        INSERT INTO 
            file (file_name, file_path, relative_id, type)
        VALUES
            ($1, $2, $3, $4)
    "#,
    )
    .bind(file.file_name().unwrap().to_str().unwrap().to_string())
    .bind(file.to_str().unwrap().to_string())
    .bind(params.relative_id)
    .bind(file.extension().unwrap().to_str().unwrap().to_string())
    .bind(file_size)
    .execute(pool.deref())
    .await
    .map_err(|e| {
        println!("error adding file: {}", e.to_string());
        e.to_string()
    })?;
    Ok(())
}
