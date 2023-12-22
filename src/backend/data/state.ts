



import { invoke } from "@tauri-apps/api"

type ReadParams = {
    databaseUrl: string
}

type ReadResult = void


export async function setDBConnection(params: ReadParams): Promise<void> {
    await invoke<ReadResult>('set_db_connection', params)
}