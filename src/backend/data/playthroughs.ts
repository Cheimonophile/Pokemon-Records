
import { Command, command } from "backend/common"
import { Parse } from "backend/models"

/**
 * Reads playthroughs from the backend
 */
export const readPlaythroughs = command(
    'read_playthroughs',
    Parse.Playthrough.array()
) satisfies Command<{}>