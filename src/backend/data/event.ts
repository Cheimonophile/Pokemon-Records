import { Command, command } from "backend/common";
import { Parse } from "backend/models";
import { z } from "zod";


/**
 * Type used to create a new event
 */
type CreateEvent = {
    playthroughIdNo: string,
    locationId: number,
    date: string,
    battle: null | {
        battleTypeId: number,
        opponent1Id: number,
        opponent2Id: number | null,
        partnerId: number | null,
        lost: boolean,
        round: number,
    },
    catch: null | {
        catchTypeId: number,
        teamMemberId: number,
    },
}


/**
 * Creates a new event
 */
export const createEvent = command(
    'create_event',
    z.number(),
) satisfies Command<{
    event: CreateEvent
}>


/**
 * Reads events from the backend
 */
export const readEvents = command(
    'read_events', 
    Parse.Event.array(),
) satisfies Command<{
  playthroughIdNo: string,
}>


/**
 * Update an event
 */
export const updateEvent = command(
    'update_event',
    z.null(),
) satisfies Command<{
    no: number,
    event: CreateEvent
}>

/**
 * Deletes an event from the backend
 */
export const deleteEvent = command(
    'delete_event',
    z.null()
) satisfies Command<{
    eventNo: number
}>