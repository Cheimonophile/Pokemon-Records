import { useCallback, useEffect, useState } from "react"
import { SwitchInput, SwitchOption } from "./generic/SwitchInput"
import { readBattleTypes } from "backend/data/battle_types"
import { message } from "@tauri-apps/api/dialog"





export function BattleTypeInput({
  battleTypeId,
  setBattleTypeId,
}: {
  battleTypeId: number | null
  setBattleTypeId: (value: number | null) => void
}) {
  const [battleTypeOptions, setBattleTypeOptions] = useState<SwitchOption[]>()
  useEffect(() => {
    (async () => {
      try {
        const battleTypes = await readBattleTypes({})
        const battleTypeOptions = battleTypes.map(battleType => {
          return {
            value: battleType.id.toString(),
            label: battleType.name,
          } satisfies SwitchOption
        })
        setBattleTypeOptions(battleTypeOptions)
        setBattleTypeId(battleTypes[0].id)
      }
      catch (error) {
        console.error(error)
        await message(`${error}`, {
          title: 'Error Reading Battle Types',
          type: 'error',
        })
      }
    })()
  }, [setBattleTypeId])

  return (
    <div>
      <SwitchInput
        value={battleTypeId?.toString() ?? undefined}
        options={battleTypeOptions}
        onChange={battleTypeId => setBattleTypeId(parseInt(battleTypeId))}
      />
    </div>
  );
}