import { Command, command } from "backend/common"
import { Parse } from "backend/models"



export const readTeamMembers = command(
    'read_team_members',
    Parse.TeamMember.array(),
) satisfies Command<{
    playthroughIdNo: string
}> 