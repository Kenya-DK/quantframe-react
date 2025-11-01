import { Box, Button, Text, Divider, Group, NumberInput } from "@mantine/core";
import { useTranslateCommon, useTranslateForms } from "@hooks/useTranslate.hook";
import { TauriTypes } from "$types";
import { useForm } from "@mantine/form";
import { useQuery } from "@tanstack/react-query";
import api from "@api/index";
import { useEffect } from "react";
import { ItemName } from "@components/DataDisplay/ItemName";
import { TooltipIcon } from "../../Shared/TooltipIcon";

export type StockItemUpdateProps = {
  values?: number[];
  onUpdate?: (data: TauriTypes.UpdateStockItem) => void;
};
export function StockItemUpdate({ values, onUpdate }: StockItemUpdateProps) {
  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`stock_item_update.${key}`, { ...context }, i18Key);
  const useTranslateFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`fields.${key}`, { ...context }, i18Key);

  const { data } = useQuery({
    queryKey: ["stock_item", values && values.length === 1 ? values[0] : null],
    queryFn: () => api.stock_item.getById(values![0]),
    enabled: !!values && values.length === 1,
  });

  const form = useForm({
    initialValues: data?.stock as TauriTypes.UpdateStockItem,
  });

  useEffect(() => {
    if (data) form.setValues(data.stock as TauriTypes.UpdateStockItem);
  }, [data]);

  return (
    <form onSubmit={form.onSubmit(async (values) => onUpdate?.(values))}>
      <Group justify="space-between" mb={"md"}>
        {data && <ItemName value={data.stock} />}
        {!data && <Text>{useTranslate("title", { count: values?.length || 0 })}</Text>}
      </Group>
      <Divider />
      <Box h={"50vh"}>
        <NumberInput
          min={0}
          label={useTranslateFields("bought_label")}
          value={form.values.bought}
          onChange={(value) => form.setFieldValue("bought", Number(value))}
        />
        <NumberInput
          min={1}
          label={useTranslateFields("owned_label")}
          value={form.values.owned}
          onChange={(value) => form.setFieldValue("owned", Number(value))}
        />
        <NumberInput
          min={-1}
          label={useTranslateFields("minimum_sma_label")}
          value={form.values.minimum_sma}
          onChange={(value) => form.setFieldValue("minimum_sma", Number(value))}
          rightSection={<TooltipIcon label={useTranslateFields("minimum_sma_tooltip")} />}
        />
        <NumberInput
          min={-1}
          label={useTranslateFields("minimum_profit_label")}
          value={form.values.minimum_profit}
          onChange={(value) => form.setFieldValue("minimum_profit", Number(value))}
          rightSection={<TooltipIcon label={useTranslateFields("minimum_profit_tooltip")} />}
        />
        <NumberInput
          min={-1}
          label={useTranslateFields("minimum_price_label")}
          value={form.values.minimum_price}
          onChange={(value) => form.setFieldValue("minimum_price", Number(value))}
          rightSection={<TooltipIcon label={useTranslateFields("minimum_price_tooltip")} />}
        />
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
      </Box>
    </form>
  );
}
