import { Box, Button, Group, Stack, Text, TextInput } from "@mantine/core";
import { DataTable, DataTableSortStatus, type DataTableColumn } from "mantine-datatable";
import { SearchField } from "@components/Forms/SearchField";
import { ActionWithTooltip } from "@components/Shared/ActionWithTooltip";
import { faPlus, faTrash, faVolumeHigh } from "@fortawesome/free-solid-svg-icons";
import { modals } from "@mantine/modals";
import { notifications } from "@mantine/notifications";
import { open as openFile } from "@tauri-apps/plugin-dialog";
import { useState } from "react";
import { TauriTypes } from "$types";
import api from "@api/index";
import { PlaySound } from "@utils/helper";
import { toCustomSoundValue } from "@utils/sound";
import { createEditNotificationSettingTranslations } from "../translations";

export type ManageSoundsViewProps = {
  query: string;
  onQueryChange: (value: string) => void;
  page: number;
  onPageChange: (value: number) => void;
  pageSize: number;
  onPageSizeChange: (value: number) => void;
  sortStatus: DataTableSortStatus<TauriTypes.CustomSound>;
  onSortStatusChange: (value: DataTableSortStatus<TauriTypes.CustomSound>) => void;
  records: TauriTypes.CustomSound[];
  totalRecords: number;
  isFetching: boolean;
  invalidateSounds: () => void;
  selectedSoundFile?: string;
  onClearSelectedSound: () => void;
  onBack: () => void;
};

export const ManageSoundsView = ({
  query,
  onQueryChange,
  page,
  onPageChange,
  pageSize,
  onPageSizeChange,
  sortStatus,
  onSortStatusChange,
  records,
  totalRecords,
  isFetching,
  invalidateSounds,
  selectedSoundFile,
  onClearSelectedSound,
  onBack,
}: ManageSoundsViewProps) => {
  const t = createEditNotificationSettingTranslations();

  // TODO: find a solution for better long text ellipsis
  // handmade text-overflow: ellipsis becouse of it being not stable in mantine (especially in modal becouse there is no width lim)
  const ellipsize = (value: string, maxLength: number = 40) =>
    value.length > maxLength ? `${value.slice(0, Math.max(0, maxLength - 3))}...` : value;

  const copy = {
    nameLabel: t.manageSounds("fields.name.label"),
    namePlaceholder: t.manageSounds("fields.name.placeholder"),
    fileLabel: t.manageSounds("fields.file.label"),
    filePlaceholder: t.manageSounds("fields.file.placeholder"),
    addTooltip: t.manageSounds("tooltips.add_sound"),
    fileFilterName: t.manageSounds("file_picker.filter_name"),
  };

  const showError = (title: string, message: string) => {
    notifications.show({ title, message, color: "red.7" });
  };

  const renderEllipsisValue = (value: string) => (
    <Text lineClamp={1} title={value} style={{ maxWidth: 240 }}>
      {value}
    </Text>
  );

  const handleAddSound = async (name: string, filePath: string) => {
    try {
      await api.sound.addCustomSound(name, filePath);
      invalidateSounds();
    } catch (error) {
      console.error(error);
      showError(
        t.manageSounds("notifications.add_error.title"),
        t.manageSounds("notifications.add_error.message")
      );
    }
  };

  const handleDeleteSound = async (sound: TauriTypes.CustomSound) => {
    try {
      await api.sound.deleteCustomSound(sound.file_name);
      invalidateSounds();
      if (selectedSoundFile === toCustomSoundValue(sound.file_name)) onClearSelectedSound();
    } catch (error) {
      console.error(error);
      showError(
        t.manageSounds("notifications.delete_error.title"),
        t.manageSounds("notifications.delete_error.message")
      );
    }
  };

  const handlePlaySound = (sound: TauriTypes.CustomSound) => {
    PlaySound(toCustomSoundValue(sound.file_name), 1.0);
  };

  const handleDeleteConfirm = (sound: TauriTypes.CustomSound) => {
    const deleteTitle = t.manageSounds("dialog.delete.title", { name: ellipsize(sound.name) });
    modals.openConfirmModal({
      title: deleteTitle,
      children: <Text size="sm">{t.manageSounds("dialog.delete.message")}</Text>,
      labels: {
        confirm: t.manageSounds("dialog.delete.confirm"),
        cancel: t.manageSounds("dialog.delete.cancel"),
      },
      confirmProps: { color: "red" },
      onConfirm: () => handleDeleteSound(sound),
    });
  };

  const columns: DataTableColumn<TauriTypes.CustomSound>[] = [
    {
      accessor: "name",
      title: t.manageSounds("table.columns.name"),
      sortable: true,
      render: (sound: TauriTypes.CustomSound) => renderEllipsisValue(sound.name),
    },
    {
      accessor: "file_name",
      title: t.manageSounds("table.columns.file_name"),
      sortable: true,
    },
    {
      accessor: "actions",
      title: t.manageSounds("table.columns.actions"),
      textAlign: "right",
      render: (sound: TauriTypes.CustomSound) => (
        <Group gap={4} justify="right" wrap="nowrap">
          <ActionWithTooltip
            tooltip={t.manageSounds("tooltips.play")}
            icon={faVolumeHigh}
            color="blue"
            iconProps={{ size: "xs" }}
            actionProps={{ variant: "transparent", size: "sm" }}
            onClick={() => handlePlaySound(sound)}
          />
          <ActionWithTooltip
            tooltip={t.manageSounds("tooltips.delete")}
            icon={faTrash}
            color="red"
            iconProps={{ size: "xs" }}
            actionProps={{ variant: "transparent", size: "sm" }}
            onClick={() => handleDeleteConfirm(sound)}
          />
        </Group>
      ),
    },
  ];

  return (
    <Stack h={"calc(80vh - 100px)"} gap="xs">
      <Box>
        <CreateSoundForm
          onConfirm={handleAddSound}
          copy={copy}
        />
      </Box>

      <SearchField
        value={query}
        onChange={onQueryChange}
        onSearch={() => {}}
      />

      <Box style={{ flex: 1, minHeight: 0 }}>
        <DataTable
          height="100%"
          mt="md"
          withColumnBorders
          withTableBorder
          striped
          fetching={isFetching}
          records={records}
          totalRecords={totalRecords}
          recordsPerPage={pageSize}
          page={page}
          onPageChange={onPageChange}
          recordsPerPageOptions={[5, 10, 15, 20, 25, 50, 100]}
          onRecordsPerPageChange={onPageSizeChange}
          sortStatus={sortStatus}
          onSortStatusChange={onSortStatusChange}
          idAccessor="file_name"
          columns={columns}
        />
      </Box>
      <Button mt="lg" variant="light" onClick={onBack}>
        {t.manageSounds("buttons.back")}
      </Button>
    </Stack>
  );
};

