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
  const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`fields.${key}`, { ...context }, i18Key);
  const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`buttons.${key}`, { ...context }, i18Key);
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
            tooltip={useTranslateButtons("system.tooltip")}
            color={form.values.system_notify.enabled ? "green.7" : "blue.7"}
            icon={faBell}
            onClick={() => {
              form.setFieldValue("system_notify.enabled", !form.values.system_notify?.enabled);
            }}
          />
          <ActionWithTooltip
            tooltip={useTranslateButtons("discord.tooltip")}
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
              {useTranslateFormFields("system.title")}
            </Title>
            <TextInput
              required
              label={useTranslateFormFields("system.title.label")}
              placeholder={useTranslateFormFields("system.title.placeholder")}
              value={form.values.system_notify.title}
              onChange={(event) => form.setFieldValue("system_notify.title", event.currentTarget.value)}
              radius="md"
            />
            <Textarea
              required
              label={useTranslateFormFields("system.content.label")}
              placeholder={useTranslateFormFields("system.content.placeholder")}
              value={form.values.system_notify.content}
              onChange={(event) => form.setFieldValue("system_notify.content", event.currentTarget.value)}
              radius="md"
              rows={3}
              maxRows={3}
            />
          </Collapse>
          <Collapse in={form.values.discord_notify.enabled}>
            <Title order={4} mb="xs" mt="sm">
              {useTranslateFormFields("discord.title")}
            </Title>
            <TextInput
              required
              label={useTranslateFormFields("discord.webhook.label")}
              placeholder={useTranslateFormFields("discord.webhook.placeholder")}
              value={form.values.discord_notify.webhook}
              onChange={(event) => form.setFieldValue("discord_notify.webhook", event.currentTarget.value)}
              radius="md"
            />
            <Textarea
              required
              label={useTranslateFormFields("discord.content.label")}
              placeholder={useTranslateFormFields("discord.content.placeholder")}
              value={form.values.discord_notify.content}
              onChange={(event) => form.setFieldValue("discord_notify.content", event.currentTarget.value)}
              radius="md"
              rows={7}
              maxRows={7}
            />
            <TextInput
              label={useTranslateFormFields("user_ids.label")}
              placeholder={useTranslateFormFields("user_ids.placeholder")}
              value={form.values.discord_notify.user_ids.join(", ")}
              rightSection={<TooltipIcon label={useTranslateFormFields("user_ids.description")} />}
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
