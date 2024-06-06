import { message, open } from "@tauri-apps/api/dialog";
import { FC, useCallback, useState } from "react"
import { setDBConnection } from "../backend/data/state";
import { DATABASE_URL } from "../constants";
import { Page } from "interfaces/components/page";


export const OpenPage: Page = () => {


    // state
    const [disabled, setDisabled] = useState<number>(0);


    const getFile = useCallback(async () => {
        setDisabled(prev => prev + 1)
        try {
            const db_file = await open();
            if (db_file === null)
                throw new Error('No file selected');
            if (Array.isArray(db_file))
                throw new Error("Multiple files selected");
            const db_url = db_file;
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
        <div className="h-full w-full flex flex-col gap-1">
            <div>
                <button disabled={disabled > 0} onClick={getFile}>Get File</button>
            </div>
        </div>
    </>)
}