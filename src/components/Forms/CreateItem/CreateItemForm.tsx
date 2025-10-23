import { Group, NumberInput, BoxProps, Box } from "@mantine/core";
import { useTranslateForms } from "@hooks/useTranslate.hook";
import { useForm } from "@mantine/form";
import { TauriTypes } from "$types";
import { SelectTradableItem } from "@components/Forms/SelectTradableItem";
import { useAppContext } from "@contexts/app.context";
import { faShoppingCart } from "@fortawesome/free-solid-svg-icons";
import { ActionWithTooltip } from "@components/Shared/ActionWithTooltip";

export type CreateItemFormProps = {
  onSubmit: (values: TauriTypes.CreateStockItem) => void;
  idField?: string;
  boxProps?: BoxProps;
  disabled?: boolean;
  hide_bought?: boolean;
  hide_sub_type?: boolean;
  hide_quantity?: boolean;
};

export function CreateItemForm({ hide_quantity, hide_sub_type, idField, hide_bought, disabled, boxProps, onSubmit }: CreateItemFormProps) {
  // Context States
  const { settings } = useAppContext();

  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`create_item.${key}`, { ...context }, i18Key);
  const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`fields.${key}`, { ...context }, i18Key);
  const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`buttons.${key}`, { ...context }, i18Key);

  // User form
  const form = useForm({
    initialValues: {
      wfm_url: "",
      bought: 0,
      quantity: 1,
      minimum_price: 0,
      sub_type: undefined as TauriTypes.SubType | undefined,
    },
    validate: {},
  });
  return (
    <Box {...boxProps}>
      <form
        onSubmit={form.onSubmit((data) => {
          if (disabled) return;
          onSubmit({ ...data, raw: idField ? (data as any)[idField] : data.wfm_url });
        })}
      >
        <Group gap="md">
          <SelectTradableItem
            value={form.values.wfm_url}
            hide_sub_type={hide_sub_type}
            onChange={(item) => {
              form.setFieldValue("wfm_url", item.wfm_url_name);
              form.setFieldValue("wfm_id", item.wfm_id);
              form.setFieldValue("sub_type", item.sub_type);
            }}
          />
          <NumberInput
            display={hide_quantity ? "none" : ""}
            w={100}
            required
            label={useTranslateFormFields("quantity.label")}
            placeholder={useTranslateFormFields("quantity.placeholder")}
            value={form.values.quantity}
            onChange={(event) => form.setFieldValue("quantity", Number(event))}
            error={form.errors.quantity && useTranslateFormFields("quantity.error")}
            radius="md"
            rightSection={
              <ActionWithTooltip
                tooltip={useTranslateButtons(
                  `add.tooltip.${settings?.live_scraper.stock_item.report_to_wfm ? "description_with_report" : "description_without_report"}`
                )}
                icon={faShoppingCart}
                color="green.7"
                onClick={() => {}}
                actionProps={{
                  style: { display: !hide_bought ? "none" : "" },
                  type: "submit",
                  disabled: form.values.wfm_url.length <= 0,
                }}
              />
            }
            rightSectionWidth={40}
          />
          <NumberInput
            display={hide_bought ? "none" : ""}
            w={100}
            required
            label={useTranslateFormFields("bought.label")}
            placeholder={useTranslateFormFields("bought.placeholder")}
            value={form.values.bought}
            onChange={(event) => form.setFieldValue("bought", Number(event))}
            error={form.errors.bought && useTranslateFormFields("bought.error")}
            radius="md"
            rightSection={
              <ActionWithTooltip
                tooltip={useTranslateButtons(
                  `add.tooltip.${settings?.live_scraper.stock_item.report_to_wfm ? "description_with_report" : "description_without_report"}`
                )}
                icon={faShoppingCart}
                color="green.7"
                onClick={() => {}}
                actionProps={{
                  type: "submit",
                  disabled: form.values.wfm_url.length <= 0,
                }}
              />
            }
            rightSectionWidth={40}
          />
        </Group>
      </form>
    </Box>
  );
}
