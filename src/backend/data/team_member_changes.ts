import { Command, command } from "backend/common";
import { z } from "zod";



type CUTeamMemberChange = {
  eventNo: number,
  teamMemberId: number,
  level: number | null,
  speciesId: number | null
}



/**
 * Creates a team member change in the backend
 */
export const createTeamMemberChange = command(
  'create_team_member_change',
  z.number()
) satisfies Command<{
  teamMemberChange: CUTeamMemberChange
}>