
import { invoke } from "@tauri-apps/api"
import { Command, command } from "backend/common"
import z from "zod"


const TResult = z.object({
    name: z.string(),
}).array()


/**
 * Read Battle Types from the backend
 */
export const readBattleTypes = command('read_battle_types', TResult) satisfies Command<{}>