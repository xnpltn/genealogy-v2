use sqlx::Row;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use std::ops::Deref;
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager};
mod repo;
mod sql;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
/*


pub struct Relative {
    id: u16,
    name: String,
    age: Option<u8>,
    sameness: Option<f32>,
    mother: Option<String>,
    father: Option<String>,
    phone: Option<String>,
    email: Option<String>,
    pinned: bool,
}
* */

#[tauri::command]
async fn test_serde() -> repo::Relative {
    println!("I was invoked from JavaScript!");
    std::thread::sleep(std::time::Duration::from_millis(500));
    let rel = repo::Relative {
        id: 12,
        age: Some(12),
        name: "Me".into(),
        sameness: None,
        mother: Some("me".into()),
        father: Some("me".into()),
        phone: Some("phone".into()),
        email: Some("email".into()),
        pinned: false,
        lost_reason: Some("None".into()),
    };

    rel
}

#[tauri::command]
async fn my_custom_command() -> String {
    println!("I was invoked from JavaScript!");
    "Hello wolrd".to_string()
}

//#[tauri::command]
//async fn save_new_relative(
//    app: AppHandle,
//    new_relative: repo::Relative,
//) -> std::result::Result<(), Box<dyn std::error::Error>> {
//    println!("I was invoked from JavaScript!");
//    println!("new_relative: {}", new_relative.name);
//    Ok(())
//}

#[tauri::command]
async fn save_new_relative(app: AppHandle, new_relative: repo::Relative) -> Result<(), String> {
    let state = app.state::<AppData>();
    let pool = state.pool.clone();
    let sum: i64 = sqlx::query("select 1 + 1 as sum")
        .fetch_one(pool.deref())
        .await
        .unwrap()
        .get("sum");
    println!("{sum}, {}", new_relative.email.unwrap_or("".into()));
    Ok(())
}

#[tauri::command]
fn download(app: AppHandle, url: String) {
    println!("{url}");
    app.emit("test", "test").unwrap();
}

async fn connect_to_db(
    db_path: &str,
) -> std::result::Result<sqlx::SqlitePool, Box<dyn std::error::Error>> {
    if !Sqlite::database_exists(std::format!("{}/store.db", db_path).as_str())
        .await
        .unwrap_or(false)
    {
        Sqlite::create_database(std::format!("{}/store.db", db_path).as_str()).await?;
        let pool = SqlitePool::connect(std::format!("{}/store.db", db_path).as_str()).await?;
        Ok(pool)
    } else {
        let pool = SqlitePool::connect(std::format!("{}/store.db", db_path).as_str()).await?;
        Ok(pool)
    }
}

struct AppData {
    pool: Arc<SqlitePool>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let pool = connect_to_db(".").await?;
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            my_custom_command,
            download,
            test_serde,
            save_new_relative,
        ])
        .setup(move |app| {
            app.manage(AppData {
                pool: Arc::new(pool),
            });
            Ok(())
        })
        .run(tauri::generate_context!())?;
    Ok(())
}
