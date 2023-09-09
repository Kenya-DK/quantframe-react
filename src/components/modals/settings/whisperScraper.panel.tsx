import { useEffect } from "react";
import { useForm } from "@mantine/form";
import { Accordion, Box, Button, Checkbox, Group, TextInput } from "@mantine/core";
import { useTranslateModal } from "@hooks/index";
import { WhisperScraperSettings, Wfm } from "$types/index";

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
    <Box h={"75vh"} w={"75vw"}>
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
                      />
                      <Checkbox
                        label={useTranslateSettingsModal('ping_on_notif')}
                        description={useTranslateSettingsModal('ping_on_notif_description')}
                        checked={roleForm.values.ping_on_notif}
                        onChange={(event) => roleForm.setFieldValue('ping_on_notif', event.currentTarget.checked)}
                      />
                    </Group>
                  </Group>
                </Accordion.Panel>
              </Accordion.Item>
            </Accordion>
          </Group>
        </Group>
        <Group position="right" mt={10}>
          <Button type="submit" variant="light" color="blue">
            {useTranslateSettingsModal('save')}
          </Button>
        </Group>
      </form>
    </Box>
  );
}