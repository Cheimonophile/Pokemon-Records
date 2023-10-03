'use client'



import { useEffect } from 'react'
import { invoke } from '@tauri-apps/api/tauri'




export default function App() {


  useEffect(() => {
    invoke('read_playthroughs', {})
      .then(console.log)
      .catch(console.error)
  }, [])

  return (
    <main>
      Hello World!
    </main>
  )
}
