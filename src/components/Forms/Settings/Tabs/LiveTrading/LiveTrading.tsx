import { Button, Group, NumberInput, Select, Stack, Tooltip, Text, Divider, Tabs, Box, Checkbox, Accordion } from "@mantine/core";
import { useForm } from "@mantine/form";
import { OrderMode, SettingsLiveScraper, StockMode } from "@api/types";
import { useTranslateEnums, useTranslateForms } from "@hooks/useTranslate.hook";
import { useState } from "react";
import { TooltipIcon } from "@components/TooltipIcon";
import { SelectMultipleTradableItems } from "@components/SelectMultipleTradableItems";

export type LiveTradingPanelProps = {
  value: SettingsLiveScraper;
  onSubmit: (value: SettingsLiveScraper) => void;
};

enum ViewMode {
  General = "general",
  Blacklist = "blacklist",
  Whitelist = "whitelist",
}

export const LiveTradingPanel = ({ onSubmit, value }: LiveTradingPanelProps) => {
  const [viewMode, setViewMode] = useState<ViewMode>(ViewMode.General);

  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`settings.tabs.live_trading.${key}`, { ...context }, i18Key);
  const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`fields.${key}`, { ...context }, i18Key);
  const useTranslateTabs = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`tabs.${key}`, { ...context }, i18Key);
  const useTranslateStockMode = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateEnums(`stock_mode.${key}`, { ...context }, i18Key);
  const useTranslateOrderMode = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateEnums(`order_mode.${key}`, { ...context }, i18Key);
  const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`buttons.${key}`, { ...context }, i18Key);

  // Form
  const form = useForm({
    initialValues: value,
    validate: {},
  });

  return (
    <Box h="100%">
      <form
        onSubmit={form.onSubmit(() => {
          onSubmit(form.values);
        })}
        style={{
          height: "100%",
          display: "flex",
          flexDirection: "column",
          justifyContent: "space-between",
        }}
      >
        {viewMode == ViewMode.General && (
          <Tabs h={"82vh"} defaultValue="item" orientation="vertical">
            <Tabs.List>
              <Tabs.Tab value="item">{useTranslateTabs("item")}</Tabs.Tab>
              <Tabs.Tab value="riven">{useTranslateTabs("riven")}</Tabs.Tab>
            </Tabs.List>

            <Tabs.Panel value="item">
              <Accordion defaultValue="general">
                <Accordion.Item value="general">
                  <Accordion.Control>{useTranslateTabs("general")}</Accordion.Control>
                  <Accordion.Panel>
                    <Group gap="md">
                      <Select
                        allowDeselect={false}
                        label={useTranslateFormFields("stock_mode.label")}
                        description={useTranslateFormFields(`stock_mode.description.${form.values.stock_mode}`)}
                        placeholder={useTranslateFormFields("stock_mode.placeholder")}
                        data={Object.values(StockMode).map((status) => {
                          return { value: status, label: useTranslateStockMode(status) };
                        })}
                        value={form.values.stock_mode}
                        onChange={(event) => form.setFieldValue("stock_mode", event as StockMode)}
                        error={form.errors.stock_mode && useTranslateFormFields("stock_mode.error")}
                        radius="md"
                      />
                      <Select
                        allowDeselect={false}
                        label={useTranslateFormFields("order_mode.label")}
                        description={useTranslateFormFields(`order_mode.description.${form.values.stock_item.order_mode}`)}
                        placeholder={useTranslateFormFields("order_mode.placeholder")}
                        data={Object.values(OrderMode).map((status) => {
                          return { value: status, label: useTranslateOrderMode(status) };
                        })}
                        value={form.values.stock_item.order_mode}
                        onChange={(event) => form.setFieldValue("stock_item.order_mode", event as OrderMode)}
                        error={form.errors.order_mode && useTranslateFormFields("order_mode.error")}
                        radius="md"
                      />
                    </Group>
                    <Group gap={"md"} mt={25}>
                      <Tooltip label={useTranslateFormFields("report_to_wfm.tooltip")}>
                        <Checkbox
                          label={useTranslateFormFields("report_to_wfm.label")}
                          checked={form.values.stock_item.report_to_wfm}
                          onChange={(event) => form.setFieldValue("stock_item.report_to_wfm", event.currentTarget.checked)}
                          error={form.errors.report_to_wfm && useTranslateFormFields("report_to_wfm.error")}
                        />
                      </Tooltip>
                      <Tooltip label={useTranslateFormFields("auto_delete.tooltip")}>
                        <Checkbox
                          label={useTranslateFormFields("auto_delete.label")}
                          checked={form.values.stock_item.auto_delete}
                          onChange={(event) => form.setFieldValue("stock_item.auto_delete", event.currentTarget.checked)}
                          error={form.errors.auto_delete && useTranslateFormFields("auto_delete.error")}
                        />
                      </Tooltip>
                      <Tooltip label={useTranslateFormFields("auto_trade.tooltip")}>
                        <Checkbox
                          label={useTranslateFormFields("auto_trade.label")}
                          checked={form.values.stock_item.auto_trade}
                          onChange={(event) => form.setFieldValue("stock_item.auto_trade", event.currentTarget.checked)}
                          error={form.errors.auto_trade && useTranslateFormFields("auto_trade.error")}
                        />
                      </Tooltip>
                      <Tooltip label={useTranslateFormFields("strict_whitelist.tooltip")}>
                        <Checkbox
                          label={useTranslateFormFields("strict_whitelist.label")}
                          checked={form.values.stock_item.strict_whitelist}
                          onChange={(event) => form.setFieldValue("stock_item.strict_whitelist", event.currentTarget.checked)}
                          error={form.errors.strict_whitelist && useTranslateFormFields("strict_whitelist.error")}
                        />
                      </Tooltip>
                    </Group>
                    <Group gap={"md"} mt={25}>
                      <Button
                        color="blue"
                        variant="light"
                        onClick={() => {
                          setViewMode(ViewMode.Blacklist);
                        }}
                      >
                        {useTranslateButtons("blacklist.label")}
                      </Button>
                      <Button
                        color="blue"
                        variant="light"
                        onClick={() => {
                          setViewMode(ViewMode.Whitelist);
                        }}
                      >
                        {useTranslateButtons("whitelist.label")}
                      </Button>
                    </Group>
                  </Accordion.Panel>
                </Accordion.Item>

                <Accordion.Item value="wtb">
                  <Accordion.Control>{useTranslateTabs("wtb")}</Accordion.Control>
                  <Accordion.Panel>
                    <Group gap="md">
                      <NumberInput
                        label={useTranslateFormFields("volume_threshold.label")}
                        placeholder={useTranslateFormFields("volume_threshold.placeholder")}
                        value={form.values.stock_item.volume_threshold}
                        onChange={(event) => form.setFieldValue("stock_item.volume_threshold", Number(event))}
                        error={form.errors.volume_threshold && useTranslateFormFields("volume_threshold.error")}
                        rightSection={<TooltipIcon label={useTranslateFormFields("volume_threshold.tooltip")} />}
                        radius="md"
                      />
                      <NumberInput
                        label={useTranslateFormFields("range_threshold.label")}
                        placeholder={useTranslateFormFields("range_threshold.placeholder")}
                        value={form.values.stock_item.range_threshold}
                        onChange={(event) => form.setFieldValue("stock_item.range_threshold", Number(event))}
                        error={form.errors.range_threshold && useTranslateFormFields("range_threshold.error")}
                        rightSection={<TooltipIcon label={useTranslateFormFields("range_threshold.tooltip")} />}
                        radius="md"
                      />
                      <NumberInput
                        label={useTranslateFormFields("avg_price_cap.label")}
                        placeholder={useTranslateFormFields("avg_price_cap.placeholder")}
                        value={form.values.stock_item.avg_price_cap}
                        onChange={(event) => form.setFieldValue("stock_item.avg_price_cap", Number(event))}
                        error={form.errors.avg_price_cap && useTranslateFormFields("avg_price_cap.error")}
                        rightSection={<TooltipIcon label={useTranslateFormFields("avg_price_cap.tooltip")} />}
                        radius="md"
                      />
                      <NumberInput
                        label={useTranslateFormFields("max_total_price_cap.label")}
                        placeholder={useTranslateFormFields("max_total_price_cap.placeholder")}
                        value={form.values.stock_item.max_total_price_cap}
                        onChange={(event) => form.setFieldValue("stock_item.max_total_price_cap", Number(event))}
                        error={form.errors.max_total_price_cap && useTranslateFormFields("max_total_price_cap.error")}
                        rightSection={<TooltipIcon label={useTranslateFormFields("max_total_price_cap.tooltip")} />}
                        radius="md"
                      />
                      <NumberInput
                        label={useTranslateFormFields("buy_quantity.label")}
                        placeholder={useTranslateFormFields("buy_quantity.placeholder")}
                        value={form.values.stock_item.buy_quantity}
                        onChange={(event) => form.setFieldValue("stock_item.buy_quantity", Number(event))}
                        error={form.errors.buy_quantity && useTranslateFormFields("buy_quantity.error")}
                        rightSection={<TooltipIcon label={useTranslateFormFields("buy_quantity.tooltip")} />}
                        radius="md"
                      />
                    </Group>
                    <Group gap="md">
                      <NumberInput
                        label={useTranslateFormFields("price_shift_threshold.label")}
                        placeholder={useTranslateFormFields("price_shift_threshold.placeholder")}
                        value={form.values.stock_item.price_shift_threshold}
                        onChange={(event) => form.setFieldValue("stock_item.price_shift_threshold", Number(event))}
                        error={form.errors.price_shift_threshold && useTranslateFormFields("price_shift_threshold.error")}
                        rightSection={<TooltipIcon label={useTranslateFormFields("price_shift_threshold.tooltip")} />}
                        radius="md"
                      />
                      <NumberInput
                        label={useTranslateFormFields("trading_tax_cap.label")}
                        placeholder={useTranslateFormFields("trading_tax_cap.placeholder")}
                        value={form.values.stock_item.trading_tax_cap}
                        onChange={(event) => form.setFieldValue("stock_item.trading_tax_cap", Number(event))}
                        error={form.errors.trading_tax_cap && useTranslateFormFields("trading_tax_cap.error")}
                        rightSection={<TooltipIcon label={useTranslateFormFields("trading_tax_cap.tooltip")} />}
                        radius="md"
                      />
                    </Group>
                  </Accordion.Panel>
                </Accordion.Item>

                <Accordion.Item value="wts">
                  <Accordion.Control>{useTranslateTabs("wts")}</Accordion.Control>
                  <Accordion.Panel>
                    <Group gap="md">
                      <NumberInput
                        label={useTranslateFormFields("item_min_profit.label")}
                        placeholder={useTranslateFormFields("item_min_profit.placeholder")}
                        value={form.values.stock_item.min_profit}
                        onChange={(event) => form.setFieldValue("stock_item.min_profit", Number(event))}
                        error={form.errors.item_min_profit && useTranslateFormFields("item_min_profit.error")}
                        rightSection={<TooltipIcon label={useTranslateFormFields("item_min_profit.tooltip")} />}
                        radius="md"
                      />
                      <NumberInput
                        label={useTranslateFormFields("min_sma.label")}
                        placeholder={useTranslateFormFields("min_sma.placeholder")}
                        value={form.values.stock_item.min_sma}
                        onChange={(event) => form.setFieldValue("stock_item.min_sma", Number(event))}
                        error={form.errors.min_sma && useTranslateFormFields("min_sma.error")}
                        rightSection={<TooltipIcon label={useTranslateFormFields("min_sma.tooltip")} />}
                        radius="md"
                      />
                    </Group>
                  </Accordion.Panel>
                </Accordion.Item>
              </Accordion>
            </Tabs.Panel>
            <Tabs.Panel value="riven" p="md">
              <Group gap="md">
                <NumberInput
                  label={useTranslateFormFields("riven_min_profit.label")}
                  placeholder={useTranslateFormFields("min_profit.placeholder")}
                  value={form.values.stock_riven.min_profit}
                  onChange={(event) => form.setFieldValue("stock_riven.min_profit", Number(event))}
                  error={form.errors.min_profit && useTranslateFormFields("min_profit.error")}
                  rightSection={<TooltipIcon label={useTranslateFormFields("min_profit.tooltip")} />}
                  radius="md"
                />
                <NumberInput
                  label={useTranslateFormFields("limit_to.label")}
                  placeholder={useTranslateFormFields("limit_to.placeholder")}
                  value={form.values.stock_riven.limit_to}
                  onChange={(event) => form.setFieldValue("stock_riven.limit_to", Number(event))}
                  error={form.errors.limit_to && useTranslateFormFields("limit_to.error")}
                  rightSection={<TooltipIcon label={useTranslateFormFields("limit_to.tooltip", { count: form.values.stock_riven.limit_to })} />}
                  radius="md"
                />
                <NumberInput
                  label={useTranslateFormFields("threshold_percentage.label")}
                  placeholder={useTranslateFormFields("threshold_percentage.placeholder")}
                  value={form.values.stock_riven.threshold_percentage}
                  onChange={(event) => form.setFieldValue("stock_riven.threshold_percentage", Number(event))}
                  error={form.errors.threshold_percentage && useTranslateFormFields("threshold_percentage.error")}
                  rightSection={
                    <TooltipIcon
                      label={useTranslateFormFields("threshold_percentage.tooltip", { value: form.values.stock_riven.threshold_percentage })}
                    />
                  }
                  suffix=" %"
                  radius="md"
                />
              </Group>
            </Tabs.Panel>
          </Tabs>
        )}
        {viewMode == ViewMode.Blacklist && (
          <Stack gap={"md"} mt={25}>
            <Text>{useTranslateFormFields("blacklist.description")}</Text>
            <Divider />
            <SelectMultipleTradableItems
              leftTitle={useTranslateFormFields("blacklist.left_title")}
              rightTitle={useTranslateFormFields("blacklist.right_title")}
              onChange={(items) => {
                form.setFieldValue("stock_item.blacklist", items);
              }}
              selectedItems={form.values.stock_item.blacklist || []}
            />
            <Button
              color="blue"
              variant="light"
              onClick={() => {
                setViewMode(ViewMode.General);
              }}
            >
              {useTranslateButtons("go_back.label")}
            </Button>
          </Stack>
        )}
        {viewMode == ViewMode.Whitelist && (
          <Stack gap={"md"} mt={25}>
            <Text>{useTranslateFormFields("whitelist.description")}</Text>
            <Divider />
            <SelectMultipleTradableItems
              leftTitle={useTranslateFormFields("whitelist.left_title")}
              rightTitle={useTranslateFormFields("whitelist.right_title")}
              onChange={(items) => {
                form.setFieldValue("stock_item.whitelist", items);
              }}
              selectedItems={form.values.stock_item.whitelist || []}
            />
            <Button
              color="blue"
              variant="light"
              onClick={() => {
                setViewMode(ViewMode.General);
              }}
            >
              {useTranslateButtons("go_back.label")}
            </Button>
          </Stack>
        )}

        {viewMode == ViewMode.General && (
          <Group
            justify="flex-end"
            style={{
              position: "absolute",
              bottom: 25,
              right: 25,
            }}
          >
            <Button type="submit" variant="light" color="blue">
              {useTranslateButtons("save.label")}
            </Button>
          </Group>
        )}
      </form>
    </Box>
  );
};
