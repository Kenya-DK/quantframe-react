import { Group, Select, Box } from "@mantine/core";
import { TauriTypes } from "$types";
import { UseFormReturnType } from "@mantine/form";
import { useTranslateForms } from "@hooks/useTranslate.hook";
export type GeneralPanelProps = {
  form: UseFormReturnType<TauriTypes.Settings>;
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
export const GeneralPanel = ({ form }: GeneralPanelProps) => {
  // States
  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`settings.tabs.general.${key}`, { ...context }, i18Key);
  const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`fields.${key}`, { ...context }, i18Key);

  return (
    <Box h="100%" p={"md"}>
      <Group gap="md">
        <Select
          allowDeselect={false}
          w={150}
          label={useTranslateFormFields("language.label")}
          placeholder={useTranslateFormFields("language.placeholder")}
          data={languages}
          {...form.getInputProps("lang")}
          radius="md"
        />
      </Group>
    </Box>
  );
};
