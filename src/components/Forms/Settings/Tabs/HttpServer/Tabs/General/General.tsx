import { Box, Stack, TextInput, Checkbox, Tooltip, Group, Button, NumberInput, Collapse } from "@mantine/core";
import { TauriTypes } from "$types";
import { useForm } from "@mantine/form";
import { useTranslateCommon, useTranslateForms } from "@hooks/useTranslate.hook";
import { TooltipIcon } from "@components/Shared/TooltipIcon";
export type GeneralPanelProps = {
  value: TauriTypes.HttpServerSettings;
  onSubmit: (value: TauriTypes.HttpServerSettings) => void;
  setHideTab?: (value: boolean) => void;
};

export const GeneralPanel = ({ value, onSubmit }: GeneralPanelProps) => {
  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`settings.tabs.http_server.general.${key}`, { ...context }, i18Key);
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
        <Stack>
          <Tooltip label={useTranslateFormFields("enabled.tooltip")}>
            <Checkbox
              label={useTranslateFormFields("enabled.label")}
              checked={form.values.enable}
              onChange={(event) => form.setFieldValue("enable", event.currentTarget.checked)}
              error={form.errors.enable && useTranslateFormFields("enable.error")}
            />
          </Tooltip>
          <Collapse in={form.values.enable}>
            <TextInput
              label={useTranslateFormFields("host.label")}
              {...form.getInputProps("host")}
              rightSection={<TooltipIcon label={useTranslateFormFields("host.tooltip")} />}
            />
            <NumberInput
              label={useTranslateFormFields("port.label")}
              min={0}
              max={65535}
              {...form.getInputProps("port")}
              rightSection={<TooltipIcon label={useTranslateFormFields("port.tooltip")} />}
            />
          </Collapse>
        </Stack>
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
