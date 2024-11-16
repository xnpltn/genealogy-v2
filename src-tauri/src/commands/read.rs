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
            select 
                *  
            from relative
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
            *
        FROM 
            relative
        WHERE
            LOWER(sex) = LOWER('female')
        ORDER BY
            pinned DESC
        ;
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
            *
        FROM 
            relative
        WHERE
            LOWER(sex) = LOWER('male') AND employable >= 0
        ORDER BY
            pinned DESC
        ;
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
            select 
                *  
            from 
                relative 
            where 
                id = $1
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
    println!("get notes for:  {active_relative_id}");
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
pub async fn files_by_relative_id(app: AppHandle, active_relative_id: u32) -> Result<(), String> {
    println!("get files for:  {active_relative_id}");
    Ok(())
}
