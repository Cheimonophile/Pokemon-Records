import { FC, Fragment, useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api'
import { readBattles } from '../backend/battles'
import { flexGrow } from '../styles'
import { message } from '@tauri-apps/api/dialog';
import { Battle, Playthrough, Trainer } from '../types';
import { readPlaythroughs } from '../backend/playthroughs';
import { readBattleTypes } from '../backend/battle_types';
import { readTrainerClasses } from '../backend/trainer_classes';
import { readTrainers } from '../backend/trainers';






export const Battles: FC<{}> = () => {





    // battle table state
    const [battles, setBattles] = useState<Battle[] | null>()

    // fetch battles
    useEffect(() => {
        (async () => {
            try {
                const battles = await readBattles()
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
    const [playthroughOptions, setPlaythroughOptions] = useState<Playthrough[]>()
    useEffect(() => {
        (async () => {
            try {
                const playthroughs = await readPlaythroughs({})
                setPlaythroughOptions(playthroughs)
            }
            catch (error) {
                console.error(error)
                await message(`${error}`, {
                    title: 'Error Reading Playthroughs',
                    type: 'error',
                })
            }
        })()
    }, [])

    // battle type
    const [battleType, setBattleType] = useState<string>("Single")
    const [battleTypeOptions, setBattleTypeOptions] = useState<string[]>()
    useEffect(() => {
        (async () => {
            try {
                const battleTypes = await readBattleTypes({})
                setBattleTypeOptions(battleTypes)
            }
            catch (error) {
                console.error(error)
                await message(`${error}`, {
                    title: 'Error Reading Battle Types',
                    type: 'error',
                })
            }
        })()
    }, [])


    // opponent 1
    const [opponent1, setOpponent1] = useState<Trainer>({
        name: "",
        class: "",
    })
    const [opponent1Validity, setOpponent1Validity] = useState<{ name: boolean, class: boolean }>({ name: false, class: false })
    useEffect(() => {
        // trainer classes
        (async () => {
            try {
                const trainerClasses = await readTrainerClasses({ name: opponent1.class })
                setOpponent1Validity(prev => ({ ...prev, class: trainerClasses.length > 0 }))
            }
            catch (error) {
                console.error(error)
                await message(`${error}`, {
                    title: 'Error Reading Trainer Classes',
                    type: 'error',
                })
            }
        })();
        // trainers
        (async () => {
            try {
                const trainers = await readTrainers({ name: opponent1.name, class: opponent1.class })
                setOpponent1Validity(prev => ({ ...prev, name: trainers.length > 0 }))
            }
            catch (error) {
                console.error(error)
                await message(`${error}`, {
                    title: 'Error Reading Trainers',
                    type: 'error',
                })
            }
        })();
    }, [opponent1])

    return (
        <div>
            {/* Playthrough selector */}
            <div>
                <label>Playthrough:</label>
                <select value={playthroughIdNo} onChange={e => setPlaythroughIdNo(e.target.value)}>
                    {playthroughOptions?.map((playthrough, i) => (
                        <option key={i} value={playthrough.idNo}>{playthrough.version} ({playthrough.adventureStarted.toISOString().slice(0, 10)})</option>
                    ))}
                </select>
            </div>

            {/* Battle Type */}
            <div>
                <label>Battle Type:</label>
                <select value={battleType} onChange={e => setBattleType(e.target.value)}>
                    {battleTypeOptions?.map((battleType, i) => (
                        <option key={i} value={battleType}>{battleType}</option>
                    ))}
                </select>
            </div>

            {/* Opponent 1 */}
            <div>
                <label>Opponent 1:</label>
                <input
                    type="text"
                    style={{
                        color: opponent1Validity.class ? undefined : 'red',
                    }}
                    value={opponent1.class}
                    onChange={e => setOpponent1(prev => ({ ...prev, class: e.target.value }))}
                />
                <input
                    type="text"
                    style={{
                        color: opponent1Validity.name ? undefined : 'red',
                    }}
                    value={opponent1.name}
                    onChange={e => setOpponent1(prev => ({ ...prev, name: e.target.value }))}
                />
            </div>

            {/* Add Button */}
            <button>Create Battle</button>
        </div>
    )
}