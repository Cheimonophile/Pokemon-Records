
import { invoke } from "@tauri-apps/api"
import { Trainer } from "../types"

const HANDLER = 'read_trainers'

type ReadParams = {
    name?: string
    class?: string
}

type ReadResult = {
    name: string,
    class: string,
}[]


export async function readTrainers(params: ReadParams): Promise<Trainer[]> {
    const trainers = await invoke<ReadResult>(HANDLER, params)
    return trainers
}