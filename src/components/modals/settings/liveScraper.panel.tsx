import { useEffect } from "react";
import { useForm } from "@mantine/form";
import { Accordion, Button, Checkbox, Group, NumberInput, TextInput } from "@mantine/core";
import { useTranslateModal } from "@hooks/index";
import { LiveScraperSettings, Wfm } from "$types/index";
import { MultiSelectListBox } from "../../multiSelectListBox";

interface LiveScraperProps {
  settings: LiveScraperSettings | undefined;
  tradable_items: Wfm.ItemDto[];
  updateSettings: (user: Partial<LiveScraperSettings>) => void;
}

export function LiveScraperPanel({ settings, updateSettings, tradable_items }: LiveScraperProps) {
  const roleForm = useForm({
    initialValues: {
      live_trading: {
        webhook: "",
        stock_item: {
          volume_threshold: 200,
          range_threshold: 200,
          avg_price_cap: 200,
          price_shift_threshold: 200,
          max_total_price_cap: 200,
          blacklist: "",
          whitelist: "",
          strict_whitelist: true,
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

  const useTranslateSettingsModal = (key: string, context?: { [key: string]: any }) => useTranslateModal(`settings.panels.live_trading.${key}`, { ...context })
  return (
    <form method="post" onSubmit={roleForm.onSubmit(async (data) => {
      console.log(data);
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
                    <TextInput
                      label={useTranslateSettingsModal('webhook')}
                      value={roleForm.values.live_trading.webhook}
                      description={useTranslateSettingsModal('webhook_description')}
                      onChange={(event) => roleForm.setFieldValue('live_trading.webhook', event.currentTarget.value)}
                      error={roleForm.errors.webhook && 'Invalid Webhook'}
                    />
                    <Checkbox
                      label={useTranslateSettingsModal('strict_whitelist')}
                      description={useTranslateSettingsModal('strict_whitelist_description')}
                      checked={roleForm.values.live_trading.stock_item.strict_whitelist}
                      onChange={(event) => roleForm.setFieldValue('live_trading.stock_item.strict_whitelist', event.currentTarget.checked)}
                    />
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
                  availableItems={tradable_items.map((warframe) => ({ ...warframe, label: warframe.item_name, value: warframe.url_name }))}
                  selectedItems={roleForm.values.live_trading.stock_item.whitelist.split(",")}
                  onChange={(value) => roleForm.setFieldValue('live_trading.stock_item.whitelist', value.join(","))}
                /></Accordion.Panel>
            </Accordion.Item>
            <Accordion.Item value="accordion_blacklist">
              <Accordion.Control>{useTranslateSettingsModal('accordion_blacklist')}</Accordion.Control>
              <Accordion.Panel>
                {useTranslateSettingsModal('blacklist_description')}
                <MultiSelectListBox
                  availableItems={tradable_items.map((warframe) => ({ ...warframe, label: warframe.item_name, value: warframe.url_name }))}
                  selectedItems={roleForm.values.live_trading.stock_item.blacklist.split(",")}
                  onChange={(value) => roleForm.setFieldValue('live_trading.stock_item.blacklist', value.join(","))}
                /></Accordion.Panel>
            </Accordion.Item>
          </Accordion>
        </Group>
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