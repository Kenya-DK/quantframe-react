import { Collapse, Group, TextInput, Title, Divider, Textarea, Paper, ScrollAreaAutosize } from "@mantine/core";
// import { useTranslateForms } from "@hooks/useTranslate.hook";
import { useForm } from "@mantine/form";
import { TauriTypes } from "$types";
import { useTranslateForms } from "@hooks/useTranslate.hook";
import { ActionWithTooltip } from "../../Shared/ActionWithTooltip";
import { faBell } from "@fortawesome/free-solid-svg-icons";
import { faDiscord } from "@fortawesome/free-brands-svg-icons";
import { TooltipIcon } from "../../Shared/TooltipIcon";

export type EditNotificationSettingProps = {
  title: string;
  value?: TauriTypes.NotificationSetting;
  onChange: (values: TauriTypes.NotificationSetting) => void;
};

export function EditNotificationSetting({ value, onChange }: EditNotificationSettingProps) {
  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`edit_notification_setting.${key}`, { ...context }, i18Key);
  const useTranslateFormSystemFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`system.fields.${key}`, { ...context }, i18Key);
  const useTranslateFormDiscordFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`discord.fields.${key}`, { ...context }, i18Key);
  // User form
  const form = useForm({
    initialValues: value,
    onValuesChange: (values) => onChange(values as TauriTypes.NotificationSetting),
    validate: {},
  });
  return (
    <form>
      <Paper shadow="xs">
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
        </Group>
        <Divider />
        <ScrollAreaAutosize mah={"calc(85vh - 100px)"} scrollbarSize={6}>
          <Collapse in={form.values.system_notify.enabled}>
            <Title order={4} mb="xs" mt="sm">
              {useTranslateForm("system.title")}
            </Title>
            <TextInput
              required
              label={useTranslateFormSystemFields("title.label")}
              placeholder={useTranslateFormSystemFields("title.placeholder")}
              value={form.values.system_notify.title}
              onChange={(event) => form.setFieldValue("system_notify.title", event.currentTarget.value)}
              radius="md"
            />
            <Textarea
              required
              label={useTranslateFormSystemFields("content.label")}
              placeholder={useTranslateFormSystemFields("content.placeholder")}
              value={form.values.system_notify.content}
              onChange={(event) => form.setFieldValue("system_notify.content", event.currentTarget.value)}
              radius="md"
              rows={3}
              maxRows={3}
            />
          </Collapse>
          <Collapse in={form.values.discord_notify.enabled}>
            <Title order={4} mb="xs" mt="sm">
              {useTranslateForm("discord.title")}
            </Title>
            <TextInput
              required
              label={useTranslateFormDiscordFields("webhook.label")}
              placeholder={useTranslateFormDiscordFields("webhook.placeholder")}
              value={form.values.discord_notify.webhook}
              onChange={(event) => form.setFieldValue("discord_notify.webhook", event.currentTarget.value)}
              radius="md"
            />
            <Textarea
              required
              label={useTranslateFormDiscordFields("content.label")}
              placeholder={useTranslateFormDiscordFields("content.placeholder")}
              value={form.values.discord_notify.content}
              onChange={(event) => form.setFieldValue("discord_notify.content", event.currentTarget.value)}
              radius="md"
              rows={7}
              maxRows={7}
            />
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
          </Collapse>
        </ScrollAreaAutosize>
      </Paper>
    </form>
  );
}
