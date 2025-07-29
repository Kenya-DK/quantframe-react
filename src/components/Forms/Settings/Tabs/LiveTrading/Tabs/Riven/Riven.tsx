import { Box, Button, Group, NumberInput } from "@mantine/core";
import { TauriTypes } from "$types";
import { useTranslateCommon, useTranslateForms } from "@hooks/useTranslate.hook";
import { useForm } from "@mantine/form";
import { TooltipIcon } from "@components/Shared/TooltipIcon";

export type RivenPanelProps = {
  value: TauriTypes.SettingsStockRiven;
  onSubmit: (value: TauriTypes.SettingsStockRiven) => void;
};

export const RivenPanel = ({ value, onSubmit }: RivenPanelProps) => {
  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`settings.tabs.live_trading.riven.${key}`, { ...context }, i18Key);
  const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`fields.${key}`, { ...context }, i18Key);

  // Form
  const form = useForm({
    initialValues: value,
    validate: {},
  });
  return (
    <Box h="100%">
      <form onSubmit={form.onSubmit((values) => onSubmit(values))}>
        <Group gap={"md"}>
          <NumberInput
            label={useTranslateFormFields("min_profit.label")}
            min={-1}
            placeholder={useTranslateFormFields("min_profit.placeholder")}
            value={form.values.min_profit}
            onChange={(event) => form.setFieldValue("min_profit", Number(event))}
            error={form.errors.min_profit && useTranslateFormFields("min_profit.error")}
            rightSection={<TooltipIcon label={useTranslateFormFields("min_profit.tooltip")} link={useTranslateFormFields("min_profit.link")} />}
            radius="md"
          />
          <NumberInput
            label={useTranslateFormFields("threshold_percentage.label")}
            min={-1}
            placeholder={useTranslateFormFields("threshold_percentage.placeholder")}
            value={form.values.threshold_percentage}
            onChange={(event) => form.setFieldValue("threshold_percentage", Number(event))}
            error={form.errors.threshold_percentage && useTranslateFormFields("threshold_percentage.error")}
            rightSection={
              <TooltipIcon
                label={useTranslateFormFields("threshold_percentage.tooltip")}
                link={useTranslateFormFields("threshold_percentage.link")}
              />
            }
            radius="md"
          />
          <NumberInput
            label={useTranslateFormFields("limit_to.label")}
            min={-1}
            placeholder={useTranslateFormFields("limit_to.placeholder")}
            value={form.values.limit_to}
            onChange={(event) => form.setFieldValue("limit_to", Number(event))}
            error={form.errors.limit_to && useTranslateFormFields("limit_to.error")}
            rightSection={<TooltipIcon label={useTranslateFormFields("limit_to.tooltip")} link={useTranslateFormFields("limit_to.link")} />}
            radius="md"
          />
          <NumberInput
            label={useTranslateFormFields("update_interval.label")}
            min={-1}
            placeholder={useTranslateFormFields("update_interval.placeholder")}
            value={form.values.update_interval}
            onChange={(event) => form.setFieldValue("update_interval", Number(event))}
            error={form.errors.update_interval && useTranslateFormFields("update_interval.error")}
            rightSection={
              <TooltipIcon label={useTranslateFormFields("update_interval.tooltip")} link={useTranslateFormFields("update_interval.link")} />
            }
            radius="md"
          />
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
            {useTranslateCommon("buttons.save.label")}
          </Button>
        </Group>
      </form>
    </Box>
  );
};
