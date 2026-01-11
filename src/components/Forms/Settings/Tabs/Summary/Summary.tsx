import { TauriTypes } from "$types";
import { useState } from "react";
import { useTranslateCommon, useTranslateForms } from "@hooks/useTranslate.hook";
import { UseFormReturnType } from "@mantine/form";
import { Box, Button, Grid, Group, NumberInput, Text, Image } from "@mantine/core";
import { TooltipIcon } from "@components/Shared/TooltipIcon";
import { SearchField } from "@components/Forms/SearchField";
import { DataTable } from "mantine-datatable";
import { ActionWithTooltip } from "@components/Shared/ActionWithTooltip";
import { faEdit, faTrashCan } from "@fortawesome/free-solid-svg-icons";
import { CreateCategorySummary } from "@components/Forms/CreateSummaryCategory";

enum Mode {
  None = "none",
  CreateOrUpdate = "create",
}

export type SummaryPanelProps = {
  form: UseFormReturnType<TauriTypes.Settings>;
};
const getFieldPath = (field: string) => `summary_settings.${field}`;
export const SummaryPanel = ({ form }: SummaryPanelProps) => {
  // State
  const [mode, setMode] = useState<Mode>(Mode.None);
  const [selectedCategory, setSelectedCategory] = useState<TauriTypes.SettingsCategorySummary | undefined>(undefined);
  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`settings.tabs.summary.${key}`, { ...context }, i18Key);
  const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`fields.${key}`, { ...context }, i18Key);
  const useTranslateDataGridBaseColumns = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`datatable.columns.${key}`, { ...context }, i18Key);

  return (
    <Box p={"md"}>
      {mode === Mode.CreateOrUpdate ? (
        <CreateCategorySummary
          value={selectedCategory}
          onSubmit={async (data) => {
            setMode(Mode.None);
            if (selectedCategory) {
              const categories = form.values.summary_settings.categories.map((cat) => (cat.name === selectedCategory.name ? data : cat));
              form.setFieldValue(getFieldPath("categories"), categories);
            } else form.setFieldValue(getFieldPath("categories"), [...form.values.summary_settings.categories, data]);

            setSelectedCategory(undefined);
          }}
        />
      ) : (
        <>
          <Grid>
            <Grid.Col span={4}>
              <Group gap="xs" grow>
                <NumberInput
                  label={useTranslateFormFields("recent_days.label")}
                  min={1}
                  max={999}
                  placeholder={useTranslateFormFields("recent_days.placeholder")}
                  rightSection={<TooltipIcon label={useTranslateFormFields("recent_days.tooltip")} />}
                  radius="md"
                  {...form.getInputProps(getFieldPath("recent_days"))}
                />
                <NumberInput
                  label={useTranslateFormFields("recent_transactions.label")}
                  min={1}
                  max={999}
                  placeholder={useTranslateFormFields("recent_transactions.placeholder")}
                  rightSection={<TooltipIcon label={useTranslateFormFields("recent_transactions.tooltip")} />}
                  radius="md"
                  {...form.getInputProps(getFieldPath("recent_transactions"))}
                />
              </Group>
            </Grid.Col>
            <Grid.Col span={8}>
              <SearchField value={""} onChange={(value) => form.setFieldValue("search", value)} onCreate={() => setMode(Mode.CreateOrUpdate)} />
              <DataTable
                height={"50vh"}
                withColumnBorders
                striped
                idAccessor={"name"}
                highlightOnHover
                records={form.values.summary_settings.categories}
                columns={[
                  {
                    accessor: "icon",
                    title: useTranslateDataGridBaseColumns("name"),
                    width: 150,
                    render: (record) => (
                      <Group gap="xs" grow>
                        <Image src={record.icon} alt={record.name} width={48} style={{ borderRadius: "50%" }} />
                        <Text>{record.name}</Text>
                      </Group>
                    ),
                  },
                  {
                    accessor: "tags",
                    title: useTranslateDataGridBaseColumns("tags"),
                    render: (record) => record.tags.join(", "),
                  },
                  {
                    accessor: "types",
                    title: useTranslateDataGridBaseColumns("types"),
                    render: (record) => record.types.join(", "),
                  },
                  {
                    accessor: "actions",
                    title: useTranslateDataGridBaseColumns("actions.title"),
                    width: 100,
                    render: (record) => (
                      <Group gap={"sm"} justify="flex-end">
                        <ActionWithTooltip
                          tooltip={useTranslateDataGridBaseColumns("actions.buttons.edit.tooltip")}
                          icon={faEdit}
                          actionProps={{ size: "sm" }}
                          iconProps={{ size: "xs" }}
                          onClick={async (e) => {
                            e.stopPropagation();
                            setSelectedCategory(record);
                            setMode(Mode.CreateOrUpdate);
                          }}
                        />
                        <ActionWithTooltip
                          tooltip={useTranslateDataGridBaseColumns("actions.buttons.delete.tooltip")}
                          color={"red.7"}
                          icon={faTrashCan}
                          actionProps={{ size: "sm" }}
                          iconProps={{ size: "xs" }}
                          onClick={async (e) => {
                            e.stopPropagation();
                            const categories = form.values.summary_settings.categories.filter((cat) => cat.name !== record.name);
                            form.setFieldValue(getFieldPath("categories"), categories);
                          }}
                        />
                      </Group>
                    ),
                  },
                ]}
              />
            </Grid.Col>
          </Grid>
        </>
      )}
    </Box>
  );
};
