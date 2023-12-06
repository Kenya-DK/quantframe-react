import { Container } from "@mantine/core";
import { useChatContext } from "../../contexts";


export default function ChatsPage() {
  const { chats } = useChatContext();
  return (
    <Container size={"100%"}>
      {chats.map((chat) => {
        return (<>
          {chat.chat_name}
        </>)
      })}
    </Container>
  );
}