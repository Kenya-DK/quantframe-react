import { Box, Button, Group, TagsInput, TextInput } from "@mantine/core";
import { useTranslateForms } from "@hooks/useTranslate.hook";
import { useForm } from "@mantine/form";
import { TauriTypes } from "$types";
import { TooltipIcon } from "@components/Shared/TooltipIcon";
import { SelectItemTags } from "../SelectItemTags";

export type CreateCategorySummaryProps = {
  value?: TauriTypes.SettingsCategorySummary;
  onSubmit: (values: TauriTypes.SettingsCategorySummary) => void;
};

export function CreateCategorySummary({ value, onSubmit }: CreateCategorySummaryProps) {
  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`create_category_summary.${key}`, { ...context }, i18Key);
  const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`fields.${key}`, { ...context }, i18Key);
  const useTranslateButton = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`buttons.${key}`, { ...context }, i18Key);
  // User form
  const form = useForm({
    initialValues: value || {
      icon: "",
      name: "",
      tags: [],
      types: [],
    },
    validate: {},
  });
  return (
    <Box w={"100%"}>
      <form
        onSubmit={form.onSubmit((data) => {
          onSubmit(data as TauriTypes.SettingsCategorySummary);
        })}
      >
        <Group mt="xs">
          <TextInput
            label={useTranslateFormFields("icon.label")}
            placeholder={useTranslateFormFields("icon.placeholder")}
            description={useTranslateFormFields("icon.description")}
            rightSection={<TooltipIcon label={useTranslateFormFields("icon.tooltip")} />}
            value={form.values.icon}
            onChange={(event) => form.setFieldValue("icon", event.currentTarget.value)}
            radius="md"
          />
          <TextInput
            label={useTranslateFormFields("name.label")}
            placeholder={useTranslateFormFields("name.placeholder")}
            description={useTranslateFormFields("name.description")}
            rightSection={<TooltipIcon label={useTranslateFormFields("name.tooltip")} />}
            value={form.values.name}
            onChange={(event) => form.setFieldValue("name", event.currentTarget.value)}
            radius="md"
          />
          <SelectItemTags value={form.values.tags} onChange={(tags) => form.setFieldValue("tags", tags)} />
          <TagsInput
            label={useTranslateFormFields("types.label")}
            description={useTranslateFormFields("types.description")}
            data={[]}
            value={form.values.types}
            onChange={(types) => form.setFieldValue("types", types)}
          />
        </Group>
        <Group mt={"xs"} justify="flex-end">
          <Button type="submit" variant="light" color="blue">
            {useTranslateButton("submit.label")}
          </Button>
        </Group>
      </form>
    </Box>
  );
}
