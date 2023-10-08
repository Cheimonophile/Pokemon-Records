'use client'



import { FC, Fragment, useEffect, useState } from 'react'
import { ReadBattlesResult, readBattles } from '../../../backend/battles'
import Link from 'next/link'




export default function Page() {

    // parameter state



    // const [battles, setBattles] = useState<ReadBattlesResult[] | null>()


    // useEffect(() => {
    //     (async () => {
    //         try {
    //             const battles = await readBattles()
    //             console.log(battles)
    //             setBattles(battles)
    //         }
    //         catch (error) {
    //             console.log(error)
    //         }
    //     })();
    // }, [])

    return (
        <div className="w-full h-full flex flex-col">
            <h1 className="text-center">Create Battle</h1>
            <div className="flex-1 p-2">
                <div>

                </div>
                <div>
                    <button className="px-1 border-2 rounded">
                        Create Battle
                    </button>
                </div>
            </div>
        </div>
    )
}