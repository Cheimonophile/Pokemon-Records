import { error } from "console";
import { DependencyList, useEffect, useState } from "react";




export function useAsyncMemo<T>(factory: () => Promise<T>, deps: DependencyList[]): T | undefined | null {
  const [value, setValue] = useState<T | null | undefined>(undefined);
  useEffect(() => {
    factory().then(setValue).catch((error) => {
      console.error(error)
      setValue(null)
    });
  }, deps);
  return value;
}