import { FC, useEffect, useState } from "react";
import { Playthrough } from "../../types";
import { readPlaythroughs } from "../../backend/playthroughs";
import { message } from "@tauri-apps/api/dialog";



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


    return (
        <div className="rounded hover:bg-gray-100">
            <select
                className="appearance-none w-full cursor-pointer px-1 bg-transparent"
                value={playthroughIdNo} onChange={e => setPlaythroughIdNo?.(e.target.value)}>
                {playthroughOptions?.map((playthrough, i) => (
                    <option key={i} value={playthrough.idNo}>{playthrough.version} ({playthrough.adventureStarted.toISOString().slice(0, 10)})</option>
                ))}
            </select>
        </div>
    )
}