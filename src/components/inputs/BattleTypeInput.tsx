import { useEffect, useState } from "react"
import { SwitchInput, SwitchOption } from "./generic/SwitchInput"
import { readBattleTypes } from "backend/data/battle_types"
import { message } from "@tauri-apps/api/dialog"





export function BattleTypeInput({
  battleType,
  setBattleType,
}: {
  battleType: string | null
  setBattleType: (value: string) => void
}) {
  const [battleTypeOptions, setBattleTypeOptions] = useState<SwitchOption[]>()
  useEffect(() => {
    (async () => {
      try {
        const battleTypes = await readBattleTypes({})
        const battleTypeOptions = battleTypes.map(battleType => {
          return {
            value: battleType.name,
            label: battleType.name,
          } satisfies SwitchOption
        })
        setBattleTypeOptions(battleTypeOptions)
        setBattleType(battleTypes[0].name)
      }
      catch (error) {
        console.error(error)
        await message(`${error}`, {
          title: 'Error Reading Battle Types',
          type: 'error',
        })
      }
    })()
  }, [])

  return (
    <div>
      <SwitchInput
        value={battleType ?? undefined}
        options={battleTypeOptions}
        onChange={battleType => setBattleType(battleType)}
      />
    </div>
  );
}