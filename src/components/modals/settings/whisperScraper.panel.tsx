import { useEffect } from "react";
import { useForm } from "@mantine/form";
import { Accordion, ActionIcon, Button, Divider, Group, TextInput, Tooltip } from "@mantine/core";
import { useTranslateModal } from "@hooks/index";
import { WhisperScraperSettings, Wfm } from "$types/index";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faBell, faBellSlash } from "@fortawesome/free-solid-svg-icons";

interface LiveScraperProps {
  settings: WhisperScraperSettings | undefined;
  tradable_items: Wfm.ItemDto[];
  updateSettings: (user: Partial<WhisperScraperSettings>) => void;
}

export function WhisperScraperPanel({ settings, updateSettings }: LiveScraperProps) {
  const roleForm = useForm({
    initialValues: {
      ping_on_notif: true,
      webhook: ""
    },
    validate: {},
  });

  useEffect(() => {
    if (!settings) return;
    roleForm.setFieldValue("webhook", settings.webhook);
    roleForm.setFieldValue("ping_on_notif", settings.ping_on_notif);
  }, [settings]);

  const useTranslateSettingsModal = (key: string, context?: { [key: string]: any }) => useTranslateModal(`settings.panels.live_trading.${key}`, { ...context })
  return (
    <form method="post" onSubmit={roleForm.onSubmit(async (data) => {
      const settingsData = {
        ping_on_notif: data.ping_on_notif,
        webhook: data.webhook
      }

      updateSettings(settingsData)
    })}>
      <Group grow>
        <Group grow>
          <Accordion defaultValue="accordion_general" w={"100%"}>
            <Accordion.Item value="accordion_general">
              <Accordion.Control>{useTranslateSettingsModal('accordion_general')}</Accordion.Control>
              <Accordion.Panel>
                <Group grow mt={10}>
                  <Group grow>
                    <TextInput
                      label={useTranslateSettingsModal('webhook')}
                      value={roleForm.values.webhook}
                      description={useTranslateSettingsModal('webhook_description')}
                      onChange={(event) => roleForm.setFieldValue('webhook', event.currentTarget.value)}
                      error={roleForm.errors.webhook && 'Invalid Webhook'}
                      rightSectionWidth={45}
                      rightSection={
                        <Group spacing={"5px"} mr={0}>
                          <Divider orientation="vertical" />
                          <Tooltip label={useTranslateSettingsModal('ping_on_notif_description')}>
                            <ActionIcon color={roleForm.values.ping_on_notif ? "green.7" : "gray.7"} variant="filled" onClick={async () => {
                              roleForm.setFieldValue('ping_on_notif', !roleForm.values.ping_on_notif)
                            }} >
                              <FontAwesomeIcon icon={roleForm.values.ping_on_notif ? faBell : faBellSlash} />
                            </ActionIcon>
                          </Tooltip>
                        </Group>
                      }
                    />
                  </Group>
                </Group>
              </Accordion.Panel>
            </Accordion.Item>
          </Accordion>
        </Group>
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