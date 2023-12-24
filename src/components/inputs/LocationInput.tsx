import { message } from "@tauri-apps/api/dialog";
import { readBattles } from "backend/data/battles";
import { readLocations } from "backend/data/locations";
import { readRegions } from "backend/data/regions";
import { Dispatch, ReactNode, SetStateAction, useEffect, useMemo, useState } from "react";
import { SwitchInput, SwitchOption } from "./generic/SwitchInput";
import { TextInput } from "./generic/TextInput";




interface Props {
  location: { region: string, name: string }
  setLocation: Dispatch<SetStateAction<{ region: string, name: string }>>
}


/**
 * Input for a location in pokemon
 */
export function LocationInput({
  location,
  setLocation,
}: Props): ReactNode {

  // options
  const [regionOptions, setRegionOptions] = useState<SwitchOption[]>()
  useEffect(() => {
    (async () => {
      try {
        const [regions, mostRecentBattle] = await Promise.all([
          readRegions({}),
          readBattles({ howMany: 1 }),
        ])
        const regionSwitchOptions = regions.map(region => {
          return {
            value: region,
            label: region,
          }
        })
        setRegionOptions(regionSwitchOptions);
        // setLocation({
        //     region: mostRecentBattle[0].event.location_region,
        //     name: mostRecentBattle[0].event.location_name
        // })
      }
      catch (error) {
        console.error(error)
        await message(`${error}`, {
          title: 'Error Reading Regions',
          type: 'error',
        })
      }
    })()
  }, [])
  const [locationValid, setLocationValid] = useState<boolean>(false)
  useEffect(() => {
    (async () => {
      try {
        const locations = await readLocations({ name: location.name, region: location.region })
        setLocationValid(locations.length > 0)
      }
      catch (error) {
        console.error(error)
        await message(`${error}`, {
          title: 'Error Reading Locations',
          type: 'error',
        })
      }
    })()
  }, [location])



  return (
    <div>
      <SwitchInput
        value={location.region}
        options={regionOptions}
        onChange={region => setLocation(prev => ({ ...prev, region }))}
      />
      <TextInput
        value={location.name}
        valid={locationValid}
        onChange={name => setLocation(prev => ({ ...prev, name }))}
      />
    </div>
  )
}