import { Box, Button, Group, SimpleGrid } from "@mantine/core";
import { SettingsNotifications } from "@api/types";
import { NotificationForm } from "@components";
import { useTranslateForms } from "@hooks/index";
import { useForm } from "@mantine/form";

export type NotificationPanelProps = {
  value: SettingsNotifications;
  onSubmit?: (value: SettingsNotifications) => void;
}
export const NotificationPanel = ({ value, onSubmit }: NotificationPanelProps) => {

  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateForms(`settings.tabs.live_trading.${key}`, { ...context }, i18Key)
  const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateForm(`buttons.${key}`, { ...context }, i18Key)


  // User form
  const form = useForm({
    initialValues: value,
    validate: {},
  });
  return (
    <Box>
      <form onSubmit={(e) => {
        e.preventDefault();
        if (onSubmit)
          onSubmit(form.values);
      }}>
        <SimpleGrid cols={2} spacing="lg">
          <NotificationForm
            title="New Conversation"
            value={form.values.on_new_conversation}
            onChanges={(n) => {
              console.log(n)
              form.setFieldValue('on_new_conversation', n)
            }}
          />
          <NotificationForm
            title="WFM Chat"
            value={form.values.on_wfm_chat_message}
            onChanges={(n) => {
              form.setFieldValue('on_wfm_chat_message', n)
            }}
          />
          <NotificationForm
            title="New Trade"
            value={form.values.on_new_trade}
            onChanges={(n) => {
              form.setFieldValue('on_new_trade', n)
            }}
          />
        </SimpleGrid>
        <Group justify="flex-end" style={{
          position: "absolute",
          bottom: 25,
          right: 25,
        }}>
          <Button type="submit" variant="light" color="blue">
            {useTranslateButtons('save.label')}
          </Button>
        </Group>
      </form>
    </Box>
  );
};