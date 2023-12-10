import { Avatar, Divider, Text, Grid, Paper, Stack, Tooltip, Box, ActionIcon, useMantineTheme } from "@mantine/core";
import { useAuthContext, useChatContext } from "../../contexts";
import { useEffect, useState } from "react";
import { faTrashCan } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { Wfm } from "$types/index";
import { wfmThumbnail } from "@api/index";
import { LabelTimeBage } from "@components/labelTimeBage";
import { getUserStatusColor } from "../../utils";
import { TextColor } from "../../components/textColor";
import { useTranslatePage } from "../../hooks";

interface ChatListItemProps {
  // Exlude user names from chat name
  exlude_user_names?: string[];
  chat: Wfm.ChatData;
  selected?: boolean;
  onClick?: (chat: Wfm.ChatData) => void;
  onDelete?: (id: string) => void;
}
export const ChatListItem = ({ selected, onClick, onDelete, exlude_user_names, chat }: ChatListItemProps) => {
  const [lastMessage, setLastMessage] = useState<Wfm.ChatMessage | undefined>(undefined);
  const [wfmUsers, setWfmUsers] = useState<Wfm.ChatWith[]>([]);
  const useTranslateChatItem = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslatePage(`chats.item.${key}`, { ...context }, i18Key)
  const th = useMantineTheme();

  useEffect(() => {
    const lastMessage = chat.messages[chat.messages.length - 1];
    setLastMessage(lastMessage);
  }, [chat.messages])
  useEffect(() => {
    if (!exlude_user_names) return setWfmUsers(chat.chat_with);
    const users = chat.chat_with.filter(x => (exlude_user_names || []).some(y => y != x.ingame_name));
    setWfmUsers(users);
  }, [chat.chat_with])

  const FindUser = (id: string) => {
    return chat.chat_with.find((user) => user.id === id)
  }

  return (
    <Paper mb={5} onClick={() => {
      if (onClick) onClick(chat);
    }} p={10} sx={{
      boxShadow: `inset 4px 0 0 0 ${getUserStatusColor(chat.chat_with[0].status)}`,
      borderRight: `${selected ? `4px solid ${th.colors.green[7]}` : "none"}`,
      '&:hover': {
        boxShadow: `inset 4px 0 0 0 ${th.colors[th.primaryColor][7]}`,
      },
    }}

    >
      <Stack spacing={0}>
        <Box p={0} m={0} sx={{ lineHeight: "1" }} >
          <Text component="span" size="lg" weight={700} >
            {chat.chat_name}
          </Text>
          {chat.unread_count > 0 && (
            <TextColor size={"lg"} sx={{ float: "inline-end" }} color="gray.6" i18nKey={useTranslateChatItem("un_read_messages", undefined, true)} values={{ count: chat.unread_count }} />
          )}
        </Box>
        <Divider />
        <Grid mt={5} mb={5}>
          <Grid.Col sm={4} md={4} lg={2.5}>
            <Avatar.Group spacing="xs" ml={10}>
              {wfmUsers.map((user) => (
                <Tooltip key={user.id} label={user.ingame_name} position="top" >
                  <Avatar sx={{
                    border: `2px solid ${getUserStatusColor(user.status)}`,
                  }} radius={50} src={wfmThumbnail(user.avatar)} />
                </Tooltip>
              ))}
            </Avatar.Group>
          </Grid.Col>
          <Grid.Col sm={9} md={9} lg={8.6} sx={{ display: "flex", alignItems: "center", justifyContent: "flex-end" }}>
            <Stack spacing={0}>
              <Text size="sm" weight={700}>
                {FindUser(lastMessage?.message_from || "")?.ingame_name}
              </Text>
              <Text size="sm" >
                {lastMessage?.raw_message}
              </Text>
            </Stack>

          </Grid.Col>
        </Grid>

        <Divider />
        <Box ml={15} mt={5}>
          <LabelTimeBage text="Last update " date={new Date(chat.last_update)} />
          <Tooltip label={useTranslateChatItem("delete")}>
            <ActionIcon color="red.7" sx={{ float: "right" }} onClick={async (e) => {
              e.stopPropagation();
              if (onDelete) onDelete(chat.id);
            }} >
              <FontAwesomeIcon icon={faTrashCan} />
            </ActionIcon>
          </Tooltip>
        </Box>
      </Stack>
    </Paper>
  );
}