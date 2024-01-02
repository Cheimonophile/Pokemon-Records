import { Command, command } from "backend/common"
import { Parse } from "backend/models"
import z from "zod"



export const readTeamMembers = command(
    'read_team_members',
    Parse.TeamMember.array(),
) satisfies Command<{
    playthroughIdNo: string
}> 