import { Container, Grid, Group, ScrollArea, Text } from "@mantine/core";
import { useChatContext } from "@contexts/chat.context";
import { ChatListItem } from "@components/ChatListItem";
import { SearchField } from "@components/SearchField";
import { ActionWithTooltip } from "@components/ActionWithTooltip";
import { useEffect, useState } from "react";
import { useAuthContext } from "@contexts/auth.context";
import api from "@api/index";
import { useTranslatePages } from "@hooks/useTranslate.hook";
import { useMutation } from "@tanstack/react-query";
import { notifications } from "@mantine/notifications";
import { faRefresh, faTrashCan } from "@fortawesome/free-solid-svg-icons";
import { modals } from "@mantine/modals";
import { ChatRome } from "../../components/ChatRome";

export default function ChatsPage() {
  const { chats } = useChatContext();
  const { user } = useAuthContext();
  // Translate general
  const useTranslatePage = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`chats.${key}`, { ...context }, i18Key);
  const useTranslatePrompt = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePage(`prompts.${key}`, { ...context }, i18Key);
  const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePage(`buttons.${key}`, { ...context }, i18Key);
  const useTranslateErrors = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePage(`errors.${key}`, { ...context }, i18Key);
  const useTranslateSuccess = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePage(`success.${key}`, { ...context }, i18Key);

  const [search, setSearch] = useState<string>("");
  const [filteredChats, setFilteredChats] = useState<any[]>([]);
  const [activeChat, setActiveChat] = useState<any>(undefined);

  useEffect(() => {
    let filteredChats = chats;
    if (search) filteredChats = chats.filter((chat) => chat.chat_name.toLowerCase().includes(search.toLowerCase()));
    setFilteredChats(filteredChats);
  }, [chats, search]);

  const refreshChatsMutation = useMutation({
    mutationFn: () => api.chat.refresh(),
    onSuccess: async (u) => {
      notifications.show({
        title: useTranslateSuccess("refresh.title"),
        message: useTranslateSuccess("refresh.message", { count: u.length }),
        color: "green.7",
      });
    },
    onError: (e) => {
      console.error(e);
      notifications.show({ title: useTranslateErrors("refresh.title"), message: useTranslateErrors("refresh.message"), color: "red.7" });
    },
  });
  const deleteChatsMutation = useMutation({
    mutationFn: (id: string) => api.chat.delete(id),
    onSuccess: async (name) => {
      notifications.show({ title: useTranslateSuccess("delete.title"), message: useTranslateSuccess("delete.message", { name }), color: "green.7" });
    },
    onError: (e) => {
      console.error(e);
      notifications.show({ title: useTranslateErrors("delete.title"), message: useTranslateErrors("delete.message"), color: "red.7" });
    },
  });
  const deleteAllChatsMutation = useMutation({
    mutationFn: () => api.chat.deleteAll(),
    onSuccess: async (u) => {
      notifications.show({
        title: useTranslateSuccess("delete_all.title"),
        message: useTranslateSuccess("delete_all.message", { count: u }),
        color: "green.7",
      });
    },
    onError: (e) => {
      console.error(e);
      notifications.show({ title: useTranslateErrors("delete_all.title"), message: useTranslateErrors("delete_all.message"), color: "red.7" });
    },
  });

  const setChatActiveMutation = useMutation({
    mutationFn: (data: { id: string; unread_count: number }) => api.chat.set_active(data.id, data.unread_count),
  });

  return (
    <Container size={"100%"}>
      <Grid>
        <Grid.Col span={activeChat ? 4 : 12}>
          <SearchField
            value={search}
            onChange={(e) => setSearch(e)}
            rightSectionWidth={63}
            rightSection={
              <Group gap={5}>
                <ActionWithTooltip
                  tooltip={useTranslateButtons("refresh.tooltip")}
                  icon={faRefresh}
                  color={"green.7"}
                  actionProps={{ size: "sm" }}
                  iconProps={{ size: "xs" }}
                  loading={refreshChatsMutation.isPending || deleteChatsMutation.isPending || deleteAllChatsMutation.isPending}
                  onClick={(e) => {
                    e.stopPropagation();
                    refreshChatsMutation.mutateAsync();
                  }}
                />
                <ActionWithTooltip
                  tooltip={useTranslateButtons("delete_all.tooltip")}
                  icon={faTrashCan}
                  loading={refreshChatsMutation.isPending || deleteChatsMutation.isPending || deleteAllChatsMutation.isPending}
                  color={"red.7"}
                  actionProps={{ size: "sm" }}
                  iconProps={{ size: "xs" }}
                  onClick={(e) => {
                    e.stopPropagation();
                    modals.openConfirmModal({
                      title: useTranslatePrompt("delete_all.title"),
                      children: <Text size="sm">{useTranslatePrompt("delete_all.message")}</Text>,
                      labels: { confirm: useTranslatePrompt("delete_all.confirm"), cancel: useTranslatePrompt("delete.cancel") },
                      onConfirm: async () => await deleteAllChatsMutation.mutateAsync(),
                    });
                  }}
                />
              </Group>
            }
          />
          <ScrollArea scrollbarSize={1} style={{ height: "calc(100vh - 156px)" }} mt={"md"}>
            {filteredChats
              ?.sort((a, b) => (new Date(a.last_update) > new Date(b.last_update) ? -1 : 1))
              .map((chat) => (
                <ChatListItem
                  compact={!!activeChat}
                  key={chat.id}
                  exclude_user_names={[user?.ingame_name || ""]}
                  chat={chat}
                  onClick={(e) => {
                    setActiveChat(chat);
                    setChatActiveMutation.mutate({ id: chat.id, unread_count: e.unread_count });
                  }}
                  onDelete={() => {
                    modals.openConfirmModal({
                      title: useTranslatePrompt("delete.title"),
                      children: <Text size="sm">{useTranslatePrompt("delete.message", { name: chat.chat_name })}</Text>,
                      labels: { confirm: useTranslatePrompt("delete.confirm"), cancel: useTranslatePrompt("delete.cancel") },
                      onConfirm: async () => await deleteChatsMutation.mutateAsync(chat.id),
                    });
                  }}
                  selected={chat.id == activeChat?.id}
                />
              ))}
          </ScrollArea>
        </Grid.Col>
        {activeChat && (
          <Grid.Col span={8}>
            <ChatRome chat={activeChat} goBack={() => setActiveChat(undefined)} />
          </Grid.Col>
        )}
      </Grid>
    </Container>
  );
}
