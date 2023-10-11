import { invoke } from "@tauri-apps/api";
import { message } from "@tauri-apps/api/dialog";
import { FC, useCallback, useState } from "react"
import { setDBConnection } from "../backend/state";


export const Open: FC = () => {


    // state
    const [disabled, setDisabled] = useState<number>(0);
    const [file, setFile] = useState<File | null>(null);


    const getFile = useCallback(async () => {
        setDisabled(prev => prev + 1)
        try {
            await setDBConnection({ databaseUrl: "sqlite://src-tauri/dev/test-db.sqlite" });
        }
        catch (error) {
            await message(`${error}`, {
                title: 'Error Leveling Up',
                type: 'error',
            })
            console.error(error);
        }
        setDisabled(prev => prev - 1)
    }, [])

    return (<>
        <div style={{
            height: '100%',
            width: '100%',
            display: 'flex',
            gap: '0.25rem',
            flexDirection: 'column'
        }}>
            <h3>Open</h3>
            <div>
                <button disabled={disabled > 0} onClick={getFile}>Get File</button>
            </div>
        </div>
    </>)
}