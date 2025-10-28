import { TauriTypes } from "$types";
import { useState } from "react";
import { useTranslateCommon, useTranslateForms } from "@hooks/useTranslate.hook";
import { useForm } from "@mantine/form";
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
  value: TauriTypes.SettingsSummary;
  onSubmit?: (value: TauriTypes.SettingsSummary) => void;
};

export const SummaryPanel = ({ value, onSubmit }: SummaryPanelProps) => {
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
  // User form
  const form = useForm({
    initialValues: value,
    validate: {},
  });

  return (
    <Box p={"md"}>
      {mode === Mode.CreateOrUpdate ? (
        <CreateCategorySummary
          value={selectedCategory}
          onSubmit={async (data) => {
            setMode(Mode.None);
            if (selectedCategory) {
              const categories = form.values.categories.map((cat) => (cat.name === selectedCategory.name ? data : cat));
              form.setFieldValue("categories", categories);
            } else form.setFieldValue("categories", [...form.values.categories, data]);

            setSelectedCategory(undefined);
          }}
        />
      ) : (
        <form
          onSubmit={(e) => {
            e.preventDefault();
            if (onSubmit) onSubmit(form.values);
          }}
        >
          <Grid>
            <Grid.Col span={4}>
              <Group gap="xs" grow>
                <NumberInput
                  label={useTranslateFormFields("recent_days.label")}
                  min={1}
                  max={999}
                  placeholder={useTranslateFormFields("recent_days.placeholder")}
                  value={form.values.recent_days}
                  onChange={(event) => form.setFieldValue("recent_days", Number(event))}
                  error={form.errors.recent_days && useTranslateFormFields("recent_days.error")}
                  rightSection={<TooltipIcon label={useTranslateFormFields("recent_days.tooltip")} />}
                  radius="md"
                />
                <NumberInput
                  label={useTranslateFormFields("recent_transactions.label")}
                  min={1}
                  max={999}
                  placeholder={useTranslateFormFields("recent_transactions.placeholder")}
                  value={form.values.recent_transactions}
                  onChange={(event) => form.setFieldValue("recent_transactions", Number(event))}
                  error={form.errors.recent_transactions && useTranslateFormFields("recent_transactions.error")}
                  rightSection={<TooltipIcon label={useTranslateFormFields("recent_transactions.tooltip")} />}
                  radius="md"
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
                records={form.values.categories}
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
                            const categories = form.values.categories.filter((cat) => cat.name !== record.name);
                            form.setFieldValue("categories", categories);
                          }}
                        />
                      </Group>
                    ),
                  },
                ]}
              />
            </Grid.Col>
          </Grid>
          <Group
            justify="flex-end"
            style={{
              position: "absolute",
              bottom: 25,
              right: 25,
            }}
          >
            <Button type="submit" variant="light" color="blue">
              {useTranslateCommon("buttons.save.label")}
            </Button>
          </Group>
        </form>
      )}
    </Box>
  );
};
