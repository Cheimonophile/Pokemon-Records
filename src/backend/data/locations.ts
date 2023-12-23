
import { Command, command } from "backend/common"
import { z } from "zod"


type ReadParams = {
    name?: string
    region?: string
}

const ReadResponse = z.object({
    name: z.string(),
    region: z.string(),
}).array()


/**
 * Reads locations from the backend
 */
export const readLocations = command('read_locations', ReadResponse) satisfies Command<ReadParams>


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