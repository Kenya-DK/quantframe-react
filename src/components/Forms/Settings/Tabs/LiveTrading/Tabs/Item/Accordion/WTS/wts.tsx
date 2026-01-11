import { Box, Group, NumberInput } from "@mantine/core";
import { TauriTypes } from "$types";
import { useTranslateForms } from "@hooks/useTranslate.hook";
import { UseFormReturnType } from "@mantine/form";
import { TooltipIcon } from "@components/Shared/TooltipIcon";

export type WTSItemAccordionProps = {
  form: UseFormReturnType<TauriTypes.Settings>;
};

export const WTSItemAccordion = ({ form }: WTSItemAccordionProps) => {
  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`settings.tabs.live_scraper.item.wts.${key}`, { ...context }, i18Key);
  const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`fields.${key}`, { ...context }, i18Key);

  const getFieldPath = (field: string) => `live_scraper.stock_item.${field}`;
  return (
    <Box h="100%">
      <Group gap={"md"}>
        <NumberInput
          label={useTranslateFormFields("min_profit.label")}
          min={-1}
          max={999}
          placeholder={useTranslateFormFields("min_profit.placeholder")}
          rightSection={<TooltipIcon label={useTranslateFormFields("min_profit.tooltip")} link={useTranslateFormFields("min_profit.link")} />}
          radius="md"
          {...form.getInputProps(getFieldPath("min_profit"))}
        />
        <NumberInput
          label={useTranslateFormFields("min_sma.label")}
          min={-1}
          placeholder={useTranslateFormFields("min_sma.placeholder")}
          rightSection={<TooltipIcon label={useTranslateFormFields("min_sma.tooltip")} link={useTranslateFormFields("min_sma.link")} />}
          radius="md"
          {...form.getInputProps(getFieldPath("min_sma"))}
        />
      </Group>
    </Box>
  );
};
