import z from "zod";
import { Command, command } from "backend/common";

const TResult = z.object({
    name: z.string(),
}).array()

/**
 * Reads pokeballs from the frontend
 */
export const readBalls = command('read_balls', TResult) satisfies Command<{}>