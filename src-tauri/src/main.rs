// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    genalogy_v2_lib::run().await?;
    Ok(())
}