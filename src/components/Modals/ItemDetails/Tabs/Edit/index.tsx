import { TauriTypes } from "$types";
import { SelectSubType } from "@components/Forms/SelectSubType";
import { TooltipIcon } from "@components/Shared/TooltipIcon";
import { useTranslateCommon, useTranslateModals } from "@hooks/useTranslate.hook";
import { Box, Button, Group, NumberInput } from "@mantine/core";
import { useForm } from "@mantine/form";

interface Properties {
  t_type?: TauriTypes.CacheTradableItemSubType;
  [key: string]: any;
}
export type EditTabProps = {
  lookup: string;
  value: TauriTypes.StockItem<Properties> | TauriTypes.WishListItem<Properties>;
  onSave?: (item: TauriTypes.UpdateStockItem | TauriTypes.UpdateWishListItem) => void;
};

export function EditTab({ lookup, value, onSave }: EditTabProps) {
  // Translate general
  const useTranslateTab = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateModals(`item_details.tabs.edit.${key}`, { ...context }, i18Key);
  const useTranslateFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTab(`fields.${key}`, { ...context }, i18Key);
  const form = useForm({
    initialValues: value as TauriTypes.UpdateStockItem | TauriTypes.UpdateWishListItem,
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
        {GetProperty("t_type") && value.sub_type && (
          <SelectSubType
            availableSubTypes={GetProperty("t_type")}
            value={GetProperty("sub_type")}
            onChange={(value) => form.setFieldValue("sub_type", value)}
          />
        )}

        <NumberInput
          min={0}
          display={ShowField(["stock_item"])}
          label={useTranslateFields("bought")}
          value={GetProperty("bought") ?? 0}
          onChange={(value) => form.setFieldValue("bought", Number(value))}
        />
        <NumberInput
          min={1}
          display={ShowField(["stock_item"])}
          label={useTranslateFields("owned")}
          value={GetProperty("owned") ?? 0}
          onChange={(value) => form.setFieldValue("owned", Number(value))}
        />
        <NumberInput
          min={1}
          display={ShowField(["wish_list_item"])}
          label={useTranslateFields("quantity")}
          value={GetProperty("quantity") ?? 0}
          onChange={(value) => form.setFieldValue("quantity", Number(value))}
        />
        <NumberInput
          min={-1}
          display={ShowField(["stock_item"])}
          label={useTranslateFields("minimum_sma")}
          value={GetProperty("min_sma")}
          onChange={(value) => form.setFieldValue("properties.min_sma", Number(value))}
          rightSection={<TooltipIcon label={useTranslateTab("tooltips.minimum_sma")} />}
        />
        <NumberInput
          min={-1}
          display={ShowField(["stock_item"])}
          label={useTranslateFields("minimum_profit")}
          value={GetProperty("min_profit")}
          onChange={(value) => form.setFieldValue("properties.min_profit", Number(value))}
          rightSection={<TooltipIcon label={useTranslateTab("tooltips.minimum_profit")} />}
        />
        <NumberInput
          min={0}
          display={ShowField(["stock_item", "wish_list_item"])}
          label={useTranslateFields("minimum_price")}
          value={GetProperty("min_price")}
          onChange={(value) => form.setFieldValue("properties.min_price", Number(value))}
          rightSection={<TooltipIcon label={useTranslateTab("tooltips.minimum_price")} />}
        />
        <NumberInput
          display={ShowField(["wish_list_item"])}
          label={useTranslateFields("maximum_price")}
          value={GetProperty("max_price")}
          onChange={(value) => form.setFieldValue("properties.max_price", Number(value))}
          rightSection={<TooltipIcon label={useTranslateTab("tooltips.maximum_price")} />}
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
