
import { Command, command } from "backend/common"
import { z } from "zod"



type ReadParams = {
    howMany?: number,
}

const TResult = z.object({
    battle_type: z.string(),
    lost: z.boolean(),
    no: z.number(),
    opponent1_class: z.string(),
    opponent1_name: z.string(),
    opponent2_class: z.string().nullable(),
    opponent2_name: z.string().nullable(),
    partner_class: z.string().nullable(),
    partner_name: z.string().nullable(),
    round: z.number(),
    event: z.object({
        location_name: z.string(),
        location_region: z.string(),
        no: z.number(),
        playthrough_id_no: z.string(),
    })
}).array()

/**
 * Reads battles from the backend
 */
export const readBattles = command('read_battles', TResult) satisfies Command<ReadParams>


/**
 * Creates a battle in the backend
 */
export const createBattle = command(
    'create_battle',
    z.number()
) satisfies Command<{
    playthroughIdNo: string,
    locationId: number,
    battleType: string,
    opponent1Name: string,
    opponent1Class: string,
    opponent2Name?: string,
    opponent2Class?: string,
    partnerName?: string,
    partnerClass?: string,
    round: number,
    lost: boolean,
}>



/**
 * Updates a battle in the backend
 */
export const updateBattle = command(
    'update_battle',
    z.null()
) satisfies Command<{
    no: number,
    lost?: boolean,
}>


type DeleteParams = {
    no: number,
}

const DeleteResult = z.null()

/**
 * Deletes a battle from the backend
 */
export const deleteBattle = command('delete_battle', DeleteResult) satisfies Command<DeleteParams>