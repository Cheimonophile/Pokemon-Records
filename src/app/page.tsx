'use client'



import { Fragment, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { sections } from '@/constants/sections'
import Link from 'next/link'




export default function App() {


  useEffect(() => {
    invoke('read_playthroughs', {})
      .then(console.log)
      .catch(console.error)
  }, [])

  return (
    <>
      <ul>
        {sections.map((section, i) => (
          <Fragment key={i}>
            <li>
              <Link href={`/${section}`}>
                {section}
              </Link>
            </li>
          </Fragment>
        ))}
      </ul>
    </>
  )
}
