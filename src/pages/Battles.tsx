import { FC, Fragment, ReactNode, useCallback, useEffect, useState } from 'react'
import { createBattle, deleteBattle, readBattles, updateBattle } from '../backend/data/battles'
import { ask, message } from '@tauri-apps/api/dialog';
import { Battle, TeamMember, Trainer } from '../types';
import { readBattleTypes } from '../backend/data/battle_types';
import { createTrainerClass, readTrainerClasses } from '../backend/data/trainer_classes';
import { createTrainer, readTrainers } from '../backend/data/trainers';
import { readRegions } from '../backend/data/regions';
import { createLocation, readLocations } from '../backend/data/locations';
import { readTeamMembers } from '../backend/data/team_members';
import ReactECharts from 'echarts-for-react';
import { teamOverTime } from 'backend/data/teamOverTime';
import { EChartsOption } from 'echarts';
import { useAppContext } from '../App';
import { readTypes } from '../backend/data/types';
import { PlaythroughInput } from '../components/inputs/PlaythroughInput';
import { createTeamMemberChange } from 'backend/data/team_member_changes';
import { LocationInput } from 'components/inputs/LocationInput';
import { BattleTypeInput } from 'components/inputs/BattleTypeInput';



/**
 * Battles Page
 */
export const Battles: FC<{}> = () => {


    // context
    const { addEffect } = useAppContext()

    // battle table state
    const [battles, setBattles] = useState<Battle[] | null | undefined>()

    // fetch battles
    useEffect(() => {
        return addEffect(async () => {
            try {
                const battleResponse = await readBattles({})
                const battles = battleResponse.map(battle => {
                    return {
                        type: battle.battle_type,
                        lost: battle.lost,
                        no: battle.no,
                        round: battle.round,
                        location: {
                            name: battle.event.location_name,
                            region: battle.event.location_region,
                        },
                        opponent1: {
                            class: battle.opponent1_class,
                            name: battle.opponent1_name,
                        },
                        opponent2: (battle.opponent2_class === null || battle.opponent2_name === null) ? undefined : {
                            class: battle.opponent2_class,
                            name: battle.opponent2_name,
                        },
                        partner: (battle.partner_class === null || battle.partner_name === null) ? undefined : {
                            class: battle.partner_class,
                            name: battle.partner_name,
                        },
                        playthroughIdNo: battle.event.playthrough_id_no,
                    } satisfies Battle
                })
                setBattles(battles)
            }
            catch (error) {
                console.error(error)
                setBattles(null)
            }
        })
    }, [addEffect])


    return (
        <div className="h-full w-full flex flex-col gap-1 p-1">

            {/* Above Table */}
            <div className="flex-none flex flex-row gap-2">

                {/* Make Battle */}
                <div>
                    <CreateBattle />
                </div>

                {/* Level Up */}
                <div className="min-w-fit">
                    {((): ReactNode => {
                        if (battles instanceof Error) {
                            return <div className="text-red-500">{battles.message}</div>
                        }
                        const battle = battles?.at(0)
                        if (battle === undefined) {
                            return <></>
                        }
                        return (<>
                            <LevelUp battle={battle} />
                        </>)
                    })()}
                </div>

                {/* Level up over time */}
                <div className="h-40 w-64">
                    <TeamMemberLevelChart mostRecentBattle={battles?.at(0)} />
                </div>
            </div>

            {/* Battles Table */}
            <div className="flex-1">
                <div className="w-full h-full overflow-y-auto p-1 border">

                    {/* table */}
                    {battles instanceof Error ? (<>
                        <div className="text-red-500">
                            {battles.message}
                        </div>
                    </>) : (<>
                        <table>
                            <tbody>
                                {battles?.map(battle => (
                                    <Fragment key={battle.no}>
                                        <BattleTableRow battle={battle} />
                                    </Fragment>
                                ))}
                            </tbody>
                        </table>
                    </>)}
                </div>
            </div>
        </div>
    )
}



