import { FC, Fragment, ReactNode, useCallback, useEffect, useState } from 'react'
import { ask, message } from '@tauri-apps/api/dialog';
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
import { TrainerInput } from 'components/inputs/TrainerInput';
import { createEvent, deleteEvent, readEvents, updateEvent } from 'backend/data/event';
import { Event, TeamMember } from 'backend/models';
import { readPlaythroughs } from 'backend/data/playthroughs';
import { todayStr } from 'utils';



/**
 * Battles Page
 */
export const Battles: FC<{}> = () => {


    // context
    const { addEffect } = useAppContext()

    // battle table state
    const [events, setEvents] = useState<Event[] | null | undefined>()

    // fetch battles
    useEffect(() => {
        return addEffect(async () => {
            try {
                const playthroughs = await readPlaythroughs({})
                const events = await readEvents({
                    playthroughIdNo: playthroughs[0].id_no,
                })
                setEvents(events)
            }
            catch (error) {
                console.error(error)
                setEvents(null)
            }
        })
    }, [addEffect])


    return (
        <div className="h-full w-full flex flex-col gap-1 p-1 overflow-hidden">

            {/* Above Table */}
            <div className="flex-none flex flex-row gap-2">

                {/* Make Battle */}
                <div>
                    <CreateBattle />
                </div>

                {/* Level Up */}
                <div className="min-w-fit">
                    {((): ReactNode => {
                        if (events === null) {
                            return <div className="text-red-500">Error</div>
                        }
                        const event = events?.at(0)
                        if (event === undefined) {
                            return <></>
                        }
                        return (<>
                            <LevelUp event={event} />
                        </>)
                    })()}
                </div>

                {/* Level up over time */}
                <div className="h-40 w-64">
                    <TeamMemberLevelChart mostRecentBattle={events?.at(0)} />
                </div>
            </div>

            {/* Battles Table */}
            <div className="flex-1 overflow-hidden">
                <div className="w-full h-full overflow-y-auto p-1 border">

                    {/* table */}
                    {!events ? (<>
                        <div className="text-red-500">
                            Error
                        </div>
                    </>) : (<>
                        <table>
                            <tbody>
                                {events?.map(event => {
                                    if (!event.battle) {
                                        return null
                                    }
                                    return (
                                        <Fragment key={event.no}>
                                            <BattleTableRow event={event} />
                                        </Fragment>
                                    )
                                })}
                            </tbody>
                        </table>
                    </>)}
                </div>
            </div>
        </div>
    )
}



