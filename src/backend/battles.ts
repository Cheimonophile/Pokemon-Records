'use client'

import { invoke } from "@tauri-apps/api"
import { Battle } from "../types"



type ReadResult = {
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
    event: {
        location_name: string
        location_region: string
        no: number
        playthrough_id_no: string
    }
}[]


export async function readBattles(): Promise<Battle[]> {
    const results = await invoke<ReadResult>('read_battles', {})
    let battles = results.map((result): Battle => {
        const opponent2 = result.opponent2_name && result.opponent2_class
            ? {
                name: result.opponent2_name,
                class: result.opponent2_class,
            } : undefined

        const partner = result.partner_name && result.partner_class
            ? {
                name: result.partner_name,
                class: result.partner_class,
            } : undefined

        return {
            type: result.battle_type,
            lost: result.lost,
            no: result.no,
            opponent1: {
                name: result.opponent1_name,
                class: result.opponent1_class,
            },
            opponent2,
            partner,
            round: result.round,
            location: {
                name: result.event.location_name,
                region: result.event.location_region,
            },
            playthrough: {
                idNo: result.event.playthrough_id_no,
                name: '',
                version: '',
                adventureStarted: new Date(),
            },
        }
    })
    return battles
}



type CreateParams = {
    playthroughIdNo: string,
    locationName: string,
    locationRegion: string,
    battleType: string,
    opponent1Name: string,
    opponent1Class: string,
    opponent2Name?: string,
    opponent2Class?: string,
    partnerName?: string,
    partnerClass?: string,
    round: number,
    lost: boolean,
}

type CreateResult = number


export async function createBattle(params: CreateParams): Promise<CreateResult> {
    const result = await invoke<CreateResult>('create_battle', params)
    return result
}