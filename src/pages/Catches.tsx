import { FC, Fragment, ReactNode, useCallback, useEffect, useState } from 'react'
import { readBattles } from '../backend/battles'
import { flexGrow } from '../styles'
import { ask, message } from '@tauri-apps/api/dialog';
import { Catch, Playthrough } from '../types';
import { readPlaythroughs } from '../backend/playthroughs';
import { readRegions } from '../backend/regions';
import { createLocation, readLocations } from '../backend/locations';
import { useAppContext } from '../App';
import { createCatch, readCatches } from '../backend/catches';
import { readCatchTypes } from '../backend/catch_types';
import { readSpecies } from '../backend/species';
import { readBalls } from '../backend/balls';


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
    }, [addEffect])


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
                <div>
                    <CatchPokemon />
                </div>

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
    }, [refresh])

    return (<tr>
        <td>
            <button onClick={onClickDeleteCatch} disabled={disabled > 0}>X</button>
        </td>
        <td>
            {props.catch.no}.
        </td>
        <td>
            {props.catch.species.name}
        </td>
        <td>
            {props.catch.location.name}
        </td>
        <td>
            {props.catch.location.region}
        </td>
    </tr>)
}



const CatchPokemon: FC<{}> = () => {

    // context
    const { refresh, addEffect } = useAppContext()

    // ui
    const [disabled, setDisabled] = useState<number>(0)

    // Playthroughs
    const [playthroughIdNo, setPlaythroughIdNo] = useState<string>("")
    const [playthroughOptions, setPlaythroughOptions] = useState<Playthrough[]>()
    useEffect(() => {
        return addEffect(async () => {
            try {
                const playthroughs = await readPlaythroughs({})
                setPlaythroughOptions(playthroughs)
                setPlaythroughIdNo(playthroughs[0].idNo)
            }
            catch (error) {
                console.error(error)
                await message(`${error}`, {
                    title: 'Error Reading Playthroughs',
                    type: 'error',
                })
            }
        })
    }, [addEffect])

    // location
    const [location, setLocation] = useState<{ name: string, region: string }>({ name: "", region: "", })
    const [regionOptions, setRegionOptions] = useState<string[]>()
    useEffect(() => {
        return addEffect(async () => {
            try {
                const [regions, mostRecentBattle] = await Promise.all([
                    readRegions({}),
                    readBattles({ howMany: 1 }),
                ])
                setRegionOptions(regions.reverse())
                setLocation({
                    region: mostRecentBattle[0].location.region,
                    name: mostRecentBattle[0].location.name
                })
            }
            catch (error) {
                console.error(error)
                await message(`${error}`, {
                    title: 'Error Reading Regions',
                    type: 'error',
                })
            }
        })
    }, [addEffect])
    const [locationValid, setLocationValid] = useState<boolean>(false)
    useEffect(() => {
        return addEffect(async () => {
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
        })
    }, [addEffect, location])

    // catch type
    const [catchType, setCatchType] = useState<string>("Grass")
    const [catchTypeOptions, setCatchTypeOptions] = useState<string[]>()
    useEffect(() => {
        return addEffect(async () => {
            try {
                const catchTypes = (await readCatchTypes({})).map(ct => ct.name)
                setCatchTypeOptions(catchTypes)
            }
            catch (error) {
                console.error(error)
                await message(`${error}`, {
                    title: 'Error Reading Catch Types',
                    type: 'error',
                })
            }
        })
    }, [addEffect])

    // slot
    const [slot, setSlot] = useState<number>(1)

    // species
    const [species, setSpecies] = useState<string>("")
    const [speciesValid, setSpeciesValid] = useState<boolean>(false)
    useEffect(() => {
        return addEffect(async () => {
            try {
                const speciesList = await readSpecies({ name: species })
                setSpeciesValid(speciesList.length > 0)
            }
            catch (error) {
                console.error(error)
                await message(`${error}`, {
                    title: 'Error Reading Species',
                    type: 'error',
                })
            }
        })
    }, [addEffect,species])

    // nickname
    const [nickname, setNickname] = useState<string>("")

    // date
    const [date, setDate] = useState<string>(new Date().toISOString().slice(0, 10))

    // level
    const [level, setLevel] = useState<number>(1)

    // ball
    const [ball, setBall] = useState<string>("Poké Ball")
    const [ballOptions, setBallOptions] = useState<string[]>()
    useEffect(() => {
        return addEffect(async () => {
            try {
                const balls = (await readBalls({})).map(t => t.name)
                setBallOptions(balls)
            }
            catch (error) {
                console.error(error)
                await message(`${error}`, {
                    title: 'Error Reading Balls',
                    type: 'error',
                })
            }
        })
    }, [addEffect])

    // gender
    const [gender, setGender] = useState<string>("M")


    // create battle button
    const createCatchOnClick = async () => {
        setDisabled(prev => prev + 1)
        try {
            // location
            await tryCreateLocation(locationValid, setLocationValid, location)
            // create the battle
            await createCatch({
                playthroughIdNo: playthroughIdNo,
                locationName: location.name,
                locationRegion: location.region,
                catchType,
                slot,
                speciesName: species,
                nickname: nickname || null,
                date,
                level,
                ball,
                gender,
            })
            // reset params
            setSpecies("")
        }
        catch (error) {
            console.error(error)
            await message(`${error}`, {
                title: 'Error Catching Pokemon',
                type: 'error',
            })
        }
        await refresh()
        setDisabled(prev => prev - 1)
    }


    return (<>
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

            {/* Catch Type */}
            <div>
                <label>Catch Type:</label>
                <select value={catchType} onChange={e => setCatchType(e.target.value)}>
                    {catchTypeOptions?.map((ct, i) => (
                        <option key={i} value={ct}>{ct}</option>
                    ))}
                </select>
            </div>

            {/* Slot */}
            <div>
                <label>Slot:</label>
                <input
                    type="number"
                    value={slot}
                    onChange={e => setSlot(parseInt(e.target.value))}
                    min={1}
                    max={6}
                />
            </div>


            {/* Species */}
            <div>
                <label>Species:</label>
                <input
                    type="text"
                    style={{
                        color: speciesValid ? undefined : 'red',
                    }}
                    value={species}
                    onChange={e => setSpecies(e.target.value)}
                />
            </div>

            {/* Nickname */}
            <div>
                <label>Nickname:</label>
                <input
                    type="text"
                    value={nickname}
                    onChange={e => setNickname(e.target.value)}
                />
            </div>

            {/* Date */}
            <div>
                <label>Date:</label>
                <input
                    type="date"
                    style={{
                        color: isNaN(new Date(date).valueOf()) ? 'red' : undefined,
                    }}
                    value={date}
                    onChange={e => setDate(e.target.value)}
                />
            </div>

            {/* Level */}
            <div>
                <label>Level:</label>
                <input
                    type="number"
                    style={{
                        color: level < 1 || level > 100 ? 'red' : undefined,
                    }}
                    value={level}
                    onChange={e => setLevel(parseInt(e.target.value))}
                    min={1}
                    max={100}
                />
            </div>

            {/* Ball */}
            <div>
                <label>Ball:</label>
                <select value={ball} onChange={e => setBall(e.target.value)}>
                    {ballOptions?.map((ball, i) => (
                        <option key={i} value={ball}>{ball}</option>
                    ))}
                </select>
            </div>

            {/* Gender */}
            <div>
                <label>Gender:</label>
                <select value={ball} onChange={e => setGender(e.target.value)}>
                    {['M', 'F', 'N'].map((ball, i) => (
                        <option key={i} value={ball}>{ball}</option>
                    ))}
                </select>
            </div>

            {/* Add Button */}
            <div>
                <button
                    onClick={createCatchOnClick}
                    disabled={disabled > 0}>
                    Create Catch
                </button>
            </div>

        </div>
    </>)
}


const tryCreateLocation = async (
    locationValid: boolean,
    setLocationValid: React.Dispatch<React.SetStateAction<boolean>>,
    location: { name: string, region: string },
) => {
    if (!locationValid) {
        if (location.name.length < 1)
            throw new Error("Blank location name")
        const doCreateNewLocation = await ask(`'${location.name}, ${location.region}' does not exist. Create it?`, {
            title: 'Create Location?',
            type: 'info',
        })
        if (!doCreateNewLocation)
            throw new Error("Location does not exist")
        await createLocation(location)
        setLocationValid(true)
    }
}
