import { createContext, useContext, useEffect, useState } from "react";
import { StockEntryDto, StatisticDto, TransactionEntryDto, Wfm } from '$types/index';
import { OnTauriUpdateDataEvent, getStatistic } from "../utils";

type WarframeMarketContextProps = {
  transactions: TransactionEntryDto[];
  orders: Wfm.OrderDto[];
  inventorys: StockEntryDto[];
  statistics: StatisticDto | undefined;
}
type WarframeMarketContextProviderProps = {
  children: React.ReactNode;
}

export const WarframeMarketContextContext = createContext<WarframeMarketContextProps>({
  transactions: [],
  inventorys: [],
  orders: [],
  statistics: undefined,
});

export const useWarframeMarketContextContext = () => useContext(WarframeMarketContextContext);

export const WarframeMarketContextProvider = ({ children }: WarframeMarketContextProviderProps) => {
  const [transactions, setTransactions] = useState<TransactionEntryDto[]>([]);
  const [inventorys, setInventorys] = useState<StockEntryDto[]>([]);
  const [statistics, setStatistics] = useState<StatisticDto | undefined>(undefined);
  const [orders, setOrders] = useState<Wfm.OrderDto[]>([]);

  // Handle update, create, delete orders
  const handleUpdateOrders = (operation: string, data: Wfm.OrderDto | Wfm.OrderDto[] | string) => {
    switch (operation) {
      case "CREATE_OR_UPDATE":
        {
          const order = data as Wfm.OrderDto;
          setOrders((inventorys) => [...inventorys.filter((item) => item.id !== order.id), order]);
        }
        break;
      case "DELETE":
        {
          const order = data as Wfm.OrderDto;
          setOrders((inventorys) => [...inventorys.filter((item) => item.id !== order.id)]);
        }
        break;
      case "SET":
        {
          const orders = data as Wfm.OrderDto[];
          setOrders(orders);
        }
        break;
    }
  }

  // Handle update, create, delete inventory
  const handleUpdateInventory = (operation: string, data: StockEntryDto | StockEntryDto[]) => {
    switch (operation) {
      case "CREATE_OR_UPDATE":
        setInventorys((inventorys) => [...inventorys.filter((item) => item.id !== (data as StockEntryDto).id), data as StockEntryDto]);
        break;
      case "DELETE":
        setInventorys((inventorys) => [...inventorys.filter((item) => item.id !== (data as StockEntryDto).id)]);
        break;
      case "SET":
        setInventorys(data as StockEntryDto[]);
        break;
    }
  }

  // Handle update, create, delete transaction
  const handleUpdateTransaction = (operation: string, data: TransactionEntryDto | TransactionEntryDto[]) => {
    switch (operation) {
      case "CREATE_OR_UPDATE":
        setTransactions((transactions) => [...transactions.filter((item) => item.id !== (data as TransactionEntryDto).id), data as TransactionEntryDto]);
        break;
      case "DELETE":
        setTransactions((transactions) => [...transactions.filter((item) => item.id !== (data as TransactionEntryDto).id)]);
        break;
      case "SET":
        setTransactions(data as TransactionEntryDto[]);
        break;
    }
  }

  // Handle update of statistics when transactions change
  useEffect(() => {
    if (!transactions) return;
    let statistics = getStatistic(transactions);
    setStatistics(statistics);
  }, [transactions]);

  // Hook on tauri events from rust side
  useEffect(() => {
    OnTauriUpdateDataEvent<StockEntryDto>("inventorys", ({ data, operation }) => handleUpdateInventory(operation, data));
    OnTauriUpdateDataEvent<TransactionEntryDto>("transactions", ({ data, operation }) => handleUpdateTransaction(operation, data));
    OnTauriUpdateDataEvent<Wfm.OrderDto>("orders", ({ data, operation }) => handleUpdateOrders(operation, data));
    return () => { }
  }, []);

  return (
    <WarframeMarketContextContext.Provider value={{ transactions, inventorys, statistics, orders }}>
      {children}
    </WarframeMarketContextContext.Provider>
  )
}