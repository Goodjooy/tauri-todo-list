import {message} from "@tauri-apps/api/dialog";
import {invoke} from "@tauri-apps/api";
import {TodoItem} from "./todoItem";

class Tag {
    id?: number
    value: string

    constructor(value: string, id?: number,) {
        this.id = id;
        this.value = value;
    }

    public static on_error: (e: any) => Promise<void> = async (e) => {
        await message(`${e}`, {
            title: 'Handling \`Tag\` Failure',
            type: 'error'
        });
    };

    public static async get_tag_id(tag_name:string):Promise<Tag>{
        return await invoke<number>("get_tag_id")
            .then((id) => {
                return new Tag(tag_name, id)
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

    public async create(): Promise<void> {
        await invoke<number>("create_tag", {tag_name: this.value})
            .then((id) => {
                this.id = id
            })
    }

    public async rename(name: string): Promise<void> {
        return await this.on_id_ok((id) => {
            return invoke<void>("rename_tag", {tag_id: id, tag_name: name})
                .then(() => {
                    self.name = name
                })

        }, () => {
        })
    }

    public async fetch_all_tag_todo_item(): Promise<TodoItem[]> {
        return await this.on_id_ok((id) => {
            return invoke<[number, TodoItem][]>("fetch_all_tag_todo_item", {tag_id: id})
                .then((list) => {
                    return list.map(([id, item]) => {
                        return TodoItem.with_id(id, item)
                    })
                })
        }, () => [])

    }

    async on_id_ok<T>(handle: (id: number) => Promise<T>, default_data: () => T): Promise<T> {
        if (this.id != undefined) {
            return await handle(this.id)
        } else {
            await Tag.on_error("The Id of Tag is Undefined")
            return default_data()
        }
    }


}

export default Tag;
