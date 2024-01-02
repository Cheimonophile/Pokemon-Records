import { message } from "@tauri-apps/api/dialog";
import { readTrainerClasses } from "backend/data/trainer_classes";
import { readTrainers } from "backend/data/trainers";
import { Dispatch, ReactNode, SetStateAction, useEffect, useState } from "react";
import { TextInput } from "./generic/TextInput";
import { TextDropdownInput, TextDropdownOption } from "./generic/TextDropdownInput";
import { set } from "date-fns";


type Trainer = {
  name: string
  class: string
}



export function TrainerInput({
  trainerId,
  setTrainerId,
}: {
  trainerId: number | null
  setTrainerId: (value: number | null) => void
}): ReactNode {

  // trainer class state
  const [trainerClassId, setTrainerClassId] = useState<number | null>(null)


  // is the opponent valid
  const [trainerClassOptions, setTrainerClassOptions] = useState<TextDropdownOption[]>()
  useEffect(() => {
    (async () => {
      try {
        const trainerClasses = await readTrainerClasses({})
        const trainerClassOptions = trainerClasses.map(trainerClass => {
          return {
            value: trainerClass.id.toString(),
            label: trainerClass.name,
          } satisfies TextDropdownOption
        })
        setTrainerClassOptions(trainerClassOptions)
      }
      catch (error) {
        console.error(error)
        await message(`${error}`, {
          title: 'Error Reading Trainer Classes',
          type: 'error',
        })
      }
    })()
  })


  // trainer options
  const [trainerOptions, setTrainerOptions] = useState<TextDropdownOption[]>()
  useEffect(() => {
    (async () => {
      try {
        const trainers = await readTrainers({
          classId: trainerClassId,
        })
        const trainerOptions = trainers.map(trainer => {
          return {
            value: trainer.id.toString(),
            label: trainer.name,
          } satisfies TextDropdownOption
        })
        setTrainerOptions(trainerOptions)
      }
      catch (error) {
        console.error(error)
        await message(`${error}`, {
          title: 'Error Reading Trainers',
          type: 'error',
        })
      }
    })()
  }, [trainerClassId])

  return (
    <div className="flex flex-row gap-1">
      <TextDropdownInput
        value={trainerClassId?.toString() ?? undefined}
        placeholder="Trainer Class"
        onChange={value => value ? setTrainerClassId(parseInt(value)) : null}
      />
      <TextDropdownInput
        value={trainerId?.toString() ?? undefined}
        placeholder="Trainer Name"
        onChange={value => value ? setTrainerId(parseInt(value)) : null}
      />
    </div>
  )
}