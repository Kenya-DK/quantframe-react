import { Box, Group, NumberInput } from "@mantine/core";
import { TauriTypes } from "$types";
import { useTranslateForms } from "@hooks/useTranslate.hook";
import { UseFormReturnType } from "@mantine/form";
import { TooltipIcon } from "@components/Shared/TooltipIcon";

export type RivenPanelProps = {
  form: UseFormReturnType<TauriTypes.Settings>;
};

export const RivenPanel = ({ form }: RivenPanelProps) => {
  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`settings.tabs.live_scraper.riven.${key}`, { ...context }, i18Key);
  const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`fields.${key}`, { ...context }, i18Key);

  // Form
  const getFieldPath = (field: string) => `live_scraper.stock_riven.${field}`;
  return (
    <Box h="100%" p={"md"}>
      <Group gap={"md"}>
        <NumberInput
          label={useTranslateFormFields("min_profit.label")}
          min={-1}
          placeholder={useTranslateFormFields("min_profit.placeholder")}
          rightSection={<TooltipIcon label={useTranslateFormFields("min_profit.tooltip")} link={useTranslateFormFields("min_profit.link")} />}
          radius="md"
          {...form.getInputProps(getFieldPath("min_profit"))}
        />
        <NumberInput
          label={useTranslateFormFields("threshold_percentage.label")}
          min={-1}
          placeholder={useTranslateFormFields("threshold_percentage.placeholder")}
          rightSection={
            <TooltipIcon label={useTranslateFormFields("threshold_percentage.tooltip")} link={useTranslateFormFields("threshold_percentage.link")} />
          }
          radius="md"
          {...form.getInputProps(getFieldPath("threshold_percentage"))}
        />
        <NumberInput
          label={useTranslateFormFields("limit_to.label")}
          min={-1}
          placeholder={useTranslateFormFields("limit_to.placeholder")}
          rightSection={<TooltipIcon label={useTranslateFormFields("limit_to.tooltip")} link={useTranslateFormFields("limit_to.link")} />}
          radius="md"
          {...form.getInputProps(getFieldPath("limit_to"))}
        />
        <NumberInput
          label={useTranslateFormFields("update_interval.label")}
          min={-1}
          placeholder={useTranslateFormFields("update_interval.placeholder")}
          rightSection={
            <TooltipIcon label={useTranslateFormFields("update_interval.tooltip")} link={useTranslateFormFields("update_interval.link")} />
          }
          radius="md"
          {...form.getInputProps(getFieldPath("update_interval"))}
        />
      </Group>
    </Box>
  );
};
