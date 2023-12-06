import { useEffect } from "react";
import { useForm } from "@mantine/form";
import { Accordion, Button, Checkbox, Collapse, Group, TextInput, Textarea } from "@mantine/core";
import { useTranslateModal } from "@hooks/index";
import { WhisperScraperSettings, Wfm, NotificationBase, NotificationDiscord } from "$types/index";

interface LiveScraperProps {
  settings: WhisperScraperSettings | undefined;
  tradable_items: Wfm.ItemDto[];
  updateSettings: (user: Partial<WhisperScraperSettings>) => void;
}


interface NotificationProps {
  i18Key: string;
  notifi: NotificationBase | NotificationDiscord;
  onChange: (key: string, event: string | boolean | string[]) => void;
}

const Notification = ({ i18Key, notifi: conversation, onChange }: NotificationProps) => {
  const useTranslateConversation = (key: string, context?: { [key: string]: any }) => useTranslateModal(`${key}`, { ...context })
  return (
    <>
      <Group grow mt={10}>
        <Checkbox
          label={useTranslateConversation(`${i18Key}.enable.title`)}
          description={useTranslateConversation(`${i18Key}.enable.description`)}
          checked={conversation.enable}
          onChange={(event) => onChange('enable', event.currentTarget.checked)}
        />
      </Group>
      <Collapse in={conversation.enable}>
        <Group grow mt={10}>
          <Group grow>
            <TextInput
              label={useTranslateConversation(`${i18Key}.title.title`)}
              description={useTranslateConversation(`${i18Key}.title.description`)}
              value={conversation.title}
              onChange={(event) => onChange('title', event.currentTarget.value)}
            />
          </Group>
        </Group>
        {((conversation as NotificationDiscord).webhook != undefined && (conversation as NotificationDiscord).user_ids) && (
          <Group grow mt={10}>
            <Group grow>
              <TextInput
                label={useTranslateConversation(`${i18Key}.webhook.title`)}
                description={useTranslateConversation(`${i18Key}.webhook.description`)}
                value={(conversation as NotificationDiscord).webhook}
                onChange={(event) => onChange('webhook', event.currentTarget.value)}
              />
            </Group>
            <Group grow>
              <TextInput
                label={useTranslateConversation(`${i18Key}.user_ids.title`)}
                description={useTranslateConversation(`${i18Key}.user_ids.description`)}
                value={(conversation as NotificationDiscord).user_ids.join(',')}
                onChange={(event) => onChange('user_ids', event.currentTarget.value.split(','))}
              />
            </Group>
          </Group>
        )}
        <Group grow mt={10}>
          <Textarea
            label={useTranslateConversation(`${i18Key}.content.title`)}
            description={useTranslateConversation(`${i18Key}.content.description`)}
            value={conversation.content}
            onChange={(event) => onChange('content', event.currentTarget.value)}
          />
        </Group>
      </Collapse>
    </>
  )
}


export function WhisperScraperPanel({ settings, updateSettings }: LiveScraperProps) {
  const roleForm = useForm({
    initialValues: {
      whisper_scraper: {
        on_new_conversation: {
          discord: {
            enable: true,
            title: "You have a new in-game conversation!",
            content: "From: [PLAYER_NAME]",
            webhook: "",
            user_ids: [] as string[],
          },
          system: {
            enable: true,
            title: "You have a new in-game conversation!",
            content: "From: [PLAYER_NAME]",
          }
        },
      },
    },
    validate: {},
  });

  useEffect(() => {
    if (!settings) return;
    roleForm.setFieldValue("whisper_scraper", settings);
  }, [settings]);

  const useTranslateSettingsModal = (key: string, context?: { [key: string]: any }) => useTranslateModal(`settings.panels.whisper_scraper.${key}`, { ...context })
  return (
    <form method="post" onSubmit={roleForm.onSubmit(async (data) => {
      updateSettings(data.whisper_scraper)
    })}>
      <Group grow>
        <Accordion defaultValue="accordion_general" w={"100%"}>
          <Accordion.Item value="accordion_general">
            <Accordion.Control>{useTranslateSettingsModal('accordion_general')}</Accordion.Control>
            <Accordion.Panel>
              <Notification
                i18Key="settings.panels.whisper_scraper.conversation.system"
                notifi={roleForm.values.whisper_scraper.on_new_conversation.system}
                onChange={(key, event) => roleForm.setFieldValue(`whisper_scraper.on_new_conversation.system.${key}`, event)}
              />
              <Notification
                i18Key="settings.panels.whisper_scraper.conversation.discord"
                notifi={roleForm.values.whisper_scraper.on_new_conversation.discord}
                onChange={(key, event) => roleForm.setFieldValue(`whisper_scraper.on_new_conversation.discord.${key}`, event)}
              />
            </Accordion.Panel>
          </Accordion.Item>
        </Accordion>
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