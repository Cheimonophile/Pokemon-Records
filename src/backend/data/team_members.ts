import { Command, command } from "backend/common"
import z from "zod"


type ReadParams = {
    playthroughIdNo: string
}


const ReadResult = z.object({
    ball: z.string(),
    caught_date: z.string(),
    caught_level: z.number(),
    caught_location_name: z.string(),
    caught_location_region: z.string(),
    caught_species_name: z.string(),
    gender: z.enum(['M', 'F', 'N']),
    id: z.number(),
    level: z.number(),
    nickname: z.string().nullable(),
    playthrough_id_no: z.string(),
    playthrough: z.object({
        adventure_started: z.string(),
        id_no: z.string(),
        name: z.string(),
        version: z.string(),
    }),
    slot: z.number(),
    species: z.object({
        dex_no: z.number(),
        generation: z.number(),
        name: z.string(),
        type1: z.string(),
        type2: z.string().nullable(),
    })
}).array()



export const readTeamMembers = command('read_team_members', ReadResult) satisfies Command<ReadParams> 