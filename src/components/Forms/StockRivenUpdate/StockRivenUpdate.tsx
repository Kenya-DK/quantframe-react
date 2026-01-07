import { TauriTypes } from "$types";
import { useForm } from "@mantine/form";
import { useQuery } from "@tanstack/react-query";
import api from "@api/index";
import { useEffect } from "react";

export type StockRivenUpdateProps = {
  values?: number[];
  onUpdate?: (data: TauriTypes.UpdateStockRiven) => void;
};
export function StockRivenUpdate({ values }: StockRivenUpdateProps) {
  // Translate general
  // const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
  //   useTranslateForms(`stock_riven_update.${key}`, { ...context }, i18Key);

  const { data } = useQuery({
    queryKey: ["stock_riven", values && values.length === 1 ? values[0] : null],
    queryFn: () => api.stock_riven.getById(values![0]),
    enabled: !!values && values.length === 1,
  });

  const form = useForm({
    initialValues: {},
    // initialValues: data?.stock as TauriTypes.UpdateStockRiven,
  });

  useEffect(() => {
    // if (data) form.setValues(data.stock as TauriTypes.UpdateStockRiven);
  }, [data]);

  return (
    <form onSubmit={form.onSubmit(async () => {})}>
      {/* <Group justify="space-between" mb={"md"}>
        {data && <ItemName value={data.stock} />}
        {!data && <Text>{useTranslate("title", { count: values?.length || 0 })}</Text>}
      </Group>
      <Divider />
      <Box h={"50vh"}>
        <NumberInput
          min={0}
          max={16}
          label={useTranslateFields("mastery_rank_label")}
          value={form.values.mastery_rank}
          onChange={(value) => form.setFieldValue("mastery_rank", Number(value))}
        />
        <NumberInput
          min={0}
          label={useTranslateFields("re_rolls_label")}
          value={form.values.re_rolls}
          onChange={(value) => form.setFieldValue("re_rolls", Number(value))}
        />
        <NumberInput
          min={-1}
          label={useTranslateFields("minimum_price_label")}
          value={form.values.minimum_price}
          onChange={(value) => form.setFieldValue("minimum_price", Number(value))}
        />
        <NumberInput
          min={0}
          label={useTranslateFields("bought_label")}
          value={form.values.bought}
          onChange={(value) => form.setFieldValue("bought", Number(value))}
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
      </Box> */}
    </form>
  );
}
