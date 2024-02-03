import { useEffect } from "react";
import { useForm } from "@mantine/form";
import { Button, Card, Collapse, Group, TextInput, Textarea, Text, Tooltip, ActionIcon, SimpleGrid, Divider } from "@mantine/core";
import { useTranslateModal } from "@hooks/index";
import { Notifications, Wfm, NotificationBase } from "$types/index";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faBell } from "@fortawesome/free-solid-svg-icons";
import { faDiscord } from "@fortawesome/free-brands-svg-icons";

interface LiveScraperProps {
  settings: Notifications | undefined;
  tradable_items: Wfm.ItemDto[];
  updateSettings: (user: Partial<Notifications>) => void;
}


interface NotificationProps {
  i18Key: string;
  notifi: NotificationBase;
  onChange: (key: string, event: string | boolean | string[]) => void;
}

const Notification = ({ i18Key, notifi: conversation, onChange }: NotificationProps) => {
  const useTranslateConversation = (key: string, context?: { [key: string]: any }) => useTranslateModal(`${i18Key}.${key}`, { ...context })
  const useTranslateFields = (key: string, context?: { [key: string]: any }) => useTranslateConversation(`fields.${key}`, { ...context })
  return (
    <Card radius="md">
      <Group position="apart" mb="xs">
        <Text weight={500}>{useTranslateConversation("title")}</Text>
        <Group>
          <Tooltip position="top" label={useTranslateConversation('tooltip.system')}>
            <ActionIcon color={conversation.system_notify ? "green.7" : "blue.7"} variant="filled" onClick={() => onChange('system_notify', !conversation.system_notify)} >
              <FontAwesomeIcon icon={faBell} />
            </ActionIcon>
          </Tooltip>
          <Tooltip position="top" label={useTranslateConversation('tooltip.discord')}>
            <ActionIcon color={conversation.discord_notify ? "green.7" : "blue.7"} variant="filled" onClick={() => onChange('discord_notify', !conversation.discord_notify)} >
              <FontAwesomeIcon icon={faDiscord as any} />
            </ActionIcon>
          </Tooltip>
        </Group>
      </Group>
      <Divider mb={5} />
      <Collapse in={conversation.discord_notify || conversation.system_notify}>
        <Group grow >
          <Group grow>
            <TextInput
              label={useTranslateFields(`title.label`)}
              description={useTranslateFields(`title.description`)}
              value={conversation.title}
              onChange={(event) => onChange('title', event.currentTarget.value)}
            />
          </Group>
        </Group>
        <Group grow mt={10}>
          <Textarea
            label={useTranslateFields(`content.label`)}
            description={useTranslateFields(`content.description`)}
            value={conversation.content}
            onChange={(event) => onChange('content', event.currentTarget.value)}
          />
        </Group>
        {conversation.discord_notify && (
          <Group grow mt={10}>
            <Group grow>
              <TextInput
                label={useTranslateFields(`webhook.label`)}
                description={useTranslateFields(`webhook.description`)}
                value={conversation.webhook}
                onChange={(event) => onChange('webhook', event.currentTarget.value)}
              />
            </Group>
            <Group grow>
              <TextInput
                label={useTranslateFields(`user_ids.label`)}
                description={useTranslateFields(`user_ids.description`)}
                value={conversation.user_ids.join(',')}
                onChange={(event) => onChange('user_ids', event.currentTarget.value.split(','))}
              />
            </Group>
          </Group>
        )}
      </Collapse>
      {/* {JSON.stringify(conversation)} */}
    </Card>
  )
}


export function NotificationsPanel({ settings, updateSettings }: LiveScraperProps) {
  const roleForm = useForm({
    initialValues: {
      notifications: {
        on_new_conversation: {
          system_notify: true,
          discord_notify: false,
          title: "You have a new in-game conversation!",
          content: "From: <PLAYER_NAME>",
          webhook: "",
          user_ids: [] as string[],
        },
        on_wfm_chat_message: {
          system_notify: true,
          discord_notify: false,
          title: "You have a new WFM chat message!",
          content: "<WFM_MESSAGE>",
          webhook: "",
          user_ids: [] as string[],
        },
      },
    },
    validate: {},
  });

  useEffect(() => {
    if (!settings) return;
    roleForm.setFieldValue("notifications", settings);
  }, [settings]);

  const useTranslateSettingsModal = (key: string, context?: { [key: string]: any }) => useTranslateModal(`settings.panels.notifications.${key}`, { ...context })
  return (
    <form method="post" onSubmit={roleForm.onSubmit(async (data) => {
      updateSettings(data.notifications)
    })}>
      <Group grow>
        <SimpleGrid
          cols={3}
          spacing="lg"
          breakpoints={[
            { maxWidth: '80rem', cols: 3, spacing: 'lg' },
            { maxWidth: '62rem', cols: 3, spacing: 'md' },
            { maxWidth: '48rem', cols: 2, spacing: 'sm' },
            { maxWidth: '36rem', cols: 1, spacing: 'sm' },
          ]}
        >
          <Notification
            i18Key="settings.panels.notifications.on_ingame_new_conversation"
            notifi={roleForm.values.notifications.on_new_conversation}
            onChange={(key, event) => roleForm.setFieldValue(`notifications.on_new_conversation.${key}`, event)}
          />
          <Notification
            i18Key="settings.panels.notifications.on_wfm_chat_message"
            notifi={roleForm.values.notifications.on_wfm_chat_message}
            onChange={(key, event) => roleForm.setFieldValue(`notifications.on_wfm_chat_message.${key}`, event)}
          />
        </SimpleGrid>
      </Group>
      <Group position="right" mt={10} sx={{
        position: "absolute",
        bottom: 0,
        right: 0,
      }}>
        <Button type="submit" variant="light" color="blue">
          {useTranslateSettingsModal('save')}
        </Button>
      </Group>
    </form>
  );
}