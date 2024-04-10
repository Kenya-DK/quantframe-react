import { createContext, useContext, useEffect, useState } from "react";
import { StatisticDto, TransactionEntryDto, Wfm } from '$types/index';
import { OnTauriUpdateDataEvent, GetStatistic } from "../utils";

type WarframeMarketContextProps = {
  transactions: TransactionEntryDto[];
  orders: Wfm.OrderDto[];
  auctions: Wfm.Auction<string>[];
  statistics: StatisticDto | undefined;
}
type WarframeMarketContextProviderProps = {
  children: React.ReactNode;
}

export const WarframeMarketContextContext = createContext<WarframeMarketContextProps>({
  transactions: [],
  orders: [],
  auctions: [],
  statistics: undefined,
});

export const useWarframeMarketContextContext = () => useContext(WarframeMarketContextContext);

export const WarframeMarketContextProvider = ({ children }: WarframeMarketContextProviderProps) => {
  const [transactions, setTransactions] = useState<TransactionEntryDto[]>([]);
  const [statistics, setStatistics] = useState<StatisticDto | undefined>(undefined);
  const [orders, setOrders] = useState<Wfm.OrderDto[]>([]);
  const [auctions, setAuctions] = useState<Wfm.Auction<string>[]>([]);

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

  // Handle update, create, delete transaction
  const handleUpdateAuction = (operation: string, data: Wfm.Auction<string> | Wfm.Auction<string>[]) => {
    switch (operation) {
      case "CREATE_OR_UPDATE":
        setAuctions((auctions) => [...auctions.filter((item) => item.id !== (data as Wfm.Auction<string>).id), data as Wfm.Auction<string>]);
        break;
      case "DELETE":
        setAuctions((auctions) => [...auctions.filter((item) => item.id !== (data as Wfm.Auction<string>).id)]);
        break;
      case "SET":
        setAuctions(data as Wfm.Auction<string>[]);
        break;
    }
  }

  // Handle update of statistics when transactions change
  useEffect(() => {
    if (!transactions) return;
    let statistics = GetStatistic(transactions);
    console.log("Statistics", statistics);

    setStatistics(statistics);
  }, [transactions]);

  // Hook on tauri events from rust side
  useEffect(() => {
    OnTauriUpdateDataEvent<TransactionEntryDto>("transactions", ({ data, operation }) => handleUpdateTransaction(operation, data));
    OnTauriUpdateDataEvent<Wfm.OrderDto>("orders", ({ data, operation }) => handleUpdateOrders(operation, data));
    OnTauriUpdateDataEvent<Wfm.Auction<string>>("auctions", ({ data, operation }) => handleUpdateAuction(operation, data));

    return () => { }
  }, []);

  return (
    <WarframeMarketContextContext.Provider value={{ transactions, statistics, orders, auctions }}>
      {children}
    </WarframeMarketContextContext.Provider>
  )
}