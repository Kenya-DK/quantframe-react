import { Group, NumberInput, Box, Button } from "@mantine/core";
import { useTranslateForms } from "@hooks/useTranslate.hook";
import { useForm } from "@mantine/form";
import { TauriTypes } from "$types";

export type UpdateTransactionProps = {
  value: TauriTypes.UpdateTransactionDto;
  onSubmit: (values: TauriTypes.UpdateTransactionDto) => void;
};

export function UpdateTransaction({ value, onSubmit }: UpdateTransactionProps) {
  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`update_transaction.${key}`, { ...context }, i18Key);
  const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`fields.${key}`, { ...context }, i18Key);
  const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`buttons.${key}`, { ...context }, i18Key);

  // User form
  const form = useForm({
    initialValues: {
      id: value.id,
      price: value.price,
      quantity: value.quantity,
    },
    validate: {},
  });
  return (
    <Box w={"100%"}>
      <form
        onSubmit={form.onSubmit((data) => {
          onSubmit(data);
        })}
      >
        <Group gap="md" grow>
          <NumberInput
            required
            w={"50%"}
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
            required
            w={"50%"}
            label={useTranslateFormFields("quantity.label")}
            description={useTranslateFormFields("quantity.description")}
            placeholder={useTranslateFormFields("quantity.placeholder")}
            min={0}
            value={form.values.quantity || 0}
            onChange={(event) => form.setFieldValue("quantity", Number(event))}
            error={form.errors.quantity && useTranslateFormFields("quantity.error")}
            radius="md"
          />
        </Group>
        <Button mt={15} type="submit" color="blue" radius="md" fullWidth>
          {useTranslateButtons("submit")}
        </Button>
      </form>
    </Box>
  );
}
