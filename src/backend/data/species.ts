
import { invoke } from "@tauri-apps/api"
import { Species } from "types"


type ReadParams = {
    name?: string
}

type ReadResult = {
    name: string
    dex_no: number
    generation: number
    type1: string
    type2: string | null
}[]


export async function readSpecies(params: ReadParams): Promise<Species[]> {
    const response = await invoke<ReadResult>("read_species", params)
    const species = response.map((result): Species => {
        return {
            name: result.name,
            dexNo: result.dex_no,
            generation: result.generation,
            type1: result.type1,
            type2: result.type2,
        }
    })
    return species
}