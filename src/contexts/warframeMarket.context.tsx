import { createContext, useContext, useEffect, useState } from "react";
import { InventoryEntryDto, StatisticDto, TransactionEntryDto, Wfm } from '$types/index';
import { OnTauriUpdateDataEvent, getStatistic } from "../utils";

type WarframeMarketContextProps = {
  transactions: TransactionEntryDto[];
  orders: Wfm.OrderDto[];
  inventorys: InventoryEntryDto[];
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
  const [inventorys, setInventorys] = useState<InventoryEntryDto[]>([]);
  const [statistics, setStatistics] = useState<StatisticDto | undefined>(undefined);
  const [orders, setOrders] = useState<Wfm.OrderDto[]>([]);

  // Handle update, create, delete orders
  const handleUpdateOrders = (operation: string, data: Wfm.OrderDto | Wfm.OrderDto[] | string) => {
    switch (operation) {
      case "create":
        {
          const order = data as Wfm.OrderDto;
          setOrders((inventorys) => [...inventorys, order]);
        }
        break;
      case "update":
        {
          const order = data as Wfm.OrderDto;
          setOrders((inventorys) => [...inventorys.filter((item) => item.id !== order.id), order]);
        }
        break;
      case "delete":
        {
          const order_id = data as string;
          setOrders((inventorys) => [...inventorys.filter((item) => item.id !== order_id)]);
        }
        break;
      case "set":
        {
          const orders = data as Wfm.OrderDto[];
          setOrders(orders);
        }
        break;
    }
  }

  // Handle update, create, delete inventory
  const handleUpdateInventory = (operation: string, data: InventoryEntryDto | InventoryEntryDto[]) => {
    switch (operation) {
      case "create":
        setInventorys((inventorys) => [...inventorys, data as InventoryEntryDto]);
        break;
      case "update":
        setInventorys((inventorys) => [...inventorys.filter((item) => item.id !== (data as InventoryEntryDto).id), data as InventoryEntryDto]);
        break;
      case "delete":
        setInventorys((inventorys) => [...inventorys.filter((item) => item.id !== (data as InventoryEntryDto).id)]);
        break;
      case "set":
        setInventorys(data as InventoryEntryDto[]);
        break;
    }
  }

  // Handle update, create, delete transaction
  const handleUpdateTransaction = (operation: string, data: TransactionEntryDto | TransactionEntryDto[]) => {
    switch (operation) {
      case "create":
        setTransactions((transactions) => [...transactions, data as TransactionEntryDto]);
        break;
      case "update":
        setTransactions((transactions) => [...transactions.filter((item) => item.id !== (data as TransactionEntryDto).id), data as TransactionEntryDto]);
        break;
      case "delete":
        setTransactions((transactions) => [...transactions.filter((item) => item.id !== (data as TransactionEntryDto).id)]);
        break;
      case "set":
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
    OnTauriUpdateDataEvent<InventoryEntryDto>("inventorys", ({ data, operation }) => handleUpdateInventory(operation, data));
    OnTauriUpdateDataEvent<TransactionEntryDto>("transactions", ({ data, operation }) => handleUpdateTransaction(operation, data));
    OnTauriUpdateDataEvent<Wfm.OrderDto | string>("orders", ({ data, operation }) => handleUpdateOrders(operation, data));
    return () => { }
  }, []);

  return (
    <WarframeMarketContextContext.Provider value={{ transactions, inventorys, statistics, orders }}>
      {children}
    </WarframeMarketContextContext.Provider>
  )
}