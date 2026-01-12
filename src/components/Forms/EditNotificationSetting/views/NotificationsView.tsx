import {
  Collapse,
  Group,
  TextInput,
  Title,
  Divider,
  Textarea,
  ScrollAreaAutosize,
  Button,
  Stack,
  Select,
  NumberInput,
} from "@mantine/core";
import { UseFormReturnType } from "@mantine/form";
import { TauriTypes } from "$types";
import { useTranslateForms } from "@hooks/useTranslate.hook";
import { ActionWithTooltip } from "@components/Shared/ActionWithTooltip";
import { faBell } from "@fortawesome/free-solid-svg-icons";
import { faDiscord } from "@fortawesome/free-brands-svg-icons";
import { TooltipIcon } from "@components/Shared/TooltipIcon";
import api from "@api/index";
import { faWebHook } from "@icons";
import { PlaySound } from "@utils/helper";

export type NotificationsViewProps = {
  id: string;
  form: UseFormReturnType<TauriTypes.NotificationSetting>;
  customSounds: TauriTypes.CustomSound[];
  onManageSounds: () => void;
};

export const NotificationsView = ({ id, form, customSounds, onManageSounds }: NotificationsViewProps) => {
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`edit_notification_setting.${key}`, { ...context }, i18Key);
  const useTranslateFormSystemFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`system.fields.${key}`, { ...context }, i18Key);
  const useTranslateFormDiscordFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`discord.fields.${key}`, { ...context }, i18Key);
  const useTranslateFormWebhookFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`webhook.fields.${key}`, { ...context }, i18Key);
  const useTranslateFormManageSounds = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`manage_sounds.${key}`, { ...context }, i18Key);

  const handleContentReset = async () => {
    const defaultNotification = await api.app.notify_reset(id);
    form.setFieldValue("discord_notify.content", defaultNotification.discord_notify.content.replace(/\\n/g, "\n"));
  };

  return (
    <>
      <Group m="xs" gap={5}>
        <ActionWithTooltip
          tooltip={useTranslateForm("system.tooltip")}
          color={form.values.system_notify.enabled ? "green.7" : "blue.7"}
          icon={faBell}
          onClick={() => {
            form.setFieldValue("system_notify.enabled", !form.values.system_notify?.enabled);
          }}
        />
        <ActionWithTooltip
          tooltip={useTranslateForm("discord.tooltip")}
          icon={faDiscord}
          color={form.values.discord_notify.enabled ? "green.7" : "blue.7"}
          onClick={() => {
            form.setFieldValue("discord_notify.enabled", !form.values.discord_notify?.enabled);
          }}
        />
        <ActionWithTooltip
          tooltip={useTranslateForm("webhook.tooltip")}
          icon={faWebHook}
          color={form.values.webhook_notify.enabled ? "green.7" : "blue.7"}
          onClick={() => {
            form.setFieldValue("webhook_notify.enabled", !form.values.webhook_notify?.enabled);
          }}
        />
      </Group>
      <Divider />
      <ScrollAreaAutosize mah={"calc(80vh - 100px)"} scrollbarSize={6}>
        <Stack gap={5}>
          <Collapse in={form.values.system_notify.enabled}>
            <Title order={4} mb="xs" mt="sm">
              {useTranslateForm("system.title")}
            </Title>
            <TextInput
              label={useTranslateFormSystemFields("title.label")}
              placeholder={useTranslateFormSystemFields("title.placeholder")}
              value={form.values.system_notify.title}
              onChange={(event) => form.setFieldValue("system_notify.title", event.currentTarget.value)}
              radius="md"
            />
            <Textarea
              label={useTranslateFormSystemFields("content.label")}
              placeholder={useTranslateFormSystemFields("content.placeholder")}
              value={form.values.system_notify.content}
              onChange={(event) => form.setFieldValue("system_notify.content", event.currentTarget.value)}
              radius="md"
              rows={3}
              maxRows={3}
            />
            <Group gap="xs" mt="sm" align="flex-end">
              <Select
                w={250}
                label={useTranslateFormSystemFields("sound.label")}
                placeholder={useTranslateFormSystemFields("sound.placeholder")}
                radius="md"
                value={form.values.system_notify.sound_file || "none"}
                onChange={(value) => form.setFieldValue("system_notify.sound_file", value || "none")}
                rightSectionPointerEvents="inherit"
                data={[
                  { value: "none", label: useTranslateFormSystemFields("sound.options.none") },
                  { value: "cat_meow.mp3", label: useTranslateFormSystemFields("sound.options.cat_meow") },
                  { value: "iphone_notification.mp3", label: useTranslateFormSystemFields("sound.options.iphone_notification") },
                  { value: "windows_notification.mp3", label: useTranslateFormSystemFields("sound.options.windows_notification") },
                  { value: "windows_xp_error.mp3", label: useTranslateFormSystemFields("sound.options.windows_xp_error") },
                  { value: "windows_xp_startup.mp3", label: useTranslateFormSystemFields("sound.options.windows_xp_startup") },
                  ...customSounds.map((sound) => ({
                    value: `custom:${sound.file_name}`,
                    label: sound.name,
                  })),
                ]}
                rightSection={
                  <ActionWithTooltip
                    tooltip={useTranslateFormSystemFields("sound.play_tooltip")}
                    icon={faBell}
                    onClick={() => {
                      if (!form.values.system_notify.enabled || form.values.system_notify.sound_file === "none") return;
                      PlaySound(form.values.system_notify.sound_file || "none", form.values.system_notify.volume || 1.0);
                    }}
                  />
                }
              />
              <NumberInput
                w={100}
                label={useTranslateFormSystemFields("volume.label")}
                placeholder={useTranslateFormSystemFields("volume.placeholder")}
                radius="md"
                min={0}
                max={1}
                step={0.1}
                value={form.values.system_notify.volume || 1.0}
                onChange={(value) => form.setFieldValue("system_notify.volume", Number(value) || 1.0)}
              />
              <Button
                size="sm"
                onClick={onManageSounds}
              >
                {useTranslateFormManageSounds("buttons.manage")}
              </Button>
            </Group>
            <Divider my="sm" />
          </Collapse>
          <Collapse in={form.values.discord_notify.enabled}>
            <Title order={4} mb="xs" mt="sm">
              {useTranslateForm("discord.title")}
            </Title>
            <TextInput
              label={useTranslateFormDiscordFields("webhook.label")}
              placeholder={useTranslateFormDiscordFields("webhook.placeholder")}
              value={form.values.discord_notify.webhook}
              onChange={(event) => form.setFieldValue("discord_notify.webhook", event.currentTarget.value)}
              radius="md"
            />
            <Textarea
              label={useTranslateFormDiscordFields("content.label")}
              placeholder={useTranslateFormDiscordFields("content.placeholder")}
              value={form.values.discord_notify.content}
              onChange={(event) => form.setFieldValue("discord_notify.content", event.currentTarget.value)}
              radius="md"
              rows={7}
              maxRows={7}
            />

            <Button mt={5} onClick={handleContentReset}>
              {useTranslateFormDiscordFields("content.reset_button")}
            </Button>
            <TextInput
              label={useTranslateFormDiscordFields("user_ids.label")}
              placeholder={useTranslateFormDiscordFields("user_ids.placeholder")}
              value={form.values.discord_notify.user_ids.join(", ")}
              rightSection={<TooltipIcon label={useTranslateFormDiscordFields("user_ids.description")} />}
              onChange={(event) =>
                form.setFieldValue(
                  "discord_notify.user_ids",
                  event.currentTarget.value.split(",").map((v) => v.trim())
                )
              }
              radius="md"
            />
            <Divider my="sm" />
          </Collapse>
          <Collapse in={form.values.webhook_notify.enabled}>
            <Title order={4} mb="xs" mt="sm">
              {useTranslateForm("webhook.title")}
            </Title>
            <TextInput
              label={useTranslateFormWebhookFields("url.label")}
              placeholder={useTranslateFormWebhookFields("url.placeholder")}
              value={form.values.webhook_notify.url}
              onChange={(event) => form.setFieldValue("webhook_notify.url", event.currentTarget.value)}
              radius="md"
            />
            <Divider my="sm" />
          </Collapse>
        </Stack>
      </ScrollAreaAutosize>
    </>
  );
};
