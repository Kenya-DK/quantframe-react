import { createContext, useContext, useEffect, useState } from "react";
import { QfSocketEvent, QfSocketEventOperation, StockItem, StockRiven } from "@api/types";
import { OnTauriDataEvent } from "@api/index";
import api from "@api/index";

export type StockContextProps = {
  items: StockItem[];
  rivens: StockRiven[];
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
});

export const useStockContextContext = () => useContext(StockContextContext);

export function StockContextProvider({ children }: StockContextProviderProps) {
  const [items, setItems] = useState<StockItem[]>([]);
  const [rivens, setRivens] = useState<StockRiven[]>([]);

  useEffect(() => {
    console.log(
      "StockContextProvider: CREATE_OR_UPDATE",
      (rivens as any[]).map((x) => (x as any).id)
    );
  }, [rivens]);

  const handleUpdate = <T extends Entity>(operation: QfSocketEventOperation, data: T | T[], setData: SetDataFunction<T[]>) => {
    switch (operation) {
      case QfSocketEventOperation.CREATE_OR_UPDATE:
        // setData(myState.map(item => item.id === id ? {...item, item.description: "new desc"} : item))
        setData((items) => {
          // Check if the item already exists in the list
          const itemExists = items.some((item) => item.id === (data as T).id);

          // If the item exists, update it; otherwise, add the new item
          if (itemExists) return items.reverse().map((item) => (item.id === (data as T).id ? (data as T) : item));
          else return [data as T, ...items.reverse()];
        });
        break;
      case QfSocketEventOperation.DELETE:
        setData((items) => items.filter((item) => item.id !== (data as T).id));
        break;
      case QfSocketEventOperation.SET:
        setData(data as T[]);
        break;
    }
    // Add this somewhere to check if setData is being called multiple times unintentionally
    console.log("Data change detected:", data);
  };

  // Hook on tauri events from rust side
  useEffect(() => {
    OnTauriDataEvent<any>(QfSocketEvent.UpdateStockItems, ({ data, operation }) => handleUpdate(operation, data, setItems));
    OnTauriDataEvent<any>(QfSocketEvent.UpdateStockRivens, ({ data, operation }) => handleUpdate(operation, data, setRivens));
    return () => {
      api.events.CleanEvent(QfSocketEvent.UpdateStockItems);
      api.events.CleanEvent(QfSocketEvent.UpdateStockRivens);
    };
  }, []);

  return <StockContextContext.Provider value={{ items, rivens }}>{children}</StockContextContext.Provider>;
}
