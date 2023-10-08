import { FC, Fragment, useEffect, useState } from 'react'
import { readBattles } from '../backend/battles'
import { flexGrow } from '../styles'
import { message } from '@tauri-apps/api/dialog';
import { Battle, Playthrough, Trainer } from '../types';
import { readPlaythroughs } from '../backend/playthroughs';
import { readBattleTypes } from '../backend/battle_types';
import { readTrainerClasses } from '../backend/trainer_classes';
import { readTrainers } from '../backend/trainers';
import { invoke } from '@tauri-apps/api';
import { readRegions } from '../backend/regions';
import { readLocations } from '../backend/locations';






export const Battles: FC<{}> = () => {

    // battle table state
    const [battles, setBattles] = useState<Battle[] | null>()

    // fetch battles
    useEffect(() => {
        const getBattles = setInterval(async () => {
            console.log('getting battles')
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
        }, 1000)
        return () => {
            clearInterval(getBattles)
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

    // location
    const [location, setLocation] = useState<{ name: string, region: string }>({ name: "", region: "", })
    const [regionOptions, setRegionOptions] = useState<string[]>()
    useEffect(() => {
        (async () => {
            try {
                const regions = (await readRegions({})).reverse()
                setRegionOptions(regions)
            }
            catch (error) {
                console.error(error)
                await message(`${error}`, {
                    title: 'Error Reading Regions',
                    type: 'error',
                })
            }
        })()
    }, [])
    const [locationValid, setLocationValid] = useState<boolean>(false)
    useEffect(() => {
        (async () => {
            try {
                const locations = await readLocations({ name: location.name, region: location.region })
                setLocationValid(locations.length > 0)
            }
            catch (error) {
                console.error(error)
                await message(`${error}`, {
                    title: 'Error Reading Locations',
                    type: 'error',
                })
            }
        })()
    }, [location])

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
    const [opponent1, setOpponent1] = useState<Trainer>({ name: "", class: "", })
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

    // opponent 2
    const [opponent2, setOpponent2] = useState<Trainer>({ name: "", class: "", })
    const [useOpponent2, setUseOpponent2] = useState<boolean>(false)
    const [opponent2Validity, setOpponent2Validity] = useState<{ name: boolean, class: boolean }>({ name: false, class: false })
    useEffect(() => {
        // trainer classes
        (async () => {
            try {
                const trainerClasses = await readTrainerClasses({ name: opponent2.class })
                setOpponent2Validity(prev => ({ ...prev, class: trainerClasses.length > 0 }))
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
                const trainers = await readTrainers({ name: opponent2.name, class: opponent2.class })
                setOpponent2Validity(prev => ({ ...prev, name: trainers.length > 0 }))
            }
            catch (error) {
                console.error(error)
                await message(`${error}`, {
                    title: 'Error Reading Trainers',
                    type: 'error',
                })
            }
        })();
    }, [opponent2])

    // partner
    const [partner, setPartner] = useState<Trainer>({ name: "", class: "", })
    const [usePartner, setUsePartner] = useState<boolean>(false)
    const [partnerValidity, setPartnerValidity] = useState<{ name: boolean, class: boolean }>({ name: false, class: false })
    useEffect(() => {
        // trainer classes
        (async () => {
            try {
                const trainerClasses = await readTrainerClasses({ name: partner.class })
                setPartnerValidity(prev => ({ ...prev, class: trainerClasses.length > 0 }))
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
                const trainers = await readTrainers({ name: partner.name, class: partner.class })
                setPartnerValidity(prev => ({ ...prev, name: trainers.length > 0 }))
            }
            catch (error) {
                console.error(error)
                await message(`${error}`, {
                    title: 'Error Reading Trainers',
                    type: 'error',
                })
            }
        })();
    }, [partner])

    // round
    const [round, setRound] = useState<number>(0)

    // lost
    const [lost, setLost] = useState<boolean>(false)

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

            {/* Location */}
            <div>
                <label>Location:</label>
                <select value={location.region} onChange={e => setLocation(prev => ({ ...prev, region: e.target.value }))}>
                    {regionOptions?.map((region, i) => (
                        <option key={i} value={region}>{region}</option>
                    ))}
                </select>
                <input
                    type="text"
                    style={{
                        color: locationValid ? undefined : 'red',
                    }}
                    value={location.name}
                    onChange={e => setLocation(prev => ({ ...prev, name: e.target.value }))}
                />
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


            {/* Opponent 2 */}
            <div>
                <label>Opponent 2:</label>
                <input
                    type="checkbox"
                    checked={useOpponent2}
                    onChange={e => setUseOpponent2(prev => !prev)}
                />
                {useOpponent2 && (<>
                    <input
                        type="text"
                        style={{
                            color: opponent2Validity.class ? undefined : 'red',
                        }}
                        value={opponent2.class}
                        onChange={e => setOpponent2(prev => ({ ...prev, class: e.target.value }))}
                    />
                    <input
                        type="text"
                        style={{
                            color: opponent2Validity.name ? undefined : 'red',
                        }}
                        value={opponent2.name}
                        onChange={e => setOpponent2(prev => ({ ...prev, name: e.target.value }))}
                    />
                </>)}
            </div>

            {/* Partner */}
            <div>
                <label>Partner:</label>
                <input
                    type="checkbox"
                    checked={usePartner}
                    onChange={e => setUsePartner(prev => !prev)}
                />
                {usePartner && (<>
                    <input
                        type="text"
                        style={{
                            color: partnerValidity.class ? undefined : 'red',
                        }}
                        value={partner.class}
                        onChange={e => setPartner(prev => ({ ...prev, class: e.target.value }))}
                    />
                    <input
                        type="text"
                        style={{
                            color: partnerValidity.name ? undefined : 'red',
                        }}
                        value={partner.name}
                        onChange={e => setPartner(prev => ({ ...prev, name: e.target.value }))}
                    />
                </>)}
            </div>

            {/* Round */}
            <div>
                <label>Round:</label>
                <input
                    type="number"
                    style={{
                        color: isNaN(round) ? 'red' : undefined,
                    }}
                    value={round}
                    onChange={e => setRound(parseInt(e.target.value))}
                />
            </div>

            {/* Lost */}
            <div>
                <label>Lost:</label>
                <input
                    type="checkbox"
                    checked={lost}
                    onChange={e => setLost(e.target.checked)}
                />
            </div>

            {/* Add Button */}
            <div>
                <button>Create Battle</button>
            </div>
        </div>
    )
}