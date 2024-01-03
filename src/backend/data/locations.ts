
import { Command, command } from "backend/common"
import { Parse } from "backend/models"
import { z } from "zod"


/**
 * Reads locations from the backend
 */
export const readLocations = command(
    'read_locations', 
    Parse.Location.array(),
) satisfies Command<{
    id?: number,
    regionId?: number,
}>



/**
 * Creates a location in the backend
 */
export const createLocation = command(
    'create_location',
    z.number(),
) satisfies Command<{
    name: string,
    regionId: number,
}>