import { Box, Divider, Group, Text } from "@mantine/core";
import { useTranslateCommon, useTranslateForms } from "@hooks/useTranslate.hook";
import { TauriTypes } from "$types";
import { useQuery } from "@tanstack/react-query";
import api from "@api/index";
import { ItemName } from "../../DataDisplay/ItemName";
import { DynamicForm } from "../DynamicForm";

export type UpdateTradeEntryProps = {
  values?: number[];
  onUpdate: (values: TauriTypes.UpdateTradeEntry) => void;
};
export function UpdateTradeEntry({ values, onUpdate }: UpdateTradeEntryProps) {
  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`trade_entry_update.${key}`, { ...context }, i18Key);
  const useTranslateFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`fields.${key}`, { ...context }, i18Key);

  const { data } = useQuery({
    queryKey: ["trade_entry", values && values.length === 1 ? values[0] : null],
    queryFn: () => api.trade_entry.getById(values![0]),
    enabled: !!values && values.length === 1,
  });

  const formValue = (data ? { ...data, tags: data?.tags?.split(",") } : {}) as TauriTypes.UpdateTradeEntry;

  return (
    <Box w={"100%"}>
      <Group justify="space-between" mb={"md"}>
        {data && <ItemName value={data} />}
        {!data && <Text>{useTranslate("title", { count: values?.length || 0 })}</Text>}
      </Group>
      <Divider />
      <DynamicForm<TauriTypes.UpdateTradeEntry>
        key={data && values?.length === 1 ? `id-${values[0]}` : "multi"}
        value={formValue}
        items={[
          { field: "price", type: "number", label: useTranslateFields("price_label"), props: { min: 0 } },
          { field: "tags", type: "tagsinput", label: useTranslateFields("tags_label") },
        ]}
        onSubmit={(values) => {
          const payload = { ...values };
          if (typeof payload.price !== "number") delete payload.price;
          if (!Array.isArray(payload.tags) || payload.tags.length === 0) delete payload.tags;
          onUpdate(payload);
        }}
        confirmLabel={useTranslateCommon("buttons.save.label")}
      />
    </Box>
  );
}
