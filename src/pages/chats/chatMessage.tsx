import {
  Alert,
  Avatar,
  Collapse,
  Group,
  Stack,
  Tooltip,
  Text
} from "@mantine/core";
import { Wfm } from "../../types";
import { useAuthContext } from "../../contexts";
import { wfmThumbnail } from "../../api";
import { useEffect, useState } from "react";
import dayjs from "dayjs";
import calendar from "dayjs/plugin/calendar";
dayjs.extend(calendar);
interface ChatMessageProps {
  // Exlude user names from chat name
  sender: Wfm.ChatWith | undefined;
  msg: Wfm.ChatMessage;
}
export const ChatMessage = ({ msg, sender }: ChatMessageProps) => {
  const { user } = useAuthContext();
  const message = user?.id == sender?.id ? "right" : "left";
  const [msgDate, setMsgDate] = useState("");
  const [opened, setOpen] = useState(false);
  useEffect(() => {
    const date = new Date(msg.send_date);
    // If message is older than 48 hours show date
    if (dayjs().diff(date, "h") > 48)
      setMsgDate(dayjs(date).format("MMMM D, YYYY h:mm A"));
    else 
      setMsgDate(dayjs(date).calendar());

    return () => { }
  }, [msg.send_date])
  return (
    <>
      <Group
        position={message}
        align="flex-end"
        noWrap
      >
        <Stack p={0} spacing={2} sx={{ maxWidth: "80%" }} align={message === "right" ? "flex-end" : "flex-start"}>
          <Group position={message} align="flex-end" spacing="xs">
            <Tooltip label={sender?.ingame_name} position="right">
              <Avatar
                src={wfmThumbnail(sender?.avatar || "")}
                radius="xl"
                hidden={message === "right" ? true : false}
              />
            </Tooltip>

            <Stack p={0} spacing={0} m={0}>
              <Group position={message} spacing={3} align="center" noWrap>
                <Alert
                  sx={{}}
                  radius="lg"
                  py={8}
                  onClick={() => {
                    setOpen((o) => !o);
                  }}
                >
                  {msg.raw_message}
                </Alert>
              </Group>
            </Stack>
          </Group>
          <Collapse in={opened} px="xs">
            <Text size="xs" align={message} color="dimmed">
              {msgDate}
            </Text>
          </Collapse>
        </Stack>
      </Group>
    </>
  );
};

export default ChatMessage;