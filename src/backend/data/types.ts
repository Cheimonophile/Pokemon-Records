



import { invoke } from "@tauri-apps/api"

type Params = {}

type Result = {
    name: string,
    color: string
}[]


export async function readTypes(params: Params): Promise<Result> {
    return await invoke<Result>('read_types', params)
}