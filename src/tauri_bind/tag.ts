import {message} from "@tauri-apps/api/dialog";
import {invoke} from "@tauri-apps/api";
import {TodoItem} from "./todoItem";
import IdIsUndefinedError from "./IdIsUndefinedError";

class Tag {
    id?: number
    value: string

    constructor(value: string, id?: number,) {
        this.id = id;
        this.value = value;
    }

    public static onError: (e: any) => Promise<void> = async (e) => {
        await message(`${e}`, {
            title: 'Handling \`Tag\` Failure', type: 'error'
        });
    };

    public static async get_tag_id(tagName: string): Promise<Tag> {
        return await invoke<number>("get_tag_id", {tagName: tagName})
            .then((id) => {
                return new Tag(tagName, id)
            })
    }

    public static async fetch_all(): Promise<Tag[]> {
        return await invoke<[number, string][]>("fetch_all_tags",)
            .then((list) => {
                return list.map(([id, value]) => {
                    return new Tag(value, id)
                })
            });
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

    public async fetch_all_tag_todo_item(): Promise<TodoItem[]> {
        return await this.on_id_ok((id) => {
            return invoke<[number, TodoItem][]>("fetch_all_tag_todo_item", {tagId: id})
                .then((list) => {
                    return list.map(([id, item]) => {
                        return TodoItem.with_id(id, item)
                    })
                })
        })

    }

    async on_id_ok<T>(handle: (id: number) => Promise<T>): Promise<T> {
        if (this.id != undefined) {
            return await handle(this.id)
        } else {
            await Tag.onError("The Id of Tag is Undefined")
            throw new IdIsUndefinedError("Tag")
        }
    }


}

export default Tag;
