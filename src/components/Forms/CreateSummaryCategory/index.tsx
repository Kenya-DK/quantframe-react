import { Box } from "@mantine/core";
import { useTranslateForms } from "@hooks/useTranslate.hook";
import { TauriTypes } from "$types";
import { TooltipIcon } from "@components/Shared/TooltipIcon";
import { SelectItemTags } from "../SelectItemTags";
import { DynamicForm } from "../DynamicForm";

export type CreateCategorySummaryProps = {
  value?: TauriTypes.SummaryCategorySetting;
  onSubmit: (values: TauriTypes.SummaryCategorySetting) => void;
};

export function CreateCategorySummary({ value, onSubmit }: CreateCategorySummaryProps) {
  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`create_category_summary.${key}`, { ...context }, i18Key);
  const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`fields.${key}`, { ...context }, i18Key);
  const useTranslateButton = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`buttons.${key}`, { ...context }, i18Key);

  return (
    <Box w={"100%"}>
      <DynamicForm<TauriTypes.SummaryCategorySetting>
        value={value ?? { icon: "", name: "", tags: [], types: [] }}
        i18n_key="components.forms.create_category_summary.fields"
        items={[
          {
            type: "group",
            props: { mt: "xs" },
            fields: [
              {
                field: "icon",
                type: "text",
                props: { rightSection: <TooltipIcon label={useTranslateFormFields("icon.tooltip")} /> },
              },
              {
                field: "name",
                type: "text",
                props: { rightSection: <TooltipIcon label={useTranslateFormFields("name.tooltip")} /> },
              },
              {
                field: "tags",
                type: "custom",
                render: ({ value, onChange }) => <SelectItemTags value={value} onChange={onChange} />,
              },
              { field: "types", type: "tagsinput" },
            ],
          },
        ]}
        onSubmit={onSubmit}
        confirmLabel={useTranslateButton("submit.label")}
      />
    </Box>
  );
}
