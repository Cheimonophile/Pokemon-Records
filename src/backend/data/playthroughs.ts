
import { invoke } from "@tauri-apps/api"
import { Command, command } from "backend/common"
import { Playthrough } from "types"
import { z } from "zod"



type ReadParams = {}

type ReadResult = {
    id_no: string,
    name: string,
    version: string,
    adventure_started: string,
}[]

const ReadResult = z.object({
    id_no: z.string(),
    name: z.string(),
    version: z.string(),
    adventure_started: z.string(),
}).array()

/**
 * Reads playthroughs from the backend
 */
export const readPlaythroughs = command('read_playthroughs', ReadResult) satisfies Command<ReadParams>