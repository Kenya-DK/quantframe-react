import { TauriTypes } from "$types";

export type NotificationsPanelProps = {
  value: TauriTypes.SettingsNotifications;
  onSubmit: (value: TauriTypes.SettingsNotifications) => void;
};

export const NotificationsPanel = ({}: NotificationsPanelProps) => {
  return <>Notifications Panel</>;
};
