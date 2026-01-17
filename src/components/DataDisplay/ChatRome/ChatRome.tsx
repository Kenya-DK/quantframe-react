import { Button, Collapse, Group, Paper, ScrollArea, Stack, Text, Textarea } from "@mantine/core";
import { TauriTypes, WFMarketTypes } from "$types/index";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faChevronLeft, faEllipsis } from "@fortawesome/free-solid-svg-icons";
import { useMutation, useQuery } from "@tanstack/react-query";
import api from "@api/index";
import { ChatMessage } from "../ChatMessage";
import { useEffect, useRef, useState } from "react";
import { useTranslateComponent } from "@hooks/useTranslate.hook";
import { Loading } from "@components/Shared/Loading";
import { useTauriEvent } from "@hooks/useTauriEvent.hook";
import { useAuthContext } from "@contexts/auth.context";
import { useHasAlert } from "../../../hooks/useHasAlert.hook";

export type ChatRomeProps = {
  disableChat?: boolean;
  chat: WFMarketTypes.ChatData;
  goBack: () => void;
};

export const ChatRome = ({ chat, goBack, disableChat }: ChatRomeProps) => {
  const { user } = useAuthContext();
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

  const handleOnMessage = (newMessage: WFMarketTypes.ChatMessage) => {
    if (newMessage.chat_id != chat.id) return;
    setMessages((msgs) => {
      let newMsgs = [...msgs];
      newMsgs.unshift(newMessage);
      return newMsgs;
    });
    if (atBottom) setTimeout(() => scrollToBottom(), 100);
  };

  const sendMessage = useMutation({
    mutationFn: (msg: string) => api.chat.send_message(chat.id, msg),
    onSuccess: async () => {
      setMsg("");
    },
    onError: (e) => {
      console.error(e);
    },
  });
  useTauriEvent(TauriTypes.Events.OnChatMessage, handleOnMessage, []);
  return (
    <Paper mt={25}>
      <Group justify="space-between" p={10} style={{ boxShadow: "0px 0px 10px 0px rgba(0,0,0,0.5)", position: "relative" }}>
        <Button leftSection={<FontAwesomeIcon icon={faChevronLeft} />} onClick={goBack}>
          {useTranslateButtons("back_label")}
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
        <ScrollArea.Autosize
          p={10}
          h={`calc(100vh - ${useHasAlert() ? "280px" : "260px"})`}
          scrollbarSize={1}
          onBottomReached={() => setAtBottom(true)}
          onScrollPositionChange={(position) => {
            if (atBottom && position.y > 100) setAtBottom(false);
          }}
          onTopReached={() => setShowCount((c) => c + defaultMsgLength)}
          viewportRef={viewport}
        >
          <Stack gap={3}>
            {filteredMessages.map((message) => (
              <ChatMessage key={message.id} msg={message} user={FindUser(message.message_from)} sender={message.message_from == user?.wfm_id} />
            ))}
          </Stack>
        </ScrollArea.Autosize>
        <Paper radius={0} display={"flex"} p={10} h={75}>
          <Textarea
            w={"90%"}
            disabled={disableChat}
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
          <Button
            disabled={msg.length > maxMsgLength || disableChat}
            color="blue"
            h={"100%"}
            ml={"md"}
            w={"10%"}
            onClick={() => sendMessage.mutateAsync(msg)}
          >
            {useTranslateButtons("send_label")}
          </Button>
        </Paper>
      </Stack>
    </Paper>
  );
};
