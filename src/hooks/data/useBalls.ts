import { readBalls } from "backend/data/balls";
import { IBall } from "interfaces/data/ball";
import { useCallback, useEffect, useMemo, useReducer, useState } from "react";




/**
 * Hook that lets you use balls from the database
 */
export const useBalls = () => {

  // state
  const [balls, setBalls] = useState<ReadonlyMap<number, Readonly<IBall>> | null | undefined>();

  // load the balls
  useEffect(() => {
    readBalls({})
      .then(ballsRes => {
        const ballsMap = new Map<number, IBall>(ballsRes.map(ball => [ball.id, ball]));
        setBalls(ballsMap);
      })
      .catch(console.error)
  }, [])

  /**
   * Balls Array
   */
  const ballsArray = useMemo(() => balls && Array.from(balls.values()), [balls]);

  return {
    balls,
    ballsArray
  }
}