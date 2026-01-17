import { Box, Button, Divider, Group, NumberInput, TagsInput, Text } from "@mantine/core";
import { useTranslateCommon, useTranslateForms } from "@hooks/useTranslate.hook";
import { useForm } from "@mantine/form";
import { TauriTypes } from "$types";
import { useQuery } from "@tanstack/react-query";
import api from "@api/index";
import { useEffect } from "react";
import { ItemName } from "../../DataDisplay/ItemName";

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

  const form = useForm({
    initialValues: {} as TauriTypes.UpdateTradeEntry,
  });
  useEffect(() => {
    if (data) form.setValues({ ...data, tags: data?.tags?.split(",") } as TauriTypes.UpdateTradeEntry);
  }, [data]);
  return (
    <form
      onSubmit={form.onSubmit((data) => {
        onUpdate(data as TauriTypes.UpdateTradeEntry);
      })}
    >
      <Group justify="space-between" mb={"md"}>
        {data && <ItemName value={data} />}
        {!data && <Text>{useTranslate("title", { count: values?.length || 0 })}</Text>}
      </Group>
      <Divider />
      <Box w={"100%"}>
        <NumberInput
          min={0}
          label={useTranslateFields("price_label")}
          value={form.values.price}
          onChange={(value) => form.setFieldValue("price", Number(value))}
        />
        <TagsInput label={useTranslateFields("tags_label")} value={form.values.tags} onChange={(value) => form.setFieldValue("tags", value)} />
        <Group justify="flex-end" mt="md">
          <Button type="submit" variant="light" color="blue">
            {useTranslateCommon("buttons.save.label")}
          </Button>
        </Group>
      </Box>
    </form>
  );
}
