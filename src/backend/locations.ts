
import { invoke } from "@tauri-apps/api"
import { Location } from "../types"


const HANDLER = 'read_locations'

type ReadParams = {
    name?: string
    region?: string
}

type ReadResponse = {
    name: string,
    region: string,
}[]


export async function readLocations(params: ReadParams): Promise<Location[]> {
    const locations = await invoke<ReadResponse>(HANDLER, params)
    return locations
}


type CreateParams = {
    name: string
    region: string
}

type CreateResponse = {
    name: string
    region: string
}

export async function createLocation(params: CreateParams): Promise<CreateResponse> {
    const location = await invoke<CreateResponse>('create_location', params)
    return location
}