
import { invoke } from "@tauri-apps/api"
import { Battle, Catch } from "../types"



type ReadResult = {
    catch_type: string,
    no: number,
    species: {
        dex_no: number,
        generation: number,
        name: string,
        type1: string,
        type2: string | null,
    },
    event: {
        location_name: string,
        location_region: string,
        no: number,
        playthrough_id_no: string,
    }
}[]


export async function readCatches(): Promise<Catch[]> {
    const results = await invoke<ReadResult>('read_catches', {})
    const catches = results.map((result): Catch => {
        return {
            playthroughIdNo: result.event.playthrough_id_no,
            type: result.catch_type,
            no: result.no,
            location: {
                name: result.event.location_name,
                region: result.event.location_region,
            },
            species: {
                dexNo: result.species.dex_no,
                generation: result.species.generation,
                name: result.species.name,
                type1: result.species.type1,
                type2: result.species.type2,
            }
        }
    })
    return catches
}



type CreateParams = {

}

type CreateResult = void


export async function createCatch(params: CreateParams): Promise<CreateResult> {
    await invoke<CreateResult>('create_catch', params)
}


// type DeleteParams = {
//     no: number,
// }

// type DeleteResult = void


// export async function deleteCatch(params: DeleteParams): Promise<DeleteResult> {
//     const result = await invoke<DeleteResult>('delete_catch', params)
//     return result
// }