import { Group, Select, Box, Button } from "@mantine/core";
import { TauriTypes } from "$types";
import { useForm } from "@mantine/form";
import { useTranslateCommon, useTranslateForms } from "@hooks/useTranslate.hook";
export type GeneralPanelProps = {
  value: TauriTypes.Settings;
  onSubmit: (value: TauriTypes.Settings) => void;
};

let languages = [
  { label: "German", value: "de" },
  { label: "English", value: "en" },
  { label: "Spanish", value: "es" },
  { label: "French", value: "fr" },
  { label: "Italian", value: "it" },
  { label: "Korean", value: "ko" },
  { label: "Polish", value: "pl" },
  { label: "Portuguese (Brazil)", value: "pt" },
  { label: "Russian", value: "ru" },
  { label: "Ukrainian", value: "uk" },
  { label: "Chinese (Simplified)", value: "zh" },
  { label: "Chinese (Traditional)", value: "tc" },
  { label: "Japanese", value: "ja" },
  { label: "Thai", value: "th" },
  { label: "Turkish", value: "tr" },
];

export const GeneralPanel = ({ onSubmit, value }: GeneralPanelProps) => {
  // States
  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`settings.tabs.general.${key}`, { ...context }, i18Key);
  const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`fields.${key}`, { ...context }, i18Key);

  // Form
  const form = useForm({
    initialValues: value,
    validate: {},
  });

  return (
    <Box h="100%" p={"md"}>
      <form
        onSubmit={form.onSubmit((values) => {
          onSubmit(values);
        })}
      >
        <Group gap="md">
          <Select
            w={150}
            label={useTranslateFormFields("language.label")}
            placeholder={useTranslateFormFields("language.placeholder")}
            data={languages}
            value={form.values.lang}
            onChange={(value) => form.setFieldValue("lang", value || "en")}
            radius="md"
          />
        </Group>

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
    </Box>
  );
};
