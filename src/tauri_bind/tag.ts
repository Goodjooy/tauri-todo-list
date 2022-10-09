import {invoke} from "@tauri-apps/api";
import {TodoItem} from "./todoItem";
import IdIsUndefinedError from "./IdIsUndefinedError";

export interface TagInterface {
    id?: number,
    value: string
}

export class Tag {
    private id?: number
    private readonly value: string

    constructor(value: string, id?: number,) {
        this.id = id;
        this.value = value;
    }

    public static async fetch(tagName: string): Promise<Tag> {
        return await invoke<number>("get_tag_id", {tagName: tagName})
            .then((id) => {
                return new Tag(tagName, id)
            })
    }

    public static async fetchAll(): Promise<Tag[]> {
        return await invoke<[number, string][]>("fetch_all_tags",)
            .then((list) => {
                return list.map(([id, value]) => {
                    return new Tag(value, id)
                })
            });
    }

    public getInner(): TagInterface {
        return {id: this.id, value: this.value}
    }

    public setId(id: number) {
        if (this.id == undefined) {
            this.id = id
        }
    }

    public getValue(): string {
        return this.value
    }

    public checkEquals(rhs: Tag): boolean {
        return this.value == rhs.value
    }

    public checkNonEquals(rhs: Tag): boolean {
        return !this.checkEquals(rhs)
    }

    public async create(): Promise<void> {
        await invoke<number>("create_tag", {tagName: this.value})
            .then((id) => {
                this.id = id
            })
    }

    public async rename(name: string): Promise<void> {
        if (this.id != undefined) {
            await invoke<void>("rename_tag", {tagId: this.id, tagName: name})
        }
        self.name = name

    }

    public async fetchAllRelateTodoItem(): Promise<TodoItem[]> {
        return await this.whenIdValid((id) => {
            return invoke<[number, TodoItem][]>("fetch_all_tag_todo_item", {tagId: id})
                .then((list) => {
                    return list.map(([id, item]) => {
                        return TodoItem.withId(id, item)
                    })
                })
        })

    }
    // Warning: call this function should consume this object
    public async removeThis() {
        await invoke("delete_tag", {tagName: this.value});
        this.id = undefined
    }


    private async whenIdValid<T>(handle: (id: number) => Promise<T>): Promise<T> {
        if (this.id != undefined) {
            return await handle(this.id)
        } else {
            throw new IdIsUndefinedError("Tag")
        }
    }


}

