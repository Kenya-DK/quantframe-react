import { useEffect, useState } from "react";
import { useForm } from "@mantine/form";
import { Box, Button, Checkbox, Group, MultiSelect, NumberInput } from "@mantine/core";
import { useTranslateModal } from "@hooks/index";
import { Settings, Wfm } from "$types/index";

interface LiveScraperProps {
  settings: Settings | undefined;
  tradable_items: Wfm.ItemDto[];
  updateSettings: (user: Partial<Settings>) => void;
}

export function LiveScraperPanel({ settings, updateSettings, tradable_items }: LiveScraperProps) {
  const [items, setItems] = useState<Array<Wfm.ItemDto & { label: string, value: string }>>([]);
  useEffect(() => {
    setItems(tradable_items.map((warframe) => ({ ...warframe, label: warframe.item_name, value: warframe.url_name })) || []);
  }, [tradable_items]);

  const roleForm = useForm({
    initialValues: {
      volume_threshold: 200,
      range_threshold: 200,
      avg_price_cap: 200,
      price_shift_threshold: 200,
      max_total_price_cap: 200,
      blacklist: "",
      whitelist: "",
      strict_whitelist: true
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
    roleForm.setFieldValue("blacklist", settings.blacklist.join(","));
    roleForm.setFieldValue("whitelist", settings.whitelist.join(","));
  }, [settings]);

  const useTranslateSettingsModal = (key: string, context?: { [key: string]: any }) => useTranslateModal(`settings.panels.live_trading.${key}`, { ...context })
  return (
    <Box h={"50vh"}>
      <form method="post" onSubmit={roleForm.onSubmit(async (data) => {
        console.log(data);
        const settingsData = {
          volume_threshold: data.volume_threshold,
          range_threshold: data.range_threshold,
          avg_price_cap: data.avg_price_cap,
          price_shift_threshold: data.price_shift_threshold,
          max_total_price_cap: data.max_total_price_cap,
          blacklist: data.blacklist.split(","),
          whitelist: data.whitelist.split(","),
          strict_whitelist: data.strict_whitelist
        }

        updateSettings(settingsData)
      })}>
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
          </Group>

        </Group>
        <Group grow mt={10}>
          <Group grow>
            <NumberInput
              required
              label={useTranslateSettingsModal('avg_price_cap')}
              value={roleForm.values.avg_price_cap}
              description={useTranslateSettingsModal('avg_price_cap_description')}
              onChange={(value) => roleForm.setFieldValue('avg_price_cap', Number(value))}
              error={roleForm.errors.avg_price_cap && 'Invalid Avg Price Cap'}
            />
            <NumberInput
              required
              label={useTranslateSettingsModal('price_shift_threshold')}
              value={roleForm.values.price_shift_threshold}
              description={useTranslateSettingsModal('price_shift_threshold_description')}
              onChange={(value) => roleForm.setFieldValue('price_shift_threshold', Number(value))}
              error={roleForm.errors.price_shift_threshold && 'Invalid Price Shift Threshold'}
            />
          </Group>
        </Group>
        <Group position="right" mt={10}>
          <MultiSelect
            data={items}
            value={roleForm.values.whitelist.split(",")}
            onChange={(value) => roleForm.setFieldValue('whitelist', value.join(","))}
            limit={10}
            mah={200}
            searchable
            maxDropdownHeight={400}
            label={useTranslateSettingsModal('whitelist_label')}
            placeholder={useTranslateSettingsModal('whitelist_placeholder')}
            description={useTranslateSettingsModal('whitelist_description')}
          />
          <MultiSelect
            mah={200}
            data={items}
            value={roleForm.values.blacklist.split(",")}
            onChange={(value) => roleForm.setFieldValue('blacklist', value.join(","))}
            description={useTranslateSettingsModal('blacklist_description')}
            label={useTranslateSettingsModal('blacklist_label')}
            placeholder={useTranslateSettingsModal('blacklist_placeholder')}
          />
          <Checkbox
            label={useTranslateSettingsModal('strict_whitelist')}
            description={useTranslateSettingsModal('strict_whitelist_description')}
            checked={roleForm.values.strict_whitelist}
            onChange={(event) => roleForm.setFieldValue('strict_whitelist', event.currentTarget.checked)}
          />
        </Group>
        <Group position="right" mt={10}>
          <Button type="submit" variant="light" color="blue">
            {useTranslateSettingsModal('save')}
          </Button>
        </Group>
      </form>
    </Box>
  );
}