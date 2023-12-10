import { Paper, ScrollArea, Stack } from "@mantine/core";
import api from "@api/index";
import { useQuery } from "@tanstack/react-query";
import ChatMessage from "./chatMessage";
import { Wfm } from "../../types";
import { useEffect, useRef, useState } from "react";
import { useChatContext } from "../../contexts";
import { ChatNavBar } from "./chatNavBar";
import { Loading } from "../../components/loading";
import { ChatBox } from "./chatBox";

interface ChatContextProps {
  chat: Wfm.ChatData | undefined;
}
export const ChatRome = ({ chat }: ChatContextProps) => {
  const viewport = useRef<HTMLDivElement>(null);
  const [scrollPosition, onScrollPositionChange] = useState({ x: 0, y: 0 });
  const [isAtBottom, setIsAtBottom] = useState(true);
  const [messages, setMessages] = useState<Wfm.ChatMessage[]>([]);
  const { chats } = useChatContext();
  useEffect(() => {
    const foundChat = chats.find((item) => item.id === chat?.id);
    // Add missing messages
    if (foundChat) {
      setMessages((preMessages) => {
        const messages = [...preMessages];
        const foundMessages = foundChat.messages.filter((item) => !messages.find((msg) => msg.id === item.id));
        return [...messages, ...foundMessages];
      })
      if (isAtBottom)
        setTimeout(() => {
          scrollToBottom();
        }, 100);
    }
    return () => {

    }
  }, [chat?.id, chats]);
  const { isFetching } = useQuery({
    queryKey: ['user', chat?.id,],
    queryFn: () => api.chat.getChat(chat?.id || ''),
    enabled: !!chat?.id,
    onSuccess: (data) => {
      setMessages(data);
      setTimeout(() => {
        scrollToBottom();
      }, 100);
    }
  })

  const scrollToBottom = () => {
    if (!viewport.current) return;
    viewport.current.scrollTo({ top: viewport.current.scrollHeight, behavior: 'smooth' });
  }

  // const useTranslateOrdersPanel = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslatePage(`chats.item.${key}`, { ...context }, i18Key)
  const FindUser = (id: string) => {
    return chat?.chat_with.find((user) => user.id === id)
  }

  useEffect(() => {
    if (!viewport.current) return;
    const isAtBottom = viewport.current.scrollHeight - viewport.current.scrollTop === viewport.current.clientHeight;
    setIsAtBottom(isAtBottom);
    return () => { };
  }, [scrollPosition.y]);
  return (
    <Paper p={5}>
      <ChatNavBar title={chat?.chat_name || ""} id={chat?.id || ""} />
      <Stack sx={{ height: "80vh" }} p={0}>
        {isFetching && <Loading />}
        {!isFetching &&
          <ScrollArea p="xs" scrollbarSize={1} sx={{ height: "80vh" }} viewportRef={viewport} onScrollPositionChange={onScrollPositionChange}>
            <Stack>
              {messages?.sort((a, b) => new Date(a.send_date) > new Date(b.send_date) ? 1 : -1).map((msg, id) => {
                return (
                  <ChatMessage
                    key={id}
                    sender={FindUser(msg.message_from) || undefined}
                    msg={msg}
                  />
                );
              })}
            </Stack>
          </ScrollArea>
        }
        <ChatBox id={chat?.id || ""} />
      </Stack>
    </Paper>
  );
}
