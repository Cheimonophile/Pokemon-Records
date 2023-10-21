
import { invoke } from "@tauri-apps/api"
import { Battle, Catch } from "../types"



type ReadResult = {
    catch_type: string,
    no: number,
    event: {
        location_name: string,
        location_region: string,
        no: number,
        playthrough_id_no: string,
    }
}[]


export async function readCatches(): Promise<Catch[]> {
    const results = await invoke<ReadResult>('read_catches', {})
    const catches = results.map<Catch>(result => {
        return {
            type: result.catch_type,
            no: result.no,
            location: {
                name: result.event.location_name,
                region: result.event.location_region,
            },
            playthroughIdNo: result.event.playthrough_id_no,
        }
    })
    return catches
}



type CreateParams = {

}

type CreateResult = number


export async function createCatch(params: CreateParams): Promise<CreateResult> {
    const result = await invoke<CreateResult>('create_catch', params)
    return result
}


// type DeleteParams = {
//     no: number,
// }

// type DeleteResult = void


// export async function deleteCatch(params: DeleteParams): Promise<DeleteResult> {
//     const result = await invoke<DeleteResult>('delete_catch', params)
//     return result
// }