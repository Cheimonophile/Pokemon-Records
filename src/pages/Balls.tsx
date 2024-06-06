import { AgGridReact } from "ag-grid-react";
import { useBalls } from "hooks/data/useBalls";
import { IBall } from "interfaces/data/ball";
import { FC, useMemo } from "react";
import { ColDef } from "ag-grid-community";


const DEFAULT_COL_DEF: ColDef<IBall> = {
  sortable: false,
  filter: false,
}


export const Balls: FC<{}> = () => {

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
      {/* <ul className="flex-1 overflow-y-scroll">
        {ballsArray?.map(ball => (
          <li key={ball.id}>
            ({ball.id}) {ball.name}
          </li>
        ))}
      </ul> */}
    </div>
  )
}