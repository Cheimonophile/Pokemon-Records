import { Dispatch, ReactNode, SetStateAction, useEffect, useState } from "react";
import { SwitchInput, SwitchOption } from "./generic/SwitchInput";
import { readCatchTypes } from "backend/data/catch_types";
import { message } from "@tauri-apps/api/dialog";




export function CatchTypeInput({
  catchType,
  setCatchType,
 }: {
  catchType: string,
  setCatchType: Dispatch<SetStateAction<string>>,
}): ReactNode {

  /**
   * Catch Type Options
   */
  const [catchTypeOptions, setCatchTypeOptions] = useState<SwitchOption[]>()
  useEffect(() => {
    (async () => {
      try {
        const catchTypes = (await readCatchTypes({})).map(ct => {
          return {
            label: ct.name,
            value: ct.name,
          }
        })
        setCatchTypeOptions(catchTypes)
      }
      catch (error) {
        console.error(error)
        await message(`${error}`, {
          title: 'Error Reading Catch Types',
          type: 'error',
        })
      }
    })();
  }, [])



  return (
    <div>
      <SwitchInput 
        value={catchType ?? undefined}
        options={catchTypeOptions}
        onChange={setCatchType}
      />
    </div>
  )
}