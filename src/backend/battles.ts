import { invoke } from "@tauri-apps/api"



export type ReadBattlesResult = {
    battle: {
        battle_type: string,
        lost: boolean
        no: number
        opponent1_class: string
        opponent1_name: string
        opponent2_class: string | null
        opponent2_name: string | null
        partner_class: string | null
        partner_name: string | null
        round: number
    }
    event: {
        location_name: string
        location_region: string
        no: number
        playthrough_id_no: string
    }
}


export async function readBattles(): Promise<ReadBattlesResult[]> {
    const result: ReadBattlesResult[] = await invoke('read_battles', {})
    return result
}