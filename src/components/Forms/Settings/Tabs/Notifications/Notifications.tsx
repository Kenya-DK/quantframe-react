import { TauriTypes } from "$types";
import { Tabs, Text } from "@mantine/core";
import { EditNotificationSetting } from "@components/Forms/EditNotificationSetting";
import { useTranslateForms } from "@hooks/useTranslate.hook";
import { UseFormReturnType } from "@mantine/form";
export type NotificationsPanelProps = {
  form: UseFormReturnType<TauriTypes.Settings>;
};

export const NotificationsPanel = ({ form }: NotificationsPanelProps) => {
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`settings.tabs.notifications.${key}`, { ...context }, i18Key);
  type NotificationKey = keyof TauriTypes.SettingsNotifications;

  const isEnabled = (da: TauriTypes.NotificationSetting) => {
    return da.system_notify.enabled || da.discord_notify.enabled || da.webhook_notify.enabled;
  };
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
          value={form.values.notifications.on_new_conversation}
          onChange={(newValue) => form.setFieldValue("notifications.on_new_conversation", newValue)}
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
          value={form.values.notifications.on_new_trade}
          onChange={(newValue) => form.setFieldValue("notifications.on_new_trade", newValue)}
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
          value={form.values.notifications.on_wfm_chat_message}
          onChange={(newValue) => form.setFieldValue("on_wfm_chat_message", newValue)}
        />
      ),
      id: "on_wfm_chat_message",
    },
  ];
  return (
    <Tabs orientation="vertical" defaultValue={tabs[0].id}>
      <Tabs.List>
        {tabs.map((tab) => (
          <Tabs.Tab value={tab.id} key={tab.id}>
            <Text size="sm" c={isEnabled(form.values.notifications[tab.id]) ? "green.7" : "red.7"}>
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
  );
};
