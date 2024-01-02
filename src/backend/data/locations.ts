
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


type CreateParams = {
    name: string
    region: string
}

const CreateResponse = z.object({
    name: z.string(),
    region: z.string(),
})



/**
 * Creates a location in the backend
 */
export const createLocation = command('create_location', CreateResponse) satisfies Command<CreateParams>