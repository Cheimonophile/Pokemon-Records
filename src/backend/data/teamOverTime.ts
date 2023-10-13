





import { invoke } from "@tauri-apps/api"

type Params = {
    playthroughIdNo: string
}

type Result = {
    level: number,
    species: {
        dex_no: number,
        generation: number,
        name: string,
        type1: string,
        type2: string | null
    },
    team_member: {
        ball: string,
        caught_date: string,
        caught_level: number,
        caught_location_name: string,
        caught_location_region: string,
        caught_species_name: string,
        gender: 'N' | 'M' | 'F',
        id: number,
        nickname: string | null,
        playthrough_id_no: string,
        slot: number
    }
}[][]


export async function teamOverTime(params: Params): Promise<Result> {
    const results = await invoke<Result>('team_over_time', params)
    return results
}