type CreateSoundFormProps = {
  onConfirm: (name: string, filePath: string) => void;
  copy: {
    namePlaceholder: string;
    nameLabel: string;
    filePlaceholder: string;
    fileLabel: string;
    addTooltip: string;
    fileFilterName: string;
  };
};

const CreateSoundForm = ({ onConfirm, copy }: CreateSoundFormProps) => {
  const [name, setName] = useState("");
  const [filePath, setFilePath] = useState("");
  const trimmedName = name.trim();
  const canConfirm = Boolean(trimmedName && filePath);

  const handleBrowse = async () => {
    try {
      const selected = await openFile({
        multiple: false,
        filters: [
          {
            name: copy.fileFilterName,
            extensions: ["mp3", "wav", "ogg"],
          },
        ],
      });
      if (selected && typeof selected === "string") {
        setFilePath(selected);
      }
    } catch (e) {
      console.error(e);
    }
  };

  const handleConfirm = () => {
    if (!canConfirm) return;
    onConfirm(trimmedName, filePath);
    setName("");
    setFilePath("");
  };

  return (
    <Group gap="md" align="flex-start">
      <TextInput
        label={copy.nameLabel}
        placeholder={copy.namePlaceholder}
        value={name}
        onChange={(e) => setName(e.currentTarget.value)}
        required
        w={200}
      />

      <TextInput
        label={copy.fileLabel}
        placeholder={copy.filePlaceholder}
        value={filePath}
        readOnly
        required
        rightSection={
          <ActionWithTooltip
            tooltip={copy.addTooltip}
            icon={faPlus}
            color="green.7"
            onClick={handleConfirm}
            actionProps={{
              disabled: !canConfirm,
            }}
          />
        }
        onClick={handleBrowse}
        style={{ flex: 1, cursor: "pointer" }}
      />
    </Group>
  );
};
