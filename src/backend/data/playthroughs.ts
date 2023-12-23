
import { Command, command } from "backend/common"
import { z } from "zod"



type ReadParams = {}

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