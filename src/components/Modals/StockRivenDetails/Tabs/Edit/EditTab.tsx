import { Grid, Group, Button, Box, NumberInput, Title } from "@mantine/core";
import { TauriTypes } from "$types";
import { useTranslateCommon, useTranslateModals } from "@hooks/useTranslate.hook";
import { useForm } from "@mantine/form";
import { PriceHistoryListItem } from "@components/DataDisplay/PriceHistoryListItem";
import { useEffect } from "react";

export type EditTabProps = {
  value: TauriTypes.StockRivenDetails | undefined;
  onUpdate?: (data: TauriTypes.UpdateStockRiven) => void;
};

export function EditTab({ value, onUpdate }: EditTabProps) {
  // Translate general
  const useTranslateTab = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateModals(`stock_riven_details.tabs.edit.${key}`, { ...context }, i18Key);
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
    <form
      onSubmit={form.onSubmit(async (values) => {
        onUpdate?.(values.stock);
      })}
    >
      <Box h={"50vh"}>
        <Grid>
          <Grid.Col span={6}>
            <NumberInput min={0} max={16} label={useTranslateFields("mastery_rank_label")} {...form.getInputProps("stock.mastery_rank")} />
            <NumberInput min={0} label={useTranslateFields("re_rolls_label")} {...form.getInputProps("stock.re_rolls")} />
            <NumberInput min={0} label={useTranslateFields("bought_label")} {...form.getInputProps("stock.bought")} />
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
