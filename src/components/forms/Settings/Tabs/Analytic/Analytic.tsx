import { Box, Button, Checkbox, Group, Tooltip } from "@mantine/core";
import { SettingsAnalytics } from "@api/types";
import { useTranslateForms } from "@hooks/useTranslate.hook";
import { useForm } from "@mantine/form";


export type AnalyticPanelProps = {
  value: SettingsAnalytics;
  onSubmit?: (value: SettingsAnalytics) => void;
}
export const AnalyticPanel = ({ value, onSubmit }: AnalyticPanelProps) => {

  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateForms(`settings.tabs.analytics.${key}`, { ...context }, i18Key)
  const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateForm(`buttons.${key}`, { ...context }, i18Key)
  const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateForm(`fields.${key}`, { ...context }, i18Key)

  // User form
  const form = useForm({
    initialValues: value,
    validate: {},
  });
  return (
    <Box p={"md"}>
      <form onSubmit={(e) => {
        e.preventDefault();
        if (onSubmit)
          onSubmit(form.values);
      }}>
        <Group gap={"md"} mt={25}>
          <Tooltip label={useTranslateFormFields('transaction.tooltip')}>
            <Checkbox
              label={useTranslateFormFields('transaction.label')}
              checked={form.values.transaction}
              onChange={(event) => form.setFieldValue('transaction', event.currentTarget.checked)}
              error={form.errors.transaction && useTranslateFormFields('transaction.error')}
            />
          </Tooltip>

        </Group>
        <Group justify="flex-end" style={{
          position: "absolute",
          bottom: 25,
          right: 25,
        }}>
          <Button type="submit" variant="light" color="blue">
            {useTranslateButtons('save.label')}
          </Button>
        </Group>
      </form>
    </Box>
  );
};