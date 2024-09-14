import { createContext, useContext, useEffect, useState } from "react";
import { Wfm } from '$types/index';
import { WFMSocketContextProvider } from "./wfmSocket.context";
import { ChatContextProvider } from "./chat.context";
import { QfSocketEventOperation, QfSocketEvent, TransactionDto, StatisticDto } from "@api/types";
import api, { OnTauriDataEvent } from "@api/index";
export type WarframeMarketContextProps = {
  transactions: TransactionDto[];
  orders: Wfm.OrderDto[];
  auctions: Wfm.Auction<string>[];
  statistics: StatisticDto | undefined;
}
export type WarframeMarketContextProviderProps = {
  children: React.ReactNode;
}

export const WarframeMarketContextContext = createContext<WarframeMarketContextProps>({
  transactions: [],
  orders: [],
  auctions: [],
  statistics: undefined,
});

export const useWarframeMarketContextContext = () => useContext(WarframeMarketContextContext);

interface Entity {
  id: string | number;
}

type SetDataFunction<T> = React.Dispatch<React.SetStateAction<T>>;

export function WarframeMarketContextProvider({ children }: WarframeMarketContextProviderProps) {
  const [transactions, setTransactions] = useState<TransactionDto[]>([]);
  const [statistics, setStatistics] = useState<StatisticDto | undefined>(undefined);
  const [orders, setOrders] = useState<Wfm.OrderDto[]>([]);
  const [auctions, setAuctions] = useState<Wfm.Auction<string>[]>([]);


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
        setData((items) => items.filter((item) => item.id !== (data as T).id));
        break;
      case QfSocketEventOperation.SET:
        setData(data as T[]);
        break;
    }
  }

  // Handle orders
  const handleUpdateOrders = (operation: QfSocketEventOperation, data: Wfm.OrderDto | Wfm.OrderDto[]) => {
    handleUpdate(operation, data, setOrders);
  }

  // Handle transactions
  const handleUpdateTransaction = (operation: QfSocketEventOperation, data: TransactionDto | TransactionDto[]) => {
    handleUpdate(operation, data, setTransactions);
  }

  // Handle auctions
  const handleUpdateAuction = (operation: QfSocketEventOperation, data: Wfm.Auction<string> | Wfm.Auction<string>[]) => {
    handleUpdate(operation, data, setAuctions);
  }

  // Handle update of statistics when transactions change
  useEffect(() => {
    if (!transactions) return;
    let statistics = api.statistic.convertFromTransaction(transactions);

    setStatistics(statistics);
  }, [transactions]);

  // Hook on tauri events from rust side
  useEffect(() => {
    OnTauriDataEvent<any>(QfSocketEvent.UpdateTransaction, ({ data, operation }) => handleUpdateTransaction(operation, data));
    OnTauriDataEvent<any>(QfSocketEvent.UpdateOrders, ({ data, operation }) => handleUpdateOrders(operation, data));
    OnTauriDataEvent<any>(QfSocketEvent.UpdateAuction, ({ data, operation }) => handleUpdateAuction(operation, data));
    return () => { }
  }, []);

  return (
    <WarframeMarketContextContext.Provider value={{ transactions, statistics, orders, auctions }}>
      <ChatContextProvider>
        <WFMSocketContextProvider>
          {children}
        </WFMSocketContextProvider>
      </ChatContextProvider>
    </WarframeMarketContextContext.Provider>
  )
}