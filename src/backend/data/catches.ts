
import { Command, command } from "backend/common"
import { z } from "zod"


const ReadResult = z.object({
    catch_type: z.string(),
    no: z.number(),
    species: z.object({
        dex_no: z.number(),
        generation: z.number(),
        name: z.string(),
        type1: z.string(),
        type2: z.string().nullable(),
    }),
    event: z.object({
        location_name: z.string(),
        location_region: z.string(),
        no: z.number(),
        playthrough_id_no: z.string(),
    })
}).array()

/**
 * Reads catches from the backend
 */
export const readCatches = command('read_catches', ReadResult) satisfies Command<{}>




type CreateParams = {
    playthroughIdNo: string,
    locationName: string,
    locationRegion: string,
    catchType: string,
    slot: number,
    speciesName: string,
    nickname: string | null,
    date: string,
    level: number,
    ball: string,
    gender: string,
}


const CreateResult = z.null()


/**
 * Creates a catch in the backend
 */
export const createCatch = command('create_catch', CreateResult) satisfies Command<CreateParams>