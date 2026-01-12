import { Box, Button, Group, Stack, Text, TextInput } from "@mantine/core";
import { DataTable, DataTableSortStatus } from "mantine-datatable";
import { SearchField } from "@components/Forms/SearchField";
import { ActionWithTooltip } from "@components/Shared/ActionWithTooltip";
import { useTranslateForms } from "@hooks/useTranslate.hook";
import { faPlus, faTrash, faVolumeHigh } from "@fortawesome/free-solid-svg-icons";
import { modals } from "@mantine/modals";
import { open as openFile } from "@tauri-apps/plugin-dialog";
import { useState } from "react";
import { TauriTypes } from "$types";
import api from "@api/index";
import { PlaySound } from "@utils/helper";

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
  invalidateSettings: () => void;
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
  invalidateSettings,
  selectedSoundFile,
  onClearSelectedSound,
  onBack,
}: ManageSoundsViewProps) => {
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`edit_notification_setting.${key}`, { ...context }, i18Key);
  const useTranslateFormManageSounds = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`manage_sounds.${key}`, { ...context }, i18Key);

  const labels = {
    namePlaceholder: useTranslateFormManageSounds("fields.name.placeholder"),
    filePlaceholder: useTranslateFormManageSounds("fields.file.placeholder"),
    addTooltip: useTranslateFormManageSounds("tooltips.add_sound"),
    fileFilterName: useTranslateFormManageSounds("file_picker.filter_name"),
  };

  const handleAddSound = async (name: string, filePath: string) => {
    await api.sound.addCustomSound(name, filePath);
    invalidateSettings();
  };

  const handleDeleteSound = async (sound: TauriTypes.CustomSound) => {
    await api.sound.deleteCustomSound(sound.file_name);
    invalidateSettings();
    if (selectedSoundFile === `custom:${sound.file_name}`) onClearSelectedSound();
  };

  return (
    <Stack h={"calc(80vh - 100px)"} gap="xs">
      <Box>
        <Text size="sm" fw={500}>
          {useTranslateFormManageSounds("title")}
        </Text>
        <CreateSoundForm
          onConfirm={handleAddSound}
          labels={labels}
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
              title: useTranslateFormManageSounds("table.columns.name"),
              sortable: true,
            },
            {
              accessor: "file_name",
              title: useTranslateFormManageSounds("table.columns.file_name"),
              sortable: true,
            },
            {
              accessor: "actions",
              title: useTranslateFormManageSounds("table.columns.actions"),
              textAlign: "right",
              render: (sound) => (
                <Group gap={4} justify="right" wrap="nowrap">
                  <ActionWithTooltip
                    tooltip={useTranslateFormManageSounds("tooltips.play")}
                    icon={faVolumeHigh}
                    color="blue"
                    iconProps={{ size: "xs" }}
                    actionProps={{ variant: "transparent", size: "sm" }}
                    onClick={() => PlaySound(`custom:${sound.file_name}`, 1.0)}
                  />
                  <ActionWithTooltip
                    tooltip={useTranslateFormManageSounds("tooltips.delete")}
                    icon={faTrash}
                    color="red"
                    iconProps={{ size: "xs" }}
                    actionProps={{ variant: "transparent", size: "sm" }}
                    onClick={() => {
                      modals.openConfirmModal({
                        title: useTranslateFormManageSounds("dialog.delete.title", { name: sound.name }),
                        children: (
                          <Text size="sm">
                            {useTranslateFormManageSounds("dialog.delete.message")}
                          </Text>
                        ),
                        labels: {
                          confirm: useTranslateFormManageSounds("dialog.delete.confirm"),
                          cancel: useTranslateFormManageSounds("dialog.delete.cancel"),
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
          {useTranslateFormManageSounds("buttons.back")}
        </Button>
    </Stack>
  );
};

type CreateSoundFormProps = {
  onConfirm: (name: string, filePath: string) => void;
  labels: {
    namePlaceholder: string;
    filePlaceholder: string;
    addTooltip: string;
    fileFilterName: string;
  };
};

const CreateSoundForm = ({ onConfirm, labels }: CreateSoundFormProps) => {
  const [name, setName] = useState("");
  const [filePath, setFilePath] = useState("");

  const handleBrowse = async () => {
    try {
      const selected = await openFile({
        multiple: false,
        filters: [
          {
            name: labels.fileFilterName,
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
        placeholder={labels.namePlaceholder}
        value={name}
        onChange={(e) => setName(e.currentTarget.value)}
        required
        w={200}
      />

      <TextInput
        placeholder={labels.filePlaceholder}
        value={filePath}
        readOnly
        rightSection={
          <ActionWithTooltip
            tooltip={labels.addTooltip}
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
