import { invoke } from "@tauri-apps/api"
import { TeamMember } from "../types"


type ReadParams = {
    playthroughIdNo: string
}

type ReadResult = {
    ball: string,
    caught_date: string,
    caught_level: number,
    caught_location_name: string,
    caught_location_region: string,
    caught_species_name: string,
    gender: 'M' | 'F' | 'N',
    id: number,
    level: number,
    nickname: string | null,
    playthrough_id_no: string,
    playthrough: {
        adventure_started: string,
        id_no: string,
        name: string,
        version: string,
    }
    slot: number,
    species: {
        dex_no: number,
        generation: number,
        name: string,
        type1: string,
        type2: string | null,
    }
}[]


export async function readTeamMembers(params: ReadParams): Promise<TeamMember[]> {
    const results = await invoke<ReadResult>('read_team_members', params)
    const teamMembers = results.map((result): TeamMember => {
        return {
            id: result.id,
            playthrough: {
                idNo: result.playthrough.id_no,
                name: result.playthrough.name,
                version: result.playthrough.version,
                adventureStarted: result.playthrough.adventure_started,
            },
            slot: result.slot,
            nickname: result.nickname,
            caughtDate: result.caught_date,
            caughtLocationName: result.caught_location_name,
            caughtLocationRegion: result.caught_location_region,
            caughtSpeciesName: result.caught_species_name,
            caughtLevel: result.caught_level,
            ball: result.ball,
            gender: result.gender,
            level: result.level,
            species: {
                dexNo: result.species.dex_no,
                generation: result.species.generation,
                name: result.species.name,
                type1: result.species.type1,
                type2: result.species.type2,
            }
        }
    })
    return teamMembers
}