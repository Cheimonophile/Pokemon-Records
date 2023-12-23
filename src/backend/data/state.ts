



import { Command, command } from "backend/common"
import { z } from "zod"

type ReadParams = {
    databaseUrl: string
}

const ReadResult = z.null()


/**
 * Reads catch types from the backend
 */
export const setDBConnection = command('set_db_connection', ReadResult) satisfies Command<ReadParams>