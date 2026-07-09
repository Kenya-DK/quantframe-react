import { TauriTypes } from "$types";
import { TooltipIcon } from "@components/Shared/TooltipIcon";
import { useTranslateForms } from "@hooks/useTranslate.hook";
import { Box, Checkbox, Collapse, NumberInput, Stack, TextInput, Tooltip } from "@mantine/core";
import { UseFormReturnType } from "@mantine/form";
export type HttpServerPanelProps = {
  form: UseFormReturnType<TauriTypes.Settings>;
};
const getFieldPath = (field: string) => `advanced_settings.http_server.${field}`;
export const HttpServerPanel = ({ form }: HttpServerPanelProps) => {
  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`settings.tabs.advanced.http_server.${key}`, { ...context }, i18Key);
  const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`fields.${key}`, { ...context }, i18Key);
  return (
    <Box h="100%" p={"md"}>
      <Stack>
        <Tooltip label={useTranslateFormFields("enabled.tooltip")}>
          <Checkbox
            label={useTranslateFormFields("enabled.label")}
            checked={form.values.advanced_settings.http_server.enable}
            onChange={(event) => form.setFieldValue(getFieldPath("enable"), event.currentTarget.checked)}
            error={form.errors.enable && useTranslateFormFields("enable.error")}
          />
        </Tooltip>
        <Collapse expanded={form.values.advanced_settings.http_server.enable}>
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