const BattleTableRow: FC<{
    battle: Battle
}> = (props) => {

    // context
    const { refresh } = useAppContext()

    // ui
    const [disabled, setDisabled] = useState<number>(0)

    // make battle title
    let title = `${props.battle.opponent1.class} ${props.battle.opponent1.name}`
    if (props.battle.opponent2) {
        title += ` and ${props.battle.opponent2.class}` + (props.battle.opponent2.name ? ` ${props.battle.opponent2.name}` : '')
    }
    if (props.battle.partner) {
        title += ` with ${props.battle.partner.class}` + (props.battle.partner.name ? ` ${props.battle.partner.name}` : '')
    }

    // delete battle
    const onClickDeleteBattle = useCallback(async () => {
        setDisabled(prev => prev + 1)
        try {
            const sure = await ask(`Are you sure you want to delete battle ${props.battle.no} against ${title}`, {
                title: 'Delete Battle?',
                type: 'info',
            })
            if (!sure) {
                setDisabled(prev => prev - 1)
                return
            }
            await deleteBattle({ no: props.battle.no })
        }
        catch (error) {
            console.error(error)
            await message(`${error}`, {
                title: 'Error Deleting Battle',
                type: 'error',
            })
        }
        await refresh()
        setDisabled(prev => prev - 1)
    }, [props.battle.no, title, refresh])

    // const on toggle lost
    const onClickToggleLost = useCallback(async () => {
        setDisabled(prev => prev + 1)
        try {
            await updateBattle({ no: props.battle.no, lost: !props.battle.lost })
            props.battle.lost = !props.battle.lost
        }
        catch (error) {
            console.error(error)
            await message(`${error}`, {
                title: 'Error Deleting Battle',
                type: 'error',
            })
        }
        await refresh()
        setDisabled(prev => prev - 1)

    }, [refresh, props.battle])

    return (<tr>
        <td>
            <button onClick={onClickDeleteBattle} disabled={disabled > 0}>X</button>
        </td>
        <td>
            {props.battle.no}.
        </td>
        <td>
            {title}{props.battle.round > 0 && ` (Round ${props.battle.round})`}
        </td>
        <td>
            <label>Lost</label>
            <input
                type="checkbox"
                checked={props.battle.lost}
                onChange={onClickToggleLost}
                disabled={disabled > 0}
            />
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

    // context
    const { refresh } = useAppContext()

    // ui
    const [disabled, setDisabled] = useState<number>(0)


    // Form State
    const [playthroughIdNo, setPlaythroughIdNo] = useState<string | undefined>()
    const [location, setLocation] = useState<{ name: string, region: string }>({ name: "", region: "", })
    const [battleType, setBattleType] = useState<string>("")


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

    // lost
    const [lost, setLost] = useState<boolean>(false)

    // create battle button
    const createBattleOnClick = async () => {
        setDisabled(prev => prev + 1)
        try {
            // opponent 1
            await tryCreateTrainer(opponent1Validity, setOpponent1Validity, opponent1)
            // opponent 2
            if (useOpponent2) {
                await tryCreateTrainer(opponent2Validity, setOpponent2Validity, opponent2)
            }
            // partner
            if (usePartner) {
                await tryCreateTrainer(partnerValidity, setPartnerValidity, partner)
            }
            // errors
            if (playthroughIdNo === undefined)
                throw new Error("No Playthrough Selected")
            // create the battle
            await createBattle({
                playthroughIdNo: playthroughIdNo,
                locationName: location.name,
                locationRegion: location.region,
                battleType: battleType,
                opponent1Class: opponent1.class,
                opponent1Name: opponent1.name,
                opponent2Class: useOpponent2 ? opponent2.class : undefined,
                opponent2Name: useOpponent2 ? opponent2.name : undefined,
                partnerClass: usePartner ? partner.class : undefined,
                partnerName: usePartner ? partner.name : undefined,
                round: 0,
                lost: lost,
            })
            setBattleType("Single")
            setOpponent1(prev => ({ ...prev, name: "", }))
            setUseOpponent2(false)
            setOpponent2(prev => ({ ...prev, name: "", }))
            setUsePartner(false)
            setPartner(prev => ({ ...prev, name: "", }))
            setLost(false)
        }
        catch (error) {
            console.error(error)
            await message(`${error}`, {
                title: 'Error Creating Battle',
                type: 'error',
            })
        }
        await refresh()
        setDisabled(prev => prev - 1)
    }

    return (
        <div>
            {/* Playthrough selector */}
            <PlaythroughInput
                playthroughIdNo={playthroughIdNo}
                setPlaythroughIdNo={setPlaythroughIdNo}
            />

            {/* Location */}
            <LocationInput
                location={location}
                setLocation={setLocation}
            />

            {/* Battle Type */}
            <BattleTypeInput
                battleType={battleType}
                setBattleType={setBattleType}
            />

            {/* Opponent 1 */}
            <div>
                <label>Opponent 1:</label>
                <input
                    type="text"
                    className={`${opponent1Validity.class || 'text-red-500'}`}
                    value={opponent1.class}
                    onChange={e => setOpponent1(prev => ({ ...prev, class: e.target.value }))}
                />
                <input
                    type="text"
                    className={`${opponent1Validity.name || 'text-red-500'}`}
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
                        className={`${opponent2Validity.class || 'text-red-500'}`}
                        value={opponent2.class}
                        onChange={e => setOpponent2(prev => ({ ...prev, class: e.target.value }))}
                    />
                    <input
                        type="text"
                        className={`${opponent2Validity.name || 'text-red-500'}`}
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
                        className={`${partnerValidity.class || 'text-red-500'}`}
                        value={partner.class}
                        onChange={e => setPartner(prev => ({ ...prev, class: e.target.value }))}
                    />
                    <input
                        type="text"
                        className={`${partnerValidity.name || 'text-red-500'}`}
                        value={partner.name}
                        onChange={e => setPartner(prev => ({ ...prev, name: e.target.value }))}
                    />
                </>)}
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
                <button
                    onClick={createBattleOnClick}
                    disabled={disabled > 0}>
                    Create Battle
                </button>
            </div>
        </div>
    )
}



