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
import { ActionWithTooltip } from "@components/Shared/ActionWithTooltip";
import { faBell } from "@fortawesome/free-solid-svg-icons";
import { faDiscord } from "@fortawesome/free-brands-svg-icons";
import { TooltipIcon } from "@components/Shared/TooltipIcon";
import api from "@api/index";
import { faWebHook } from "@icons";
import { PlaySound } from "@utils/helper";
import { toCustomSoundValue } from "@utils/sound";
import { createEditNotificationSettingTranslations } from "../translations";

const DEFAULT_SOUND_OPTIONS = [
  { value: "cat_meow.mp3", labelKey: "sound.options.cat_meow" },
  { value: "iphone_notification.mp3", labelKey: "sound.options.iphone_notification" },
  { value: "windows_notification.mp3", labelKey: "sound.options.windows_notification" },
  { value: "windows_xp_error.mp3", labelKey: "sound.options.windows_xp_error" },
  { value: "windows_xp_startup.mp3", labelKey: "sound.options.windows_xp_startup" },
];

const buildSystemSoundOptions = (
  translateSystemFields: (key: string, context?: { [key: string]: any }, i18Key?: boolean) => string
) => [
  { value: "none", label: translateSystemFields("sound.options.none") },
  ...DEFAULT_SOUND_OPTIONS.map((option) => ({
    value: option.value,
    label: translateSystemFields(option.labelKey),
  })),
];

export type NotificationsViewProps = {
  id: string;
  form: UseFormReturnType<TauriTypes.NotificationSetting>;
  customSounds: TauriTypes.CustomSound[];
  onManageSounds: () => void;
};

export const NotificationsView = ({ id, form, customSounds, onManageSounds }: NotificationsViewProps) => {
  const t = createEditNotificationSettingTranslations();
  const systemSoundOptions = buildSystemSoundOptions(t.systemFields);
  const customSoundOptions = customSounds.map((sound) => ({
    value: toCustomSoundValue(sound.file_name),
    label: sound.name,
  }));

  const handleContentReset = async () => {
    const defaultNotification = await api.app.notify_reset(id);
    form.setFieldValue("discord_notify.content", defaultNotification.discord_notify.content.replace(/\\n/g, "\n"));
  };

  return (
    <>
      <Group m="xs" gap={5}>
        <ActionWithTooltip
          tooltip={t.form("system.tooltip")}
          color={form.values.system_notify.enabled ? "green.7" : "blue.7"}
          icon={faBell}
          onClick={() => {
            form.setFieldValue("system_notify.enabled", !form.values.system_notify?.enabled);
          }}
        />
        <ActionWithTooltip
          tooltip={t.form("discord.tooltip")}
          icon={faDiscord}
          color={form.values.discord_notify.enabled ? "green.7" : "blue.7"}
          onClick={() => {
            form.setFieldValue("discord_notify.enabled", !form.values.discord_notify?.enabled);
          }}
        />
        <ActionWithTooltip
          tooltip={t.form("webhook.tooltip")}
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
              {t.form("system.title")}
            </Title>
            <TextInput
              label={t.systemFields("title.label")}
              placeholder={t.systemFields("title.placeholder")}
              value={form.values.system_notify.title}
              onChange={(event) => form.setFieldValue("system_notify.title", event.currentTarget.value)}
              radius="md"
            />
            <Textarea
              label={t.systemFields("content.label")}
              placeholder={t.systemFields("content.placeholder")}
              value={form.values.system_notify.content}
              onChange={(event) => form.setFieldValue("system_notify.content", event.currentTarget.value)}
              radius="md"
              rows={3}
              maxRows={3}
            />
            <Group gap="xs" mt="sm" align="flex-end">
              <Select
                w={250}
                label={t.systemFields("sound.label")}
                placeholder={t.systemFields("sound.placeholder")}
                radius="md"
                value={form.values.system_notify.sound_file || "none"}
                onChange={(value) => form.setFieldValue("system_notify.sound_file", value || "none")}
                rightSectionPointerEvents="inherit"
                data={[...systemSoundOptions, ...customSoundOptions]}
                rightSection={
                  <ActionWithTooltip
                    tooltip={t.systemFields("sound.play_tooltip")}
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
                label={t.systemFields("volume.label")}
                placeholder={t.systemFields("volume.placeholder")}
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
                {t.manageSounds("buttons.manage")}
              </Button>
            </Group>
            <Divider my="sm" />
          </Collapse>
          <Collapse in={form.values.discord_notify.enabled}>
            <Title order={4} mb="xs" mt="sm">
              {t.form("discord.title")}
            </Title>
            <TextInput
              label={t.discordFields("webhook.label")}
              placeholder={t.discordFields("webhook.placeholder")}
              value={form.values.discord_notify.webhook}
              onChange={(event) => form.setFieldValue("discord_notify.webhook", event.currentTarget.value)}
              radius="md"
            />
            <Textarea
              label={t.discordFields("content.label")}
              placeholder={t.discordFields("content.placeholder")}
              value={form.values.discord_notify.content}
              onChange={(event) => form.setFieldValue("discord_notify.content", event.currentTarget.value)}
              radius="md"
              rows={7}
              maxRows={7}
            />

            <Button mt={5} onClick={handleContentReset}>
              {t.discordFields("content.reset_button")}
            </Button>
            <TextInput
              label={t.discordFields("user_ids.label")}
              placeholder={t.discordFields("user_ids.placeholder")}
              value={form.values.discord_notify.user_ids.join(", ")}
              rightSection={<TooltipIcon label={t.discordFields("user_ids.description")} />}
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
              {t.form("webhook.title")}
            </Title>
            <TextInput
              label={t.webhookFields("url.label")}
              placeholder={t.webhookFields("url.placeholder")}
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
