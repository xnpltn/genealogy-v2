use directories::UserDirs;
use std::fs;
use std::io;
use std::sync::Arc;
use tauri::Manager;
mod commands;
mod database;
mod types;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() -> std::result::Result<(), Box<dyn std::error::Error>> {
    if let Some(user_dirs) = UserDirs::new() {
        let home_dir = user_dirs.home_dir();
        let work_dir = String::from(std::format!("{}/.geneapp", home_dir.to_str().unwrap()));
        let files_dir = String::from(std::format!("{}/files", work_dir));
        let images_dir = String::from(std::format!("{}/images", work_dir));

        match fs::create_dir_all(std::format!("{work_dir}/files")) {
            Ok(()) => {
                //pass
            }
            Err(e) => {
                if e.kind() != io::ErrorKind::AlreadyExists {
                    return Err(Box::new(e));
                }
            }
        }
        match fs::create_dir_all(std::format!("{images_dir}")) {
            Ok(()) => {
                //pass
            }
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
        let count_: (i64,) = sqlx::query_as(&database::query::check_maiden_name())
            .fetch_one(&pool)
            .await?;
        if count_.0 < 1 {
            sqlx::query(&database::query::add_maiden_name_column())
                .execute(&pool)
                .await?;
        }

        tauri::Builder::default()
            .plugin(tauri_plugin_dialog::init())
            .plugin(tauri_plugin_shell::init())
            .register_uri_scheme_protocol("asset", |_ctx, request| {
                let cleaned_path = percent_encoding::percent_decode_str(&request.uri().path())
                    .decode_utf8()
                    .unwrap()
                    .as_bytes()[1..]
                    .to_vec();
                let path_string = String::from_utf8(cleaned_path).unwrap_or_default();
                let path = std::path::Path::new(&path_string);
                match std::fs::read(path) {
                    Ok(data) => {
                        let content_type = if let Some(ext) = path.extension() {
                            if let Some(ext) = ext.to_str() {
                                match ext.to_lowercase().as_str() {
                                    "jpg" | "jpeg" => "image/jpeg",
                                    "png" => "image/png",
                                    "gif" => "image/gif",
                                    "webp" => "image/webp",
                                    "bmp" => "image/bmp",
                                    "ico" => "image/x-icon",
                                    "svg" => "image/svg+xml",
                                    _ => "image/*",
                                }
                            } else {
                                "image/*"
                            }
                        } else {
                            "image/*"
                        };

                        return tauri::http::Response::builder()
                            .status(200)
                            .header(tauri::http::header::CONTENT_TYPE, content_type)
                            .body(data)
                            .unwrap();
                    }
                    Err(e) => {
                        println!("error: {}", e.to_string());
                        return tauri::http::Response::builder()
                            .status(200)
                            .header(tauri::http::header::CONTENT_TYPE, "text/plain")
                            .body("failed to read file".as_bytes().to_vec())
                            .unwrap();
                    }
                }
            })
            .invoke_handler(tauri::generate_handler![
                commands::create::create_relative,
                commands::update::update_relative,
                commands::read::male_parents,
                commands::read::female_parents,
                commands::read::all_relatives,
                commands::read::relative_by_id,
                commands::read::all_females,
                commands::read::all_employees,
                commands::read::notes_by_relative_id,
                commands::read::files_by_relative_id,
                commands::create::create_note,
                commands::delete::delete_note,
                commands::update::update_note,
                commands::delete::delete_relative,
                commands::create::create_file,
                commands::delete::delete_file,
                commands::update::pin_file,
                commands::update::unpin_file,
                commands::update::pin_note,
                commands::update::unpin_note,
                commands::create::create_image,
                commands::read::images_by_relative_id,
                commands::update::pin_image,
                commands::delete::delete_image,
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
