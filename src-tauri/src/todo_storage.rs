use futures::future::ok;
use futures::StreamExt;
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tap::Pipe;
use tauri::State;

use crate::database::models::tags::TagEntity;
use crate::database::models::tags::TagModel;

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoItem {
    message: String,
    priority: PriorityLevel,
    done: bool,
    tags: Vec<Tag>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PriorityLevel {
    VeryHigh,
    High,
    Medium,
    Low,
    VeryLow,
}

pub type Tag = String;

// TODO Item operates
#[tauri::command]
pub async fn save_full_todo_item(
    pool: State<'_, SqlitePool>,
    TodoItem {
        message,
        priority,
        done,
        tags,
    }: TodoItem,
) -> Result<i32, String> {
    // save tags
    let ids = tags
        .pipe(futures::stream::iter)
        .then(|tag| TagEntity::new(&pool, tag))
        .try_collect::<Vec<_>>()
        .await
        .map_err(|e| e.to_string())?;

    // save todo item

    // bind tags with items

    // return the id of this todo item
    todo!()
}
#[tauri::command]
pub async fn fetch_all_todo_item(pool: State<'_, SqlitePool>) -> Result<(i32, TodoItem), String> {
    // fetch all tags
    // fetch all todo items with its id
    // bind the tags and todo items

    todo!()
}
#[tauri::command]
pub async fn edit_message(
    pool: State<'_, SqlitePool>,
    item_id: i32,
    new_message: String,
) -> Result<(), String> {
    // update message

    todo!()
}
#[tauri::command]
pub async fn edit_priority(
    pool: State<'_, SqlitePool>,
    item_id: i32,
    priority: PriorityLevel,
) -> Result<(), String> {
    // update priority
    todo!()
}
#[tauri::command]
pub async fn state_revert(pool: State<'_, SqlitePool>, item_id: i32) -> Result<(), String> {
    // update done
    todo!()
}
#[derive(Debug, Serialize, Deserialize)]
pub enum EditMode {
    Add,
    Remove,
}
#[tauri::command]
pub async fn edit_tag(
    pool: State<'_, SqlitePool>,
    item_id: i32,
    mode: EditMode,
    tag_name: Tag,
) -> Result<i32, String> {
    match mode {
        // adding tag , create tag first then bind to the todo item
        EditMode::Add => todo!(),
        // remove tag , remove the bind between the tag and todo item
        EditMode::Remove => todo!(),
    }
}
#[tauri::command]
pub async fn clean_tag(pool: State<'_, SqlitePool>, item_id: i32) -> Result<(), String> {
    // remove all bind on todo item
    todo!()
}
#[tauri::command]
pub async fn delete_todo_item(
    pool: State<'_, SqlitePool>,
    item_id: i32,
) -> Result<TodoItem, String> {
    todo!()
}

// tag Operate
#[tauri::command]
pub async fn fetch_all_tags(pool: State<'_, SqlitePool>) -> Result<Vec<(i32, String)>, String> {
    // get all tags,group with tag id and value
    TagEntity::fetch_all(&pool, None)
        .await
        .map(|list| {
            list.into_iter()
                .map(|TagModel { id, value }| (id, value))
                .collect()
        })
        .map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn fetch_all_tag_todo_item(
    pool: State<'_, SqlitePool>,
    tag_id: i32,
) -> Result<(i32, TodoItem), String> {
    todo!()
}
#[tauri::command]
pub async fn rename_tag(
    pool: State<'_, SqlitePool>,
    tag_id: i32,
    tag_name: Tag,
) -> Result<(), String> {
    TagEntity::edit(&pool, tag_id, &tag_name)
        .await
        .map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn create_tag(pool: State<'_, SqlitePool>, tag_name: Tag) -> Result<i32, String> {
    TagEntity::new(&pool, &tag_name)
        .await
        .map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn get_tag_id(pool: State<'_, SqlitePool>, tag_name: Tag) -> Result<i32, String> {
    TagEntity::get_id(&pool, &tag_name)
        .await
        .map_err(|e| e.to_string())
}
#[tauri::command]
pub async fn delete_tag(pool: State<'_, SqlitePool>, tag_name: Tag) -> Result<(), String> {
    // todo!("remove bind with todo Item");

    // remove this
    TagEntity::remove(&pool ,& tag_name)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
