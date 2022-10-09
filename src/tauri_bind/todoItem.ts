import Tag from "./tag";
import {invoke} from "@tauri-apps/api";


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
        await invoke<number>("save_full_todo_item", {TodoItem: this})
            .then((id) => {
                this.id = id
            })
    }

    public async edit_message(msg: string): Promise<void> {
        if (this.id != undefined) {
            await invoke<void>("edit_message", {itemId: this.id, newMessage: msg})
        }
        this.message = msg
    }

    public async edit_priority(priority: Priority): Promise<void> {
        if (this.id != undefined) {
            await invoke<void>("edit_priority", {itemId: this.id, priority: priority})
        }
        this.priority = priority
    }

    public async revert_state(): Promise<void> {
        if (this.id != undefined) {

            await invoke<void>("state_revert", {itemId: this.id})
        }
        this.done = !this.done
    }

    public async editTag(tag: Tag | string, mode: TagOpsMode) {
        let tagObj: Tag;
        if (tag instanceof Tag) {
            tagObj = tag
        } else {
            tagObj = new Tag(tag)
        }

        if (this.id != undefined) {
            await invoke<number>("edit_tag",
                {
                    itemId: this.id,
                    mode: mode,
                    tagName: tagObj.getValue()
                }
            ).then(tagObj.setId);
        }
        // add or remove
        if (mode == TagOpsMode.Add) {
            this.tags.push(tagObj)
        } else {
            this.tags = this.tags.filter(tagObj.checkNonEquals)
        }

    }

    public async cleanTags() {
        if (this.id != undefined) {
            await invoke<void>("clean_tag", {itemId: this.id})
        }

        this.tags = this.tags.filter(() => {
            return false
        })
    }
}

enum Priority {
    VeryHigh = "VeryHigh",
    High = "High",
    Medium = "Medium",
    Low = "Low",
    VeryLow = "VeryLow",
}

enum TagOpsMode {
    Add = "Add",
    Remove = " Remove"
}

export {TodoItem, Priority, TagOpsMode}