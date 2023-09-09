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
      volume_threshold: 200,
      range_threshold: 200,
      avg_price_cap: 200,
      price_shift_threshold: 200,
      max_total_price_cap: 200,
      blacklist: "",
      whitelist: "",
      strict_whitelist: true,
      ping_on_notif: true,
      webhook: ""
    },
    validate: {},
  });

  useEffect(() => {
    if (!settings) return;
    roleForm.setFieldValue("volume_threshold", settings.volume_threshold);
    roleForm.setFieldValue("range_threshold", settings.range_threshold);
    roleForm.setFieldValue("max_total_price_cap", settings.max_total_price_cap);
    roleForm.setFieldValue("avg_price_cap", settings.avg_price_cap);
    roleForm.setFieldValue("price_shift_threshold", settings.price_shift_threshold);
    roleForm.setFieldValue("strict_whitelist", settings.strict_whitelist);
    roleForm.setFieldValue("webhook", settings.webhook);
    roleForm.setFieldValue("blacklist", settings.blacklist.join(","));
    roleForm.setFieldValue("whitelist", settings.whitelist.join(","));
  }, [settings]);

  const useTranslateSettingsModal = (key: string, context?: { [key: string]: any }) => useTranslateModal(`settings.panels.live_trading.${key}`, { ...context })
  return (
    <form method="post" onSubmit={roleForm.onSubmit(async (data) => {
      const settingsData = {
        volume_threshold: data.volume_threshold,
        range_threshold: data.range_threshold,
        avg_price_cap: data.avg_price_cap,
        price_shift_threshold: data.price_shift_threshold,
        max_total_price_cap: data.max_total_price_cap,
        blacklist: data.blacklist.split(","),
        whitelist: data.whitelist.split(","),
        strict_whitelist: data.strict_whitelist,
        ping_on_notif: data.ping_on_notif,
        webhook: data.webhook
      }

      updateSettings(settingsData)
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
                      value={roleForm.values.volume_threshold}
                      description={useTranslateSettingsModal('volume_threshold_description')}
                      onChange={(value) => roleForm.setFieldValue('volume_threshold', Number(value))}
                      error={roleForm.errors.volume_threshold && 'Invalid Volume Threshold'}
                    />
                    <NumberInput
                      required
                      label={useTranslateSettingsModal('range_threshold')}
                      value={roleForm.values.range_threshold}
                      description={useTranslateSettingsModal('range_threshold_description')}
                      onChange={(value) => roleForm.setFieldValue('range_threshold', Number(value))}
                      error={roleForm.errors.range_threshold && 'Invalid Range Threshold'}
                    />
                    <NumberInput
                      required
                      label={useTranslateSettingsModal('max_total_price_cap')}
                      value={roleForm.values.max_total_price_cap}
                      description={useTranslateSettingsModal('max_total_price_cap_description')}
                      onChange={(value) => roleForm.setFieldValue('max_total_price_cap', Number(value))}
                      error={roleForm.errors.max_total_price_cap && 'Invalid Range Threshold'}
                    />
                    <NumberInput
                      required
                      label={useTranslateSettingsModal('avg_price_cap')}
                      value={roleForm.values.avg_price_cap}
                      description={useTranslateSettingsModal('avg_price_cap_description')}
                      onChange={(value) => roleForm.setFieldValue('avg_price_cap', Number(value))}
                      error={roleForm.errors.avg_price_cap && 'Invalid Avg Price Cap'}
                    />
                  </Group>

                </Group>
                <Group grow mt={10}>
                  <Group grow>
                    <NumberInput
                      required
                      label={useTranslateSettingsModal('price_shift_threshold')}
                      value={roleForm.values.price_shift_threshold}
                      description={useTranslateSettingsModal('price_shift_threshold_description')}
                      onChange={(value) => roleForm.setFieldValue('price_shift_threshold', Number(value))}
                      error={roleForm.errors.price_shift_threshold && 'Invalid Price Shift Threshold'}
                    />
                    <TextInput
                      label={useTranslateSettingsModal('webhook')}
                      value={roleForm.values.webhook}
                      description={useTranslateSettingsModal('webhook_description')}
                      onChange={(event) => roleForm.setFieldValue('webhook', event.currentTarget.value)}
                      error={roleForm.errors.webhook && 'Invalid Webhook'}
                    />
                    <Checkbox
                      label={useTranslateSettingsModal('strict_whitelist')}
                      description={useTranslateSettingsModal('strict_whitelist_description')}
                      checked={roleForm.values.strict_whitelist}
                      onChange={(event) => roleForm.setFieldValue('strict_whitelist', event.currentTarget.checked)}
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
                  selectedItems={roleForm.values.whitelist.split(",")}
                  onChange={(value) => roleForm.setFieldValue('whitelist', value.join(","))}
                /></Accordion.Panel>
            </Accordion.Item>
            <Accordion.Item value="accordion_blacklist">
              <Accordion.Control>{useTranslateSettingsModal('accordion_blacklist')}</Accordion.Control>
              <Accordion.Panel>
                {useTranslateSettingsModal('blacklist_description')}
                <MultiSelectListBox
                  availableItems={tradable_items.map((warframe) => ({ ...warframe, label: warframe.item_name, value: warframe.url_name }))}
                  selectedItems={roleForm.values.blacklist.split(",")}
                  onChange={(value) => roleForm.setFieldValue('blacklist', value.join(","))}
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