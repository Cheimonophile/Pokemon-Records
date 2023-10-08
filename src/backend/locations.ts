
import { invoke } from "@tauri-apps/api"
import { Location } from "../types"


const HANDLER = 'read_locations'

type ReadParams = {
    name?: string
    region?: string
}

type ReadResult = {
    name: string,
    region: string,
}[]


export async function readLocations(params: ReadParams): Promise<Location[]> {
    const locations = await invoke<ReadResult>(HANDLER, params)
    return locations
}