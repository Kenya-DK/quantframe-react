import { createContext, useContext, useEffect, useState } from "react";
import { TauriTypes } from "$types";
import { OnTauriDataEvent } from "@api/index";
import api from "@api/index";

export type StockContextProps = {
  items: TauriTypes.StockItem[];
  rivens: TauriTypes.StockRiven[];
  wish_lists: TauriTypes.WishListItem[];
};
export type StockContextProviderProps = {
  children: React.ReactNode;
};
interface Entity {
  id: string | number;
}

type SetDataFunction<T> = React.Dispatch<React.SetStateAction<T>>;
export const StockContextContext = createContext<StockContextProps>({
  rivens: [],
  items: [],
  wish_lists: [],
});

export const useStockContextContext = () => useContext(StockContextContext);

export function StockContextProvider({ children }: StockContextProviderProps) {
  const [items, setItems] = useState<TauriTypes.StockItem[]>([]);
  const [rivens, setRivens] = useState<TauriTypes.StockRiven[]>([]);
  const [wish_list, setWishList] = useState<TauriTypes.WishListItem[]>([]);

  const handleUpdate = <T extends Entity>(operation: TauriTypes.EventOperations, data: T | T[], setData: SetDataFunction<T[]>) => {
    switch (operation) {
      case TauriTypes.EventOperations.CREATE_OR_UPDATE:
        // setData(myState.map(item => item.id === id ? {...item, item.description: "new desc"} : item))
        setData((items) => {
          // Check if the item already exists in the list
          const itemExists = items.some((item) => item.id === (data as T).id);

          // If the item exists, update it; otherwise, add the new item
          if (itemExists) return items.reverse().map((item) => (item.id === (data as T).id ? (data as T) : item));
          else return [data as T, ...items.reverse()];
        });
        break;
      case TauriTypes.EventOperations.DELETE:
        setData((items) => items.filter((item) => item.id !== (data as T).id));
        break;
      case TauriTypes.EventOperations.SET:
        setData(data as T[]);
        break;
    }
  };

  // Hook on tauri events from rust side
  useEffect(() => {
    OnTauriDataEvent<any>(TauriTypes.Events.UpdateStockItems, ({ data, operation }) => handleUpdate(operation, data, setItems));
    OnTauriDataEvent<any>(TauriTypes.Events.UpdateStockRivens, ({ data, operation }) => handleUpdate(operation, data, setRivens));
    OnTauriDataEvent<any>(TauriTypes.Events.UpdateWishList, ({ data, operation }) => handleUpdate(operation, data, setWishList));
    return () => {
      api.events.CleanEvent(TauriTypes.Events.UpdateStockItems);
      api.events.CleanEvent(TauriTypes.Events.UpdateStockRivens);
    };
  }, []);

  return <StockContextContext.Provider value={{ items, rivens, wish_lists: wish_list }}>{children}</StockContextContext.Provider>;
}
