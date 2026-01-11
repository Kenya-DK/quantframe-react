import { Box, Stack, TextInput, Checkbox, Tooltip, NumberInput, Collapse } from "@mantine/core";
import { TauriTypes } from "$types";
import { UseFormReturnType } from "@mantine/form";
import { useTranslateForms } from "@hooks/useTranslate.hook";
import { TooltipIcon } from "@components/Shared/TooltipIcon";
export type GeneralPanelProps = {
  form: UseFormReturnType<TauriTypes.Settings>;
  setHideTab?: (value: boolean) => void;
};
const getFieldPath = (field: string) => `http_server.${field}`;
export const GeneralPanel = ({ form }: GeneralPanelProps) => {
  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`settings.tabs.http_server.general.${key}`, { ...context }, i18Key);
  const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`fields.${key}`, { ...context }, i18Key);

  return (
    <Box h="100%" p={"md"}>
      <Stack>
        <Tooltip label={useTranslateFormFields("enabled.tooltip")}>
          <Checkbox
            label={useTranslateFormFields("enabled.label")}
            checked={form.values.http_server.enable}
            onChange={(event) => form.setFieldValue(getFieldPath("enable"), event.currentTarget.checked)}
            error={form.errors.enable && useTranslateFormFields("enable.error")}
          />
        </Tooltip>
        <Collapse in={form.values.http_server.enable}>
          <TextInput
            label={useTranslateFormFields("host.label")}
            rightSection={<TooltipIcon label={useTranslateFormFields("host.tooltip")} />}
            {...form.getInputProps(getFieldPath("host"))}
          />
          <NumberInput
            label={useTranslateFormFields("port.label")}
            min={0}
            max={65535}
            {...form.getInputProps(getFieldPath("port"))}
            rightSection={<TooltipIcon label={useTranslateFormFields("port.tooltip")} />}
          />
        </Collapse>
      </Stack>
    </Box>
  );
};
