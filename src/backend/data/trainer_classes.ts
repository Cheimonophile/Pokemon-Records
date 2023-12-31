
import { Command, command } from "backend/common"
import { Parse } from "backend/models"
import { z } from "zod"

/**
 * Reads regions from the backend
 */
export const readTrainerClasses = command(
    'read_trainer_classes',
    Parse.TrainerClass.array()
) satisfies Command<{}>


/**
 * Creates a trainer class in the backend
 */
export const createTrainerClass = command(
    'create_trainer_class',
    z.number()
) satisfies Command<{
    name: string,
}>