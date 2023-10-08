
import { invoke } from "@tauri-apps/api"

const HANDLER = 'read_battle_types'

type ReadParams = {}

type ReadResult = {
    name: string,
}[]


export async function readBattleTypes(params: ReadParams): Promise<string[]> {
    const results = await invoke<ReadResult>(HANDLER, params)
    const playthroughs = results.map(battle_type => battle_type.name)
    return playthroughs
}