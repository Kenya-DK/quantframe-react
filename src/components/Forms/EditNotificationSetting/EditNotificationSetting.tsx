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

  const {
    customSounds,
    query,
    setQuery,
    setPage,
    pageSize,
    setPageSize,
    sortStatus,
    setSortStatus,
    paginatedSounds,
    totalRecords,
    safePage,
    isFetching,
    invalidateSounds,
  } = useCustomSoundsTable();

  return (
    <Box p="md" pb={0}>
      {viewMode === ViewMode.Notifications ? (
        <NotificationsView
          id={id}
          form={form}
          customSounds={customSounds}
          onManageSounds={() => setViewMode(ViewMode.ManageSounds)}
        />
      ) : (
        <ManageSoundsView
          query={query}
          onQueryChange={setQuery}
          page={safePage}
          onPageChange={setPage}
          pageSize={pageSize}
          onPageSizeChange={setPageSize}
          sortStatus={sortStatus}
          onSortStatusChange={setSortStatus}
          records={paginatedSounds}
          totalRecords={totalRecords}
          isFetching={isFetching}
          invalidateSounds={invalidateSounds}
          selectedSoundFile={form.values.system_notify.sound_file}
          onClearSelectedSound={() => form.setFieldValue("system_notify.sound_file", "none")}
          onBack={() => setViewMode(ViewMode.Notifications)}
        />
      )}
    </Box>
  );
}
