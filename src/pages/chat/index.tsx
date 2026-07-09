import { TauriTypes, WFMarketTypes } from "$types";
import api, { WFMThumbnail } from "@api/index";
import { ChatRome } from "@components/DataDisplay/ChatRome";
import { SearchField } from "@components/Forms/SearchField";
import { ActionWithTooltip } from "@components/Shared/ActionWithTooltip";
import { useAppContext } from "@contexts/app.context";
import { useAuthContext } from "@contexts/auth.context";
import { faRefresh, faTrash, faTrashCan } from "@fortawesome/free-solid-svg-icons";
import { useHasAlert } from "@hooks/useHasAlert.hook";
import { useTauriEvent } from "@hooks/useTauriEvent.hook";
import { useTranslatePages } from "@hooks/useTranslate.hook";
import { Avatar, Container, Divider, Grid, Group, Indicator, ScrollArea, SimpleGrid, Stack, Text, Tooltip } from "@mantine/core";
import { useLocalStorage } from "@mantine/hooks";
import { useEffect, useState } from "react";
import { PaginationFooter } from "../../components/Shared/PaginationFooter";
import { PreviewCard } from "../../components/Shared/PreviewCard";
import { TimerStamp } from "../../components/Shared/TimerStamp";
import classes from "./Chat.module.css";
import { useModals } from "./modals";
import { useMutations } from "./mutations";
import { useChatQueries } from "./queries";

export default function ChatPage() {
  // Contexts
  const { user } = useAuthContext();
  const { app_error } = useAppContext();
  // States
  const [queryData, setQueryData] = useLocalStorage<WFMarketTypes.WfmChatDataControllerGetListParams>({
    key: "chat_query_key",
    getInitialValueInEffect: false,
    defaultValue: { page: 1, limit: 10, sort_by: "last_update", sort_direction: "desc" },
  });

  // Translate general
  const useTranslateTabOrder = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`chat.${key}`, { ...context }, i18Key);
  const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabOrder(`buttons.${key}`, { ...context }, i18Key);
  const useTranslateErrors = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabOrder(`errors.${key}`, { ...context }, i18Key);
  const useTranslateSuccess = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabOrder(`success.${key}`, { ...context }, i18Key);
  const useTranslateBasePrompt = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTabOrder(`prompts.${key}`, { ...context }, i18Key);

  const [activeChat, setActiveChat] = useState<WFMarketTypes.ChatData | undefined>(undefined);
  // Queries
  const { paginationQuery, refetchQueries, updateChatData } = useChatQueries({ queryData });
  // Queries
  const { deleteMutation } = useMutations({ useTranslateSuccess, useTranslateErrors, refetchQueries, setLoadingRows: () => {} });
  // Queries
  const { OpenDeleteModal } = useModals({ deleteMutation, useTranslateBasePrompt });

  const handleOnMessage = (message: WFMarketTypes.ChatMessage) => {
    if (message.requirer_refresh) return refetchQueries();
    if (message.chat_id != (activeChat?.id || "")) updateChatData({ id: message.chat_id, messages: [message], unread_count: 1 });
  };

  useEffect(() => {
    if (!activeChat) api.chat.set_active(undefined);
    else updateChatData({ id: activeChat.id, unread_count: -1 });
    return () => {
      api.chat.set_active(undefined);
    };
  }, [activeChat]);

  const FindUser = (id: string) => {
    if (!activeChat) return undefined;
    return activeChat.chat_with.find((user) => user.id === id);
  };

  useTauriEvent(TauriTypes.Events.OnChatMessage, handleOnMessage, [activeChat]);
  return (
    <Container size={"100%"}>
      <Grid data-has-alert={useHasAlert()} className={`${classes.container}`}>
        <Grid.Col span={activeChat ? 4 : 12}>
          <SearchField
            value={queryData.query || ""}
            onChange={(text) => setQueryData((prev) => ({ ...prev, query: text }))}
            rightSectionWidth={63}
            rightSection={
              <Group gap={5}>
                <ActionWithTooltip
                  tooltip={useTranslateButtons("refresh_tooltip")}
                  icon={faRefresh}
                  color={"green.7"}
                  actionProps={{ size: "sm" }}
                  iconProps={{ size: "xs" }}
                  onClick={(e) => {
                    e.stopPropagation();
                  }}
                />
                <ActionWithTooltip
                  tooltip={useTranslateButtons("delete_all_tooltip")}
                  icon={faTrashCan}
                  color={"red.7"}
                  actionProps={{ size: "sm" }}
                  iconProps={{ size: "xs" }}
                  onClick={(e) => {
                    e.stopPropagation();
                  }}
                />
              </Group>
            }
          />
          <ScrollArea scrollbarSize={1} mt={"md"} data-has-alert={useHasAlert()} className={`${classes.chats}`}>
            <SimpleGrid cols={1} spacing="xs">
              {paginationQuery.data?.results?.map((chat, i) => (
                <PreviewCard
                  key={i}
                  value={chat}
                  onClick={() => setActiveChat(chat)}
                  data-selected={chat.id == activeChat?.id}
                  data-user-status={chat.chat_with[0].status}
                  data-color-mode="border"
                  headerLeft={
                    <Text size="mg" fw={700}>
                      {chat.chat_name}
                    </Text>
                  }
                  renderBody={() => (
                    <Grid>
                      <Grid.Col span={activeChat ? 12 : 6}>
                        <Group mt={"xs"}>
                          <Indicator inline label={chat.unread_count} size={16} color="red" disabled={chat.unread_count == 0}>
                            <Avatar.Group spacing="xs">
                              {chat?.chat_with
                                .filter((u) => u.id !== user?.wfm_id)
                                .map((user) => (
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
                      {!activeChat && (
                        <Grid.Col span={6}>
                          <Group justify="space-between">
                            <Stack gap={0} maw={"90%"}>
                              <Text size="sm" fw={700} truncate="end">
                                {FindUser(chat.messages[chat.messages.length - 1]?.message_from || "")?.ingame_name}
                              </Text>
                              {
                                <Text
                                  size="sm"
                                  lineClamp={1}
                                  dangerouslySetInnerHTML={{ __html: chat.messages[chat.messages.length - 1]?.message || "" }}
                                />
                              }
                            </Stack>
                            <ActionWithTooltip
                              tooltip={useTranslateButtons("delete_tooltip")}
                              color="red.7"
                              icon={faTrash}
                              onClick={(e) => {
                                e.stopPropagation();
                                OpenDeleteModal(chat.id);
                              }}
                            />
                          </Group>
                        </Grid.Col>
                      )}
                    </Grid>
                  )}
                  footerLeft={<TimerStamp text={useTranslateTabOrder("chat_item.last_update")} date={new Date(chat.last_update)} />}
                />
              ))}
            </SimpleGrid>
          </ScrollArea>
          <Divider mt={"md"} />
          {!activeChat && (
            <PaginationFooter
              page={queryData.page}
              limit={queryData.limit || 2}
              total={paginationQuery.data?.total || 0}
              onPageChange={(page) => setQueryData((prev) => ({ ...prev, page }))}
              onLimitChange={(limit) => setQueryData((prev) => ({ ...prev, page: 1, limit }))}
            />
          )}
        </Grid.Col>
        {activeChat && (
          <Grid.Col span={8}>
            <ChatRome
              chat={activeChat}
              goBack={() => setActiveChat(undefined)}
              disableChat={app_error && app_error.hasOperation("Chat:Disconnected")}
            />
          </Grid.Col>
        )}
      </Grid>
    </Container>
  );
}
