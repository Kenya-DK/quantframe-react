import { Group, NumberInput, BoxProps, Box, MultiSelect } from "@mantine/core";
import { useTranslateForms } from "@hooks/useTranslate.hook";
import { useForm } from "@mantine/form";
import { TauriTypes } from "$types";
import { SelectTradableItem } from "@components/Forms/SelectTradableItem";
import { faPlus } from "@fortawesome/free-solid-svg-icons";
import { ActionWithTooltip } from "@components/Shared/ActionWithTooltip";
import classes from "./DebuggingLiveItemEntryForm.module.css";

export type DebuggingLiveItemEntryFormProps = {
  onSubmit: (values: TauriTypes.DebuggingLiveItemEntry) => void;
  boxProps?: BoxProps;
  disabled?: boolean;
  initialValues?: Partial<TauriTypes.DebuggingLiveItemEntry>;
};

const OPERATION_OPTIONS = [
  { value: "Buy", label: "Buy" },
  { value: "Sell", label: "Sell" },
  { value: "WishList", label: "WishList" },
];

const ORDER_TYPE_OPTIONS = [
  { value: "buy", label: "Buy" },
  { value: "sell", label: "Sell" },
  { value: "closed", label: "Closed" },
];

export function DebuggingLiveItemEntryForm({ disabled, boxProps, onSubmit, initialValues }: DebuggingLiveItemEntryFormProps) {
  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`debugging_live_item_entry.${key}`, { ...context }, i18Key);
  const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`fields.${key}`, { ...context }, i18Key);
  const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`buttons.${key}`, { ...context }, i18Key);

  // Form
  const form = useForm<TauriTypes.DebuggingLiveItemEntry>({
    initialValues: {
      stock_id: initialValues?.stock_id || null,
      wish_list_id: initialValues?.wish_list_id || null,
      wfm_url: initialValues?.wfm_url || "",
      sub_type: initialValues?.sub_type || null,
      priority: initialValues?.priority || 0,
      buy_quantity: initialValues?.buy_quantity || 0,
      sell_quantity: initialValues?.sell_quantity || 0,
      operation: initialValues?.operation || [],
      order_type: initialValues?.order_type || "closed",
    },
    validate: {
      wfm_url: (value) => (value.length === 0 ? useTranslateFormFields("wfm_url.error") : null),
      priority: (value) => (value < 0 ? useTranslateFormFields("priority.error") : null),
      buy_quantity: (value) => (value < 0 ? useTranslateFormFields("buy_quantity.error") : null),
      sell_quantity: (value) => (value < 0 ? useTranslateFormFields("sell_quantity.error") : null),
      operation: (value) => (value.length === 0 ? useTranslateFormFields("operation.error") : null),
    },
  });

  return (
    <Box {...boxProps}>
      <form
        onSubmit={form.onSubmit((data) => {
          if (disabled) return;
          onSubmit(data);
        })}
      >
        <Group>
          <SelectTradableItem
            value={form.values.wfm_url}
            onChange={(item) => {
              form.setFieldValue("wfm_url", item.wfm_url_name);
              form.setFieldValue("sub_type", item.sub_type);
            }}
          />{" "}
          {/* Stock ID */}
          <NumberInput
            label={useTranslateFormFields("stock_id.label")}
            placeholder={useTranslateFormFields("stock_id.placeholder")}
            value={form.values.stock_id || undefined}
            onChange={(value) => form.setFieldValue("stock_id", value ? Number(value) : null)}
            error={form.errors.stock_id}
            radius="md"
            allowNegative={false}
          />
          {/* Wish List ID */}
          <NumberInput
            label={useTranslateFormFields("wish_list_id.label")}
            placeholder={useTranslateFormFields("wish_list_id.placeholder")}
            value={form.values.wish_list_id || undefined}
            onChange={(value) => form.setFieldValue("wish_list_id", value ? Number(value) : null)}
            error={form.errors.wish_list_id}
            radius="md"
            allowNegative={false}
          />
          {/* Priority */}
          <NumberInput
            required
            label={useTranslateFormFields("priority.label")}
            placeholder={useTranslateFormFields("priority.placeholder")}
            value={form.values.priority}
            onChange={(value) => form.setFieldValue("priority", Number(value))}
            error={form.errors.priority}
            radius="md"
            allowNegative={false}
          />
        </Group>
        <Group gap="md" align="end" className={classes.formGroup}>
          {/* Buy Quantity */}
          <NumberInput
            label={useTranslateFormFields("buy_quantity.label")}
            placeholder={useTranslateFormFields("buy_quantity.placeholder")}
            value={form.values.buy_quantity}
            onChange={(value) => form.setFieldValue("buy_quantity", Number(value))}
            error={form.errors.buy_quantity}
            radius="md"
            allowNegative={false}
          />

          {/* Sell Quantity */}
          <NumberInput
            label={useTranslateFormFields("sell_quantity.label")}
            placeholder={useTranslateFormFields("sell_quantity.placeholder")}
            value={form.values.sell_quantity}
            onChange={(value) => form.setFieldValue("sell_quantity", Number(value))}
            error={form.errors.sell_quantity}
            radius="md"
            allowNegative={false}
          />

          {/* Operation */}
          <MultiSelect
            required
            label={useTranslateFormFields("operation.label")}
            placeholder={useTranslateFormFields("operation.placeholder")}
            data={OPERATION_OPTIONS}
            value={form.values.operation}
            onChange={(value) => form.setFieldValue("operation", value)}
            error={form.errors.operation}
            radius="md"
          />

          {/* Submit Button */}
          <ActionWithTooltip
            tooltip={useTranslateButtons("add.tooltip")}
            icon={faPlus}
            color="green.7"
            onClick={() => {}}
            actionProps={{
              type: "submit",
              disabled: disabled || form.values.wfm_url.length <= 0,
            }}
          />
        </Group>
      </form>
    </Box>
  );
}
