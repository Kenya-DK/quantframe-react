import { Box, Button, Checkbox, Group, Tooltip } from "@mantine/core";
import { TauriTypes } from "$types";
import { useTranslateForms } from "@hooks/useTranslate.hook";
import { useForm } from "@mantine/form";

export type AnalyticPanelProps = {
  value: TauriTypes.SettingsAnalytics;
  onSubmit?: (value: TauriTypes.SettingsAnalytics) => void;
};
export const AnalyticPanel = ({ value, onSubmit }: AnalyticPanelProps) => {
  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`settings.tabs.analytics.${key}`, { ...context }, i18Key);
  const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`buttons.${key}`, { ...context }, i18Key);
  const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`fields.${key}`, { ...context }, i18Key);

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
        <Group gap={"md"} mt={25}>
          <Tooltip label={useTranslateFormFields("stock_item.tooltip")}>
            <Checkbox
              label={useTranslateFormFields("stock_item.label")}
              checked={form.values.stock_item}
              onChange={(event) => form.setFieldValue("stock_item", event.currentTarget.checked)}
              error={form.errors.stock_item && useTranslateFormFields("stock_item.error")}
            />
          </Tooltip>
          <Tooltip label={useTranslateFormFields("stock_riven.tooltip")}>
            <Checkbox
              label={useTranslateFormFields("stock_riven.label")}
              checked={form.values.stock_riven}
              onChange={(event) => form.setFieldValue("stock_riven", event.currentTarget.checked)}
              error={form.errors.stock_riven && useTranslateFormFields("stock_riven.error")}
            />
          </Tooltip>
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
            {useTranslateButtons("save.label")}
          </Button>
        </Group>
      </form>
    </Box>
  );
};
