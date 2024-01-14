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
    idNo: z.string(),
    name: z.string(),
    adventureStarted: z.string(),
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

  /**
   * Zod parser for a pokeball in the database
   */
  export const Ball = z.object({
    id: z.number(),
    name: z.string(),
  })


  /**
   * Zod parser for a battle type in the database
   */
  export const BattleType = z.object({
    id: z.number(),
    name: z.string(),
  })


  /**
   * Zod parser for a trainer class in the database
   */
  export const TrainerClass = z.object({
    id: z.number(),
    name: z.string(),
  })


  /**
   * Zod parser for a trainer in the database
   */
  export const Trainer = z.object({
    id: z.number(),
    name: z.string(),
    class: TrainerClass,
  })


  /**
   * Zod parser for a catch type in the database
   */
  export const CatchType = z.object({
    id: z.number(),
    name: z.string(),
  })

  /**
   * Zod parser for a type in the database
   */
  export const Type = z.object({
    id: z.number(),
    name: z.string(),
    color: z.string(),
  })

  /**
   * Zod parser for a type in the database
   */
  export const Species = z.object({
    id: z.number(),
    name: z.string(),
    form: z.string().nullable(),
    dexNo: z.number(),
    generation: z.number(),
    type1: Type,
    type2: Type.nullable(),
  })


  /**
   * Zod parser for a team member in the database
   */
  export const TeamMember = z.object({
    id: z.number(),
    playthrough: Playthrough,
    slot: z.number(),
    nickname: z.string().nullable(),
    caughtDate: z.string(),
    caughtLocation: Location,
    caughtSpecies: Species,
    caughtLevel: z.number(),
    ball: Ball,
    gender: z.string(),

    // flow fields
    species: Species,
    level: z.number(),
  })

  /**
   * Zod parser for an event in the database
   */
  export const Event = z.object({
    no: z.number(),
    playthrough: Playthrough,
    location: Location,
    date: z.string(),

    // battle subtype
    battle: z.object({
      battleType: BattleType,
      opponent1: Trainer,
      opponent2: Trainer.nullable(),
      partner: Trainer.nullable(),
      lost: z.boolean(),
      round: z.number(),
    }).nullable(),

    // catch subtype
    catch: z.object({
      catchType: CatchType,
      teamMember: TeamMember,
    }).nullable(),
  })
}

// create types from the models
export type Version = z.infer<typeof Parse.Version>
export type Playthrough = z.infer<typeof Parse.Playthrough>
export type Region = z.infer<typeof Parse.Region>
export type Location = z.infer<typeof Parse.Location>
export type Ball = z.infer<typeof Parse.Ball>
export type BattleType = z.infer<typeof Parse.BattleType>
export type TrainerClass = z.infer<typeof Parse.TrainerClass>
export type Trainer = z.infer<typeof Parse.Trainer>
export type CatchType = z.infer<typeof Parse.CatchType>
export type Type = z.infer<typeof Parse.Type>
export type Species = z.infer<typeof Parse.Species>
export type TeamMember = z.infer<typeof Parse.TeamMember>
export type Event = z.infer<typeof Parse.Event>





