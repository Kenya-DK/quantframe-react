import { Container, Grid, Group, ScrollArea } from "@mantine/core";
import { useChatQueries } from "./queries";
import { useMutations } from "./mutations";
import { useModals } from "./modals";
import { TauriTypes, WFMarketTypes } from "$types";
import { useLocalStorage } from "@mantine/hooks";
import { ChatListItem } from "@components/DataDisplay/ChatListItem";
import { useAuthContext } from "@contexts/auth.context";
import { ChatRome } from "@components/DataDisplay/ChatRome";
import { useEffect, useState } from "react";
import api from "@api/index";
import { ActionWithTooltip } from "@components/Shared/ActionWithTooltip";
import { SearchField } from "@components/Forms/SearchField";
import { faRefresh, faTrashCan } from "@fortawesome/free-solid-svg-icons";
import { useTranslatePages } from "@hooks/useTranslate.hook";
import { useTauriEvent } from "@hooks/useTauriEvent.hook";
import classes from "./Chat.module.css";
import { useHasAlert } from "@hooks/useHasAlert.hook";

export default function ChatPage() {
  // Contexts
  const { user } = useAuthContext();
  // States
  const [queryData, setQueryData] = useLocalStorage<WFMarketTypes.WfmChatDataControllerGetListParams>({
    key: "chat_query_key",
    getInitialValueInEffect: false,
    defaultValue: { page: 1, limit: -1, sort_by: "last_update", sort_direction: "desc" },
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
            {paginationQuery.data?.results?.map((chat) => (
              <ChatListItem
                compact={!!activeChat}
                key={chat.id}
                exclude_user_names={[user?.wfm_username || ""]}
                chat={chat}
                onClick={() => setActiveChat(chat)}
                onDelete={() => OpenDeleteModal(chat.id)}
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
