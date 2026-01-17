import { Box, Button, Text, Divider, Group, NumberInput } from "@mantine/core";
import { useTranslateCommon, useTranslateForms } from "@hooks/useTranslate.hook";
import { TauriTypes } from "$types";
import { useForm } from "@mantine/form";
import { useQuery } from "@tanstack/react-query";
import api from "@api/index";
import { useEffect } from "react";
import { ItemName } from "@components/DataDisplay/ItemName";

export type WishListItemUpdateProps = {
  values?: number[];
  onUpdate?: (data: TauriTypes.UpdateWishListItem) => void;
};
export function WishListItemUpdate({ values, onUpdate }: WishListItemUpdateProps) {
  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`wish_list_item_update.${key}`, { ...context }, i18Key);
  const useTranslateFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`fields.${key}`, { ...context }, i18Key);

  const { data } = useQuery({
    queryKey: ["wish_list_item", values && values.length === 1 ? values[0] : null],
    queryFn: () => api.wish_list.getById(values![0]),
    enabled: !!values && values.length === 1,
  });

  const form = useForm({
    initialValues: data?.stock as TauriTypes.UpdateWishListItem,
  });

  useEffect(() => {
    if (data) form.setValues(data.stock as TauriTypes.UpdateWishListItem);
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
          label={useTranslateFields("quantity_label")}
          value={form.values.quantity}
          onChange={(value) => form.setFieldValue("quantity", Number(value))}
        />
        <NumberInput
          min={0}
          label={useTranslateFields("minimum_price_label")}
          value={form.values.minimum_price}
          onChange={(value) => form.setFieldValue("minimum_price", Number(value))}
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
