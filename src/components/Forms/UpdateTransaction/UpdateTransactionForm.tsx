import { Box, Button, Group, NumberInput } from "@mantine/core";
import { useTranslateCommon, useTranslateForms } from "@hooks/useTranslate.hook";
import { useForm } from "@mantine/form";
import { TauriTypes } from "$types";

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
        <Group justify="flex-end" mt="md">
          <Button type="submit" variant="light" color="blue">
            {useTranslateCommon("buttons.save.label")}
          </Button>
        </Group>
      </form>
    </Box>
  );
}
