import { Alert, Avatar, Collapse, Group, Stack, Text, Tooltip } from "@mantine/core";
import { WFMarketTypes } from "$types/index";
import { WFMThumbnail } from "@api/index";
import classes from "./ChatMessage.module.css";
import { useEffect, useState } from "react";
import dayjs from "dayjs";
import calendar from "dayjs/plugin/calendar";
dayjs.extend(calendar);

export type ChatMessageProps = {
  user: WFMarketTypes.User | undefined;
  sender: boolean;
  msg: WFMarketTypes.ChatMessage;
};

export const ChatMessage = ({ user, msg, sender }: ChatMessageProps) => {
  const position = sender ? "right" : "left";
  const [msgDate, setMsgDate] = useState("");
  const [opened, setOpen] = useState(false);
  useEffect(() => {
    const date = new Date(msg.send_date);
    // If message is older than 48 hours show date
    if (dayjs().diff(date, "h") > 48) setMsgDate(dayjs(date).format("MMMM D, YYYY h:mm A"));
    else setMsgDate(dayjs(date).calendar());

    return () => {};
  }, [msg.send_date]);
  return (
    <Group align="flex-end" style={{ width: "100%" }} data-position={position} classNames={classes}>
      <Stack p={0} gap={2} style={{ maxWidth: "80%" }} align={position === "right" ? "flex-end" : "flex-start"}>
        <Group data-position={position} align="flex-end" gap="xs">
          <Tooltip label={user?.ingame_name} position="right">
            <Avatar src={WFMThumbnail(user?.avatar || "")} radius="xl" hidden={position === "right" ? true : false} />
          </Tooltip>

          <Stack p={0} gap={0} m={0}>
            <Group data-position={position} gap={3} align="center">
              <Alert
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
          <Text size="xs" c="dimmed">
            {msgDate}
          </Text>
        </Collapse>
      </Stack>
    </Group>
  );
};
