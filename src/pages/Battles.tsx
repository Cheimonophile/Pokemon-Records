import { FC, Fragment, useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api'
import { readBattles } from '../backend/battles'
import { flexGrow } from '../styles'
import { message } from '@tauri-apps/api/dialog';
import { Battle, Playthrough } from '../types';
import { readPlaythroughs } from '../backend/playthroughs';






export const Battles: FC<{}> = () => {





    // battle table state
    const [battles, setBattles] = useState<Battle[] | null>()

    // fetch battles
    useEffect(() => {
        (async () => {
            try {
                const battles = await readBattles()
                console.log(battles)
                setBattles(battles)
            }
            catch (error) {
                console.error(error)
                await message(`${error}`, {
                    title: 'Error',
                    type: 'error',
                })
                setBattles(null)
            }
        })()
        return () => {
            setBattles(undefined)
        }
    }, [])


    return (
        <div style={{
            height: '100%',
            width: '100%',
            display: 'flex',
            gap: '0.25rem',
            flexDirection: 'column'
        }}>
            <h1>Battles</h1>

            {/* Create Battle */}
            <div style={{
                flex: 'none'
            }}>
                <CreateBattle />
            </div>

            {/* Battles Table */}
            <div style={{
                flex: flexGrow,
            }}>
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
    battle: Battle
}> = (props) => {

    // make battle title
    let title = `${props.battle.opponent1.class} ${props.battle.opponent1.name}`
    if (props.battle.opponent2) {
        title += ` and ${props.battle.opponent2.class}` + (props.battle.opponent2.name ? ` ${props.battle.opponent2.name}` : '')
    }
    if (props.battle.partner) {
        title += ` with ${props.battle.partner.class}` + (props.battle.partner.name ? ` ${props.battle.partner.name}` : '')
    }
    if (props.battle.lost) {
        title += " (lost)"
    }

    return (<tr>
        <td>
            {props.battle.no}.
        </td>
        <td>
            {title}
        </td>
        <td>
            {props.battle.location.name}
        </td>
        <td>
            {props.battle.location.region}
        </td>
    </tr>)
}



/**
 * Is the form to create a battle
 * 
 * @returns 
 */
const CreateBattle: FC<{}> = () => {


    // Playthroughs
    const [playthroughIdNo, setPlaythroughIdNo] = useState<string>("")
    const [playthroughs, setPlaythroughs] = useState<Playthrough[] | null>()
    useEffect(() => {
        (async () => {
            try {
                const playthroughs = await readPlaythroughs({})
                setPlaythroughs(playthroughs)
                console.log(playthroughs)
            }
            catch (error) {
                console.error(error)
                await message(`${error}`, {
                    title: 'Error',
                    type: 'error',
                })
                setPlaythroughs(null)
            }
        })()
        return () => {
            setPlaythroughs(undefined)
        }
    }, [])
    

    return (
        <div>
            {/* Playthrough selector */}
            <div>
                <label>Playthrough:</label>
                <select value={playthroughIdNo} onChange={e => setPlaythroughIdNo(e.target.value)}>
                    {playthroughs?.map((playthrough, i) => (
                        <option key={i} value={playthrough.idNo}>{playthrough.version} ({playthrough.adventureStarted.toISOString().slice(0,10)})</option>
                    ))}
                </select>
            </div>

            {/* Add Button */}
            <button>Create Battle</button>
        </div>
    )
}