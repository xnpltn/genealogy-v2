use std::sync::Arc;
use tauri::Manager;
mod commands;
mod database;
mod types;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let pool = database::connect_to_db(".").await?;
    sqlx::query(&database::query::create_tables())
        .execute(&pool)
        .await?;
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![commands::create::create_relative,])
        .setup(move |app| {
            app.manage(types::State {
                pool: Arc::new(pool),
            });
            Ok(())
        })
        .run(tauri::generate_context!())?;
    Ok(())
}
