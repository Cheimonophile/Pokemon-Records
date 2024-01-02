import React, { FC, ReactNode, createContext, useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { Battles } from './pages/Battles';
import { Open } from './pages/Open';
import { setDBConnection } from './backend/data/state';
import { DATABASE_URL } from './constants';
import { message } from '@tauri-apps/api/dialog';
// import { Catches } from './pages/Catches';
import { Nav } from './components/Nav';
// import './App.css';


/**
 * The pages in the application
 */
export const PAGES = Object.freeze({
  Open: Open,
  Battles: Battles,
  // Catches: Catches
} satisfies {
  [key: string]: FC<{}>
})
export type PageKey = keyof typeof PAGES;

type AppContextState = {
  currentPage: ReactNode;
  setCurrentPage: (page: PageKey) => void;
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

  const [currentPage, setCurrentPage] = useState<PageKey>('Open');
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
      currentPage,
      setCurrentPage,
      addEffect,
      refresh
    };
  }, [currentPage, addEffect, refresh]);

  /**
   * The component for the page
   */
  const CurrentPage = PAGES[currentPage]

  return (
    <AppContext.Provider value={appContextState}>
      <div className="h-full w-full flex flex-col overflow-hidden">

        {/* Nav Bar */}
        <Nav />
        <hr />

        {/* Body */}
        <div className="flex-1 overflow-hidden">

          <CurrentPage key={currentPage} />

        </div>
      </div>
    </AppContext.Provider>
  );
}

export default App;
