
import { Command, command } from "backend/common"
import { Parse } from "backend/models"

type ReadParams = {
    name?: string
}

/**
 * Reads regions from the backend
 */
export const readRegions = command(
    'read_regions',
    Parse.Region.array(),
) satisfies Command<{}>