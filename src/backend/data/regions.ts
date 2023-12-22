
import { invoke } from "@tauri-apps/api"

const HANDLER = 'read_regions'

type ReadParams = {
    name?: string
}

type ReadResult = string[]


export async function readRegions(params: ReadParams): Promise<string[]> {
    const regions = await invoke<ReadResult>(HANDLER, params)
    return regions
}