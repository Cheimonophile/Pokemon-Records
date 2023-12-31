import z from 'zod';





export namespace Parse {

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
    adventure_started: z.string(),
    version: Version,
  })

  /**
   * Zod parser for a region in the database
   */
  export const Region = z.object({
    id: z.number(),
    name: z.string(),
  })


  /**
   * Zod parser for a location in the database
   */
  export const Location = z.object({
    id: z.number(),
    name: z.string(),
    region: Region,
  })
}

// create types from the models
export type Version = z.infer<typeof Parse.Version>
export type Playthrough = z.infer<typeof Parse.Playthrough>
export type Region = z.infer<typeof Parse.Region>
export type Location = z.infer<typeof Parse.Location>




