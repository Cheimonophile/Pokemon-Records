



import { invoke } from "@tauri-apps/api"
import { Command, command } from "backend/common"
import z from "zod"

type Params = {}

type Result = {
    name: string,
    color: string
}[]

const Result = z.object({
    name: z.string(),
    color: z.string()
}).array()


/**
 * Reads catch types from the backend
 */
export const readTypes = command('read_types', Result) satisfies Command<Params>