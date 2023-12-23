



import { invoke } from "@tauri-apps/api"
import { Command, command } from "backend/common"
import { z } from "zod"

type Params = {}

const Result = z.object({
    name: z.string(),
}).array()


/**
 * Reads catch types from the backend
 */
export const readCatchTypes = command('read_catch_types', Result) satisfies Command<Params>