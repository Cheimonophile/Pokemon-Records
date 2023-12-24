import { message } from "@tauri-apps/api/dialog";
import { readTrainerClasses } from "backend/data/trainer_classes";
import { readTrainers } from "backend/data/trainers";
import { Dispatch, ReactNode, SetStateAction, useEffect, useState } from "react";
import { TextInput } from "./generic/TextInput";


type Trainer = {
  name: string
  class: string
}



export function TrainerInput({
  trainer,
  setTrainer
}: {
  trainer: Trainer
  setTrainer: Dispatch<SetStateAction<Trainer>>
}): ReactNode {


  // is the opponent valid
  const [trainerValidity, setTrainerValidity] = useState<{ name: boolean, class: boolean }>({ name: false, class: false })
  useEffect(() => {
    // trainer classes
    (async () => {
      try {
        const trainerClasses = await readTrainerClasses({ name: trainer.class })
        setTrainerValidity(prev => ({ ...prev, class: trainerClasses.length > 0 }))
      }
      catch (error) {
        console.error(error)
        await message(`${error}`, {
          title: 'Error Reading Trainer Classes',
          type: 'error',
        })
      }
    })();
    // trainers
    (async () => {
      try {
        const trainers = await readTrainers({ name: trainer.name, class: trainer.class })
        setTrainerValidity(prev => ({ ...prev, name: trainers.length > 0 }))
      }
      catch (error) {
        console.error(error)
        await message(`${error}`, {
          title: 'Error Reading Trainers',
          type: 'error',
        })
      }
    })();
  }, [trainer])



  return (
    <div className="flex flex-row gap-1">
      <TextInput
        value={trainer.class}
        placeholder="Trainer Class"
        onChange={value => setTrainer(prev => ({ ...prev, class: value }))}
        valid={trainerValidity.class}
      />
      <TextInput
        value={trainer.name}
        placeholder="Trainer Name"
        onChange={value => setTrainer(prev => ({ ...prev, name: value }))}
        valid={trainerValidity.name}
      />
    </div>
  )
}