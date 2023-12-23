
import { Command, command } from "backend/common"
import { z } from "zod"

type ReadParams = {
    name?: string
}

const ReadResult = z.string().array()

/**
 * Reads regions from the backend
 */
export const readRegions = command('read_regions', ReadResult) satisfies Command<ReadParams>