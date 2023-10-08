import React, { useEffect } from 'react';
import logo from './logo.svg';
import { invoke } from '@tauri-apps/api';
// import './App.css';

function App() {


  useEffect(() => {
    invoke('read_playthroughs', {})
      .then(console.log)
      .catch(console.error)
  }, [])


  return (
    <div style={{
      height: '100vh',
      width: '100vw',
    }}>
      <div style={{
        height: '100%',
        // width: '100%',
        display: 'flex',
        flexDirection: 'column',
        padding: '0.25rem',
        gap: '0.25rem',
      }}>

        {/* Nav Bar */}
        <div style={{
          flex: 'none',
        }}>
          Home
        </div>

        {/* Body */}
        <div style={{
          flex: '1 0 auto',
        }}>
          Children
        </div>
      </div>
    </div>
  );
}

export default App;
