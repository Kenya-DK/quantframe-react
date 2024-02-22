import { createContext, useContext, useEffect } from "react";
import { CacheDataId, Wfm } from '$types/index';
import { OnTauriEvent } from "../utils";
import { useLocalStorage } from "@mantine/hooks";

type CacheContextProps = {
  items: Wfm.ItemDto[];
  riven_items: Wfm.RivenItemTypeDto[];
  riven_attributes: Wfm.RivenAttributeInfoDto[];
  images_map: Record<string, string>;
}

type CacheContextProviderProps = {
  children: React.ReactNode;
}




export const CacheContext = createContext<CacheContextProps>({
  items: [],
  riven_items: [],
  riven_attributes: [],
  images_map: {},
});

export const useCacheContext = () => useContext(CacheContext);

export const CacheContextProvider = ({ children }: CacheContextProviderProps) => {
  const [riven_items, setCacheWeapons] = useLocalStorage<Wfm.RivenItemTypeDto[]>({ key: CacheDataId.RivenItems, defaultValue: [] });
  const [riven_attributes, setCacheAttributes] = useLocalStorage<Wfm.RivenAttributeInfoDto[]>({ key: CacheDataId.RivenAttributes, defaultValue: [] });
  const [items, setCacheItems] = useLocalStorage<Wfm.ItemDto[]>({ key: CacheDataId.Items, defaultValue: [] });
  const [images_map, setCacheImages] = useLocalStorage<Record<string, string>>({ key: CacheDataId.ImagesMap, defaultValue: {} });

  useEffect(() => {
    const records: Record<string, string> = {};
    for (const item of items)
      records[item.url_name] = item.thumb;
    for (const item of riven_items)
      records[item.url_name] = item.icon;
    setCacheImages(records);
  }, [items, riven_items]);

  // Hook on tauri events from rust side
  useEffect(() => {
    OnTauriEvent("Cache:Update:Items", (data: Wfm.ItemDto[]) => setCacheItems(data));
    OnTauriEvent("Cache:Update:RivenTypes", (data: Wfm.RivenItemTypeDto[]) => setCacheWeapons(data));
    OnTauriEvent("Cache:Update:RivenAttributes", (data: Wfm.RivenAttributeInfoDto[]) => setCacheAttributes(data));
    return () => { }
  }, []);

  return (
    <CacheContext.Provider value={{ items, riven_items, riven_attributes, images_map }}>
      {children}
    </CacheContext.Provider>
  )
}