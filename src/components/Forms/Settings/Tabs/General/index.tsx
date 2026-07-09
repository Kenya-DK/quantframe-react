import { TauriTypes } from "$types";
import api, { SendTauriEvent } from "@api/index";
import { useTranslateCommon, useTranslateForms } from "@hooks/useTranslate.hook";
import { Box, Button, Group, Select, Stack, Text } from "@mantine/core";
import { UseFormReturnType } from "@mantine/form";
import { modals } from "@mantine/modals";
import { useEffect, useState } from "react";
export type GeneralPanelProps = {
  form: UseFormReturnType<TauriTypes.Settings>;
};

const languages = [
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
  const [defaultSettings, setDefaultSettings] = useState<TauriTypes.Settings | null>(null);

  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`settings.tabs.general.${key}`, { ...context }, i18Key);
  const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`fields.${key}`, { ...context }, i18Key);
  const useTranslateFormButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`buttons.${key}`, { ...context }, i18Key);
  const useTranslateFormPrompt = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`prompt.${key}`, { ...context }, i18Key);

  useEffect(() => {
    api.app.getDefaultSettings().then(setDefaultSettings).catch(console.error);
  }, []);

  const isDefaultSettings = () => {
    return JSON.stringify(form.values) != JSON.stringify(defaultSettings);
  };

  const handleReset = () => {
    modals.openConfirmModal({
      title: useTranslateFormPrompt("reset_settings.title"),
      children: <Text size="sm">{useTranslateFormPrompt("reset_settings.message")}</Text>,
      labels: { confirm: useTranslateFormButtons("prompt_reset_label"), cancel: useTranslateCommon("buttons.cancel.label") },
      confirmProps: { color: "red" },
      onConfirm: async () => {
        const defaults = await api.app.getDefaultSettings();
        await api.app.updateSettings(defaults);
        modals.closeAll();
        SendTauriEvent(TauriTypes.Events.RefreshSettings);
      },
    });
  };

  return (
    <Box h="100%" p={"md"}>
      <Stack>
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
        {isDefaultSettings() && (
          <Button onClick={handleReset} color="red.7" pos={"absolute"} bottom={55} right={45}>
            {useTranslateFormButtons("reset_settings_label")}
          </Button>
        )}
      </Stack>
    </Box>
  );
};

