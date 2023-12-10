import { faChevronLeft, faEllipsisV } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
  Button,
  Group,
  Paper,
  useMantineTheme,
  Text,
  Menu,
} from "@mantine/core";
import { SendSocketEvent } from "@utils/index";
import { useTranslatePage } from "@hooks/index";
import api from "@api/index";

interface ChatMessageProps {
  title: string;
  id: string;
}
export const ChatNavBar = ({ title, id }: ChatMessageProps) => {
  const useTranslateChatNavnBar = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslatePage(`chats.navbar.${key}`, { ...context }, i18Key)
  const th = useMantineTheme();
  return (
    <>
      <Paper
        radius={0}
        sx={{ boxShadow: `0px 2px 0px 0px ${th.colors.gray[7]}`, position: "relative" }}
        p={10}
      >
        <Group
          position="apart"
          sx={{ height: "4vh" }}
          noWrap
        >
          <Button leftIcon={<FontAwesomeIcon icon={faChevronLeft} />} onClick={() => SendSocketEvent("chats/SET_CHAT", undefined)}>
            {useTranslateChatNavnBar("back")}
          </Button>
          <Text color={th.colors.blue[5]}>
            {title}
          </Text>
          <Menu shadow="md" width={200}>
            <Menu.Target>
              <Button rightIcon={<FontAwesomeIcon icon={faEllipsisV} />}>
                {useTranslateChatNavnBar("options")}
              </Button>
            </Menu.Target>
            <Menu.Dropdown>
              <Menu.Item color="red.7" onClick={() => api.chat.delete(id)}>{useTranslateChatNavnBar("delete")}</Menu.Item>
              <Menu.Item color="red.7">{useTranslateChatNavnBar("ignore")}</Menu.Item>
            </Menu.Dropdown>
          </Menu>
        </Group>
      </Paper>
    </>
  );
};

