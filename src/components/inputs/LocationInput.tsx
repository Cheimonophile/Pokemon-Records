import { message } from "@tauri-apps/api/dialog";
import { readLocations } from "backend/data/locations";
import { readRegions } from "backend/data/regions";
import { Dispatch, ReactNode, SetStateAction, useEffect, useState } from "react";
import { SwitchInput, SwitchOption } from "./generic/SwitchInput";
import { TextInput } from "./generic/TextInput";
import { Location } from "backend/models";
import { set } from "date-fns";
import { TextDropdownInput, TextDropdownOption } from "./generic/TextDropdownInput";




interface Props {
  locationId?: number | null,
  setLocationId: (location_id: number | null) => void,
}


/**
 * Input for a location in pokemon
 */
export function LocationInput({
  locationId,
  setLocationId,
}: Props): ReactNode {

  // set retion
  const [regionId, setRegionId] = useState<number | null>(null)

  // region options
  const [regionOptions, setRegionOptions] = useState<SwitchOption[]>()
  useEffect(() => {
    (async () => {
      try {
        const [regions] = await Promise.all([
          readRegions({}),
        ])
        const regionSwitchOptions = regions.map(region => {
          return {
            value: region.id.toString(),
            label: region.name,
          } satisfies SwitchOption
        })
        setRegionOptions(regionSwitchOptions);
        setRegionId(regions[0].id)
        setLocationId(0)
      }
      catch (error) {
        console.error(error)
        await message(`${error}`, {
          title: 'Error Reading Regions',
          type: 'error',
        })
      }
    })()
  }, [setLocationId])


  // location options
  const [locationOptions, setLocationOptions] = useState<TextDropdownOption[]>()
  useEffect(() => {
    (async () => {
      try {
        const [locations] = await Promise.all([
          readLocations({ regionId: regionId ?? undefined }),
        ])
        const locationDropdownOptions = locations.map(location => {
          return {
            value: location.id.toString(),
            label: location.name,
          } satisfies TextDropdownOption
        })
        setLocationOptions(locationDropdownOptions);
      }
      catch (error) {
        console.error(error)
        await message(`${error}`, {
          title: 'Error Reading Locations',
          type: 'error',
        })
      }
    })()
  }, [regionId])



  return (
    <div className="flex flex-row">
      <SwitchInput
        value={regionId?.toString()}
        options={regionOptions}
        onChange={regionId => setRegionId(parseInt(regionId))}
      />
      <TextDropdownInput
        value={locationId?.toString()}
        placeholder="Location"
        options={locationOptions}
        onChange={locationId => setLocationId(locationId ? parseInt(locationId) : null)}
      />
    </div>
  )
}