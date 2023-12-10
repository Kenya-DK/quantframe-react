import { Grid } from "@mantine/core";
import { useAuthContext, useChatContext } from "../../contexts";
import { ChatListItem } from "./chatItem";
import { ChatRome } from "./chatRome";
import { SendSocketEvent } from "../../utils";
import { useEffect } from "react";
import api from "@api/index";

export default function ChatsPage() {
  const { chats } = useChatContext();
  const { user } = useAuthContext();
  const { aktive_chat } = useChatContext();
  useEffect(() => {
    return () => { SendSocketEvent("chats/SET_CHAT", undefined) }
  }, [])
  return (
    <Grid>
      <Grid.Col span={aktive_chat ? 3 : 12}>
        {chats?.map((chat: any) => (
          <ChatListItem exlude_user_names={[user?.ingame_name || ""]} chat={chat}
            onClick={(chat) => SendSocketEvent("chats/SET_CHAT", chat)}
            selected={aktive_chat?.id === chat.id}
            onDelete={async (id) => {
              await api.chat.delete(id);
              SendSocketEvent("chats/SET_CHAT", undefined);
            }}
          />
        ))}
      </Grid.Col>
      {aktive_chat && (
        <Grid.Col span={9}>
          <ChatRome chat={aktive_chat} />
        </Grid.Col>
      )}
    </Grid>
  );
}