import { Box } from "@mantine/core";
import { useForm } from "@mantine/form";
import { useLocalStorage } from "@mantine/hooks";
import { useEffect } from "react";
import { TauriTypes } from "$types";
import { useCustomSoundsTable } from "./queries";
import { ManageSoundsView, NotificationsView } from "./views";

export type EditNotificationSettingProps = {
  title: string;
  id: string;
  value?: TauriTypes.NotificationSetting;
  onChange: (values: TauriTypes.NotificationSetting) => void;
};

enum ViewMode {
  Notifications = "notifications",
  ManageSounds = "manage_sounds",
}

export function EditNotificationSetting({ id, value, onChange }: EditNotificationSettingProps) {
  const [viewMode, setViewMode] = useLocalStorage<ViewMode>({
    key: `edit_notification_setting.view_mode.${id}`,
    defaultValue: ViewMode.Notifications,
  });
  const showNotifications = viewMode === ViewMode.Notifications;

  const form = useForm({
    initialValues: value,
    onValuesChange: (values) => onChange(values as TauriTypes.NotificationSetting),
    validate: {},
  });

  useEffect(() => {
    if (!value) return;
    if (JSON.stringify(form.values) === JSON.stringify(value)) return;
    form.setValues(value);
    form.resetDirty(value);
  }, [value]);

  const soundsTable = useCustomSoundsTable();
  const selectedSoundFile = form.values.system_notify.sound_file;
  const handleManageSounds = () => setViewMode(ViewMode.ManageSounds);
  const handleBack = () => setViewMode(ViewMode.Notifications);
  const handleClearSelectedSound = () => form.setFieldValue("system_notify.sound_file", "none");

  return (
    <Box p="md" pb={0}>
      {showNotifications ? (
        <NotificationsView
          id={id}
          form={form}
          customSounds={soundsTable.customSounds}
          onManageSounds={handleManageSounds}
        />
      ) : (
        <ManageSoundsView
          query={soundsTable.query}
          onQueryChange={soundsTable.setQuery}
          page={soundsTable.safePage}
          onPageChange={soundsTable.setPage}
          pageSize={soundsTable.pageSize}
          onPageSizeChange={soundsTable.setPageSize}
          sortStatus={soundsTable.sortStatus}
          onSortStatusChange={soundsTable.setSortStatus}
          records={soundsTable.paginatedSounds}
          totalRecords={soundsTable.totalRecords}
          isFetching={soundsTable.isFetching}
          invalidateSounds={soundsTable.invalidateSounds}
          selectedSoundFile={selectedSoundFile}
          onClearSelectedSound={handleClearSelectedSound}
          onBack={handleBack}
        />
      )}
    </Box>
  );
}
