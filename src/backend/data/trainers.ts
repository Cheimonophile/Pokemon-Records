
import { invoke } from "@tauri-apps/api"
import { Command, command } from "backend/common"
import { Trainer } from "types"
import { z } from "zod"

type ReadParams = {
    name?: string
    class?: string
}

const ReadResult = z.object({
    name: z.string(),
    class: z.string(),
}).array()


/**
 * Reads trainers from the backend
 */
export const readTrainers = command('read_trainers', ReadResult) satisfies Command<ReadParams>



type CreateParams = {
    name: string
    class: string
}


const CreateResult = z.object({
    name: z.string(),
    class: z.string(),
})

/**
 * Creates a trainer in the backend
 */
export const createTrainer = command('create_trainer', CreateResult) satisfies Command<CreateParams>