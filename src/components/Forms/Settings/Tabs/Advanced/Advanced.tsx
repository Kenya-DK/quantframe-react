import { TauriTypes } from "$types";
import { useForm } from "@mantine/form";
import { useTranslateCommon, useTranslateForms } from "@hooks/useTranslate.hook";
import { Box, Button, Grid, Group, TextInput } from "@mantine/core";
import { TooltipIcon } from "@components/Shared/TooltipIcon";
import api from "@api/index";
export type AdvancedPanelProps = {
  value: TauriTypes.SettingsAdvanced;
  onSubmit: (value: TauriTypes.SettingsAdvanced) => void;
};

export const AdvancedPanel = ({ value, onSubmit }: AdvancedPanelProps) => {
  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`settings.tabs.advanced.${key}`, { ...context }, i18Key);
  const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`fields.${key}`, { ...context }, i18Key);

  const exportLogsMutation = api.log.export_logs();

  // User form
  const form = useForm({
    initialValues: value,
    validate: {},
  });
  return (
    <Box p={"md"}>
      <form
        onSubmit={(e) => {
          e.preventDefault();
          if (onSubmit) onSubmit(form.values);
        }}
      >
        <Grid>
          <Grid.Col span={4}>
            <Group gap="xs" grow>
              <TextInput
                w={350}
                label={useTranslateFormFields("wf_log_path.label")}
                placeholder={useTranslateFormFields("wf_log_path.placeholder")}
                rightSection={<TooltipIcon label={useTranslateFormFields("wf_log_path.tooltip")} />}
                value={form.values.wf_log_path}
                onChange={(event) => form.setFieldValue("wf_log_path", event.currentTarget.value)}
                radius="md"
              />
            </Group>
            <Button mt="md" onClick={() => exportLogsMutation.mutate()} color="blue">
              {useTranslateForm("button_export_logs")}
            </Button>
          </Grid.Col>
          <Grid.Col span={8}>
            <Group></Group>
          </Grid.Col>
        </Grid>
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
