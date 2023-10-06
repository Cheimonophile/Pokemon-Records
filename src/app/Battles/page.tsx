'use client'



import { FC, Fragment, useEffect, useState } from 'react'
import { ReadBattlesResult, readBattles } from '@/backend/battles'
import Link from 'next/link'




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
                <div>

                </div>
                <div className="w-full h-full border-2 p-1 overflow-y-scroll oveflow-x-hidden">
                    <table className="border-separate border-spacing-x-2 border-spacing-y-1">
                        <tbody>
                            {battles?.map((battle, i) => (
                                <Fragment key={i}>
                                    <BattleTableRow battle={battle} />
                                </Fragment>
                            ))}
                        </tbody>
                    </table>
                </div>

            </div>
        </div>
    )
}



const BattleTableRow: FC<{
    battle: ReadBattlesResult
}> = (props) => {

    // make battle title
    let title = `${props.battle.opponent1_class} ${props.battle.opponent1_name}`
    if (props.battle.opponent2_class) {
        title += ` and ${props.battle.opponent2_class}` + (props.battle.opponent2_name ? ` ${props.battle.opponent2_name}` : '')
    }
    if (props.battle.partner_class) {
        title += ` with ${props.battle.partner_class}` + (props.battle.partner_name ? ` ${props.battle.partner_name}` : '')
    }
    if (props.battle.lost) {
        title += " (lost)"
    }

    return (<tr className="gap-1">
        <td>
            {props.battle.event.no}.
        </td>
        <td>
            {title}
        </td>
        <td>
            {props.battle.event.location_name}
        </td>
        <td>
            {props.battle.event.location_region}
        </td>
    </tr>)
}