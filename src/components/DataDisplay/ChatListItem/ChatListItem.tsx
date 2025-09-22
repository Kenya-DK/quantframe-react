import { Paper, Divider, Group, Avatar, Tooltip, Text, Grid, Indicator, Stack } from "@mantine/core";
import { WFMarketTypes } from "$types/index";
import classes from "./ChatListItem.module.css";
import { useEffect, useState } from "react";
import { WFMThumbnail } from "@api/index";
import { faTrash } from "@fortawesome/free-solid-svg-icons";
import { useTranslateComponent } from "@hooks/useTranslate.hook";
import { ActionWithTooltip } from "@components/Shared/ActionWithTooltip";
import { TimerStamp } from "@components/Shared/TimerStamp";

export type ChatListItemProps = {
  chat: WFMarketTypes.ChatData;
  exclude_user_names?: string[];
  selected?: boolean;
  onClick?: (chat: WFMarketTypes.ChatData) => void;
  onDelete?: (id: string) => void;
  compact?: boolean;
};

export const ChatListItem = ({ onClick, compact, onDelete, selected, exclude_user_names, chat }: ChatListItemProps) => {
  const [lastMessage, setLastMessage] = useState<WFMarketTypes.ChatMessage | undefined>(undefined);
  const [wfmUsers, setWfmUsers] = useState<WFMarketTypes.User[]>([]);
  const useTranslateChatItem = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateComponent(`chat_item.${key}`, { ...context }, i18Key);

  useEffect(() => {
    const lastMessage = chat.messages[chat.messages.length - 1];
    setLastMessage(lastMessage);
  }, [chat.messages]);

  useEffect(() => {
    if (!exclude_user_names) return setWfmUsers(chat.chat_with);
    const users = chat.chat_with.filter((x) => (exclude_user_names || []).some((y) => y != x.ingame_name));
    setWfmUsers(users);
  }, [chat.chat_with]);

  const FindUser = (id: string) => {
    return chat.chat_with.find((user) => user.id === id);
  };
  return (
    <Paper
      classNames={classes}
      mb={5}
      onClick={() => {
        if (onClick) onClick(chat);
      }}
      data-user-status={chat.chat_with[0].status}
      data-color-mode="box-shadow"
      data-selected={selected}
      p={10}
    >
      <Grid>
        <Grid.Col span={compact ? 12 : 6}>
          <Group mt={"xs"}>
            <Indicator inline label={chat.unread_count} size={16} color="red" disabled={chat.unread_count == 0}>
              <Avatar.Group spacing="xs">
                {wfmUsers.map((user) => (
                  <Tooltip key={user.id} label={user.ingame_name} position="top">
                    <Avatar styles={{}} radius={50} src={WFMThumbnail(user.avatar || "")} />
                  </Tooltip>
                ))}
              </Avatar.Group>
            </Indicator>
            <Text size="lg" fw={700} c={"blue.7"}>
              {chat.chat_name}
            </Text>
          </Group>
        </Grid.Col>
        {!compact && (
          <Grid.Col span={6}>
            <Group justify="space-between">
              <Stack gap={0} maw={"90%"}>
                <Text size="sm" fw={700} truncate="end">
                  {FindUser(lastMessage?.message_from || "")?.ingame_name}
                </Text>
                {<Text size="sm" lineClamp={1} dangerouslySetInnerHTML={{ __html: lastMessage?.message || "" }} />}
              </Stack>
              <ActionWithTooltip
                tooltip={useTranslateChatItem("delete_tooltip")}
                color="red.7"
                icon={faTrash}
                onClick={(e) => {
                  e.stopPropagation();
                  if (onDelete) onDelete(chat.id);
                }}
              />
            </Group>
          </Grid.Col>
        )}
      </Grid>
      <Divider mb={"xs"} mt={"xs"} />
      <Group justify="space-between">
        <TimerStamp text={useTranslateChatItem("last_update")} date={new Date(chat.last_update)} />
      </Group>
    </Paper>
  );
};
