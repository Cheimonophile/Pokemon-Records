
import { invoke } from "@tauri-apps/api"
import { Command, command } from "backend/common"
import { Species } from "types"
import { z } from "zod"


type ReadParams = {
    name?: string
}

const ReadResult = z.object({
    name: z.string(),
    dex_no: z.number(),
    generation: z.number(),
    type1: z.string(),
    type2: z.string().nullable(),
}).array()


/**
 * Reads species from the backend
 */
export const readSpecies = command('read_species', ReadResult) satisfies Command<ReadParams>