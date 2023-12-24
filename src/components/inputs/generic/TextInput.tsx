import { ReactNode } from "react";




/**
 * Raw HTML Input with Styling
 */
export function TextInput({
  value,
  valid,
  onChange,
}: {
  value: string,
  valid: boolean,
  onChange: (value: string) => void,
}): ReactNode {
  return (
    <input
      type="text"
      className={`${valid || 'text-red-500'} border-b px-1`}
      placeholder="Location"
      value={value}
      onChange={e => onChange(e.target.value)}
    />
  )
}