import { createContext, useContext, useEffect, useState } from "react";
import { Wfm } from '$types/index';

export type ChatContextProps = {
  chats: Wfm.ChatData[];
  unread_messages: number;
  active_chat: Wfm.ChatData | undefined;
}

export type ChatContextProviderProps = {
  children: React.ReactNode;
}

export const ChatContext = createContext<ChatContextProps>({
  chats: [],
  unread_messages: 0,
  active_chat: undefined,
});

export const useChatContext = () => useContext(ChatContext);

export function ChatContextProvider({ children }: ChatContextProviderProps) {
  const [state] = useState<{
    active_chat: Wfm.ChatData | undefined,
    chats: Wfm.ChatData[]

  }>({
    chats: useChatContext().chats,
    active_chat: useChatContext().active_chat
  });
  const [unread_messages, setUnreadMessages] = useState<number>(0);

  useEffect(() => {
    setUnreadMessages(state.chats.reduce((acc, item) => acc + item.unread_count, 0));
    return () => { }
  }, [state]);

  return (
    <ChatContext.Provider value={{ chats: state.chats, unread_messages, active_chat: state.active_chat }}>
      {children}
    </ChatContext.Provider>
  )
}