import { createContext, useContext, useEffect, useState } from "react";
import { Wfm } from '$types/index';
import { OnSocketEvent, OnTauriUpdateDataEvent } from "../utils";
import api from "@api/index";



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

  const AddChatMessage = (chat_id: string, message: Wfm.ChatMessage) => {
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
    // setInterval(() => {
    //   const test_chat = Math.random().toString(36).substring(7);
    //   const chat = {
    //     "chat_id": "656b2ed385339a17bf0fc118",
    //     "message": `<p>Test message ${test_chat}<p/>`,
    //     "raw_message": `Test message ${test_chat}`,
    //     "message_from": "61c96830493dc90e94c8dfde",
    //     "send_date": new Date().toISOString(),
    //     "id": Math.random().toString(36).substring(7)
    //   }
    //   SendSocketEvent("chats/NEW_MESSAGE", chat);
    // }, 1000);
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