import { Box, Button, Group, NumberInput } from "@mantine/core";
import { TauriTypes } from "$types";
import { useTranslateCommon, useTranslateForms } from "@hooks/useTranslate.hook";
import { useForm } from "@mantine/form";
import { TooltipIcon } from "@components/TooltipIcon";

export type WTBItemAccordionProps = {
  value: TauriTypes.SettingsStockItem;
  onSubmit: (value: TauriTypes.SettingsStockItem) => void;
};

export const WTBItemAccordion = ({ value, onSubmit }: WTBItemAccordionProps) => {
  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`settings.tabs.live_trading.item.wtb.${key}`, { ...context }, i18Key);
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
            label={useTranslateFormFields("volume_threshold.label")}
            min={-1}
            max={999}
            placeholder={useTranslateFormFields("volume_threshold.placeholder")}
            value={form.values.volume_threshold}
            onChange={(event) => form.setFieldValue("volume_threshold", Number(event))}
            error={form.errors.volume_threshold && useTranslateFormFields("volume_threshold.error")}
            rightSection={
              <TooltipIcon label={useTranslateFormFields("volume_threshold.tooltip")} link={useTranslateFormFields("volume_threshold.link")} />
            }
            radius="md"
          />
          <NumberInput
            label={useTranslateFormFields("profit_threshold.label")}
            min={-1}
            max={999}
            placeholder={useTranslateFormFields("profit_threshold.placeholder")}
            value={form.values.profit_threshold}
            onChange={(event) => form.setFieldValue("profit_threshold", Number(event))}
            error={form.errors.profit_threshold && useTranslateFormFields("profit_threshold.error")}
            rightSection={
              <TooltipIcon label={useTranslateFormFields("profit_threshold.tooltip")} link={useTranslateFormFields("profit_threshold.link")} />
            }
            radius="md"
          />
          <NumberInput
            label={useTranslateFormFields("avg_price_cap.label")}
            min={-1}
            placeholder={useTranslateFormFields("avg_price_cap.placeholder")}
            value={form.values.avg_price_cap}
            onChange={(event) => form.setFieldValue("avg_price_cap", Number(event))}
            error={form.errors.avg_price_cap && useTranslateFormFields("avg_price_cap.error")}
            rightSection={<TooltipIcon label={useTranslateFormFields("avg_price_cap.tooltip")} link={useTranslateFormFields("avg_price_cap.link")} />}
            radius="md"
          />
          <NumberInput
            label={useTranslateFormFields("max_total_price_cap.label")}
            min={-1}
            max={999999}
            placeholder={useTranslateFormFields("max_total_price_cap.placeholder")}
            value={form.values.max_total_price_cap}
            onChange={(event) => form.setFieldValue("max_total_price_cap", Number(event))}
            error={form.errors.max_total_price_cap && useTranslateFormFields("max_total_price_cap.error")}
            rightSection={
              <TooltipIcon label={useTranslateFormFields("max_total_price_cap.tooltip")} link={useTranslateFormFields("max_total_price_cap.link")} />
            }
            radius="md"
          />
          <NumberInput
            label={useTranslateFormFields("min_wtb_profit_margin.label")}
            min={-1}
            placeholder={useTranslateFormFields("min_wtb_profit_margin.placeholder")}
            value={form.values.min_wtb_profit_margin}
            onChange={(event) => form.setFieldValue("min_wtb_profit_margin", Number(event))}
            error={form.errors.min_wtb_profit_margin && useTranslateFormFields("min_wtb_profit_margin.error")}
            rightSection={
              <TooltipIcon
                label={useTranslateFormFields("min_wtb_profit_margin.tooltip")}
                link={useTranslateFormFields("min_wtb_profit_margin.link")}
              />
            }
            radius="md"
          />
        </Group>
        <Group gap={"md"}>
          <NumberInput
            label={useTranslateFormFields("trading_tax_cap.label")}
            min={-1}
            placeholder={useTranslateFormFields("trading_tax_cap.placeholder")}
            value={form.values.trading_tax_cap}
            onChange={(event) => form.setFieldValue("trading_tax_cap", Number(event))}
            error={form.errors.trading_tax_cap && useTranslateFormFields("trading_tax_cap.error")}
            rightSection={
              <TooltipIcon label={useTranslateFormFields("trading_tax_cap.tooltip")} link={useTranslateFormFields("trading_tax_cap.link")} />
            }
            radius="md"
          />

          <NumberInput
            label={useTranslateFormFields("price_shift_threshold.label")}
            min={-1}
            max={100}
            placeholder={useTranslateFormFields("price_shift_threshold.placeholder")}
            value={form.values.price_shift_threshold}
            onChange={(event) => form.setFieldValue("price_shift_threshold", Number(event))}
            error={form.errors.price_shift_threshold && useTranslateFormFields("price_shift_threshold.error")}
            rightSection={
              <TooltipIcon
                label={useTranslateFormFields("price_shift_threshold.tooltip")}
                link={useTranslateFormFields("price_shift_threshold.link")}
              />
            }
            radius="md"
          />
          <NumberInput
            label={useTranslateFormFields("buy_quantity.label")}
            min={1}
            max={999}
            placeholder={useTranslateFormFields("buy_quantity.placeholder")}
            value={form.values.buy_quantity}
            onChange={(event) => form.setFieldValue("buy_quantity", Number(event))}
            error={form.errors.buy_quantity && useTranslateFormFields("buy_quantity.error")}
            rightSection={<TooltipIcon label={useTranslateFormFields("buy_quantity.tooltip")} link={useTranslateFormFields("buy_quantity.link")} />}
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
