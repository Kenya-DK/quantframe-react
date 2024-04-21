import { createContext, useContext, useEffect, useState } from "react";
import { QfSocketEvent, QfSocketEventOperation, StockItem, StockRiven } from "@api/types";
import { OnTauriDataEvent } from "@api/index";
import api from "@api/index";

export type StockContextProps = {
  items: StockItem[];
  rivens: StockRiven[];
}
export type StockContextProviderProps = {
  children: React.ReactNode;
}
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

  const handleUpdate = <T extends Entity>(operation: QfSocketEventOperation, data: T | T[], setData: SetDataFunction<T[]>) => {
    switch (operation) {
      case QfSocketEventOperation.CREATE_OR_UPDATE:
        setData((items) => {
          const index = items.reverse().findIndex((item) => item.id === (data as T).id);
          if (index == -1)
            return [...items, data as T];
          const newItems = [...items];
          newItems[index] = data as T;
          return newItems;
        });
        break;
      case QfSocketEventOperation.DELETE:
        setData((items) => items.reverse().filter((item) => item.id !== (data as T).id));
        break;
      case QfSocketEventOperation.SET:
        setData(data as T[]);
        break;
    }
  }

  // Hook on tauri events from rust side
  useEffect(() => {
    OnTauriDataEvent<any>(QfSocketEvent.UpdateStockItems, ({ data, operation }) => handleUpdate(operation, data, setItems));
    OnTauriDataEvent<any>(QfSocketEvent.UpdateStockRivens, ({ data, operation }) => handleUpdate(operation, data, setRivens));
    return () => {
      api.events.CleanEvent(QfSocketEvent.UpdateStockItems);
      api.events.CleanEvent(QfSocketEvent.UpdateStockRivens);
    }
  }, []);

  return (
    <StockContextContext.Provider value={{ items, rivens }}>
      {children}
    </StockContextContext.Provider>
  )
}