import { TauriTypes } from "$types";
import { EditNotificationSetting } from "@components/Forms/EditNotificationSetting";
import { useTranslateForms } from "@hooks/useTranslate.hook";
import { Tabs, Text } from "@mantine/core";
import { UseFormReturnType } from "@mantine/form";
import { useState } from "react";
export type NotificationsPanelProps = {
  form: UseFormReturnType<TauriTypes.Settings>;
  onHideButtons?: (value: boolean) => void;
};

export const NotificationsPanel = ({ form, onHideButtons }: NotificationsPanelProps) => {
  const [hideTab, setHideTab] = useState<boolean>(false);
  const t = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`settings.tabs.notifications.${key}`, { ...context }, i18Key);
  type NotificationKey = keyof TauriTypes.NotificationsSetting;
  const notificationValues = form.values.notifications;

  const isEnabled = (setting: TauriTypes.NotificationSetting) =>
    setting.system_notify.enabled || setting.discord_notify.enabled || setting.webhook_notify.enabled;

  const updateNotification = (key: NotificationKey, next: TauriTypes.NotificationSetting) => {
    form.setFieldValue(`notifications.${key}`, next);
  };

  const tabs = [
    { id: "on_new_conversation", labelKey: "on_new_conversation_title" },
    { id: "on_new_trade", labelKey: "on_new_trade_title" },
    { id: "on_wfm_chat_message", labelKey: "on_wfm_chat_message_title" },
  ] as const satisfies Array<{ id: NotificationKey; labelKey: string }>;

  const panels = tabs.map(({ id, labelKey }) => {
    const label = t(labelKey);
    return {
      id,
      label,
      component: (
        <EditNotificationSetting
          id={id}
          title={label}
          value={notificationValues[id]}
          onChange={(newValue) => updateNotification(id, newValue)}
          setHideTab={(v) => setHideTab(v)}
          setHideButtons={(v) => onHideButtons?.(v)}
        />
      ),
    };
  });

  return (
    <Tabs h={"82vh"} orientation="vertical" defaultValue={panels[0].id}>
      <Tabs.List display={hideTab ? "none" : ""}>
        {panels.map((panel) => (
          <Tabs.Tab value={panel.id} key={panel.id}>
            <Text size="sm" c={isEnabled(notificationValues[panel.id]) ? "green.7" : "red.7"}>
              {panel.label}
            </Text>
          </Tabs.Tab>
        ))}
      </Tabs.List>
      {panels.map((panel) => (
        <Tabs.Panel value={panel.id} key={panel.id}>
          {panel.component}
        </Tabs.Panel>
      ))}
    </Tabs>
  );
};

