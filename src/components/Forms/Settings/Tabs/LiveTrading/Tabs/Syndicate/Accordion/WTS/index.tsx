import { TauriTypes } from "$types";
import api from "@api/index";
import { TooltipIcon } from "@components/Shared/TooltipIcon";
import { useTranslateForms } from "@hooks/useTranslate.hook";
import { Box, Group, MultiSelect, NumberInput } from "@mantine/core";
import { UseFormReturnType } from "@mantine/form";
import { useQuery } from "@tanstack/react-query";

export type WTSItemAccordionProps = {
  form: UseFormReturnType<TauriTypes.Settings>;
};

export const WTSItemAccordion = ({ form }: WTSItemAccordionProps) => {
  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`settings.tabs.live_scraper.syndicate.wts.${key}`, { ...context }, i18Key);
  const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`fields.${key}`, { ...context }, i18Key);

  // Fetch data from rust side
  const { data } = useQuery({
    queryKey: ["cache_syndicates"],
    queryFn: () => api.cache.getSyndicates(),
  });

  const getFieldPath = (field: string) => `live_scraper.syndicate.wts.${field}`;
  return (
    <Box h="100%">
      <Group gap={"md"}>
        <NumberInput
          label={useTranslateFormFields("max_standing_cost.label")}
          min={-1}
          max={999}
          placeholder={useTranslateFormFields("max_standing_cost.placeholder")}
          rightSection={
            <TooltipIcon label={useTranslateFormFields("max_standing_cost.tooltip")} link={useTranslateFormFields("max_standing_cost.link")} />
          }
          radius="md"
          {...form.getInputProps(getFieldPath("max_standing_cost"))}
        />
        <NumberInput
          label={useTranslateFormFields("volume_threshold.label")}
          min={-1}
          placeholder={useTranslateFormFields("volume_threshold.placeholder")}
          rightSection={
            <TooltipIcon label={useTranslateFormFields("volume_threshold.tooltip")} link={useTranslateFormFields("volume_threshold.link")} />
          }
          radius="md"
          {...form.getInputProps(getFieldPath("volume_threshold"))}
        />
        <NumberInput
          label={useTranslateFormFields("max_price_drop.label")}
          min={-1}
          placeholder={useTranslateFormFields("max_price_drop.placeholder")}
          rightSection={<TooltipIcon label={useTranslateFormFields("max_price_drop.tooltip")} link={useTranslateFormFields("max_price_drop.link")} />}
          radius="md"
          {...form.getInputProps(getFieldPath("max_price_drop"))}
        />
        <NumberInput
          label={useTranslateFormFields("min_listings_below.label")}
          min={-1}
          placeholder={useTranslateFormFields("min_listings_below.placeholder")}
          rightSection={
            <TooltipIcon label={useTranslateFormFields("min_listings_below.tooltip")} link={useTranslateFormFields("min_listings_below.link")} />
          }
          radius="md"
          {...form.getInputProps(getFieldPath("min_listings_below"))}
        />
        <MultiSelect
          label={useTranslateFormFields("max_rank_for_type.label")}
          w="20%"
          data={[
            { label: useTranslateFormFields("max_rank_for_type.options.mod"), value: "mod" },
            { label: useTranslateFormFields("max_rank_for_type.options.arcane"), value: "arcane" },
          ]}
          {...form.getInputProps(getFieldPath("max_rank_for_type"))}
        />
        <MultiSelect
          label={useTranslateFormFields("syndicates.label")}
          w="20%"
          data={data?.filter((syndicate) => syndicate.canSelect).map((syndicate) => ({ label: syndicate.name, value: syndicate.uniqueName })) || []}
          {...form.getInputProps(getFieldPath("syndicates"))}
        />
      </Group>
      <Box mt="md"></Box>
    </Box>
  );
};
