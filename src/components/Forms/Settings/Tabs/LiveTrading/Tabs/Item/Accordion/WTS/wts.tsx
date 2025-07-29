import { Box, Button, Group, NumberInput } from "@mantine/core";
import { TauriTypes } from "$types";
import { useTranslateCommon, useTranslateForms } from "@hooks/useTranslate.hook";
import { useForm } from "@mantine/form";
import { TooltipIcon } from "@components/Shared/TooltipIcon";

export type WTSItemAccordionProps = {
  value: TauriTypes.SettingsStockItem;
  onSubmit: (value: TauriTypes.SettingsStockItem) => void;
};

export const WTSItemAccordion = ({ value, onSubmit }: WTSItemAccordionProps) => {
  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`settings.tabs.live_trading.item.wts.${key}`, { ...context }, i18Key);
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
            max={999}
            placeholder={useTranslateFormFields("min_profit.placeholder")}
            value={form.values.min_profit}
            onChange={(event) => form.setFieldValue("min_profit", Number(event))}
            error={form.errors.min_profit && useTranslateFormFields("min_profit.error")}
            rightSection={<TooltipIcon label={useTranslateFormFields("min_profit.tooltip")} link={useTranslateFormFields("min_profit.link")} />}
            radius="md"
          />
          <NumberInput
            label={useTranslateFormFields("min_sma.label")}
            min={-1}
            placeholder={useTranslateFormFields("min_sma.placeholder")}
            value={form.values.min_sma}
            onChange={(event) => form.setFieldValue("min_sma", Number(event))}
            error={form.errors.min_sma && useTranslateFormFields("min_sma.error")}
            rightSection={<TooltipIcon label={useTranslateFormFields("min_sma.tooltip")} link={useTranslateFormFields("min_sma.link")} />}
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
