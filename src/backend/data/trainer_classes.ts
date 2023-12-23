
import { Command, command } from "backend/common"
import { z } from "zod"

type ReadParams = {
    name?: string
}

const ReadResult = z.string().array()

/**
 * Reads regions from the backend
 */
export const readTrainerClasses = command('read_trainer_classes', ReadResult) satisfies Command<ReadParams>


/**
 * Creates a trainer class in the backend
 */
export const createTrainerClass = command(
    'create_trainer_class',
    z.null()
) satisfies Command<{
    name: string,
}>