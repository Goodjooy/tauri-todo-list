use std::collections::HashMap;

use futures::StreamExt;
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tap::Pipe;
use tap::Tap;
use tauri::command;
use tauri::State;

use crate::database::models::tag_item_bind::BindEntity;
use crate::database::models::tag_item_bind::BindModel;
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
#[command]
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
    BindEntity::save_all(
        &pool,
        ids.into_iter()
            .map(|tag_id| BindModel::new(tag_id, todo_item_id)),
    )
    .await
    .err_to_str()?;
    // return the id of this todo item
    Ok(todo_item_id)
}
#[command]
pub async fn fetch_all_todo_item(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<(i32, TodoItem)>, String> {
    // fetch all tags
    let all_tags = TagEntity::fetch_all(&pool, None)
        .await
        .err_to_str()?
        .into_iter()
        .map(|TagModel { id, value }| (id, value))
        .collect::<HashMap<_, _>>();
    // fetch all tags-item bind;
    let mut all_binds = BindEntity::fetch_all(&pool)
        .await
        .err_to_str()?
        .into_iter()
        .fold(
            HashMap::<i32, Vec<i32>>::new(),
            |mut map, BindModel { tag_id, item_id }| {
                map.entry(item_id)
                    .and_modify(|v| v.push(tag_id))
                    .or_insert(vec![tag_id]);
                map
            },
        );
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
        .map(|item| {
            item.tap_mut(|(item_id, item)| {
                if let Some(vec) = all_binds.remove(&item_id) {
                    item.tags.extend(
                        vec.into_iter()
                            .filter_map(|tag_id| all_tags.get(&tag_id))
                            .cloned(),
                    )
                }
            })
        })
        .collect::<Vec<_>>();
    // bind the tags and todo items

    Ok(all_todo_items)
}

#[command]
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

#[command]
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

#[command]
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
#[command]
pub async fn edit_tag(
    pool: State<'_, SqlitePool>,
    item_id: i32,
    mode: EditMode,
    tag_name: Tag,
) -> Result<i32, String> {
    let tag_id = TagEntity::new(&pool, &tag_name).await.err_to_str()?;

    match mode {
        // adding tag , create tag first then bind to the todo item
        EditMode::Add => BindEntity::save_all(&pool, [BindModel::new(tag_id, item_id)])
            .await
            .err_to_str(),
        // remove tag , remove the bind between the tag and todo item
        EditMode::Remove => BindEntity::remove(&pool, tag_id, item_id)
            .await
            .err_to_str(),
    }
    .map(|_| tag_id)
}

#[command]
pub async fn clean_tag(pool: State<'_, SqlitePool>, item_id: i32) -> Result<(), String> {
    // remove all bind on todo item
    BindEntity::remove_bind_item_id(&pool, item_id)
        .await
        .err_to_str()
}
#[command]
pub async fn delete_todo_item(pool: State<'_, SqlitePool>, item_id: i32) -> Result<(), String> {
    // remove tag-item bind
    clean_tag(pool.clone(), item_id).await?;
    // remove item
    TodoItemEntity::remove(&pool, item_id).await.err_to_str()?;

    Ok(())
}

// tag Operate
#[command]
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
#[command]
pub async fn fetch_all_tag_todo_item(
    pool: State<'_, SqlitePool>,
    tag_id: i32,
) -> Result<Vec<(i32, TodoItem)>, String> {
    let item_ids = BindEntity::fetch_all_tag_id(&pool, tag_id)
        .await
        .err_to_str()?;

    let mut resp = Vec::new();
    for TodoItemModel {
        id,
        message,
        priority,
        done,
    } in TodoItemEntity::find_all_by_id(&pool, item_ids)
        .await
        .err_to_str()?
    {
        resp.push((
            id,
            TodoItem {
                message,
                priority: priority.into(),
                done,
                tags: TagEntity::find_all_by_id(
                    &pool,
                    BindEntity::fetch_all_item_id(&pool, id)
                        .await
                        .err_to_str()?,
                )
                .await
                .err_to_str()?
                .into_iter()
                .map(|TagModel { value, .. }| value)
                .collect(),
            },
        ))
    }

    Ok(resp)
}
#[command]
pub async fn rename_tag(
    pool: State<'_, SqlitePool>,
    tag_id: i32,
    tag_name: Tag,
) -> Result<(), String> {
    TagEntity::edit(&pool, tag_id, &tag_name).await.err_to_str()
}
#[command]
pub async fn create_tag(pool: State<'_, SqlitePool>, tag_name: Tag) -> Result<i32, String> {
    TagEntity::new(&pool, &tag_name).await.err_to_str()
}
#[command]
pub async fn get_tag_id(pool: State<'_, SqlitePool>, tag_name: Tag) -> Result<i32, String> {
    TagEntity::get_id(&pool, &tag_name).await.err_to_str()
}
#[command]
pub async fn delete_tag(pool: State<'_, SqlitePool>, tag_name: Tag) -> Result<(), String> {
    let tag_id = TagEntity::get_id(&pool, &tag_name).await.err_to_str()?;

    BindEntity::remove_bind_tag_id(&pool, tag_id)
        .await
        .err_to_str()?;
    // remove this
    TagEntity::remove(&pool, &tag_name).await.err_to_str()?;

    Ok(())
}
