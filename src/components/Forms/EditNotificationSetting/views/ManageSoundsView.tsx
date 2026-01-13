import { Box, Button, Group, Stack, Text, TextInput } from "@mantine/core";
import { DataTable, DataTableSortStatus } from "mantine-datatable";
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

  const copy = {
    namePlaceholder: t.manageSounds("fields.name.placeholder"),
    filePlaceholder: t.manageSounds("fields.file.placeholder"),
    addTooltip: t.manageSounds("tooltips.add_sound"),
    fileFilterName: t.manageSounds("file_picker.filter_name"),
  };

  const handleAddSound = async (name: string, filePath: string) => {
    try {
      await api.sound.addCustomSound(name, filePath);
      invalidateSounds();
    } catch (error) {
      console.error(error);
      notifications.show({
        title: t.manageSounds("notifications.add_error.title"),
        message: t.manageSounds("notifications.add_error.message"),
        color: "red.7",
      });
    }
  };

  const handleDeleteSound = async (sound: TauriTypes.CustomSound) => {
    try {
      await api.sound.deleteCustomSound(sound.file_name);
      invalidateSounds();
      if (selectedSoundFile === toCustomSoundValue(sound.file_name)) onClearSelectedSound();
    } catch (error) {
      console.error(error);
      notifications.show({
        title: t.manageSounds("notifications.delete_error.title"),
        message: t.manageSounds("notifications.delete_error.message"),
        color: "red.7",
      });
    }
  };

  return (
    <Stack h={"calc(80vh - 100px)"} gap="xs">
      <Box>
        <Text size="sm" fw={500}>
          {t.manageSounds("title")}
        </Text>
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
          columns={[
            {
              accessor: "name",
              title: t.manageSounds("table.columns.name"),
              sortable: true,
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
              render: (sound) => (
                <Group gap={4} justify="right" wrap="nowrap">
                  <ActionWithTooltip
                    tooltip={t.manageSounds("tooltips.play")}
                    icon={faVolumeHigh}
                    color="blue"
                    iconProps={{ size: "xs" }}
                    actionProps={{ variant: "transparent", size: "sm" }}
                    onClick={() => PlaySound(toCustomSoundValue(sound.file_name), 1.0)}
                  />
                  <ActionWithTooltip
                    tooltip={t.manageSounds("tooltips.delete")}
                    icon={faTrash}
                    color="red"
                    iconProps={{ size: "xs" }}
                    actionProps={{ variant: "transparent", size: "sm" }}
                    onClick={() => {
                      modals.openConfirmModal({
                        title: t.manageSounds("dialog.delete.title", { name: sound.name }),
                        children: (
                          <Text size="sm">
                            {t.manageSounds("dialog.delete.message")}
                          </Text>
                        ),
                        labels: {
                          confirm: t.manageSounds("dialog.delete.confirm"),
                          cancel: t.manageSounds("dialog.delete.cancel"),
                        },
                        confirmProps: { color: "red" },
                        onConfirm: async () => {
                          await handleDeleteSound(sound);
                        },
                      });
                    }}
                  />
                </Group>
              ),
            },
          ]}
        />
      </Box>
      <Button
          mt="lg"
          variant="light"
          onClick={onBack}
        >
          {t.manageSounds("buttons.back")}
        </Button>
    </Stack>
  );
};

type CreateSoundFormProps = {
  onConfirm: (name: string, filePath: string) => void;
  copy: {
    namePlaceholder: string;
    filePlaceholder: string;
    addTooltip: string;
    fileFilterName: string;
  };
};

const CreateSoundForm = ({ onConfirm, copy }: CreateSoundFormProps) => {
  const [name, setName] = useState("");
  const [filePath, setFilePath] = useState("");

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

  return (
    <Group gap="md" align="flex-start">
      <TextInput
        placeholder={copy.namePlaceholder}
        value={name}
        onChange={(e) => setName(e.currentTarget.value)}
        required
        w={200}
      />

      <TextInput
        placeholder={copy.filePlaceholder}
        value={filePath}
        readOnly
        rightSection={
          <ActionWithTooltip
            tooltip={copy.addTooltip}
            icon={faPlus}
            color="green.7"
            onClick={() => {
              if (name && filePath) {
                onConfirm(name, filePath);
                setName("");
                setFilePath("");
              }
            }}
            actionProps={{
              disabled: !name || !filePath,
            }}
          />
        }
        onClick={handleBrowse}
        style={{ flex: 1, cursor: "pointer" }}
      />
    </Group>
  );
};
