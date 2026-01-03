import { Box, Button, Collapse, Group, JsonInput, NumberInput, TextInput } from "@mantine/core";
import { useTranslateCommon, useTranslateForms } from "@hooks/useTranslate.hook";
import { useForm } from "@mantine/form";
import { TauriTypes } from "$types";
import { DateTimePicker } from "@mantine/dates";
import dayjs from "dayjs";
import utc from "dayjs/plugin/utc";
import { useState } from "react";

dayjs.extend(utc);

export type UpdateTransactionProps = {
  value?: TauriTypes.TransactionDto;
  onSubmit: (values: TauriTypes.UpdateTransaction) => void;
};
export function UpdateTransaction({ value, onSubmit }: UpdateTransactionProps) {
  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`update_transaction.${key}`, { ...context }, i18Key);
  const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`fields.${key}`, { ...context }, i18Key);

  const [properties, setProperties] = useState<string>(value?.properties ? JSON.stringify(value.properties, null, 2) : "");
  const [showProperties, setShowProperties] = useState<boolean>(false);
  // User form
  const form = useForm({
    initialValues: {
      ...value,
    },
    validate: {},
  });

  return (
    <Box w={"100%"}>
      <form
        onSubmit={form.onSubmit((data) => {
          onSubmit(data as TauriTypes.UpdateTransaction);
        })}
      >
        <NumberInput
          label={useTranslateFormFields("price.label")}
          description={useTranslateFormFields("price.description")}
          placeholder={useTranslateFormFields("price.placeholder")}
          min={0}
          value={form.values.price || 0}
          onChange={(event) => form.setFieldValue("price", Number(event))}
          error={form.errors.price && useTranslateFormFields("price.error")}
          radius="md"
        />
        <NumberInput
          label={useTranslateFormFields("quantity.label")}
          description={useTranslateFormFields("quantity.description")}
          placeholder={useTranslateFormFields("quantity.placeholder")}
          min={0}
          value={form.values.quantity || 0}
          onChange={(event) => form.setFieldValue("quantity", Number(event))}
          error={form.errors.quantity && useTranslateFormFields("quantity.error")}
          radius="md"
        />
        <TextInput
          label={useTranslateFormFields("user_name.label")}
          description={useTranslateFormFields("user_name.description")}
          placeholder={useTranslateFormFields("user_name.placeholder")}
          value={form.values.user_name || ""}
          onChange={(event) => form.setFieldValue("user_name", event.currentTarget.value)}
          error={form.errors.user_name && useTranslateFormFields("user_name.error")}
          radius="md"
          mt="md"
        />
        <DateTimePicker
          label={useTranslateFormFields("created_at.label")}
          description={useTranslateFormFields("created_at.description")}
          placeholder={useTranslateFormFields("created_at.placeholder")}
          value={form.values.created_at ? new Date(form.values.created_at) : null}
          onChange={(e) => form.setFieldValue("created_at", e ? dayjs(e).utc().toISOString() : undefined)}
          error={form.errors.created_at && useTranslateFormFields("created_at.error")}
          radius="md"
          mt="md"
        />
        <Button mt="md" onClick={() => setShowProperties((prev) => !prev)}>
          {showProperties ? useTranslateForm("buttons.hide_properties.label") : useTranslateForm("buttons.show_properties.label")}
        </Button>
        <Collapse in={showProperties}>
          <JsonInput
            label={useTranslateFormFields("properties.label")}
            description={useTranslateFormFields("properties.description")}
            placeholder={useTranslateFormFields("properties.placeholder")}
            value={properties}
            onChange={(event) => {
              try {
                setProperties(event);
                const parsed = JSON.parse(event);
                form.setFieldValue("properties", parsed);
              } catch {
                // form.setFieldValue("properties", undefined);
              }
            }}
            error={form.errors.properties && useTranslateFormFields("properties.error")}
            radius="md"
            mt="md"
            autosize
            minRows={4}
          />
        </Collapse>
        <Group justify="flex-end" mt="md">
          <Button type="submit" variant="light" color="blue">
            {useTranslateCommon("buttons.save.label")}
          </Button>
        </Group>
      </form>
    </Box>
  );
}