const tryCreateTrainer = async (
    validity: { name: boolean, class: boolean },
    setValidity: React.Dispatch<React.SetStateAction<{
        name: boolean;
        class: boolean;
    }>>,
    trainer: Trainer,
) => {
    if (!validity.class) {
        if (trainer.class.length < 1)
            throw new Error("Blank trainer class")
        const doCreateNewTrainerClass = await ask(`'${trainer.class}' does not exist. Create it?`, {
            title: 'Create Trainer Class?',
            type: 'info',
        })
        if (!doCreateNewTrainerClass)
            throw new Error("Trainer Class does not exist")
        await createTrainerClass({ name: trainer.class })
        setValidity(prev => ({ ...prev, class: true }))
    }
    if (!validity.name) {
        if (trainer.name.length < 1)
            throw new Error("Blank trainer name")
        const doCreateNewTrainer = await ask(`'${trainer.class} ${trainer.name}' does not exist. Create them?`, {
            title: 'Create Trainer?',
            type: 'info',
        })
        if (!doCreateNewTrainer)
            throw new Error("Trainer does not exist")
        await createTrainer({ name: trainer.name, class: trainer.class })
        setValidity(prev => ({ ...prev, name: true }))
    }
}


const LevelUp: FC<{
    battle: Battle
}> = (props) => {

    // context
    const { addEffect } = useAppContext()

    // battle table state
    const [teamMembers, setTeamMembers] = useState<TeamMember[] | Error | undefined>()

    // fetch battles
    useEffect(() => {
        return addEffect(async () => {
            try {
                const readTeamMembersResponse = await readTeamMembers({ playthroughIdNo: props.battle.playthroughIdNo })
                const teamMembers = readTeamMembersResponse.map(teamMember => {
                    return {
                        id: teamMember.id,
                        level: teamMember.level,
                        nickname: teamMember.nickname,
                        species: {
                            name: teamMember.species.name,
                            generation: teamMember.species.generation,
                            dexNo: teamMember.species.dex_no,
                            type1: teamMember.species.type1,
                            type2: teamMember.species.type2,
                        },
                        playthrough: {
                            idNo: teamMember.playthrough.id_no,
                            version: teamMember.playthrough.version,
                            adventureStarted: teamMember.playthrough.adventure_started,
                            name: teamMember.playthrough.name,
                        },
                        slot: teamMember.slot,
                        caughtDate: teamMember.caught_date,
                        caughtLocationName: teamMember.caught_location_name,
                        caughtLocationRegion: teamMember.caught_location_region,
                        caughtSpeciesName: teamMember.caught_species_name,
                        caughtLevel: teamMember.caught_level,
                        ball: teamMember.ball,
                        gender: teamMember.gender
                    } satisfies TeamMember
                })
                setTeamMembers(teamMembers)
            }
            catch (error) {
                console.error(error)
                setTeamMembers(new Error(`${error}`))
            }
        })
    }, [
        addEffect,
        props.battle.playthroughIdNo
    ])

    return <>
        <div>
            {teamMembers instanceof Error ? (<>
                <div className="text-red-500">{teamMembers.message}</div>
            </>) : (<>
                <table>
                    <tbody>
                        {teamMembers?.map(teamMember => (
                            <Fragment key={teamMember.id}>
                                <TeamMemberRow teamMember={teamMember} battle={props.battle} />
                            </Fragment>
                        ))}
                    </tbody>
                </table>
            </>)}
        </div>
    </>
}


