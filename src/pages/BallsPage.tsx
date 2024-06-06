import { AgGridReact } from "ag-grid-react";
import { useBalls } from "hooks/data/useBalls";
import { IBall } from "interfaces/data/ball";
import { FC, useMemo } from "react";
import { ColDef } from "ag-grid-community";
import { Page } from "interfaces/components/page";


/**
 * Default column definitions
 */
const DEFAULT_COL_DEF: ColDef<IBall> = {
  sortable: false,
  filter: false,
}

/**
 * Page that displays all of the balls
 */
export const BallsPage: Page = () => {

  // state
  const { ballsArray } = useBalls();


  /**
   * Col Defs
   */
  const colDefs = useMemo(() => {
    const colDefs: ColDef<IBall>[] = [
      { 
        headerName: 'ID', 
        valueGetter: params => params.data?.id,
        width: 50
      },
      { 
        headerName: 'Name', 
        valueGetter: params => params.data?.name,
        flex: 1
      }
    ]
    return colDefs;
  }, [])

  return (
    <div className="h-full flex flex-col flex-1 p-2 gap-2">
      <h1>Balls</h1>
      <div className="flex-1">
        <AgGridReact<IBall>
          className="ag-theme-balham"
          defaultColDef={DEFAULT_COL_DEF}
          columnDefs={colDefs}
          rowData={ballsArray}
        />
      </div>
    </div>
  )
}