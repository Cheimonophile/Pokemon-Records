import { RefObject, useEffect, useRef } from "react";

/**
 * This Hook can be used for detecting clicks outside the Opened Menu
 */
export function useClickOutside(ref: RefObject<HTMLElement>, onClickOutside: () => void) {

  const callbackRef = useRef(onClickOutside);
  callbackRef.current = onClickOutside;


  useEffect(() => {
    /**
     * Invoke Function onClick outside of element
     */
    function handleClickOutside(event: MouseEvent) {
      if (
        event.target instanceof Node &&
        ref.current &&
        !ref.current.contains(event.target)
      ) {
        callbackRef.current();
      }
    }
    // Bind
    document.addEventListener("mouseup", handleClickOutside);
    return () => {
      // dispose
      document.removeEventListener("mouseup", handleClickOutside);
    };
  }, [ref]);
}