use std::collections::HashMap;

use futures::future::ok;
use futures::StreamExt;
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tap::Pipe;
use tauri::State;

use crate::database::models::tags::TagEntity;
use crate::database::models::tags::TagModel;
use crate::database::models::todo_item::TodoItemEntity;
use crate::database::models::todo_item::TodoItemModel;
use crate::util::ErrMapString;

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
        .err_to_str()?;

    // save todo item
    let todo_item_id = TodoItemEntity::save(&pool, message, priority.into(), done)
        .await
        .err_to_str()?;
    // bind tags with items

    // return the id of this todo item
    todo!()
}
#[tauri::command]
pub async fn fetch_all_todo_item(pool: State<'_, SqlitePool>) -> Result<(i32, TodoItem), String> {
    // fetch all tags
    let all_tags = TagEntity::fetch_all(&pool, None)
        .await
        .err_to_str()?
        .into_iter()
        .map(|TagModel { id, value }| (id, value))
        .collect::<HashMap<_, _>>();
    // fetch all tags-item bind;

    // fetch all todo items with its id
    let all_todo_items = TodoItemEntity::fetch_all(&pool, None)
        .await
        .err_to_str()?
        .into_iter()
        .map(
            |TodoItemModel {
                 id,
                 message,
                 priority,
                 done,
             }| {
                (
                    id,
                    TodoItem {
                        message,
                        priority: priority.into(),
                        done,
                        tags: vec![],
                    },
                )
            },
        )
        .collect::<Vec<_>>();
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
    TodoItemEntity::update_message(&pool, item_id, new_message)
        .await
        .err_to_str()
}
#[tauri::command]
pub async fn edit_priority(
    pool: State<'_, SqlitePool>,
    item_id: i32,
    priority: PriorityLevel,
) -> Result<(), String> {
    // update priority
    TodoItemEntity::update_priority(&pool, item_id, priority.into())
        .await
        .err_to_str()
}
#[tauri::command]
pub async fn state_revert(pool: State<'_, SqlitePool>, item_id: i32) -> Result<(), String> {
    // update done
    TodoItemEntity::revert_done(&pool, item_id)
        .await
        .err_to_str()
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
    // remove tag-time bind
    
    // remove item
    TodoItemEntity::remove(&pool,item_id).await.err_to_str()?;

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
        .err_to_str()
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
    TagEntity::edit(&pool, tag_id, &tag_name).await.err_to_str()
}
#[tauri::command]
pub async fn create_tag(pool: State<'_, SqlitePool>, tag_name: Tag) -> Result<i32, String> {
    TagEntity::new(&pool, &tag_name).await.err_to_str()
}
#[tauri::command]
pub async fn get_tag_id(pool: State<'_, SqlitePool>, tag_name: Tag) -> Result<i32, String> {
    TagEntity::get_id(&pool, &tag_name).await.err_to_str()
}
#[tauri::command]
pub async fn delete_tag(pool: State<'_, SqlitePool>, tag_name: Tag) -> Result<(), String> {
    // todo!("remove bind with todo Item");

    // remove this
    TagEntity::remove(&pool, &tag_name).await.err_to_str()?;

    Ok(())
}
