import { FC, useEffect, useMemo, useState } from "react";
import { readPlaythroughs } from "../../backend/data/playthroughs";
import { message } from "@tauri-apps/api/dialog";
import { SwitchInput, SwitchOption } from "./generic/SwitchInput";
import { Playthrough } from "backend/models";



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
                setPlaythroughOptions(readPlaythroughResponse)
                if (!playthroughIdNo) {
                    setPlaythroughIdNo?.(readPlaythroughResponse[0].idNo)
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
            const playthroughName = playthrough.version.name
            return {
                value: playthrough.idNo,
                label: <>{playthroughName} ({playthrough.adventureStarted})</>,
            } satisfies SwitchOption
        })
    }, [playthroughOptions])


    return (
        <SwitchInput
            value={playthroughIdNo}
            onChange={setPlaythroughIdNo}
            options={switchOptions}
        />
    )
}