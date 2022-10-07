import Tag from "./tag";
import {it} from "node:test";
import {invoke} from "@tauri-apps/api";

enum TagOpsMode {
    Add = "Add",
    Remove = " Remove"
}

class TodoItem {
    id?: number
    message: string
    priority: Priority
    done: boolean = false
    tags: Tag[] = []

    constructor(message: string, priority: Priority, id?: number, done?: boolean, tags?: Tag[]) {
        this.id = id;
        this.message = message;
        this.priority = priority;
        if (done != undefined) {
            this.done = done;
        }
        if (tags != undefined) {
            this.tags = tags
        }
    }

    public static with_id(id: number, item: TodoItem): TodoItem {
        item.id = id;
        return item
    }

    public static async fetch_all(): Promise<TodoItem[]> {
        return await invoke<[number, TodoItem][]>("fetch_all_todo_item")
            .then((list) => {
                return list.map(([id, item]) => {
                    return TodoItem.with_id(id, item)
                })
            })
    }

    public async save(): Promise<void> {
        await invoke<number>("save_full_todo_item", {todo_item: this})
            .then((id) => {
                this.id = id
            })
    }

    public async edit_message(msg: string): Promise<void> {
        // fixme: this.id maybe a `undefined`
        await invoke<void>("edit_message", {item_id: this.id, new_message: msg})
            .then((_) => {
                this.message = msg
            })
    }

    public async edit_priority(priority: Priority): Promise<void> {
        // fixme: this.id maybe a `undefined`
        await invoke<void>("edit_priority", {item_id: this.id, priority: priority})
            .then((_) => {
                this.priority = priority
            })
    }

    public async revert_state(): Promise<void> {
        // fixme: this.id maybe a `undefined`
        await invoke<void>("state_revert", {item_id: this.id})
            .then((_) => this.done = !this.done)
    }

    public async add_tag(tag: Tag | string) {
        let tagObj: Tag;
        if (tag instanceof Tag) {
            tagObj = tag
        } else {
            tagObj = new Tag(tag)
        }

        // not save self, save tag first
        if (this.id == undefined) {
            if (tagObj.id != undefined) {
                await tagObj.create()
            }
        } else {
            await invoke<number>("edit_tag",
                {item_id: this.id, mode: TagOpsMode.Add, tag_name: tagObj.value})
                .then((id) => {
                    if (tagObj.id == undefined) {
                        tagObj.id = id
                    }
                });
        }
        // push
        this.tags.push(tagObj)

    }

    public async remove_tag(target: Tag) {
        // self id is undefined , just remove
        if (this.id == undefined) {
            // do nothing
        } else {
            //remove first
            await invoke<number>("edit_tag",
                {item_id: this.id, mode: TagOpsMode.Remove, tag_name: target.value})
            //
        }
        this.tags = this.tags.filter((tag) => {
            return tag.value != target.value
        })
    }

    public async clean_tags() {
        if (this.id != undefined) {
            await invoke<void>("clean_tag", {item_id: this.id})
        }

        this.tags = this.tags.filter(() => {
            return false
        })
    }
}

enum Priority {
    VeryHigh = "VeryHigh",
    High = "High,",
    Medium = "Medium,",
    Low = "Low,",
    VeryLow = "VeryLow,",
}

export {TodoItem, Priority}