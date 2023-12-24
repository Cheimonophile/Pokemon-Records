import { FC, Fragment, useCallback, useEffect, useState } from 'react'
import { message } from '@tauri-apps/api/dialog';
import { Catch } from '../types';
import { useAppContext } from '../App';
import { createCatch, readCatches } from '../backend/data/catches';
import { readSpecies } from '../backend/data/species';
import { readBalls } from '../backend/data/balls';
import { PlaythroughInput } from '../components/inputs/PlaythroughInput';
import { startOfToday, formatISO } from 'date-fns'
import { LocationInput } from 'components/inputs/LocationInput';
import { CatchTypeInput } from 'components/inputs/CatchTypeInput';


export const Catches: FC<{}> = () => {


    // context
    const { addEffect } = useAppContext()

    // battle table state
    const [catches, setCatches] = useState<Catch[] | Error | undefined>()

    // fetch battles
    useEffect(() => {
        return addEffect(async () => {
            try {
                const catchesResult = await readCatches({})
                const catches = catchesResult.map(c => {
                    return {
                        type: c.catch_type,
                        no: c.no,
                        species: {
                            name: c.species.name,
                            dexNo: c.species.dex_no,
                            generation: c.species.generation,
                            type1: c.species.type1,
                            type2: c.species.type2,
                        },
                        location: {
                            name: c.event.location_name,
                            region: c.event.location_region,
                        },
                        playthroughIdNo: c.event.playthrough_id_no,
                    } satisfies Catch
                })
                setCatches(catches)
            }
            catch (error) {
                console.error(error)
                setCatches(new Error(`${error}`))
            }
        })
    }, [addEffect])


    return (
        <div className="h-full w-full flex flex-col gap-1 p-1">

            {/* Above Table */}
            <div className="flex flex-row gap-2">
                {/* Catch Pokemon */}
                <div>
                    <CatchPokemon />
                </div>
            </div>

            {/* Battles Table */}
            <div className="flex-1">
                <div className="h-full w-full overflow-y-auto p-1 border">

                    {/* table */}
                    {catches instanceof Error ? (<>
                        <div className='text-red-500'>
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

    // form state
    const [playthroughIdNo, setPlaythroughIdNo] = useState<string | undefined>()
    const [location, setLocation] = useState<{ name: string, region: string }>({ name: "", region: "", })
    const [catchType, setCatchType] = useState<string>("Grass")

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
    }, [addEffect, species])

    // nickname
    const [nickname, setNickname] = useState<string>("")

    // date
    const [date, setDate] = useState<string>(formatISO(startOfToday(), { representation: 'date' }))

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
            // errors
            if (playthroughIdNo === undefined)
                throw new Error("No Playthrough Selected")
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
            <PlaythroughInput
                playthroughIdNo={playthroughIdNo}
                setPlaythroughIdNo={setPlaythroughIdNo}
            />

            {/* Location */}
            <LocationInput
                location={location}
                setLocation={setLocation}
            />

            {/* Catch Type */}
            <CatchTypeInput
                catchType={catchType}
                setCatchType={setCatchType}
            />

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
                <select value={gender} onChange={e => setGender(e.target.value)}>
                    {['M', 'F', 'N'].map((gender, i) => (
                        <option key={i} value={gender}>{gender}</option>
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


// const tryCreateLocation = async (
//     locationValid: boolean,
//     setLocationValid: React.Dispatch<React.SetStateAction<boolean>>,
//     location: { name: string, region: string },
// ) => {
//     if (!locationValid) {
//         if (location.name.length < 1)
//             throw new Error("Blank location name")
//         const doCreateNewLocation = await ask(`'${location.name}, ${location.region}' does not exist. Create it?`, {
//             title: 'Create Location?',
//             type: 'info',
//         })
//         if (!doCreateNewLocation)
//             throw new Error("Location does not exist")
//         await createLocation(location)
//         setLocationValid(true)
//     }
// }
