#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use database::init_sqlite;

use crate::todo_storage::{
    clean_tag, create_tag, delete_tag, delete_todo_item, edit_message, edit_priority, edit_tag,
    fetch_all_tag_todo_item, fetch_all_tags, fetch_all_todo_item, get_tag_id, rename_tag,
    save_full_todo_item, state_revert,
};

mod database;
mod todo_storage;
mod util;
#[tokio::main]
async fn main() {
    let pool = init_sqlite().await;
    tauri::Builder::default()
        .manage(pool)
        .invoke_handler(tauri::generate_handler![
            // todo
            save_full_todo_item,
            fetch_all_todo_item,
            edit_message,
            edit_priority,
            state_revert,
            edit_tag,
            clean_tag,
            delete_todo_item,
            // tag
            fetch_all_tags,
            fetch_all_tag_todo_item,
            rename_tag,
            create_tag,
            get_tag_id,
            delete_tag
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
