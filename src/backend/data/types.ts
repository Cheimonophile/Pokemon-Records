



import { Command, command } from "backend/common"
import z from "zod"

type Params = {}

const Result = z.object({
    name: z.string(),
    color: z.string()
}).array()


/**
 * Reads catch types from the backend
 */
export const readTypes = command('read_types', Result) satisfies Command<Params>