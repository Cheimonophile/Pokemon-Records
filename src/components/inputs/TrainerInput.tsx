import { message } from "@tauri-apps/api/dialog";
import { createTrainerClass, readTrainerClasses } from "backend/data/trainer_classes";
import { createTrainer, readTrainers } from "backend/data/trainers";
import { ReactNode, useCallback, useEffect, useState } from "react";
import { TextDropdownInput, TextDropdownOption } from "./generic/TextDropdownInput";
import { useAppContext } from "App";
import { confirm } from "@tauri-apps/api/dialog";



export function TrainerInput({
  trainerId,
  setTrainerId,
}: {
  trainerId: number | null
  setTrainerId: (value: number | null) => void
}): ReactNode {

  // context
  const { addEffect } = useAppContext()

  // trainer class state
  const [trainerClassId, setTrainerClassId] = useState<number | null>(null)

  // make sure trainer class is from trainer
  useEffect(() => {
    (async () => {
      if (trainerId) {
        const [trainer] = await readTrainers({
          id: trainerId,
          classId: null,
        })
        setTrainerClassId(trainer.class.id)
      }
    })()
  }, [trainerId])


  // is the opponent valid
  const [trainerClassOptions, setTrainerClassOptions] = useState<TextDropdownOption[]>()
  useEffect(() => {
    return addEffect(async () => {
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
    })
  }, [addEffect])

  // trainer options
  const [trainerOptions, setTrainerOptions] = useState<TextDropdownOption[]>()
  useEffect(() => {
    addEffect(async () => {
      try {
        const trainers = await readTrainers({
          id: null,
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
    })
  }, [addEffect, trainerClassId])

  /**
   * Create a new trainer class
   */
  const createNewTrainerClass = useCallback(async (name: string): Promise<string | undefined> => {
    try {
      const ok = await confirm(`Create New Trainer Class '${name}'?`)
      if (ok) {
        const newTrainerClassId = await createTrainerClass({
          name,
        })
        return newTrainerClassId.toString()
      }
    }
    catch (error) {
      console.error(error)
      await message(`${error}`, {
        title: 'Error Creating New Trainer Class',
        type: 'error',
      })
    }
  }, [])

  /**
   * Create a new trainer
   */
  const createNewTrainer = useCallback(async (name: string): Promise<string | undefined> => {
    try {
      if (trainerClassId === null) 
        throw new Error("Trainer Class ID is null")
      const ok = await confirm(`Create New Trainer '${name}'?`)
      if (ok) {
        const newTrainerId = await createTrainer({
          name,
          classId: trainerClassId,
        })
        return newTrainerId.toString()
      }
    }
    catch (error) {
      console.error(error)
      await message(`${error}`, {
        title: 'Error Creating New Trainer',
        type: 'error',
      })
    }
  }, [trainerClassId])



  return (
    <div className="flex flex-row gap-1">
      <TextDropdownInput
        value={trainerClassId?.toString() ?? undefined}
        placeholder="Trainer Class"
        options={trainerClassOptions}
        onChange={value => setTrainerClassId(value ? parseInt(value) : null)}
        createNew={createNewTrainerClass}
      />
      <TextDropdownInput
        value={trainerId?.toString() ?? undefined}
        placeholder="Trainer Name"
        options={trainerOptions}
        onChange={value => setTrainerId(value ? parseInt(value) : null)}
        createNew={createNewTrainer}
      />
    </div>
  )
}