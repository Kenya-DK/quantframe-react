import { Grid, Group, Button, Box, NumberInput, Title } from "@mantine/core";
import { TauriTypes } from "$types";
import { useTranslateCommon, useTranslateModals } from "@hooks/useTranslate.hook";
import { useForm } from "@mantine/form";
import { PriceHistoryListItem } from "@components/DataDisplay/PriceHistoryListItem";
import api from "@api/index";
import { useEffect } from "react";

export type EditTabProps = {
  value: TauriTypes.StockItemDetails | undefined;
  onUpdate?: (data: TauriTypes.UpdateStockItem) => void;
};

export function EditTab({ value, onUpdate }: EditTabProps) {
  // Translate general
  const useTranslateTab = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateModals(`stock_item_details.tabs.edit.${key}`, { ...context }, i18Key);
  const useTranslateFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTab(`fields.${key}`, { ...context }, i18Key);

  if (!value) return <></>;
  const form = useForm({
    initialValues: value,
  });
  useEffect(() => {
    form.setValues(value);
  }, [value]);

  return (
    // bought
    // owned
    // price_history
    <form
      onSubmit={form.onSubmit(async (values) => {
        await api.stock_item.update(values.stock);
        onUpdate?.(values.stock);
      })}
    >
      <Box h={"50vh"}>
        <Grid>
          <Grid.Col span={6}>
            <NumberInput min={0} label={useTranslateFields("bought.label")} {...form.getInputProps("stock.bought")} />
            <NumberInput min={1} label={useTranslateFields("owned.label")} {...form.getInputProps("stock.owned")} />
          </Grid.Col>
          <Grid.Col span={6}>
            <Title order={3}>{useTranslateFields("listed")}</Title>
            {form.values.stock.price_history.length > 0 &&
              form.values.stock.price_history
                .sort((a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime())
                .map((price, index) => (
                  <PriceHistoryListItem
                    index={index}
                    onDelete={(index) => {
                      console.log(index);
                      const newHistory = form.values.stock.price_history.filter((_, i) => i !== index);
                      form.setFieldValue("stock.price_history", newHistory);
                    }}
                    key={index}
                    history={price}
                  />
                ))}
          </Grid.Col>
        </Grid>
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
