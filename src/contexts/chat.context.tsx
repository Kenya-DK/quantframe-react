import { createContext, useContext, useEffect, useState } from "react";
import { Wfm } from '$types/index';
import { OnSocketEvent, OnTauriUpdateDataEvent } from "../utils";
import api from "@api/index";
import { useAppContext, useAuthContext } from ".";



type ChatContextProps = {
  chats: Wfm.ChatData[];
  unread_messages: number;
  aktive_chat: Wfm.ChatData | undefined;
}

type ChatContextProviderProps = {
  children: React.ReactNode;
}

export const ChatContext = createContext<ChatContextProps>({
  chats: [],
  unread_messages: 0,
  aktive_chat: undefined,
});

export const useChatContext = () => useContext(ChatContext);

export const ChatContextProvider = ({ children }: ChatContextProviderProps) => {
  const { user } = useAuthContext();
  const { settings } = useAppContext();
  const [state, setState] = useState<{
    aktive_chat: Wfm.ChatData | undefined,
    chats: Wfm.ChatData[]

  }>({
    chats: useChatContext().chats,
    aktive_chat: useChatContext().aktive_chat
  });
  const [unread_messages, setUnreadMessages] = useState<number>(0);

  useEffect(() => {
    setUnreadMessages(state.chats.reduce((acc, item) => acc + item.unread_count, 0));
    return () => { }
  }, [state]);

  // Handle update, create, delete orders
  const handleUpdateItems = (operation: string, data: Wfm.ChatData | Wfm.ChatData[] | string) => {
    switch (operation) {
      case "CREATE_OR_UPDATE":
        {
          const order = data as Wfm.ChatData;
          setState((preState) => {
            const newState = { ...preState };
            newState.chats = [...newState.chats.filter((item) => item.id !== order.id), order];
            return newState;
          })
        }
        break;
      case "DELETE":
        {
          const order = data as Wfm.ChatData;
          setState((preState) => {
            const newState = { ...preState };
            newState.chats = newState.chats.filter((item) => item.id !== order.id);
            return newState;
          })
        }
        break;
      case "SET":
        {
          const stocks = data as Wfm.ChatData[];
          setState((preState) => {
            const newState = { ...preState };
            newState.chats = stocks;
            return newState;
          })
        }
        break;
    }
  }

  const AddChatMessage = async (chat_id: string, message: Wfm.ChatMessage) => {
    await api.chat.on_new_wfm_message(message);
    setState((preState) => {
      const newState = { ...preState };
      const foundChat = newState.chats.find((item) => item.id === chat_id);
      if (!foundChat) {
        api.chat.refresh_chats(newState.chats.map((item) => item.id));
        return newState;
      } else {
        if (foundChat.id === newState.aktive_chat?.id)
          foundChat.unread_count = 0;
        else
          foundChat.unread_count = foundChat.unread_count + 1;
        foundChat.messages = [...foundChat.messages, message];
        newState.chats = [...newState.chats.filter((item) => item.id !== chat_id), foundChat];
        return newState;
      }
    })
  }


  // Hook on tauri events from rust side
  useEffect(() => {
    OnTauriUpdateDataEvent<Wfm.ChatData>("ChatMessages", ({ data, operation }) => handleUpdateItems(operation, data));
    OnSocketEvent("chats/NEW_MESSAGE", (data: Wfm.ChatMessage) => AddChatMessage(data.chat_id, data));
    OnSocketEvent("chats/SET_CHAT", (data: Wfm.ChatData | undefined) => {
      setState((preState) => {
        const newState = { ...preState };
        const foundChat = newState.chats.find((item) => item.id === data?.id);
        if (foundChat)
          foundChat.unread_count = 0;
        newState.aktive_chat = data;
        return newState;
      })
    });
    return () => { }
  }, []);

  return (
    <ChatContext.Provider value={{ chats: state.chats, unread_messages, aktive_chat: state.aktive_chat }}>
      {children}
    </ChatContext.Provider>
  )
}