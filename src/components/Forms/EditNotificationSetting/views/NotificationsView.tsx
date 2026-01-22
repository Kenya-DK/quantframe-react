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
import { FALLBACK_SOUND, isCustomSound, toCustomSoundValue } from "@utils/sound";
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

const normalizeVolume = (value: number | string | null | undefined) =>
  typeof value === "number" && !Number.isNaN(value) ? value : 1.0;

type EditNotificationSettingTranslations = ReturnType<typeof createEditNotificationSettingTranslations>;
type NotificationSectionKey = "system_notify" | "discord_notify" | "webhook_notify";

export type NotificationsViewProps = {
  id: string;
  form: UseFormReturnType<TauriTypes.NotificationSetting>;
  customSounds: TauriTypes.CustomSound[];
  onManageSounds: () => void;
};

const NotificationToggles = ({
  form,
  t,
}: {
  form: UseFormReturnType<TauriTypes.NotificationSetting>;
  t: EditNotificationSettingTranslations;
}) => {
  const toggleConfig: Array<{
    key: NotificationSectionKey;
    icon: typeof faBell;
    tooltipKey: string;
  }> = [
    { key: "system_notify", icon: faBell, tooltipKey: "system.tooltip" },
    { key: "discord_notify", icon: faDiscord, tooltipKey: "discord.tooltip" },
    { key: "webhook_notify", icon: faWebHook, tooltipKey: "webhook.tooltip" },
  ];

  const toggle = (key: NotificationSectionKey) => {
    const enabled = form.values[key].enabled;
    form.setFieldValue(`${key}.enabled`, !enabled);
  };

  return (
    <Group m="xs" gap={5}>
      {toggleConfig.map((config) => (
        <ActionWithTooltip
          key={config.key}
          tooltip={t.form(config.tooltipKey)}
          icon={config.icon}
          color={form.values[config.key].enabled ? "green.7" : "blue.7"}
          onClick={() => toggle(config.key)}
        />
      ))}
    </Group>
  );
};

const SystemNotificationSection = ({
  form,
  t,
  customSoundOptions,
  onManageSounds,
}: {
  form: UseFormReturnType<TauriTypes.NotificationSetting>;
  t: EditNotificationSettingTranslations;
  customSoundOptions: Array<{ value: string; label: string }>;
  onManageSounds: () => void;
}) => {
  const systemSoundOptions = buildSystemSoundOptions(t.systemFields);
  const volumeValue = normalizeVolume(form.values.system_notify.volume);

  const handlePlaySound = () => {
    if (!form.values.system_notify.enabled || form.values.system_notify.sound_file === "none") return;
    const soundFile = form.values.system_notify.sound_file || "none";
    if (isCustomSound(soundFile)) {
      const exists = customSoundOptions.some((option) => option.value === soundFile);
      if (!exists) {
        form.setFieldValue("system_notify.sound_file", FALLBACK_SOUND);
        PlaySound(FALLBACK_SOUND, volumeValue);
        return;
      }
    }
    PlaySound(soundFile, volumeValue);
  };

  return (
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
              onClick={handlePlaySound}
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
          value={volumeValue}
          onChange={(value) => form.setFieldValue("system_notify.volume", normalizeVolume(value))}
        />
        <Button size="sm" onClick={onManageSounds}>
          {t.manageSounds("buttons.manage")}
        </Button>
      </Group>
      <Divider my="sm" />
    </Collapse>
  );
};

const DiscordNotificationSection = ({
  id,
  form,
  t,
}: {
  id: string;
  form: UseFormReturnType<TauriTypes.NotificationSetting>;
  t: EditNotificationSettingTranslations;
}) => {
  const handleContentReset = async () => {
    const defaultNotification = await api.app.notify_reset(id);
    form.setFieldValue("discord_notify.content", defaultNotification.discord_notify.content.replace(/\\n/g, "\n"));
  };

  const handleUserIdsChange = (value: string) =>
    form.setFieldValue(
      "discord_notify.user_ids",
      value
        .split(",")
        .map((entry) => entry.trim())
        .filter(Boolean)
    );

  return (
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
        onChange={(event) => handleUserIdsChange(event.currentTarget.value)}
        radius="md"
      />
      <Divider my="sm" />
    </Collapse>
  );
};

const WebhookNotificationSection = ({
  form,
  t,
}: {
  form: UseFormReturnType<TauriTypes.NotificationSetting>;
  t: EditNotificationSettingTranslations;
}) => (
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
);

export const NotificationsView = ({ id, form, customSounds, onManageSounds }: NotificationsViewProps) => {
  const t = createEditNotificationSettingTranslations();
  const customSoundOptions = customSounds.map((sound) => ({
    value: toCustomSoundValue(sound.file_name),
    label: sound.name,
  }));

  return (
    <>
      <NotificationToggles form={form} t={t} />
      <Divider />
      <ScrollAreaAutosize mah={"calc(80vh - 100px)"} scrollbarSize={6}>
        <Stack gap={5}>
          <SystemNotificationSection
            form={form}
            t={t}
            customSoundOptions={customSoundOptions}
            onManageSounds={onManageSounds}
          />
          <DiscordNotificationSection id={id} form={form} t={t} />
          <WebhookNotificationSection form={form} t={t} />
        </Stack>
      </ScrollAreaAutosize>
    </>
  );
};
