import z from 'zod';





export namespace Zod {

  /**
   * Zod parser for a version in the database
   */
  export const Version = z.object({
    id: z.number(),
    name: z.string(),
    generation: z.number(),
  })


  /**
   * Zod parser for a playthrough in the database
   */
  export const Playthrough = z.object({
    id_no: z.string(),
    name: z.string(),
    version_id: z.number(),
    adventure_started: z.string(),
    version: Version.optional(),
  })



}

// create types from the models
export type Version = z.infer<typeof Zod.Version>
export type Playthrough = z.infer<typeof Zod.Playthrough>




