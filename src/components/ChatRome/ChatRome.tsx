import { Button, Collapse, Group, Paper, ScrollArea, Stack, Text, Textarea } from "@mantine/core";
import { WFMarketTypes } from "$types/index";
import classes from "./ChatRome.module.css";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faChevronLeft, faEllipsis } from "@fortawesome/free-solid-svg-icons";
import { useMutation, useQuery } from "@tanstack/react-query";
import api, { OnTauriEvent } from "@api/index";
import { Loading } from "../Loading";
import { ChatMessage } from "../ChatMessage";
import { useEffect, useRef, useState } from "react";
import { useTranslateComponent } from "@hooks/useTranslate.hook";
import { TauriTypes } from "$types";

export type ChatRomeProps = {
  chat: WFMarketTypes.ChatData;
  goBack: () => void;
};

export const ChatRome = ({ chat, goBack }: ChatRomeProps) => {
  const defaultMsgLength = 30;
  const maxMsgLength = 400;

  // State's
  const [messages, setMessages] = useState<WFMarketTypes.ChatMessage[]>([]);
  const [filteredMessages, setFilteredMessages] = useState<WFMarketTypes.ChatMessage[]>([]);
  const [showCount, setShowCount] = useState(defaultMsgLength);
  const [msg, setMsg] = useState<string>("");
  const [atBottom, setAtBottom] = useState(true);
  const [isOptionsOpen, setIsOptionsOpen] = useState(false);
  const viewport = useRef<HTMLDivElement>(null);

  // Fetch data from rust side
  const { isFetching, data, isError, error } = useQuery({
    queryKey: ["chat_messages", chat.id],
    queryFn: () => api.chat.getChatMessages(chat.id),
  });

  useEffect(() => {
    setShowCount(defaultMsgLength);
    if (!data) return;
    //Filter by data
    setMessages(data);
    setTimeout(() => scrollToBottom(), 100);
  }, [data]);

  useEffect(() => {
    console.log("Messages", messages);
    if (messages.length > 0) {
      setFilteredMessages(messages.slice(0, showCount).reverse());
    }
  }, [messages, showCount]);

  // Translate general
  const useTranslateChatRome = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateComponent(`chat_rome.${key}`, { ...context }, i18Key);
  const useTranslateFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateChatRome(`fields.${key}`, { ...context }, i18Key);
  const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateChatRome(`buttons.${key}`, { ...context }, i18Key);

  // Methods
  const scrollToBottom = () => viewport?.current!.scrollTo({ top: viewport.current!.scrollHeight, behavior: "smooth" });
  const FindUser = (id: string) => {
    return chat.chat_with.find((user) => user.id === id);
  };

  const handleNewMessage = (newMessage: WFMarketTypes.ChatMessage) => {
    console.log("New message", newMessage);
    if (newMessage.chat_id != chat.id) return;
    setMessages((msgs) => {
      let newMsgs = [...msgs];
      newMsgs.unshift(newMessage);
      return newMsgs;
    });
    if (atBottom) setTimeout(() => scrollToBottom(), 100);
  };

  // Hook on tauri events from rust side
  useEffect(() => {
    OnTauriEvent(TauriTypes.Events.ChatReceiveMessage, (data: WFMarketTypes.ChatMessage) => handleNewMessage(data));
    OnTauriEvent(TauriTypes.Events.ChatMessageSent, (data: WFMarketTypes.ChatMessageSent) => handleNewMessage(data.message));
    return () => {};
  }, []);

  const sendMessage = useMutation({
    mutationFn: (msg: string) => api.chat.send_message(chat.id, msg),
    onSuccess: async () => {
      setMsg("");
    },
    onError: (e) => {
      console.error(e);
    },
  });

  return (
    <Paper classNames={classes} mt={25}>
      <Group justify="space-between" p={10} style={{ boxShadow: "0px 0px 10px 0px rgba(0,0,0,0.5)", position: "relative" }}>
        <Button leftSection={<FontAwesomeIcon icon={faChevronLeft} />} onClick={goBack}>
          {" "}
          {useTranslateButtons("back.label")}
        </Button>
        <Text fw={700} c={"blue.7"}>
          {chat.chat_name}
        </Text>
        <Button leftSection={<FontAwesomeIcon icon={faEllipsis} />} onClick={() => setIsOptionsOpen((o) => !o)}>
          {useTranslateButtons("options.label")}
        </Button>
        <Collapse in={isOptionsOpen} style={{ position: "absolute", top: 56, right: 0, width: "100%", zIndex: 100, background: "blue" }}>
          <Paper p={10} radius={0} style={{ background: "blue" }}>
            <Text>{useTranslateButtons("options.delete")}</Text>
          </Paper>
        </Collapse>
      </Group>
      <Stack gap={0}>
        {isError && <Text c="red">Error: {JSON.stringify(error)}</Text>}
        {isFetching && <Loading />}
        <ScrollArea
          p={10}
          scrollbarSize={1}
          onBottomReached={() => setAtBottom(true)}
          onScrollPositionChange={(position) => {
            if (atBottom && position.y > 100) setAtBottom(false);
          }}
          style={{ height: "calc(100vh - 260px)" }}
          onTopReached={() => setShowCount((c) => c + defaultMsgLength)}
          viewportRef={viewport}
        >
          <Stack gap={3}>
            {filteredMessages.map((message) => (
              <ChatMessage key={message.id} msg={message} sender={FindUser(message.message_from) || undefined} />
            ))}
          </Stack>
        </ScrollArea>
        <Paper radius={0} display={"flex"} p={10} h={75}>
          <Textarea
            w={"90%"}
            value={msg}
            onChange={(e) => setMsg(e.currentTarget.value)}
            error={msg.length > maxMsgLength ? useTranslateFields("message.too_long") : undefined}
            onKeyDown={(e) => {
              if (e.key === "Enter" && !e.shiftKey) {
                e.preventDefault();
                sendMessage.mutateAsync(msg);
              }
            }}
            placeholder={useTranslateFields("message.placeholder")}
            maxRows={5}
          ></Textarea>
          <Button disabled={msg.length > maxMsgLength} color="blue" h={"100%"} ml={"md"} w={"10%"} onClick={() => sendMessage.mutateAsync(msg)}>
            {useTranslateButtons("send.label")}
          </Button>
        </Paper>
      </Stack>
    </Paper>
  );
};
