

export type Battle = {
    type: string,
    lost: boolean
    no: number
    opponent1: Trainer
    opponent2?: Trainer
    partner?: Trainer
    round: number
    location: Location
    playthrough: Playthrough
}



export type Playthrough = {
    idNo: string,
    name: string,
    version: string,
    adventureStarted: Date,
}


export type Location = {
    name: string,
    region: string,
}

export type Trainer = {
    name: string,
    class: string,
}

export type TeamMember = {
    id: number,
    playthrough: Playthrough,
    slot: number,
    nickname: string | null,
    caughtDate: Date,
    caughtLocationName: string,
    caughtLocationRegion: string,
    caughtSpeciesName: string,
    caughtLevel: number,
    ball: string,
    gender: 'M' | 'F' | 'N',
    level: number,
    species: Species
}

export type Species = {
    dexNo: number,
    generation: number,
    name: string,
    type1: string,
    type2: string | null,
}


export type Catch = {
    type: string,
    no: number,
    playthroughIdNo: string,
    location: Location,
    species: Species
}