import { FC, Fragment, ReactNode, useCallback, useEffect, useState } from 'react'
import { createBattle, deleteBattle, readBattles } from '../backend/battles'
import { flexGrow } from '../styles'
import { ask, message } from '@tauri-apps/api/dialog';
import { Battle, Catch, Playthrough, TeamMember, Trainer } from '../types';
import { readPlaythroughs } from '../backend/playthroughs';
import { readBattleTypes } from '../backend/battle_types';
import { readTrainerClasses } from '../backend/trainer_classes';
import { createTrainer, readTrainers } from '../backend/trainers';
import { invoke } from '@tauri-apps/api';
import { readRegions } from '../backend/regions';
import { createLocation, readLocations } from '../backend/locations';
import { readTeamMembers } from '../backend/team_members';
import ReactECharts from 'echarts-for-react';
import { teamOverTime } from '../backend/data/teamOverTime';
import { EChartsOption, use } from 'echarts';
import { useAppContext } from '../App';
import { readTypes } from '../backend/types';
import { readCatches } from '../backend/catches';


export const Catches: FC<{}> = () => {


    // context
    const { addEffect } = useAppContext()

    // battle table state
    const [catches, setCatches] = useState<Catch[] | Error | undefined>()

    // fetch battles
    useEffect(() => {
        return addEffect(async () => {
            try {
                const catches = await readCatches()
                setCatches(catches)
            }
            catch (error) {
                console.error(error)
                setCatches(new Error(`${error}`))
            }
        })
    }, [])


    return (
        <div style={{
            height: '100%',
            width: '100%',
            display: 'flex',
            gap: '0.25rem',
            flexDirection: 'column'
        }}>
            <h3>Catches</h3>

            {/* Above Table */}
            <div style={{
                flex: 'none',
                display: 'flex',
                flexDirection: 'row',
                gap: '0.5rem',
            }}>
                {/* Catch Pokemon */}

                {/* Level Up */}
                <div style={{
                    width: '18rem',
                }}>
                    {((): ReactNode => {
                        if (catches instanceof Error) {
                            return <div style={{ color: 'red' }}>{catches.message}</div>
                        }
                        if (catches?.at(0) === undefined) {
                            return <>No Catches</>
                        }
                        return (<>
                            {/* <LevelUp battle={battle} /> */}
                        </>)
                    })()}
                </div>
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

                    {/* table */}
                    {catches instanceof Error ? (<>
                        <div style={{
                            color: 'red',
                        }}>
                            {catches.message}
                        </div>
                    </>) : (<>
                        <table>
                            <tbody>
                                {catches?.map(c => (
                                    <Fragment key={c.no}>
                                        <CatchTableRow catch={c} />
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



const CatchTableRow: FC<{
    catch: Catch
}> = (props) => {

    // context
    const { refresh } = useAppContext()

    // ui
    const [disabled, setDisabled] = useState<number>(0)

    // delete battle
    const onClickDeleteCatch = useCallback(async () => {
        setDisabled(prev => prev + 1)
        try {
            throw new Error('Not Implemented')
            // const sure = await ask(`Are you sure you want to delete catch ${props.catch.no}`, {
            //     title: 'Delete Catch?',
            //     type: 'info',
            // })
            // if (sure) {
            //     await deleteCatch({ no: props.catch.no })
            // }
        }
        catch (error) {
            console.error(error)
            await message(`${error}`, {
                title: 'Error Deleting Catch',
                type: 'error',
            })
        }
        await refresh()
        setDisabled(prev => prev - 1)
    }, [props.catch])

    return (<tr>
        <td>
            <button onClick={onClickDeleteCatch} disabled={disabled > 0}>X</button>
        </td>
        <td>
            {props.catch.no}.
        </td>
        {/* <td>
            {}
        </td> */}
        <td>
            {props.catch.location.name}
        </td>
        <td>
            {props.catch.location.region}
        </td>
    </tr>)
}

