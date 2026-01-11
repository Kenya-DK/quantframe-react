import { TauriTypes } from "$types";
import { UseFormReturnType } from "@mantine/form";
import { useTranslateForms } from "@hooks/useTranslate.hook";
import { Box, Button, Grid, Group, TextInput } from "@mantine/core";
import { TooltipIcon } from "@components/Shared/TooltipIcon";
import { exists } from "@tauri-apps/plugin-fs";
import api from "@api/index";
export type AdvancedPanelProps = {
  form: UseFormReturnType<TauriTypes.Settings>;
};

const getFieldPath = (field: string) => `advanced_settings.${field}`;

export const AdvancedPanel = ({ form }: AdvancedPanelProps) => {
  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`settings.tabs.advanced.${key}`, { ...context }, i18Key);
  const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`fields.${key}`, { ...context }, i18Key);

  const exportLogsMutation = api.log.export_logs();

  const handleBlurValidation = async () => {
    try {
      if (form.values.advanced_settings.wf_log_path.length > 0) {
        const fileExists = await exists(form.values.advanced_settings.wf_log_path);
        console.log("File exists:", fileExists);
        if (!fileExists) {
          form.setFieldValue("has_error", true);
          form.setFieldError(getFieldPath("wf_log_path"), useTranslateFormFields("wf_log_path.errors.not_exists"));
        } else {
          form.setFieldValue("has_error", false);
          form.clearFieldError(getFieldPath("wf_log_path"));
        }
      }
    } catch (error) {
      form.setFieldValue("has_error", true);
      form.setFieldError(getFieldPath("wf_log_path"), useTranslateFormFields("wf_log_path.errors.not_exists"));
    }
  };
  return (
    <Box p={"md"}>
      <Grid>
        <Grid.Col span={4}>
          <Group gap="xs" grow>
            <TextInput
              w={350}
              label={useTranslateFormFields("wf_log_path.label")}
              placeholder={useTranslateFormFields("wf_log_path.placeholder")}
              rightSection={<TooltipIcon label={useTranslateFormFields("wf_log_path.tooltip")} />}
              onBlur={handleBlurValidation}
              radius="md"
              {...form.getInputProps(getFieldPath("wf_log_path"))}
            />
          </Group>
          <Button mt="md" onClick={() => exportLogsMutation.mutate()} color="blue">
            {useTranslateForm("button_export_logs")}
          </Button>
        </Grid.Col>
      </Grid>
    </Box>
  );
};
