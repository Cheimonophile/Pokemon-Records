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
                    setPlaythroughIdNo?.(readPlaythroughResponse[0].id_no)
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
                value: playthrough.id_no,
                label: <>{playthroughName} ({playthrough.adventure_started})</>,
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