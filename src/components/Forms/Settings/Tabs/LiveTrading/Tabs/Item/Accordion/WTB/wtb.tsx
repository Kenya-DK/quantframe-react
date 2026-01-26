import { Box, Group, NumberInput } from "@mantine/core";
import { TauriTypes } from "$types";
import { useTranslateForms } from "@hooks/useTranslate.hook";
import { UseFormReturnType } from "@mantine/form";
import { TooltipIcon } from "@components/Shared/TooltipIcon";

export type WTBItemAccordionProps = {
  form: UseFormReturnType<TauriTypes.Settings>;
};

export const WTBItemAccordion = ({ form }: WTBItemAccordionProps) => {
  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`settings.tabs.live_scraper.item.wtb.${key}`, { ...context }, i18Key);
  const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`fields.${key}`, { ...context }, i18Key);

  const getFieldPath = (field: string) => `live_scraper.stock_item.${field}`;
  return (
    <Box h="100%">
      <Group gap={"md"}>
        <NumberInput
          label={useTranslateFormFields("volume_threshold.label")}
          min={-1}
          max={999}
          placeholder={useTranslateFormFields("volume_threshold.placeholder")}
          rightSection={
            <TooltipIcon label={useTranslateFormFields("volume_threshold.tooltip")} link={useTranslateFormFields("volume_threshold.link")} />
          }
          radius="md"
          {...form.getInputProps(getFieldPath("volume_threshold"))}
        />
        <NumberInput
          label={useTranslateFormFields("profit_threshold.label")}
          min={-1}
          max={999}
          placeholder={useTranslateFormFields("profit_threshold.placeholder")}
          rightSection={
            <TooltipIcon label={useTranslateFormFields("profit_threshold.tooltip")} link={useTranslateFormFields("profit_threshold.link")} />
          }
          radius="md"
          {...form.getInputProps(getFieldPath("profit_threshold"))}
        />
        <NumberInput
          label={useTranslateFormFields("avg_price_cap.label")}
          min={-1}
          placeholder={useTranslateFormFields("avg_price_cap.placeholder")}
          rightSection={<TooltipIcon label={useTranslateFormFields("avg_price_cap.tooltip")} link={useTranslateFormFields("avg_price_cap.link")} />}
          radius="md"
          {...form.getInputProps(getFieldPath("avg_price_cap"))}
        />
        <NumberInput
          label={useTranslateFormFields("max_total_price_cap.label")}
          min={-1}
          max={150_000}
          placeholder={useTranslateFormFields("max_total_price_cap.placeholder")}
          rightSection={
            <TooltipIcon label={useTranslateFormFields("max_total_price_cap.tooltip")} link={useTranslateFormFields("max_total_price_cap.link")} />
          }
          radius="md"
          {...form.getInputProps(getFieldPath("max_total_price_cap"))}
        />
        <NumberInput
          label={useTranslateFormFields("min_wtb_profit_margin.label")}
          min={-1}
          placeholder={useTranslateFormFields("min_wtb_profit_margin.placeholder")}
          rightSection={
            <TooltipIcon
              label={useTranslateFormFields("min_wtb_profit_margin.tooltip")}
              link={useTranslateFormFields("min_wtb_profit_margin.link")}
            />
          }
          radius="md"
          {...form.getInputProps(getFieldPath("min_wtb_profit_margin"))}
        />
      </Group>
      <Group gap={"md"}>
        <NumberInput
          label={useTranslateFormFields("trading_tax_cap.label")}
          min={-1}
          placeholder={useTranslateFormFields("trading_tax_cap.placeholder")}
          rightSection={
            <TooltipIcon label={useTranslateFormFields("trading_tax_cap.tooltip")} link={useTranslateFormFields("trading_tax_cap.link")} />
          }
          radius="md"
          {...form.getInputProps(getFieldPath("trading_tax_cap"))}
        />

        <NumberInput
          label={useTranslateFormFields("price_shift_threshold.label")}
          min={-1}
          max={100}
          placeholder={useTranslateFormFields("price_shift_threshold.placeholder")}
          rightSection={
            <TooltipIcon
              label={useTranslateFormFields("price_shift_threshold.tooltip")}
              link={useTranslateFormFields("price_shift_threshold.link")}
            />
          }
          radius="md"
          {...form.getInputProps(getFieldPath("price_shift_threshold"))}
        />
        <NumberInput
          label={useTranslateFormFields("buy_quantity.label")}
          min={1}
          max={6}
          placeholder={useTranslateFormFields("buy_quantity.placeholder")}
          rightSection={<TooltipIcon label={useTranslateFormFields("buy_quantity.tooltip")} link={useTranslateFormFields("buy_quantity.link")} />}
          radius="md"
          {...form.getInputProps(getFieldPath("buy_quantity"))}
        />
        <NumberInput
          label={useTranslateFormFields("quantity_per_trade.label")}
          min={1}
          max={999}
          placeholder={useTranslateFormFields("quantity_per_trade.placeholder")}
          rightSection={
            <TooltipIcon label={useTranslateFormFields("quantity_per_trade.tooltip")} link={useTranslateFormFields("quantity_per_trade.link")} />
          }
          radius="md"
          {...form.getInputProps(getFieldPath("quantity_per_trade"))}
        />
        <NumberInput
          label={useTranslateFormFields("max_stock_quantity.label")}
          min={-1}
          max={999}
          placeholder={useTranslateFormFields("max_stock_quantity.placeholder")}
          rightSection={
            <TooltipIcon label={useTranslateFormFields("max_stock_quantity.tooltip")} link={useTranslateFormFields("max_stock_quantity.link")} />
          }
          radius="md"
          {...form.getInputProps(getFieldPath("max_stock_quantity"))}
        />
      </Group>
    </Box>
  );
};
