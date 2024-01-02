
import { Command, command } from "backend/common"
import { Parse } from "backend/models"
import { z } from "zod"


/**
 * Reads trainers from the backend
 */
export const readTrainers = command(
    'read_trainers', 
    Parse.Trainer.array()
) satisfies Command<{
    id: number | null,
    classId: number | null
}>



/**
 * Creates a trainer in the backend
 */
export const createTrainer = command(
    'create_trainer', 
    z.number()
) satisfies Command<{
    name: string,
    class: string
}>