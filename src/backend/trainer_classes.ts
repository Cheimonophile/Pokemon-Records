
import { invoke } from "@tauri-apps/api"

const HANDLER = 'read_trainer_classes'

type ReadParams = {}

type ReadResult = string[]


export async function readTrainerClasses(params: ReadParams): Promise<string[]> {
    const trainer_classes = await invoke<ReadResult>(HANDLER, params)
    return trainer_classes
}