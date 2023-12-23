





import { invoke } from "@tauri-apps/api"
import { Command, command } from "backend/common"
import { z } from "zod"

type Params = {
    playthroughIdNo: string
}


const Result = z.object({
    level: z.number(),
    species: z.object({
        dex_no: z.number(),
        generation: z.number(),
        name: z.string(),
        type1: z.string(),
        type2: z.string().nullable(),
    }),
    team_member: z.object({
        ball: z.string(),
        caught_date: z.string(),
        caught_level: z.number(),
        caught_location_name: z.string(),
        caught_location_region: z.string(),
        caught_species_name: z.string(),
        gender: z.enum(['N', 'M', 'F']),
        id: z.number(),
        nickname: z.string().nullable(),
        playthrough_id_no: z.string(),
        slot: z.number(),
    }),
}).array().array()


/**
 * Reads team members from the backend
 */
export const teamOverTime = command('team_over_time', Result) satisfies Command<Params>