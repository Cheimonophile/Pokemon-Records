import { message, open } from "@tauri-apps/api/dialog";
import { FC, useCallback, useState } from "react"
import { setDBConnection } from "../backend/state";
import { DATABASE_URL } from "../constants";


export const Open: FC = () => {


    // state
    const [disabled, setDisabled] = useState<number>(0);


    const getFile = useCallback(async () => {
        setDisabled(prev => prev + 1)
        try {
            const db_file = await open();
            const db_url = `sqlite://${db_file}`;
            await setDBConnection({ databaseUrl: db_url });
            localStorage.setItem(DATABASE_URL, db_url);
        }
        catch (error) {
            await message(`${error}`, {
                title: 'Error Opening Database',
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