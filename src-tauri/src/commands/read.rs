#![allow(unused)]

use crate::types;
use std::ops::Deref;
use tauri::{AppHandle, Manager};

#[tauri::command]
pub async fn all_relatives(
    app: AppHandle,
) -> std::result::Result<Vec<types::RelativeIndividual>, String> {
    let state = app.state::<types::State>();
    let pool = state.pool.clone();
    let relatives: Vec<types::RelativeIndividual> = sqlx::query_as(
        r#"
        SELECT 
            r.*,
            m.fname AS mother,
            f.fname AS father
        FROM 
            relative r
        LEFT JOIN 
            relative m ON r.mother_id = m.id
        LEFT JOIN 
            relative f ON r.father_id = f.id
        ORDER BY
            r.pinned DESC
        "#,
    )
    .fetch_all(pool.deref())
    .await
    .map_err(|e| {
        println!("error reading all: {}", e.to_string());
        e.to_string()
    })?;
    Ok(relatives)
}

#[tauri::command]
pub async fn all_females(
    app: AppHandle,
) -> std::result::Result<Vec<types::RelativeIndividual>, String> {
    let state = app.state::<types::State>();
    let pool = state.pool.clone();
    let relatives: Vec<types::RelativeIndividual> = sqlx::query_as(
        r#"
        SELECT 
            r.*,
            m.fname AS mother,
            f.fname AS father
        FROM 
            relative r
        LEFT JOIN 
            relative m ON r.mother_id = m.id
        LEFT JOIN 
            relative f ON r.father_id = f.id
        WHERE
            LOWER(r.sex) = LOWER('female')
        ORDER BY
            r.pinned DESC
        "#,
    )
    .fetch_all(pool.deref())
    .await
    .map_err(|e| {
        println!("error reading all: {}", e.to_string());
        e.to_string()
    })?;
    Ok(relatives)
}

#[tauri::command]
pub async fn all_employees(
    app: AppHandle,
) -> std::result::Result<Vec<types::RelativeIndividual>, String> {
    let state = app.state::<types::State>();
    let pool = state.pool.clone();
    let relatives: Vec<types::RelativeIndividual> = sqlx::query_as(
        r#"
        SELECT 
            r.*,
            m.fname AS mother,
            f.fname AS father
        FROM 
            relative r
        LEFT JOIN 
            relative m ON r.mother_id = m.id
        LEFT JOIN 
            relative f ON r.father_id = f.id
        WHERE
            LOWER(r.sex) = LOWER('male') AND r.employable >= 0
        ORDER BY
            r.pinned DESC
        "#,
    )
    .fetch_all(pool.deref())
    .await
    .map_err(|e| {
        println!("error reading all: {}", e.to_string());
        e.to_string()
    })?;
    Ok(relatives)
}

#[tauri::command]
pub async fn relative_by_id(app: AppHandle, id: u32) -> Result<types::RelativeIndividual, String> {
    let state = app.state::<types::State>();
    let pool = state.pool.clone();
    let relative: types::RelativeIndividual = sqlx::query_as(
        r#"
        SELECT 
            r.*,
            m.fname AS mother,
            f.fname AS father
        FROM 
            relative r
        LEFT JOIN 
            relative m ON r.mother_id = m.id
        LEFT JOIN 
            relative f ON r.father_id = f.id
        WHERE
            r.id = $1
        "#,
    )
    .bind(id)
    .fetch_one(pool.deref())
    .await
    .map_err(|e| {
        println!("error getting one: {}", e.to_string());
        e.to_string()
    })?;

    Ok(relative)
}

#[tauri::command]
pub async fn notes_by_relative_id(
    app: AppHandle,
    active_relative_id: u32,
) -> Result<Vec<types::note::Note>, String> {
    let state = app.state::<types::State>();
    let pool = state.pool.clone();
    let notes: Vec<types::note::Note> = sqlx::query_as("select * from note where relative_id= $1")
        .bind(active_relative_id)
        .fetch_all(pool.deref())
        .await
        .map_err(|e| {
            println!("{}", e.to_string());
            e.to_string()
        })?;
    Ok(notes)
}

#[tauri::command]
pub async fn files_by_relative_id(
    app: AppHandle,
    active_relative_id: u32,
) -> Result<Vec<types::file::File>, String> {
    let state = app.state::<types::State>();
    let pool = state.pool.clone();
    let files: Vec<types::file::File> = sqlx::query_as(
        r#"
            SELECT 
                id, file_name, file_path, type, size, pinned
            FROM 
                file 
            WHERE 
                relative_id = $1;
        "#,
    )
    .bind(active_relative_id)
    .fetch_all(pool.deref())
    .await
    .map_err(|e| {
        println!("{}", e.to_string());
        e.to_string()
    })?;

    Ok(files)
}

#[tauri::command]
pub async fn female_parents(
    app: AppHandle,
    relative_id: u32,
) -> Result<Vec<types::ParantRelative>, String> {
    let state = app.state::<types::State>();
    let pool = state.pool.clone();
    let females: Vec<types::ParantRelative> = sqlx::query_as(
        r#"
            SELECT 
                id, fname, lname, mname
            FROM 
                relative
            WHERE
                id != $1 AND LOWER(sex) == 'female'
        "#,
    )
    .bind(relative_id)
    .fetch_all(pool.deref())
    .await
    .map_err(|e| {
        println!("{}", e.to_string());
        e.to_string()
    })?;
    Ok(females)
}

#[tauri::command]
pub async fn male_parents(
    app: AppHandle,
    relative_id: u32,
) -> Result<Vec<types::ParantRelative>, String> {
    let state = app.state::<types::State>();
    let pool = state.pool.clone();
    let females: Vec<types::ParantRelative> = sqlx::query_as(
        r#"
            SELECT 
                id, fname, lname, mname
            FROM 
                relative
            WHERE
                id != $1 AND LOWER(sex) == 'male'
        "#,
    )
    .bind(relative_id)
    .fetch_all(pool.deref())
    .await
    .map_err(|e| {
        println!("{}", e.to_string());
        e.to_string()
    })?;
    Ok(females)
}
