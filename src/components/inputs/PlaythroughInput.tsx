import { FC, useEffect, useMemo, useState } from "react";
import { Playthrough } from "../../types";
import { readPlaythroughs } from "../../backend/data/playthroughs";
import { message } from "@tauri-apps/api/dialog";
import { Switch, SwitchOption } from "./generic/Switch";



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
                const playthroughs = await readPlaythroughs({})
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