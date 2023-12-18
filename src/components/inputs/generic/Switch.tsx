import { ReactNode } from "react"



/**
 * Option for a Switch
 */
export interface SwitchOption {
  value: string,
  label: ReactNode,
}


/**
 * Generic switch for the app
 */
export const Switch = (props: {
  value?: string,
  options?: SwitchOption[],
  setValue?: (value: string) => void,
}) => {
  return (
    <select
      className="appearance-none cursor-pointer px-1 bg-transparent rounded hover:bg-gray-100"
      value={props.value} onChange={e => props.setValue?.(e.target.value)}>
      {props.options?.map((option, i) => (
        <option key={i} value={option.value}>{option.label}</option>
      ))}
    </select>
  )
}