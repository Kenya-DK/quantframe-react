import { TauriTypes } from "$types";
import { Button, Container, Group, Tabs, Text } from "@mantine/core";
import { EditNotificationSetting } from "@components/Forms/EditNotificationSetting";
import { useTranslateCommon, useTranslateForms } from "@hooks/useTranslate.hook";
import { useForm } from "@mantine/form";
export type NotificationsPanelProps = {
  value: TauriTypes.SettingsNotifications;
  onSubmit: (value: TauriTypes.SettingsNotifications) => void;
};

export const NotificationsPanel = ({ value, onSubmit }: NotificationsPanelProps) => {
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`settings.tabs.notifications.${key}`, { ...context }, i18Key);
  type NotificationKey = keyof TauriTypes.SettingsNotifications;

  const isEnabled = (da: TauriTypes.NotificationSetting) => {
    return da.system_notify.enabled || da.discord_notify.enabled || da.webhook_notify.enabled;
  };

  const form = useForm({
    initialValues: value,
    onValuesChange: (values) => {
      console.log("Form values changed", values);
      // onChange?.(values as TauriTypes.SettingsNotifications);
    },
    validate: {},
  });
  const tabs: {
    label: string;
    component: JSX.Element;
    id: NotificationKey;
  }[] = [
    {
      label: useTranslateForm("on_new_conversation_title"),
      component: (
        <EditNotificationSetting
          id="on_new_conversation"
          title={useTranslateForm("on_new_conversation_title")}
          value={form.values.on_new_conversation}
          onChange={(newValue) => form.setFieldValue("on_new_conversation", newValue)}
        />
      ),
      id: "on_new_conversation",
    },
    {
      label: useTranslateForm("on_new_trade_title"),
      component: (
        <EditNotificationSetting
          id="on_new_trade"
          title={useTranslateForm("on_new_trade_title")}
          value={form.values.on_new_trade}
          onChange={(newValue) => form.setFieldValue("on_new_trade", newValue)}
        />
      ),
      id: "on_new_trade",
    },
    {
      label: useTranslateForm("on_wfm_chat_message_title"),
      component: (
        <EditNotificationSetting
          id="on_wfm_chat_message"
          title={useTranslateForm("on_wfm_chat_message_title")}
          value={form.values.on_wfm_chat_message}
          onChange={(newValue) => form.setFieldValue("on_wfm_chat_message", newValue)}
        />
      ),
      id: "on_wfm_chat_message",
    },
  ];
  return (
    <form onSubmit={form.onSubmit((values) => onSubmit(values))}>
      <Container size={"100%"} h={"75vh"} p={0}>
        <Tabs h={"82vh"} orientation="vertical">
          <Tabs.List>
            {tabs.map((tab) => (
              <Tabs.Tab value={tab.id} key={tab.id}>
                <Text size="sm" c={isEnabled(form.values[tab.id]) ? "green.7" : "red.7"}>
                  {tab.label}
                </Text>
              </Tabs.Tab>
            ))}
          </Tabs.List>
          {tabs.map((tab) => (
            <Tabs.Panel value={tab.id} key={tab.id}>
              {tab.component}
            </Tabs.Panel>
          ))}
        </Tabs>
        <Group
          justify="flex-end"
          style={{
            position: "absolute",
            bottom: 25,
            right: 25,
          }}
        >
          <Button type="submit" variant="light" color="blue">
            {useTranslateCommon("buttons.save.label")}
          </Button>
        </Group>
      </Container>
    </form>
  );
};
