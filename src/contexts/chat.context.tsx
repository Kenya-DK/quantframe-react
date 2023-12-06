import { createContext, useContext, useEffect, useState } from "react";
import { Wfm } from '$types/index';
import { OnTauriUpdateDataEvent } from "../utils";

type ChatContextProps = {
  chats: Wfm.ChatData[];
  unread_messages: number;

}

type ChatContextProviderProps = {
  children: React.ReactNode;
}

export const ChatContext = createContext<ChatContextProps>({
  chats: [],
  unread_messages: 0,
});

export const useChatContext = () => useContext(ChatContext);

export const ChatContextProvider = ({ children }: ChatContextProviderProps) => {
  const [chats, setChats] = useState<Wfm.ChatData[]>([]);
  const [unread_messages, setUnreadMessages] = useState<number>(0);

  useEffect(() => {
    setUnreadMessages(chats.reduce((acc, item) => acc + item.unread_count, 0));
    return () => { }
  }, [chats]);

  // Handle update, create, delete orders
  const handleUpdateItems = (operation: string, data: Wfm.ChatData | Wfm.ChatData[] | string) => {
    switch (operation) {
      case "CREATE_OR_UPDATE":
        {
          const order = data as Wfm.ChatData;
          setChats((stocks) => [...stocks.filter((item) => item.id !== order.id), order]);
        }
        break;
      case "DELETE":
        {
          const order = data as Wfm.ChatData;
          setChats((stocks) => [...stocks.filter((item) => item.id !== order.id)]);
        }
        break;
      case "SET":
        {
          const stocks = data as Wfm.ChatData[];
          setChats(stocks);
        }
        break;
    }
  }


  // Hook on tauri events from rust side
  useEffect(() => {
    OnTauriUpdateDataEvent<Wfm.ChatData>("ChatMessages", ({ data, operation }) => handleUpdateItems(operation, data));
    return () => { }
  }, []);

  return (
    <ChatContext.Provider value={{ chats, unread_messages }}>
      {children}
    </ChatContext.Provider>
  )
}