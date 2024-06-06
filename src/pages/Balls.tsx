import { useBalls } from "hooks/data/useBalls";
import { FC } from "react";


export const Balls: FC<{}> = () => {

  // state
  const { ballsArray } = useBalls();

  return (
    <div className="h-full flex flex-col flex-1 p-2 gap-2">
      <h1>Balls</h1>
      <ul className="flex-1 overflow-y-scroll">
        {ballsArray?.map(ball => (
          <li key={ball.id}>
            ({ball.id}) {ball.name}
          </li>
        ))}
      </ul>
    </div>
  )
}