const BattleTableRow: FC<{
    event: Event
}> = ({ event }) => {

    // context
    const { refresh } = useAppContext()

    // ui
    const [disabled, setDisabled] = useState<number>(0)

    // make battle title
    let title = `${event.battle?.opponent1.class} ${event.battle?.opponent1.name}`
    if (event.battle?.opponent2) {
        title += ` and ${event.battle.opponent2.class}` + (event.battle.opponent2.name ? ` ${event.battle.opponent2.name}` : '')
    }
    if (event.battle?.partner) {
        title += ` with ${event.battle.partner.class}` + (event.battle.partner.name ? ` ${event.battle.partner.name}` : '')
    }

    // delete battle
    const onClickDeleteBattle = useCallback(async () => {
        setDisabled(prev => prev + 1)
        try {
            const sure = await ask(`Are you sure you want to delete battle ${event.no} against ${title}`, {
                title: 'Delete Battle?',
                type: 'info',
            })
            if (!sure) {
                setDisabled(prev => prev - 1)
                return
            }
            await deleteEvent({ eventNo: event.no })
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
    }, [event.no, title, refresh])

    // const on toggle lost
    const onClickToggleLost = useCallback(async () => {
        setDisabled(prev => prev + 1)
        try {
            if (event.battle) {
                await updateEvent({
                    no: event.no,
                    event: {
                        playthroughIdNo: event.playthrough.id_no,
                        locationId: event.location.id,
                        date: event.date,
                        battle: {
                            battleTypeId: event.battle.battle_type.id,
                            opponent1Id: event.battle.opponent1.id,
                            opponent2Id: event.battle.opponent2?.id ?? null,
                            partnerId: event.battle.partner?.id ?? null,
                            lost: !event.battle.lost,
                            round: event.battle.round,
                        },
                        catch: null,
                    }
                })
                event.battle.lost = !event.battle?.lost
            }
            else {
                throw new Error("The event is not a battle")
            }
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

    }, [refresh, event])

    return (<tr>
        <td>
            <button onClick={onClickDeleteBattle} disabled={disabled > 0}>X</button>
        </td>
        <td>
            {event.no}.
        </td>
        <td>
            {title}{event.battle && event.battle.round > 0 && ` (Round ${event.battle.round})`}
        </td>
        <td>
            <label>Lost</label>
            <input
                type="checkbox"
                checked={event.battle?.lost}
                onChange={onClickToggleLost}
                disabled={disabled > 0}
            />
        </td>
        <td>
            {event.location.name}
        </td>
        <td>
            {event.location.region.name}
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
    const [locationId, setLocationId] = useState<number | null>(null)
    const [battleTypeId, setBattleTypeId] = useState<number | null>(null)
    const [opponent1Id, setOpponent1Id] = useState<number | null>(null)
    const [opponent2Id, setOpponent2Id] = useState<number | null>(null)
    const [useOpponent2, setUseOpponent2] = useState<boolean>(false)
    const [partnerId, setPartnerId] = useState<number | null>(null)
    const [usePartner, setUsePartner] = useState<boolean>(false)

    // lost
    const [lost, setLost] = useState<boolean>(false)

    // create battle button
    const createBattleOnClick = async () => {
        setDisabled(prev => prev + 1)
        try {
            // errors
            if (playthroughIdNo === undefined)
                throw new Error("No Playthrough Selected")
            if (locationId === null)
                throw new Error("No Location Selected")
            if (battleTypeId === null)
                throw new Error("No Battle Type Selected")
            if (opponent1Id === null)
                throw new Error("No Opponent 1 Selected")
            if (useOpponent2 && opponent2Id === null)
                throw new Error("No Opponent 2 Selected")
            if (usePartner && partnerId === null)
                throw new Error("No Partner Selected")
            // create the battle
            await createEvent({
                event: {
                    playthroughIdNo,
                    locationId,
                    date: todayStr(),
                    battle: {
                        battleTypeId,
                        opponent1Id,
                        opponent2Id,
                        partnerId,
                        round: 0,
                        lost,
                    },
                    catch: null,
                }
            })
            setBattleTypeId(null)
            setOpponent1Id(null)
            setUseOpponent2(false)
            setOpponent2Id(null)
            setUsePartner(false)
            setPartnerId(null)
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
                locationId={locationId}
                setLocationId={setLocationId}
            />

            {/* Battle Type */}
            <BattleTypeInput
                battleTypeId={battleTypeId}
                setBattleTypeId={setBattleTypeId}
            />

            {/* Opponent 1 */}
            <div className="flex flex-row">
                <div>
                    Opponent 1:
                </div>
                <TrainerInput
                    trainerId={opponent1Id}
                    setTrainerId={setOpponent1Id}
                />
            </div>


            {/* Opponent 2 */}
            <div className="flex flex-row gap-1 items-center">
                <div>Opponent 2:</div>
                <input
                    type="checkbox"
                    checked={useOpponent2}
                    onChange={e => setUseOpponent2(prev => !prev)}
                />
                {useOpponent2 && (
                    <TrainerInput
                        trainerId={opponent2Id}
                        setTrainerId={setOpponent2Id}
                    />
                )}
            </div>

            {/* Partner */}
            <div className="flex flex-row gap-1 items-center">
                <div>Partner:</div>
                <input
                    type="checkbox"
                    checked={usePartner}
                    onChange={e => setUsePartner(prev => !prev)}
                />
                {usePartner && (<>
                    <TrainerInput
                        trainerId={partnerId}
                        setTrainerId={setPartnerId}
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



// const tryCreateTrainer = async (
//     validity: { name: boolean, class: boolean },
//     setValidity: React.Dispatch<React.SetStateAction<{
//         name: boolean;
//         class: boolean;
//     }>>,
//     trainer: Trainer,
// ) => {
//     if (!validity.class) {
//         if (trainer.class.length < 1)
//             throw new Error("Blank trainer class")
//         const doCreateNewTrainerClass = await ask(`'${trainer.class}' does not exist. Create it?`, {
//             title: 'Create Trainer Class?',
//             type: 'info',
//         })
//         if (!doCreateNewTrainerClass)
//             throw new Error("Trainer Class does not exist")
//         await createTrainerClass({ name: trainer.class })
//         setValidity(prev => ({ ...prev, class: true }))
//     }
//     if (!validity.name) {
//         if (trainer.name.length < 1)
//             throw new Error("Blank trainer name")
//         const doCreateNewTrainer = await ask(`'${trainer.class} ${trainer.name}' does not exist. Create them?`, {
//             title: 'Create Trainer?',
//             type: 'info',
//         })
//         if (!doCreateNewTrainer)
//             throw new Error("Trainer does not exist")
//         await createTrainer({ name: trainer.name, class: trainer.class })
//         setValidity(prev => ({ ...prev, name: true }))
//     }
// }


const LevelUp: FC<{
    event: Event
}> = ({ event }) => {

    // context
    const { addEffect } = useAppContext()

    // battle table state
    const [teamMembers, setTeamMembers] = useState<TeamMember[] | Error | undefined>()

    // fetch battles
    useEffect(() => {
        return addEffect(async () => {
            try {
                const teamMembers = await readTeamMembers({ playthroughIdNo: event.playthrough.id_no })
                setTeamMembers(teamMembers)
            }
            catch (error) {
                console.error(error)
                setTeamMembers(new Error(`${error}`))
            }
        })
    }, [
        addEffect,
        event
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
                                <TeamMemberRow teamMember={teamMember} event={event} />
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
    event: Event,
}> = ({ teamMember, event }) => {

    // context
    const { refresh } = useAppContext()

    // ui state
    const [disabled, setDisabled] = useState<number>(0)

    // on click level change
    const onClickLevelChange = async (change: number) => {
        setDisabled(prev => prev + 1)
        try {
            await createTeamMemberChange({
                teamMemberChange: {
                    teamMemberId: teamMember.id,
                    eventNo: event.no,
                    level: teamMember.level + change,
                    speciesId: null,
                }
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
                {teamMember.nickname ?? teamMember.species.name}
            </td>
            <td>
                {teamMember.level}
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
    mostRecentBattle?: Event,
}> = ({ mostRecentBattle }) => {

    // context
    const { addEffect } = useAppContext()

    // state
    const [data, setData] = useState<EChartsOption | Error | undefined>()

    useEffect(() => {
        return addEffect(async () => {
            try {
                if (mostRecentBattle === undefined)
                    throw new Error("Most Recent Battle not defined")
                const results = await teamOverTime({
                    playthroughIdNo: mostRecentBattle.playthrough.id_no,
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
    }, [addEffect, mostRecentBattle])



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