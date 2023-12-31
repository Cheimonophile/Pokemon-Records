import { set } from "date-fns"
import { Fragment, ReactNode, useCallback, useEffect, useMemo, useRef, useState } from "react"



/**
 * Option for a Switch
 */
export interface TextDropdownOption {
  value: string,
  label: string,
}


/**
 * Generic switch for the app
 */
export const TextDropdownInput = ({
  value,
  options,
  placeholder,
  onChange,
}: {
  value?: string,
  options?: TextDropdownOption[],
  placeholder?: string,
  onChange?: (value: string | null) => void,
}) => {



  // state
  const [open, setOpen] = useState(false)
  const [text, setText] = useState("")
  const inputRef = useRef<HTMLInputElement>(null)

  const filteredOptions = useMemo(() => {
    return options?.filter(option => {
      return option.label.toLowerCase().includes(text?.toLowerCase() ?? "")
    })
  }, [options, text])

  const selected = useMemo(() => {
    return options?.find(option => {
      return option.value === value
    })
  }, [options, value])


  useEffect(() => {
    setText(selected?.label ?? "")
  }, [selected])


  const selectOption = useCallback((value: string) => {
    const option = options?.find(option => {
      return option.value === value
    })
    if (option) {
      console.log(option)
      setText(option?.label ?? "")
      onChange?.(option?.value ?? null)
    }
    setOpen(false)
  }, [onChange, options, value])

  const onFocus = useCallback(() => {
    inputRef.current?.select()
    setOpen(true)
  }, [])

  const onBlur = useCallback(() => {
    setText(selected?.label ?? "")
    setOpen(false)
  }, [selectOption])


  const onKeyUp = useCallback((e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Enter") {
      if (filteredOptions) {
        selectOption(filteredOptions[0]?.value ?? null)
      }
      inputRef.current?.blur()
    }
    else if (e.key === "Escape") {
      inputRef.current?.blur()
    }
  }, [text, selectOption])

  return (
    <div className="relative overflow-visible">
      <div>
        <input
          type="text"
          onFocus={onFocus}
          onBlur={onBlur}
          className={`border-b px-1`}
          placeholder={placeholder}
          onKeyUp={onKeyUp}
          ref={inputRef}
          value={text}
          onClick={() => inputRef.current?.select()}
          onChange={(e) => setText(e.target.value)}
        />
      </div>
      {open && options && (
        <div className="absolute mt-1 z-50 border bg-white rounded px-1 py-0.5 shadow w-full max-h-96 overflow-y-scroll">
          {filteredOptions?.map(option => {
            return (
              <Fragment key={option.value}>
                <div
                  className="cursor-pointer hover:bg-gray-200"
                  onClick={() => selectOption(option.value)}
                >
                  {option.label}
                </div>
              </Fragment>
            )
          })}
        </div>
      )}
    </div>

  )
}