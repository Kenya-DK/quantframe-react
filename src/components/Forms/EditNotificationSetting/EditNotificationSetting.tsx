import { TauriTypes } from "$types";
import { Box } from "@mantine/core";
import { useForm } from "@mantine/form";
import { FALLBACK_SOUND, isCustomSound, stripCustomSoundPrefix } from "@utils/sound";
import { useEffect, useState } from "react";
import { useCustomSoundsTable } from "./queries";
import { ManageSoundsView, NotificationsView } from "./views";

export type EditNotificationSettingProps = {
  title: string;
  id: string;
  value?: TauriTypes.NotificationSetting;
  onChange: (values: TauriTypes.NotificationSetting) => void;
  setHideTab?: (value: boolean) => void;
  setHideButtons?: (value: boolean) => void;
};

enum ViewMode {
  Notifications = "notifications",
  ManageSounds = "manage_sounds",
}

export function EditNotificationSetting({ id, value, onChange, setHideTab, setHideButtons }: EditNotificationSettingProps) {
  const [viewMode, setViewMode] = useState<ViewMode>(ViewMode.Notifications);

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

  useEffect(() => {
    if (!soundsTable.isLoaded) return;
    if (!selectedSoundFile || !isCustomSound(selectedSoundFile)) return;
    const selectedFileName = stripCustomSoundPrefix(selectedSoundFile);
    const exists = soundsTable.customSounds.some((sound) => sound.file_name === selectedFileName);
    if (!exists) {
      form.setFieldValue("system_notify.sound_file", FALLBACK_SOUND);
    }
  }, [form, selectedSoundFile, soundsTable.customSounds, soundsTable.isLoaded]);
  const handleManageSounds = () => {
    setHideTab && setHideTab(true);
    setHideButtons && setHideButtons(true);
    setViewMode(ViewMode.ManageSounds);
  };
  const handleBack = () => {
    setViewMode(ViewMode.Notifications);
    setHideTab && setHideTab(false);
    setHideButtons && setHideButtons(false);
  };
  const handleClearSelectedSound = () => form.setFieldValue("system_notify.sound_file", FALLBACK_SOUND);

  return (
    <Box p="md" pb={0}>
      {viewMode === ViewMode.Notifications && (
        <NotificationsView id={id} form={form} customSounds={soundsTable.customSounds} onManageSounds={handleManageSounds} />
      )}
      {viewMode === ViewMode.ManageSounds && (
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
