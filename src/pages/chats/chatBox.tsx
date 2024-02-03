import {
  Paper,
  useMantineTheme,
  Textarea,
  Button,
} from "@mantine/core";
import { useState } from "react";
import { useAuthContext, useSocketContextContext } from "../../contexts";
import { SendSocketEvent } from "../../utils";
import { useTranslatePage } from "../../hooks";

interface ChatBoxProps {
  // Exlude user names from chat name
  id: string;
}
export const ChatBox = ({ id }: ChatBoxProps) => {
  const useTranslateChatBox = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslatePage(`chats.msgbox.${key}`, { ...context }, i18Key)
  const { socket } = useSocketContextContext();
  const { user } = useAuthContext();
  const th = useMantineTheme();
  const [msg, setMsg] = useState<string>("");
  const maxMsgLength = 400;

  const SendMsg = () => {
    const temp_id = Math.random().toString(36).substring(7);
    socket?.send(JSON.stringify({
      type: "@WS/chats/SEND_MESSAGE",
      payload: {
        chat_id: id,
        temp_id: temp_id,
        message: msg,
      }
    }));
    const chat = {
      chat_id: id,
      message: `<p>${msg}<p/>`,
      raw_message: msg,
      message_from: user?.id || "",
      send_date: new Date().toISOString(),
      id: temp_id
    }
    SendSocketEvent("chats/NEW_MESSAGE", chat);
    setMsg("");
  }

  return (
    <>
      <Paper
        radius={0}
        display={"flex"}
        sx={{ boxShadow: `0px 0px 2px 0px ${th.colors.gray[7]}`, position: "relative" }}
        p={10}
      >
        <Textarea
          w={"100%"}
          value={msg}
          onChange={(e) => setMsg(e.currentTarget.value)}
          error={msg.length > maxMsgLength ? useTranslateChatBox("error.msg_to_long") : undefined}
          onKeyDown={(e) => {
            if (e.key === "Enter" && !e.shiftKey) {
              e.preventDefault();
              SendMsg();
            }
          }}
          placeholder={useTranslateChatBox("placeholder")}
          maxRows={5}
        >
        </Textarea>
        <Button disabled={msg.length > maxMsgLength} color="blue" h={"100%"} ml={10} onClick={SendMsg}>
          {useTranslateChatBox("send")}
        </Button>
      </Paper>
    </>
  );
};

