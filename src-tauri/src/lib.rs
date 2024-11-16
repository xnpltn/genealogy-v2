use directories::UserDirs;
use std::fs;
use std::io;
use std::sync::Arc;
use tauri::Manager;
mod commands;
mod database;
mod types;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() -> std::result::Result<(), Box<dyn std::error::Error>> {
    if let Some(user_dirs) = UserDirs::new() {
        let home_dir = user_dirs.home_dir();
        let work_dir = String::from(std::format!("{}/geneapp", home_dir.to_str().unwrap()));
        let files_dir = String::from(std::format!("{}/files", work_dir));
        let images_dir = String::from(std::format!("{}/images", work_dir));

        match fs::create_dir_all(std::format!("{work_dir}/files")) {
            Ok(()) => println!("working dir created"),
            Err(e) => {
                if e.kind() != io::ErrorKind::AlreadyExists {
                    return Err(Box::new(e));
                }
            }
        }
        match fs::create_dir_all(std::format!("{images_dir}")) {
            Ok(()) => println!("working dir files directory"),
            Err(e) => {
                if e.kind() != io::ErrorKind::AlreadyExists {
                    return Err(Box::new(e));
                }
            }
        }

        let pool = database::connect(&work_dir).await?;
        sqlx::query(&database::query::create_tables())
            .execute(&pool)
            .await?;
        tauri::Builder::default()
            .plugin(tauri_plugin_dialog::init())
            .plugin(tauri_plugin_shell::init())
            .invoke_handler(tauri::generate_handler![
                commands::create::create_relative,
                commands::read::all_relatives,
                commands::read::relative_by_id,
                commands::read::all_females,
                commands::read::all_employees,
                commands::read::notes_by_relative_id,
                commands::read::files_by_relative_id,
                commands::create::create_note,
                commands::delete::delete_note,
                commands::update::edit_note,
                commands::create::create_file,
            ])
            .setup(move |app| {
                app.manage(types::State {
                    pool: Arc::new(pool),
                    files_dir,
                    images_dir,
                });
                Ok(())
            })
            .run(tauri::generate_context!())?;
    }
    Ok(())
}
