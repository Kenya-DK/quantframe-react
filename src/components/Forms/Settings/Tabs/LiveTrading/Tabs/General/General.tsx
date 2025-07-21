import { Group, Select, Tooltip, Box, Checkbox, MultiSelect, Button } from "@mantine/core";
import { TauriTypes } from "$types";
import { useForm } from "@mantine/form";
import { useTranslateCommon, useTranslateEnums, useTranslateForms } from "@hooks/useTranslate.hook";
export type GeneralPanelProps = {
  value: TauriTypes.SettingsLiveScraper;
  onSubmit: (value: TauriTypes.SettingsLiveScraper) => void;
};

export const GeneralPanel = ({ value, onSubmit }: GeneralPanelProps) => {
  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`settings.tabs.live_trading.general.${key}`, { ...context }, i18Key);
  const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`fields.${key}`, { ...context }, i18Key);
  const useTranslateStockMode = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateEnums(`stock_mode.${key}`, { ...context }, i18Key);
  const useTranslateOrderMode = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateEnums(`trade_mode.${key}`, { ...context }, i18Key);

  // Form
  const form = useForm({
    initialValues: value,
    validate: {},
  });
  return (
    <Box h="100%" p={"md"}>
      <form
        onSubmit={form.onSubmit((values) => {
          onSubmit(values);
        })}
      >
        <Group gap="md">
          <Select
            allowDeselect={false}
            label={useTranslateFormFields("stock_mode.label")}
            description={useTranslateFormFields(`stock_mode.description.${form.values.stock_mode}`)}
            placeholder={useTranslateFormFields("stock_mode.placeholder")}
            data={Object.values(TauriTypes.StockMode).map((status) => {
              return { value: status, label: useTranslateStockMode(status) };
            })}
            value={form.values.stock_mode}
            onChange={(event) => form.setFieldValue("stock_mode", event as TauriTypes.StockMode)}
            error={form.errors.stock_mode && useTranslateFormFields("stock_mode.error")}
            radius="md"
          />
          <MultiSelect
            disabled={form.values.stock_mode != TauriTypes.StockMode.Item && form.values.stock_mode != TauriTypes.StockMode.All}
            label={useTranslateFormFields("trade_modes.label")}
            w={250}
            description={useTranslateFormFields(`trade_modes.description`)}
            data={Object.values(TauriTypes.TradeMode).map((status) => {
              return { value: status, label: useTranslateOrderMode(status) };
            })}
            value={form.values.trade_modes}
            onChange={(event) => form.setFieldValue("trade_modes", event as TauriTypes.TradeMode[])}
            error={form.errors.trade_modes && useTranslateFormFields("trade_mode.error")}
            radius="md"
          />
        </Group>
        <Group gap={"md"} mt={25}>
          <Tooltip label={useTranslateFormFields("report_to_wfm.tooltip")}>
            <Checkbox
              label={useTranslateFormFields("report_to_wfm.label")}
              checked={form.values.report_to_wfm}
              onChange={(event) => form.setFieldValue("report_to_wfm", event.currentTarget.checked)}
              error={form.errors.report_to_wfm && useTranslateFormFields("report_to_wfm.error")}
            />
          </Tooltip>
          <Tooltip label={useTranslateFormFields("auto_delete.tooltip")}>
            <Checkbox
              label={useTranslateFormFields("auto_delete.label")}
              checked={form.values.auto_delete}
              onChange={(event) => form.setFieldValue("auto_delete", event.currentTarget.checked)}
              error={form.errors.auto_delete && useTranslateFormFields("auto_delete.error")}
            />
          </Tooltip>
          <Tooltip label={useTranslateFormFields("auto_trade.tooltip")}>
            <Checkbox
              label={useTranslateFormFields("auto_trade.label")}
              checked={form.values.auto_trade}
              onChange={(event) => form.setFieldValue("auto_trade", event.currentTarget.checked)}
              error={form.errors.auto_trade && useTranslateFormFields("auto_trade.error")}
            />
          </Tooltip>
          <Tooltip label={useTranslateFormFields("should_delete_other_types.tooltip")}>
            <Checkbox
              label={useTranslateFormFields("should_delete_other_types.label")}
              checked={form.values.should_delete_other_types}
              onChange={(event) => form.setFieldValue("should_delete_other_types", event.currentTarget.checked)}
              error={form.errors.should_delete_other_types && useTranslateFormFields("should_delete_other_types.error")}
            />
          </Tooltip>
        </Group>
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
      </form>
    </Box>
  );
};
