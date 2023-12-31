import { Command, command } from "backend/common";
import { Parse } from "backend/models";
import { z } from "zod";



/**
 * Reads events from the backend
 */
export const readEvents = command(
    'read_events', 
    Parse.Event.array(),
) satisfies Command<{
  playthroughIdNo: number,
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