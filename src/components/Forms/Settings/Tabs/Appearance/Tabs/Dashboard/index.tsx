import { TauriTypes } from "$types";
import { CreateCategorySummary } from "@components/Forms/CreateSummaryCategory";
import { SearchField } from "@components/Forms/SearchField";
import { ActionWithTooltip } from "@components/Shared/ActionWithTooltip";
import { TooltipIcon } from "@components/Shared/TooltipIcon";
import { faEdit, faTrashCan } from "@fortawesome/free-solid-svg-icons";
import { useTranslateForms } from "@hooks/useTranslate.hook";
import { Box, Button, Divider, Group, Image, NumberInput, Stack, Text } from "@mantine/core";
import { UseFormReturnType } from "@mantine/form";
import { DataTable } from "mantine-datatable";
import { useEffect, useState } from "react";

enum Mode {
  None = "none",
  CreateOrUpdate = "create",
  EditCategories = "edit_categories",
}
const getFieldPath = (field: string) => `summary_settings.${field}`;
export type DashboardPanelProps = {
  form: UseFormReturnType<TauriTypes.Settings>;
  setHideTab?: (value: boolean) => void;
  setHideButtons?: (value: boolean) => void;
};
export const DashboardPanel = ({ form, setHideTab, setHideButtons }: DashboardPanelProps) => {
  const [mode, setMode] = useState<Mode>(Mode.None);
  const [selectedCategory, setSelectedCategory] = useState<TauriTypes.SummaryCategorySetting | undefined>(undefined);
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`settings.tabs.appearance.dashboard.${key}`, { ...context }, i18Key);
  const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`fields.${key}`, { ...context }, i18Key);
  const useTranslateFormButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`buttons.${key}`, { ...context }, i18Key);
  const useTranslateDataGridBaseColumns = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`datatable.columns.${key}`, { ...context }, i18Key);

  useEffect(() => {
    if (mode === Mode.None) {
      setHideTab && setHideTab(false);
      setHideButtons && setHideButtons(false);
    } else {
      setHideTab && setHideTab(true);
      setHideButtons && setHideButtons(true);
    }
  }, [mode]);

  return (
    <Box p={"md"}>
      {mode === Mode.None && (
        <Stack>
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
          <Divider />
          <Button onClick={() => setMode(Mode.EditCategories)}>{useTranslateFormButtons("edit_categories")}</Button>
        </Stack>
      )}
      {mode === Mode.EditCategories && (
        <Box>
          <SearchField value={""} onChange={(value) => form.setFieldValue("search", value)} onCreate={() => setMode(Mode.CreateOrUpdate)} />
          <DataTable
            height={"65vh"}
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
          <Button fullWidth mt="md" onClick={() => setMode(Mode.None)}>
            {useTranslateFormButtons("back")}
          </Button>
        </Box>
      )}
      {mode === Mode.CreateOrUpdate && (
        <Box>
          <CreateCategorySummary
            value={selectedCategory}
            onSubmit={async (data) => {
              setMode(Mode.EditCategories);
              if (selectedCategory) {
                const categories = form.values.summary_settings.categories.map((cat) => (cat.name === selectedCategory.name ? data : cat));
                form.setFieldValue(getFieldPath("categories"), categories);
              } else form.setFieldValue(getFieldPath("categories"), [...form.values.summary_settings.categories, data]);

              setSelectedCategory(undefined);
              setHideTab && setHideTab(false);
            }}
          />
          <Button fullWidth mt="md" onClick={() => setMode(Mode.EditCategories)}>
            {useTranslateFormButtons("back")}
          </Button>
        </Box>
      )}
    </Box>
  );
};
