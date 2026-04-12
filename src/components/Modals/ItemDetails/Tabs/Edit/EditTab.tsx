import { Box, Button, Group, NumberInput } from "@mantine/core";
import { TauriTypes } from "$types";
import { useForm } from "@mantine/form";
import { useTranslateModals, useTranslateCommon } from "@hooks/useTranslate.hook";
import { TooltipIcon } from "@components/Shared/TooltipIcon";
export type EditTabProps = {
  lookup: string;
  value: TauriTypes.StockItem | TauriTypes.WishListItem;
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
    return value[key as keyof typeof value] ?? value.properties?.[key];
  };

  const ShowField = (lookupKeys: string[]) => {
    return lookupKeys.some((key) => key === lookup) ? "block" : "none";
  };

  return (
    <form onSubmit={form.onSubmit(async (values) => onSave?.(values))}>
      <Box>
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
          value={GetProperty("minimum_sma") ?? 0}
          onChange={(value) => form.setFieldValue("minimum_sma", Number(value))}
          rightSection={<TooltipIcon label={useTranslateTab("tooltips.minimum_sma")} />}
        />
        <NumberInput
          min={-1}
          display={ShowField(["stock_item"])}
          label={useTranslateFields("minimum_profit")}
          value={GetProperty("minimum_profit") ?? 0}
          onChange={(value) => form.setFieldValue("minimum_profit", Number(value))}
          rightSection={<TooltipIcon label={useTranslateTab("tooltips.minimum_profit")} />}
        />
        <NumberInput
          min={-1}
          display={ShowField(["stock_item", "wish_list_item"])}
          label={useTranslateFields("minimum_price")}
          value={GetProperty("minimum_price") ?? 0}
          onChange={(value) => form.setFieldValue("minimum_price", Number(value))}
          rightSection={<TooltipIcon label={useTranslateTab("tooltips.minimum_price")} />}
        />
        <NumberInput
          min={-1}
          display={ShowField(["wish_list_item"])}
          label={useTranslateFields("maximum_price")}
          value={GetProperty("maximum_price") ?? 0}
          onChange={(value) => form.setFieldValue("maximum_price", Number(value))}
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
