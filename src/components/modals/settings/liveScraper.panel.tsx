import { useEffect } from "react";
import { useForm } from "@mantine/form";
import { Accordion, Button, Checkbox, Group, NumberInput, Select } from "@mantine/core";
import { useTranslateModal } from "@hooks/index";
import { ISearchKeyParameter, LiveScraperSettings, Wfm } from "$types/index";
import { MultiSelectListBox } from "../../multiSelectListBox";
import { searchByPropertys } from "../../../utils/search.helper";
import { MinMaxField } from "../../MinMaxField";

interface LiveScraperProps {
  settings: LiveScraperSettings | undefined;
  tradable_items: Wfm.ItemDto[];
  updateSettings: (user: Partial<LiveScraperSettings>) => void;
}

export function LiveScraperPanel({ settings, updateSettings, tradable_items }: LiveScraperProps) {
  const roleForm = useForm({
    initialValues: {
      filter: {
        blacklist: {
          tax: {
            min: 0,
            max: "" as number | "",
          },
          mr: {
            min: 0,
            max: "" as number | "",
          },
        },
        whitelist: {
          tax: {
            min: 0,
            max: "" as number | "",
          },
          mr: {
            min: 0,
            max: "" as number | "",
          },
        },
      },
      live_trading: {
        webhook: "",
        stock_mode: "",
        stock_item: {
          volume_threshold: 200,
          range_threshold: 200,
          avg_price_cap: 200,
          price_shift_threshold: 200,
          max_total_price_cap: 200,
          blacklist: "",
          whitelist: "",
          strict_whitelist: true,
          auto_delete: true,
          report_to_wfm: true,
          auto_trade: false,
          order_mode: "both",
        },
        stock_riven: {
          range_threshold: 25,
        },
      },
    },
    validate: {},
  });

  useEffect(() => {
    if (!settings) return;
    // Set Settings from live Scraper
    roleForm.setFieldValue("live_trading", { ...settings, stock_item: { ...settings.stock_item, blacklist: settings.stock_item.blacklist.join(","), whitelist: settings.stock_item.whitelist.join(",") } });
  }, [settings]);


  const getAvailableItems = (filter: { tax: { max: number | "", min: number }, mr: { max: number | "", min: number } }) => {
    let items = tradable_items;
    const filters: ISearchKeyParameter = {};
    if (filter.tax) {
      const { min, max } = filter.tax;
      if (min > 0) {
        filters["trade_tax"] = {
          filters: [
            { operator: "gteq", value: min }
          ]
        }
      }
      if (max != "") {
        filters["trade_tax"] = {
          filters: [
            ...filters["trade_tax"]?.filters ?? [],
            { operator: "lteq", value: max },
          ]
        }
      }

    }
    if (filter.mr) {
      const { min, max } = filter.mr;
      if (min > 0) {
        filters["mr_requirement"] = {
          filters: [
            { operator: "gteq", value: min }
          ]
        }
      }
      if (max != "") {
        filters["mr_requirement"] = {
          filters: [
            ...filters["mr_requirement"]?.filters ?? [],
            { operator: "lteq", value: max },
          ]
        }
      }

    }
    return searchByPropertys(items, filters);
  }

  const useTranslateSettingsModal = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateModal(`settings.panels.live_trading.${key}`, { ...context }, i18Key)
  const useTranslateFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateSettingsModal(`fields.${key}`, { ...context }, i18Key)
  return (
    <form method="post" onSubmit={roleForm.onSubmit(async (data) => {
      updateSettings({
        ...data.live_trading,
        stock_item: {
          ...data.live_trading.stock_item,
          blacklist: data.live_trading.stock_item.blacklist.split(","),
          whitelist: data.live_trading.stock_item.whitelist.split(","),
        },
      })
    })}>
      <Group grow>
        <Accordion defaultValue="accordion_general" w={"100%"}>
          <Accordion.Item value="accordion_general">
            <Accordion.Control>{useTranslateSettingsModal('accordion_general')}</Accordion.Control>
            <Accordion.Panel>
              <Group >
                <Group>
                  <NumberInput
                    required
                    label={useTranslateSettingsModal('volume_threshold')}
                    value={roleForm.values.live_trading.stock_item.volume_threshold}
                    description={useTranslateSettingsModal('volume_threshold_description')}
                    onChange={(value) => roleForm.setFieldValue('live_trading.stock_item.volume_threshold', Number(value))}
                    error={roleForm.errors.volume_threshold && 'Invalid Volume Threshold'}
                  />
                  <NumberInput
                    required
                    label={useTranslateSettingsModal('range_threshold')}
                    value={roleForm.values.live_trading.stock_item.range_threshold}
                    description={useTranslateSettingsModal('range_threshold_description')}
                    onChange={(value) => roleForm.setFieldValue('live_trading.stock_item.range_threshold', Number(value))}
                    error={roleForm.errors.range_threshold && 'Invalid Range Threshold'}
                  />
                  <NumberInput
                    required
                    label={useTranslateSettingsModal('max_total_price_cap')}
                    value={roleForm.values.live_trading.stock_item.max_total_price_cap}
                    description={useTranslateSettingsModal('max_total_price_cap_description')}
                    onChange={(value) => roleForm.setFieldValue('live_trading.stock_item.max_total_price_cap', Number(value))}
                    error={roleForm.errors.max_total_price_cap && 'Invalid Range Threshold'}
                  />
                  <NumberInput
                    required
                    label={useTranslateSettingsModal('avg_price_cap')}
                    value={roleForm.values.live_trading.stock_item.avg_price_cap}
                    description={useTranslateSettingsModal('avg_price_cap_description')}
                    onChange={(value) => roleForm.setFieldValue('live_trading.stock_item.avg_price_cap', Number(value))}
                    error={roleForm.errors.avg_price_cap && 'Invalid Avg Price Cap'}
                  />
                </Group>

              </Group>
              <Group grow mt={10}>
                <Group grow>
                  <NumberInput
                    required
                    label={useTranslateSettingsModal('price_shift_threshold')}
                    value={roleForm.values.live_trading.stock_item.price_shift_threshold}
                    description={useTranslateSettingsModal('price_shift_threshold_description')}
                    onChange={(value) => roleForm.setFieldValue('live_trading.stock_item.price_shift_threshold', Number(value))}
                    error={roleForm.errors.price_shift_threshold && 'Invalid Price Shift Threshold'}
                  />
                  {/* <TextInput
                      label={useTranslateSettingsModal('webhook')}
                      value={roleForm.values.live_trading.webhook}
                      description={useTranslateSettingsModal('webhook_description')}
                      onChange={(event) => roleForm.setFieldValue('live_trading.webhook', event.currentTarget.value)}
                      error={roleForm.errors.webhook && 'Invalid Webhook'}
                    /> */}
                  <Select
                    label={useTranslateFields("stock_mode.label")}
                    description={useTranslateFields(`stock_mode.${roleForm.values.live_trading.stock_mode}_description`)}
                    value={roleForm.values.live_trading.stock_mode}
                    onChange={(event) => roleForm.setFieldValue('live_trading.stock_mode', event || "")}
                    data={[
                      { description: useTranslateFields(`stock_mode.all_description`), value: "all", label: useTranslateFields("stock_mode.options.all") },
                      { description: useTranslateFields(`stock_mode.item_description`), value: "item", label: useTranslateFields("stock_mode.options.item") },
                      { description: useTranslateFields(`stock_mode.riven_description`), value: "riven", label: useTranslateFields("stock_mode.options.riven") },
                    ]}
                  />
                  <Select
                    label={useTranslateFields("order_mode.label")}
                    description={useTranslateFields(`order_mode.${roleForm.values.live_trading.stock_item.order_mode}_description`)}
                    value={roleForm.values.live_trading.stock_item.order_mode}
                    onChange={(event) => roleForm.setFieldValue('live_trading.stock_item.order_mode', event || "")}
                    data={[
                      { description: useTranslateFields(`order_mode.both_description`), value: "both", label: useTranslateFields("order_mode.options.both") },
                      { description: useTranslateFields(`order_mode.buy_description`), value: "buy", label: useTranslateFields("order_mode.options.buy") },
                      { description: useTranslateFields(`order_mode.sell_description`), value: "sell", label: useTranslateFields("order_mode.options.sell") },
                    ]}
                  />
                  <Checkbox
                    label={useTranslateSettingsModal('strict_whitelist')}
                    description={useTranslateSettingsModal('strict_whitelist_description')}
                    checked={roleForm.values.live_trading.stock_item.strict_whitelist}
                    onChange={(event) => roleForm.setFieldValue('live_trading.stock_item.strict_whitelist', event.currentTarget.checked)}
                  />
                  <Checkbox
                    label={useTranslateSettingsModal('report_to_wfm')}
                    description={useTranslateSettingsModal('report_to_wfm_description')}
                    checked={roleForm.values.live_trading.stock_item.report_to_wfm}
                    onChange={(event) => roleForm.setFieldValue('live_trading.stock_item.report_to_wfm', event.currentTarget.checked)}
                  />
                  <Checkbox
                    label={useTranslateSettingsModal('auto_delete')}
                    description={useTranslateSettingsModal('auto_delete_description')}
                    checked={roleForm.values.live_trading.stock_item.auto_delete}
                    onChange={(event) => roleForm.setFieldValue('live_trading.stock_item.auto_delete', event.currentTarget.checked)}
                  />
                  {/* <Checkbox
                    label={useTranslateSettingsModal('auto_trade')}
                    description={useTranslateSettingsModal('auto_trade_description')}
                    checked={roleForm.values.live_trading.stock_item.auto_trade}
                    onChange={(event) => roleForm.setFieldValue('live_trading.stock_item.auto_trade', event.currentTarget.checked)}
                  /> */}
                </Group>
              </Group>
              <Group grow mt={10}>
                <Group grow>
                  <NumberInput
                    required
                    label={useTranslateSettingsModal('riven_range_threshold')}
                    value={roleForm.values.live_trading.stock_riven.range_threshold}
                    description={useTranslateSettingsModal('riven_range_threshold_description')}
                    onChange={(value) => roleForm.setFieldValue('live_trading.stock_riven.range_threshold', Number(value))}
                    error={roleForm.errors.price_shift_threshold && 'Invalid Price Shift Threshold'}
                  />
                </Group>
              </Group>
            </Accordion.Panel>
          </Accordion.Item>
          <Accordion.Item value="accordion_whitelist">
            <Accordion.Control>{useTranslateSettingsModal('accordion_whitelist')}</Accordion.Control>
            <Accordion.Panel>
              {useTranslateSettingsModal('whitelist_description')}
              <MultiSelectListBox
                availableItems={getAvailableItems(roleForm.values.filter.whitelist).map((warframe) => ({ ...warframe, label: warframe.item_name, value: warframe.url_name }))}
                selectedItems={roleForm.values.live_trading.stock_item.whitelist.split(",")}
                onChange={(value) => roleForm.setFieldValue('live_trading.stock_item.whitelist', value.join(","))}
                actions={
                  <Group>
                    <MinMaxField
                      min={roleForm.values.filter.whitelist.tax.min}
                      minAllowed={0}
                      max={roleForm.values.filter.whitelist.tax.max}
                      maxAllowed={2100000}
                      label={useTranslateSettingsModal('filter.tax')}
                      onChange={(min: number, max: number | "") => {
                        roleForm.setFieldValue('filter.whitelist.tax.min', min)
                        roleForm.setFieldValue('filter.whitelist.tax.max', max)
                      }} />
                    <MinMaxField
                      min={roleForm.values.filter.whitelist.mr.min}
                      minAllowed={0}
                      max={roleForm.values.filter.whitelist.mr.max}
                      maxAllowed={2100000}
                      label={useTranslateSettingsModal('filter.mr')}
                      onChange={(min: number, max: number | "") => {
                        roleForm.setFieldValue('filter.whitelist.mr.min', min)
                        roleForm.setFieldValue('filter.whitelist.mr.max', max)
                      }} />
                  </Group>
                }
              /></Accordion.Panel>
          </Accordion.Item>
          <Accordion.Item value="accordion_blacklist">
            <Accordion.Control>{useTranslateSettingsModal('accordion_blacklist')}</Accordion.Control>
            <Accordion.Panel>
              {useTranslateSettingsModal('blacklist_description')}
              <MultiSelectListBox
                availableItems={getAvailableItems(roleForm.values.filter.blacklist).map((warframe) => ({ ...warframe, label: warframe.item_name, value: warframe.url_name }))}
                selectedItems={roleForm.values.live_trading.stock_item.blacklist.split(",")}
                onChange={(value) => roleForm.setFieldValue('live_trading.stock_item.blacklist', value.join(","))}
                actions={
                  <Group>
                    <MinMaxField
                      min={roleForm.values.filter.blacklist.tax.min}
                      minAllowed={0}
                      max={roleForm.values.filter.blacklist.tax.max}
                      maxAllowed={2100000}
                      label={useTranslateSettingsModal('filter.tax')}
                      onChange={(min: number, max: number | "") => {
                        roleForm.setFieldValue('filter.blacklist.tax.min', min)
                        roleForm.setFieldValue('filter.blacklist.tax.max', max)
                      }} />
                    <MinMaxField
                      min={roleForm.values.filter.blacklist.mr.min}
                      minAllowed={0}
                      max={roleForm.values.filter.blacklist.mr.max}
                      maxAllowed={2100000}
                      label={useTranslateSettingsModal('filter.mr')}
                      onChange={(min: number, max: number | "") => {
                        roleForm.setFieldValue('filter.blacklist.mr.min', min)
                        roleForm.setFieldValue('filter.blacklist.mr.max', max)
                      }} />
                  </Group>
                }
              /></Accordion.Panel>
          </Accordion.Item>
        </Accordion>
      </Group>
      <Group position="right" mt={10} sx={{
        position: "absolute",
        bottom: 0,
        right: 0,
      }}>
        <Button type="submit" variant="light" color="blue">
          {useTranslateSettingsModal('save')}
        </Button>
      </Group>
    </form>
  );
}