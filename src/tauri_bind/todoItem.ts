import {invoke} from "@tauri-apps/api";
import IdIsUndefinedError from "./IdIsUndefinedError";
import {Tag, TagInterface} from "./tag";

export interface Todo {
    id?: number
    message: string,
    priority: Priority,
    done: boolean,
    tags: TagInterface[]
}

export class TodoItem {
    private id?: number
    private message: string
    private priority: Priority
    private done: boolean = false
    private tags: Tag[] = []

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

    public static withId(id: number, item: TodoItem): TodoItem {
        item.id = id;
        return item
    }

    public static async fetchAll(): Promise<TodoItem[]> {
        return await invoke<[number, TodoItem][]>("fetch_all_todo_item")
            .then((list) => {
                return list.map(([id, item]) => {
                    return TodoItem.withId(id, item)
                })
            })
    }

    public getInner(): Todo {
        return {
            id: this.id, message: this.message, priority: this.priority, done: this.done, tags: this.tags.map((tag) => {
                return tag.getInner()
            })

        }
    }

    public async save(): Promise<void> {
        await invoke<number>("save_full_todo_item", {TodoItem: this})
            .then((id) => {
                this.id = id
            })
    }

    public async editMessage(msg: string): Promise<void> {
        if (this.id != undefined) {
            await invoke<void>("edit_message", {itemId: this.id, newMessage: msg})
        }
        this.message = msg
    }

    public async editPriority(priority: Priority): Promise<void> {
        if (this.id != undefined) {
            await invoke<void>("edit_priority", {itemId: this.id, priority: priority})
        }
        this.priority = priority
    }

    public async revertState(): Promise<void> {
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
            await invoke<number>("edit_tag", {
                itemId: this.id, mode: mode, tagName: tagObj.getValue()
            }).then(tagObj.setId);
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

    // Warning: call this function should consume this object
    public async removeThis() {
        if (this.id == undefined) {
            throw new IdIsUndefinedError('TodoItem')
        }

        await invoke<void>("delete_todo_item", {itemId: this.id});
        this.id = undefined
    }
}

export enum Priority {
    VeryHigh = "VeryHigh", High = "High", Medium = "Medium", Low = "Low", VeryLow = "VeryLow",
}

export enum TagOpsMode {
    Add = "Add", Remove = " Remove"
}

