import { createContext, useContext, useEffect, useState } from "react";
import { WFMarketTypes } from "$types/index";
import { WFMSocketContextProvider } from "./wfmSocket.context";
import { ChatContextProvider } from "./chat.context";
import { TauriTypes } from "$types";
import api, { OnTauriDataEvent } from "@api/index";
export type WarframeMarketContextProps = {
  transactions: TauriTypes.TransactionDto[];
  orders: WFMarketTypes.OrderDto[];
  auctions: WFMarketTypes.Auction<string>[];
  statistics: TauriTypes.StatisticDto | undefined;
};
export type WarframeMarketContextProviderProps = {
  children: React.ReactNode;
};

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
  const [transactions, setTransactions] = useState<TauriTypes.TransactionDto[]>([]);
  const [statistics, setStatistics] = useState<TauriTypes.StatisticDto | undefined>(undefined);
  const [orders, setOrders] = useState<WFMarketTypes.OrderDto[]>([]);
  const [auctions, setAuctions] = useState<WFMarketTypes.Auction<string>[]>([]);

  const handleUpdate = <T extends Entity>(operation: TauriTypes.EventOperations, data: T | T[], setData: SetDataFunction<T[]>) => {
    switch (operation) {
      case TauriTypes.EventOperations.CREATE_OR_UPDATE:
        setData((items) => {
          const index = items.reverse().findIndex((item) => item.id === (data as T).id);
          if (index == -1) return [...items, data as T];
          const newItems = [...items];
          newItems[index] = data as T;
          return newItems;
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

  // Handle orders
  const handleUpdateOrders = (operation: TauriTypes.EventOperations, data: WFMarketTypes.OrderDto | WFMarketTypes.OrderDto[]) => {
    handleUpdate(operation, data, setOrders);
  };

  // Handle transactions
  const handleUpdateTransaction = (operation: TauriTypes.EventOperations, data: TauriTypes.TransactionDto | TauriTypes.TransactionDto[]) => {
    handleUpdate(operation, data, setTransactions);
  };

  // Handle auctions
  const handleUpdateAuction = (operation: TauriTypes.EventOperations, data: WFMarketTypes.Auction<string> | WFMarketTypes.Auction<string>[]) => {
    handleUpdate(operation, data, setAuctions);
  };

  // Handle update of statistics when transactions change
  useEffect(() => {
    if (!transactions) return;
    let statistics = api.statistic.convertFromTransaction(transactions);

    setStatistics(statistics);
  }, [transactions]);

  // Hook on tauri events from rust side
  useEffect(() => {
    OnTauriDataEvent<any>(TauriTypes.Events.UpdateTransaction, ({ data, operation }) => handleUpdateTransaction(operation, data));
    OnTauriDataEvent<any>(TauriTypes.Events.UpdateOrders, ({ data, operation }) => handleUpdateOrders(operation, data));
    OnTauriDataEvent<any>(TauriTypes.Events.UpdateAuction, ({ data, operation }) => handleUpdateAuction(operation, data));
    return () => {};
  }, []);

  return (
    <WarframeMarketContextContext.Provider value={{ transactions, statistics, orders, auctions }}>
      <ChatContextProvider>
        <WFMSocketContextProvider>{children}</WFMSocketContextProvider>
      </ChatContextProvider>
    </WarframeMarketContextContext.Provider>
  );
}
