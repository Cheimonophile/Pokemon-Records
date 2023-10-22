import React, { Fragment, ReactNode, createContext, useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { flexGrow } from './styles';
import { Battles } from './pages/Battles';
import { Open } from './pages/Open';
import { setDBConnection } from './backend/state';
import { DATABASE_URL } from './constants';
import { message } from '@tauri-apps/api/dialog';
import { Catches } from './pages/Catches';
// import './App.css';





interface NavItem {
  name: string;
  page: ReactNode;
}

const NAV_ITEMS: NavItem[] = [
  {
    name: 'Open',
    page: <Open />,
  },
  {
    name: 'Battles',
    page: <Battles />,
  },
  {
    name: 'Catches',
    page: <Catches />,
  }
]

export type AppContextState = {
  page: ReactNode;
  setPage: (page: ReactNode) => void;
  addEffect: (callback: () => Promise<void>) => () => void;
  refresh: () => Promise<void>;
};


export const AppContext = createContext<AppContextState | null>(null);


export function useAppContext(): AppContextState {
  const appContext = React.useContext(AppContext);
  if (!appContext) {
    throw new Error("useAppContext must be used within the app");
  }
  return appContext;
}




function App() {

  const [page, setPage] = useState<ReactNode>(<Open />);
  const refreshCallbackRef = useRef<Set<() => Promise<void>>>(new Set());

  // add an effect to the refreshCallbackRef
  const addEffect = useCallback((callback: () => Promise<void>) => {
    refreshCallbackRef.current.add(callback);
    callback();
    return () => {
      refreshCallbackRef.current.delete(callback);
    }
  }, [])

  // call all refresh callbacks
  const refresh = useCallback(async () => {
    for (const callback of Array.from(refreshCallbackRef.current)) {
      await callback();
    }
  }, [])

  // load db connection
  useEffect(() => {
    (async () => {
      try {
        const db_url = localStorage.getItem(DATABASE_URL);
        if (!db_url) {
          return;
        }
        await setDBConnection({ databaseUrl: db_url });
      }
      catch (error) {
        await message(`${error}`, {
          title: 'Error Opening Database',
          type: 'error',
        })
        console.error(error);
      }
    })()
  }, [])


  const appContextState = useMemo<AppContextState>(() => {
    return {
      page,
      setPage,
      addEffect,
      refresh
    };
  }, [page, addEffect, refresh]);

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
        display: 'flex',
        gap: '0.25rem',
      }}>
        {NAV_ITEMS.map((navItem) => (
          <Fragment key={navItem.name}>
            <div>
              <span style={{
                appearance: 'none',
                cursor: 'pointer',
              }}
                onClick={() => setPage(navItem.page)}>
                {navItem.name}
              </span>
            </div>
          </Fragment>
        ))}
      </div>

      {/* Body */}
      <div style={{
        flex: flexGrow,
      }}>
        <AppContext.Provider value={appContextState}>
          {page}
        </AppContext.Provider>
      </div>
    </div>
  );
}

export default App;
