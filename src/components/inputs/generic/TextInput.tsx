import { ReactNode } from "react";




/**
 * Raw HTML Input with Styling
 */
export function TextInput({
  value,
  valid,
  placeholder,
  onChange,
}: {
  value: string,
  valid: boolean,
  placeholder: string,
  onChange: (value: string) => void,
}): ReactNode {
  return (
    <div>
      <input
        type="text"
        className={`${valid || 'text-red-500'} border-b px-1`}
        placeholder={placeholder}
        value={value}
        onChange={e => onChange(e.target.value)}
      />
    </div>

  )
}