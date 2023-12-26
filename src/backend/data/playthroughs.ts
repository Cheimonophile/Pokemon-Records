
import { Command, command } from "backend/common"
import { Zod } from "backend/models"

/**
 * Reads playthroughs from the backend
 */
export const readPlaythroughs = command(
    'read_playthroughs',
    Zod.Playthrough.array()
) satisfies Command<{}>