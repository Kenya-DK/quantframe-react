import { createContext, useContext, useEffect, useState } from "react";
import { Wfm } from '$types/index';
import { OnSocketEvent, OnTauriUpdateDataEvent } from "../utils";

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


  const AddChatMessage = (chat_id: string, message: Wfm.ChatMessage) => {

    setChats((chats) => {
      const chat = chats.find((item) => item.id === chat_id);
      if (!chat) return chats;
      const new_chat = { ...chat };
      new_chat.unread_count = new_chat.unread_count + 1;
      new_chat.messages = [...new_chat.messages, message];
      return [...chats.filter((item) => item.id !== chat_id), new_chat];
    })
  }


  // Hook on tauri events from rust side
  useEffect(() => {
    OnTauriUpdateDataEvent<Wfm.ChatData>("ChatMessages", ({ data, operation }) => handleUpdateItems(operation, data));
    OnSocketEvent("chats/NEW_MESSAGE", (data: Wfm.ChatMessage) => {
      console.log("NEW_MESSAGE", data);

      AddChatMessage(data.chat_id, data);
    });
    return () => { }
  }, []);

  return (
    <ChatContext.Provider value={{ chats, unread_messages }}>
      {children}
    </ChatContext.Provider>
  )
}