const TeamMemberRow: FC<{
    teamMember: TeamMember,
    battle: Battle,
}> = (props) => {

    // context
    const { refresh } = useAppContext()

    // ui state
    const [disabled, setDisabled] = useState<number>(0)

    // on click level change
    const onClickLevelChange = async (change: number) => {
        setDisabled(prev => prev + 1)
        try {
            await createTeamMemberChange({
                eventNo: props.battle.no,
                teamMemberId: props.teamMember.id,
                level: props.teamMember.level + change
            })
        }
        catch (error) {
            console.error(error)
            await message(`${error}`, {
                title: 'Error Leveling Up',
                type: 'error',
            })
        }
        await refresh()
        setDisabled(prev => prev - 1)
    }

    return <>
        <tr>
            <td className="truncate">
                {props.teamMember.nickname ?? props.teamMember.species.name}
            </td>
            <td>
                {props.teamMember.level}
            </td>
            <td>
                <button disabled={disabled > 0} onClick={() => onClickLevelChange(1)}>+</button>
            </td>
            <td>
                <button disabled={disabled > 0} onClick={() => onClickLevelChange(-1)}>-</button>
            </td>
        </tr>
    </>
}


const TeamMemberLevelChart: FC<{
    mostRecentBattle?: Battle,
}> = (props) => {

    // context
    const { addEffect } = useAppContext()

    // state
    const [data, setData] = useState<EChartsOption | Error | undefined>()

    useEffect(() => {
        return addEffect(async () => {
            try {
                if (props.mostRecentBattle === undefined)
                    throw new Error("Most Recent Battle not defined")
                const results = await teamOverTime({
                    playthroughIdNo: props.mostRecentBattle.playthroughIdNo,
                })
                const types = await readTypes({})
                let maxLevel = 0
                results.forEach(row => {
                    row.forEach(cell => {
                        if (cell.level > maxLevel) {
                            maxLevel = cell.level
                        }
                    })
                })
                const data: Map<number, {
                    color: string | undefined,
                    data: [number, number][]
                }> = new Map()
                results.forEach((row, r) => {
                    for (const cell of row) {
                        if (data.get(cell.team_member.id) === undefined) {
                            data.set(cell.team_member.id, {
                                color: types.find(type => type.name === cell.species.type1)?.color ?? undefined,
                                data: []
                            })
                        }
                        const xPowBase = 1.05
                        const yPowBase = 1.1
                        data.get(cell.team_member.id)?.data.push([Math.pow(xPowBase, r), Math.pow(yPowBase, cell.level)])
                    }
                })
                const option: EChartsOption = {
                    // tooltip: {
                    //     trigger: 'axis'
                    // },
                    grid: {
                        left: 1,
                        top: 1,
                        right: 1,
                        bottom: 1
                    },
                    xAxis: {
                        show: false,
                        type: 'value',
                        min: (val) => val.min,
                        max: (val) => val.max
                    },
                    yAxis: {
                        show: false,
                        type: 'value',
                        min: (val) => val.min,
                        max: (val) => val.max
                    },
                    series: Array.from(data).map(([team_member_id, team_member_data]) => ({
                        symbol: 'none',
                        type: 'line',
                        name: team_member_id,
                        data: team_member_data.data,
                        lineStyle: {
                            color: team_member_data.color
                        }
                    })),
                    // options: {
                    //     responsive: false
                    // }
                };
                setData(option)
            }
            catch (error) {
                console.error(error)
                setData(new Error(`${error}`))
            }
        })
    }, [addEffect, props.mostRecentBattle])



    return <>
        {data === undefined
            ? <></>
            : data instanceof Error
                ? <div className="text-red-500">{data.message}</div>
                : <ReactECharts option={data} style={{
                    width: '100%',
                    height: '100%',
                }} />}
    </>
}