





import { Command, command } from "backend/common"
import { Parse } from "backend/models"
import { z } from "zod"

/**
 * Reads team members from the backend
 */
export const teamOverTime = command(
    'team_over_time',
    z.object({
        level: z.number(),
        species: Parse.Species,
        team_member: Parse.TeamMember,
    }).array().array()
) satisfies Command<{
    playthroughIdNo: string
}>