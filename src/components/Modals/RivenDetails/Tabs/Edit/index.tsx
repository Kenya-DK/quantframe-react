import { TauriTypes } from "$types";
import { TooltipIcon } from "@components/Shared/TooltipIcon";
import { useTranslateCommon, useTranslateModals } from "@hooks/useTranslate.hook";
import { Box, Button, Group, NumberInput } from "@mantine/core";
import { useForm } from "@mantine/form";
export type EditTabProps = {
  lookup: string;
  value: TauriTypes.StockRiven;
  onSave?: (item: TauriTypes.UpdateStockRiven) => void;
};

export function EditTab({ lookup, value, onSave }: EditTabProps) {
  // Translate general
  const useTranslateTab = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateModals(`riven_details.tabs.edit.${key}`, { ...context }, i18Key);
  const useTranslateFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTab(`fields.${key}`, { ...context }, i18Key);
  const form = useForm({
    initialValues: value as TauriTypes.UpdateStockRiven,
  });

  const GetProperty = (key: string) => {
    return (form.values as any)[key] ?? form.values.properties?.[key];
  };

  const ShowField = (lookupKeys: string[]) => {
    return lookupKeys.some((key) => key === lookup) ? "block" : "none";
  };

  return (
    <form onSubmit={form.onSubmit(async (values) => onSave?.(values))}>
      <Box>
        <NumberInput
          min={0}
          display={ShowField(["stock_riven"])}
          label={useTranslateFields("bought")}
          value={GetProperty("bought") ?? 0}
          onChange={(value) => form.setFieldValue("bought", Number(value))}
        />
        <pre>{JSON.stringify(GetProperty("properties.min_price"), null, 2)}</pre>
        <NumberInput
          min={0}
          display={ShowField(["stock_riven"])}
          label={useTranslateFields("mastery_rank")}
          value={GetProperty("mastery_rank") ?? 0}
          onChange={(value) => form.setFieldValue("mastery_rank", Number(value))}
        />
        <NumberInput
          min={0}
          display={ShowField(["stock_riven"])}
          label={useTranslateFields("re_rolls")}
          value={GetProperty("re_rolls") ?? 0}
          onChange={(value) => form.setFieldValue("re_rolls", Number(value))}
        />
        <NumberInput
          min={-1}
          display={ShowField(["stock_riven"])}
          label={useTranslateFields("minimum_price")}
          value={GetProperty("min_price") ?? 0}
          onChange={(value) => form.setFieldValue("properties.min_price", Number(value))}
          rightSection={<TooltipIcon label={useTranslateTab("tooltips.minimum_price")} />}
        />
        <Group mt="md" justify="flex-end">
          <Button type="submit" variant="light" color="blue">
            {useTranslateCommon("buttons.save.label")}
          </Button>
        </Group>
      </Box>
    </form>
  );
}
