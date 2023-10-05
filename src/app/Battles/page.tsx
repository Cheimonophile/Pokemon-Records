'use client'



import { FC, Fragment, useEffect, useState } from 'react'
import { ReadBattlesResult, readBattles } from '@/backend/battles'




export default function Page() {


    const [battles, setBattles] = useState<ReadBattlesResult[] | null>()


    useEffect(() => {
        (async () => {
            try {
                const battles = await readBattles()
                console.log(battles)
                setBattles(battles)
            }
            catch (error) {
                console.log(error)
            }
        })();
    }, [])

    return (
        <div className="w-full h-full flex flex-col">
            <h1 className="text-center">Battles</h1>
            <div className="flex-1 p-2">

                {/* Battles Table */}
                <div className="w-full h-full border-2 p-1 gap-1 overflow-y-scroll oveflow-x-hidden">
                    {battles?.map((battle, i) => (
                        <Fragment key={i}>
                            <BattleTableRow battle={battle} />
                        </Fragment>
                    ))}
                </div>

            </div>
        </div>
    )
}



const BattleTableRow: FC<{
    battle: ReadBattlesResult
}> = (props) => {

    // make battle title
    let title = `${props.battle.battle.opponent1_class} ${props.battle.battle.opponent1_name}`
    if (props.battle.battle.opponent2_class) {
        title += ` and ${props.battle.battle.opponent2_class}` + (props.battle.battle.opponent2_name ? ` ${props.battle.battle.opponent2_name}` : '')
    }
    if (props.battle.battle.partner_class) {
        title += ` with ${props.battle.battle.partner_class}` + (props.battle.battle.partner_name ? ` ${props.battle.battle.partner_name}` : '')
    }
    title += ` at ${props.battle.event.location_name}, ${props.battle.event.location_region}`
    if (props.battle.battle.lost) {
        title += " (lost)"
    }

    return (
        <div className="flex gap-1">
            <div>
                {props.battle.event.no}.
            </div>
            <div>
                {title}
            </div>
        </div>
    )
}