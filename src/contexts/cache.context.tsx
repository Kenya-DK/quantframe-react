import { createContext, useContext, useEffect, useState } from "react";
import { Wfm } from '$types/index';
import { OnTauriEvent } from "../utils";

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
  const [items, setItems] = useState<Wfm.ItemDto[]>([]);
  const [riven_items, setRivenItems] = useState<Wfm.RivenItemTypeDto[]>([]);
  const [riven_attributes, setRivenAttributes] = useState<Wfm.RivenAttributeInfoDto[]>([]);
  const [images_map, setImagesMap] = useState<Record<string, string>>({});

  useEffect(() => {
    const records: Record<string, string> = {};
    for (const item of items)
      records[item.url_name] = item.thumb;
    for (const item of riven_items)
      records[item.url_name] = item.icon;
    setImagesMap(records);
  }, [items, riven_items]);

  // Hook on tauri events from rust side
  useEffect(() => {
    OnTauriEvent("Cache:Update:Items", (data: Wfm.ItemDto[]) => setItems(data));
    OnTauriEvent("Cache:Update:RivenTypes", (data: Wfm.RivenItemTypeDto[]) => setRivenItems(data));
    OnTauriEvent("Cache:Update:RivenAttributes", (data: Wfm.RivenAttributeInfoDto[]) => setRivenAttributes(data));
    return () => { }
  }, []);

  return (
    <CacheContext.Provider value={{ items, riven_items, riven_attributes, images_map }}>
      {children}
    </CacheContext.Provider>
  )
}