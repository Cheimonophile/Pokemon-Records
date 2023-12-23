import { Command, command } from "backend/common";
import { z } from "zod";




/**
 * Creates a team member change in the backend
 */
export const createTeamMemberChange = command(
  'create_team_member_change',
  z.null()
) satisfies Command<{
  eventNo: number,
  teamMemberId: number,
  level: number,
}>