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
    <div className="h-full w-full flex flex-col p-1 g-1">

      {/* Nav Bar */}
      <div className="flex-none flex flex-row gap-1">
        {NAV_ITEMS.map((navItem) => (
          <Fragment key={navItem.name}>
            <div>
              <button
                onClick={() => setPage(navItem.page)}>
                {navItem.name}
              </button>
            </div>
          </Fragment>
        ))}
      </div>

      {/* Body */}
      <div className="flex-1">
        <AppContext.Provider value={appContextState}>
          {page}
        </AppContext.Provider>
      </div>
    </div>
  );
}

export default App;
