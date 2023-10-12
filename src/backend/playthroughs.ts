
import { invoke } from "@tauri-apps/api"
import { Playthrough } from "../types"



type ReadParams = {}

type ReadResult = {
    id_no: string,
    name: string,
    version: string,
    adventure_started: string,
}[]


export async function readPlaythroughs(params: ReadParams): Promise<Playthrough[]> {
    const results = await invoke<ReadResult>('read_playthroughs', params)
    const playthroughs = results.map((result): Playthrough => {
        return {
            idNo: result.id_no,
            name: result.name,
            version: result.version,
            adventureStarted: new Date(result.adventure_started),
        }
    })
    return playthroughs
}