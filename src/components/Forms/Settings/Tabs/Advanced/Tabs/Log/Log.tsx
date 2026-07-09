import { TauriTypes } from "$types";
import api from "@api/index";
import { TooltipIcon } from "@components/Shared/TooltipIcon";
import { useTranslateForms } from "@hooks/useTranslate.hook";
import { Box, Button, Grid, Group, TextInput } from "@mantine/core";
import { UseFormReturnType } from "@mantine/form";
export type LogPanelProps = {
  form: UseFormReturnType<TauriTypes.Settings>;
};
const getFieldPath = (field: string) => `log_settings.${field}`;
export const LogPanel = ({ form }: LogPanelProps) => {
  const exportLogsMutation = api.log.export_logs();
  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`settings.tabs.advanced.log.${key}`, { ...context }, i18Key);
  const useTranslateFormButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`buttons.${key}`, { ...context }, i18Key);
  const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`fields.${key}`, { ...context }, i18Key);

  return (
    <Box p={"md"}>
      <Grid>
        <Grid.Col span={4}>
          <Group gap="xs" grow>
            <TextInput
              w={350}
              label={useTranslateFormFields("ee_log_path.label")}
              placeholder={useTranslateFormFields("ee_log_path.placeholder")}
              rightSection={<TooltipIcon label={useTranslateFormFields("ee_log_path.tooltip")} />}
              radius="md"
              {...form.getInputProps(getFieldPath("ee_log_path"))}
            />
          </Group>
          <Button mt="md" onClick={() => exportLogsMutation.mutate()} color="blue" loading={exportLogsMutation.isPending}>
            {useTranslateFormButtons("export_logs")}
          </Button>
        </Grid.Col>
      </Grid>
    </Box>
  );
};
