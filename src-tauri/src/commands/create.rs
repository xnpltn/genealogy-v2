use sqlx::Row;
use std::ops::Deref;
use tauri::{AppHandle, Manager};

use crate::types;

#[tauri::command]
pub async fn create_relative(
    app: AppHandle,
    new_relative: types::CreateRelativeParams,
) -> Result<(), String> {
    let state = app.state::<types::State>();
    let pool = state.pool.clone();
    let sum: i64 = sqlx::query("select 1 + 1 as sum")
        .fetch_one(pool.deref())
        .await
        .unwrap()
        .get("sum");
    println!("{sum}, {}", new_relative.email.unwrap_or("".into()));
    Ok(())
}
