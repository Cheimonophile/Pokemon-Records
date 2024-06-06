import z from "zod";
import { Command, command } from "backend/common";
import { id } from "date-fns/locale";

/**
 * Reads pokeballs from the frontend
 */
export const readBalls = command(
  'read_balls',
  z.object({
    id: z.number(),
    name: z.string(),
  }).array()
) satisfies Command<{}>

