import { FC, Fragment, useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api'
import { ReadBattlesResult, readBattles } from '../backend/battles'
import { flexGrow } from '../styles'




export const Battles: FC<{}> = () => {


    const [battles, setBattles] = useState<ReadBattlesResult[] | null>()


    useEffect(() => {
        (async () => {
            try {
                const battles = await readBattles()
                console.log(battles)
                setBattles(battles)
            }
            catch (err) {
                console.error(err)
            }
        })()
    }, [])

    return (
        <div style={{
            height: '100%',
            width: '100%',
            display: 'flex',
            flexDirection: 'column'
        }}>
            <h1>Battles</h1>
            <div style={{
                flex: flexGrow,
                // padding: "0.5rem"
            }}>

                {/* Battles Table */}
                <div style={{
                    width: '100%',
                    height: '100%',
                    overflowY: 'auto',
                    padding: '0.25rem',
                    borderStyle: 'solid',
                }}>
                    <table className="border-separate border-spacing-x-2 border-spacing-y-1" style={{
                        borderCollapse: 'separate',
                        borderSpacing: '0.5rem 0.25rem',
                    }}>
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
    battle: any
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

    return (<tr>
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