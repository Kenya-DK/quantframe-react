import { createContext, useContext, useEffect, useState } from "react";
import { WFMarketTypes } from "$types/index";
import { OnTauriDataEvent } from "../api";
import { TauriTypes } from "$types";

export type ChatContextProps = {
  chats: WFMarketTypes.ChatData[];
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
  const [chats, setChats] = useState<WFMarketTypes.ChatData[]>([]);

  const handleUpdate = <T extends WFMarketTypes.ChatData>(operation: TauriTypes.EventOperations, data: T | T[], setData: SetDataFunction<T[]>) => {
    switch (operation) {
      case TauriTypes.EventOperations.CREATE_OR_UPDATE:
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
      case TauriTypes.EventOperations.DELETE:
        setData((items) => items.filter((item) => item.id !== (data as T).id));
        break;
      case TauriTypes.EventOperations.SET:
        setData(data as T[]);
        break;
    }
  };

  // Hook on tauri events from rust side
  useEffect(() => {
    OnTauriDataEvent<any>(TauriTypes.Events.UpdateChats, ({ data, operation }) => handleUpdate(operation, data, setChats));
    OnTauriDataEvent<any>(TauriTypes.Events.ChatReceiveMessage, ({ data, operation }) => handleUpdate(operation, data, setChats));
    return () => {};
  }, []);

  return <ChatContext.Provider value={{ chats }}>{children}</ChatContext.Provider>;
}
