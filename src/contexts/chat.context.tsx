import { createContext, useContext, useEffect, useState } from "react";
import { Wfm } from "$types/index";
import wfmSocket from "../models/wfmSocket";
import api, { OnTauriDataEvent, SendTauriEvent } from "../api";
import { QfSocketEvent, QfSocketEventOperation } from "../api/types";

export type ChatContextProps = {
  chats: Wfm.ChatData[];
};

export type ChatContextProviderProps = {
  children: React.ReactNode;
};
type SetDataFunction<T> = React.Dispatch<React.SetStateAction<T>>;

export const ChatContext = createContext<ChatContextProps>({
  chats: [],
});

export const useChatContext = () => useContext(ChatContext);

export function ChatContextProvider({ children }: ChatContextProviderProps) {
  const [chats, setChats] = useState<Wfm.ChatData[]>([]);

  const handleUpdate = <T extends Wfm.ChatData>(operation: QfSocketEventOperation, data: T | T[], setData: SetDataFunction<T[]>) => {
    switch (operation) {
      case QfSocketEventOperation.CREATE_OR_UPDATE:
        const chat = data as T;
        setData((items) => {
          // Check if the item already exists in the list
          let itemFound = items.find((item) => item.id == chat.id);
          if (itemFound) {
            let originalUnreadCount = itemFound.unread_count;
            let unReadCount = chat.unread_count;
            itemFound = { ...itemFound, ...data } as T;
            if (unReadCount == 0) unReadCount = itemFound.unread_count;
            else if (unReadCount > 0) unReadCount = itemFound.unread_count + originalUnreadCount;
            else if (unReadCount < 0) unReadCount = itemFound.unread_count - originalUnreadCount;
            itemFound.unread_count = unReadCount;

            return items.map((item) => (item.id == itemFound?.id ? itemFound : item));
          } else return [data as T, ...items];
        });
        break;
      case QfSocketEventOperation.DELETE:
        setData((items) => items.filter((item) => item.id !== (data as T).id));
        break;
      case QfSocketEventOperation.SET:
        setData(data as T[]);
        break;
    }
  };

  // Hook on tauri events from rust side
  useEffect(() => {
    OnTauriDataEvent<any>(QfSocketEvent.UpdateChats, ({ data, operation }) => handleUpdate(operation, data, setChats));
    wfmSocket.on("chats/NEW_MESSAGE", async (data: Wfm.ChatMessage) => api.chat.on_message(data));
    wfmSocket.on("chats/MESSAGE_SENT", async (data: Wfm.ChatMessageSent) => SendTauriEvent(QfSocketEvent.ChatMessageSent, data));
    return () => {};
  }, []);

  return <ChatContext.Provider value={{ chats }}>{children}</ChatContext.Provider>;
}
