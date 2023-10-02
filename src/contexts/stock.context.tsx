import { createContext, useContext, useEffect, useState } from "react";
import { StockItemDto, StockRivenDto } from '$types/index';
import { OnTauriUpdateDataEvent } from "../utils";

type StockContextProps = {
  items: StockItemDto[];
  rivens: StockRivenDto[];
}
type StockContextProviderProps = {
  children: React.ReactNode;
}

export const StockContextContext = createContext<StockContextProps>({
  rivens: [],
  items: [],
});

export const useStockContextContext = () => useContext(StockContextContext);

export const StockContextProvider = ({ children }: StockContextProviderProps) => {
  const [items, setItems] = useState<StockItemDto[]>([]);
  const [rivens, setRivens] = useState<StockRivenDto[]>([]);

  // Handle update, create, delete orders
  const handleUpdateItems = (operation: string, data: StockItemDto | StockItemDto[] | string) => {
    switch (operation) {
      case "CREATE_OR_UPDATE":
        {
          const order = data as StockItemDto;
          setItems((stocks) => [...stocks.filter((item) => item.id !== order.id), order]);
        }
        break;
      case "DELETE":
        {
          const order = data as StockItemDto;
          setItems((stocks) => [...stocks.filter((item) => item.id !== order.id)]);
        }
        break;
      case "SET":
        {
          const stocks = data as StockItemDto[];
          setItems(stocks);
        }
        break;
    }
  }
  // Handle update, create, delete orders
  const handleUpdateRiven = (operation: string, data: StockRivenDto | StockRivenDto[] | string) => {
    switch (operation) {
      case "CREATE_OR_UPDATE":
        {
          const order = data as StockRivenDto;
          setRivens((stocks) => [...stocks.filter((item) => item.id !== order.id), order]);
        }
        break;
      case "DELETE":
        {
          const order = data as StockRivenDto;
          setRivens((stocks) => [...stocks.filter((item) => item.id !== order.id)]);
        }
        break;
      case "SET":
        {
          const stocks = data as StockRivenDto[];
          setRivens(stocks);
        }
        break;
    }
  }
  // Hook on tauri events from rust side
  useEffect(() => {
    OnTauriUpdateDataEvent<StockItemDto>("StockItems", ({ data, operation }) => handleUpdateItems(operation, data));
    OnTauriUpdateDataEvent<StockRivenDto>("StockRivens", ({ data, operation }) => handleUpdateRiven(operation, data));

    return () => { }
  }, []);

  return (
    <StockContextContext.Provider value={{ items, rivens }}>
      {children}
    </StockContextContext.Provider>
  )
}