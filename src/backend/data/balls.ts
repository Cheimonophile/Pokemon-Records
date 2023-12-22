



import { invoke } from "@tauri-apps/api"

type Params = {}

type Result = {
    name: string,
}[]


export async function readBalls(params: Params): Promise<Result> {
    return await invoke<Result>('read_balls', params)
}