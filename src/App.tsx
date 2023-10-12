import React, { Fragment, ReactNode, createContext, useMemo, useState } from 'react';
import { flexGrow } from './styles';
import { Battles } from './pages/Battles';
import { Open } from './pages/Open';
import { Link } from './components/Link';
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
]

export type AppContextState = {
  page: ReactNode;
  setPage: (page: ReactNode) => void;
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

  const appContextState = useMemo(() => {
    return {
      page,
      setPage,
    };
  }, [page]);

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
