
import { Command, command } from "backend/common"
import { Parse } from "backend/models"
import z from "zod"


/**
 * Read Battle Types from the backend
 */
export const readBattleTypes = command(
    'read_battle_types',
    Parse.BattleType.array(),
) satisfies Command<{}>