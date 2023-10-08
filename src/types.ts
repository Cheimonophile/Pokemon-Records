

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