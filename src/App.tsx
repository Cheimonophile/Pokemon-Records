import React, { FC, ReactNode, useEffect, useState } from 'react';
import logo from './logo.svg';
import { invoke } from '@tauri-apps/api';
import { flexGrow } from './styles';
import { Battles } from './pages/Battles';
// import './App.css';


const ROUTES = Object.freeze({
  Battles: <Battles />,
})

type Route = keyof typeof ROUTES;





function App() {


  const [route, setRoute] = useState<Route>('Battles');


  return (
    <div style={{
      height: '100vh',
      width: '100vw',
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
          flex: flexGrow,
        }}>
          {ROUTES[route]}
        </div>
      </div>
  );
}

export default App;
