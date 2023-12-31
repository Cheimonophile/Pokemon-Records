import { Command, command } from "backend/common";
import { Parse } from "backend/models";



/**
 * Reads events from the backend
 */
export const readEvents = command(
    'read_events', 
    Parse.Event.array(),
) satisfies Command<{
  playthroughIdNo: number,
}>