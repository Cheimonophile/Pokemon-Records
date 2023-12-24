import { FC, useEffect, useMemo, useState } from "react";
import { Playthrough } from "../../types";
import { readPlaythroughs } from "../../backend/data/playthroughs";
import { message } from "@tauri-apps/api/dialog";
import { Switch, SwitchOption } from "./generic/SwitchInput";



/**
 * Playthrough Dropdown
 */
export const PlaythroughInput: FC<{
    playthroughIdNo?: string,
    setPlaythroughIdNo?: (playthroughIdNo: string) => void
}> = ({ playthroughIdNo, setPlaythroughIdNo }) => {

    /**
     * Playthrough Options
     */
    const [playthroughOptions, setPlaythroughOptions] = useState<Playthrough[]>()
    useEffect(() => {
        (async () => {
            try {
                const readPlaythroughResponse = await readPlaythroughs({})
                const playthroughs = readPlaythroughResponse.map(playthrough => {
                    return {
                        idNo: playthrough.id_no,
                        version: playthrough.version,
                        adventureStarted: playthrough.adventure_started,
                        name: playthrough.name,
                    } satisfies Playthrough
                });
                setPlaythroughOptions(playthroughs)
                if (!playthroughIdNo) {
                    setPlaythroughIdNo?.(playthroughs[0].idNo)
                }
            }
            catch (error) {
                console.error(error)
                await message(`${error}`, {
                    title: 'Error Reading Playthroughs',
                    type: 'error',
                })
            }
        })()
    }, [playthroughIdNo, setPlaythroughIdNo])


    /**
     * Switch Options
     */
    const switchOptions = useMemo(() => {
        return playthroughOptions?.map(playthrough => {
            return {
                value: playthrough.idNo,
                label: <>{playthrough.version} ({playthrough.adventureStarted})</>,
            } satisfies SwitchOption
        })
    }, [playthroughOptions])


    return (
        <Switch
            value={playthroughIdNo}
            setValue={setPlaythroughIdNo}
            options={switchOptions}
        />
    )
}