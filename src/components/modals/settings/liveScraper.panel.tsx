import { useEffect } from "react";
import { useTauriContext } from "../../../contexts";
import { useForm } from "@mantine/form";
import { Box, Button, Group, NumberInput } from "@mantine/core";
import { useTranslateModal } from "../../../hooks";
import { invoke } from "@tauri-apps/api";


export function LiveScraperPanel() {
  const { settings, user, updateSettings } = useTauriContext();
  const roleForm = useForm({
    initialValues: {
      volume_threshold: 200,
      range_threshold: 200,
      avg_price_cap: 200,
      price_shift_threshold: 200,
      max_total_price_cap: 200,
      blacklist: "",
      whitelist: ""
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
    roleForm.setFieldValue("blacklist", settings.blacklist.join(","));
    roleForm.setFieldValue("whitelist", settings.whitelist.join(","));
  }, [settings]);

  const useTranslateSettingsModal = (key: string, context?: { [key: string]: any }) => useTranslateModal(`settings.panels.live_trading.${key}`, { ...context })
  return (
    <Box w={"100%"}>
      <form method="post" onSubmit={roleForm.onSubmit(async (data) => {
        console.log(data);
        const settingsData = {
          volume_threshold: data.volume_threshold,
          range_threshold: data.range_threshold,
          avg_price_cap: data.avg_price_cap,
          price_shift_threshold: data.price_shift_threshold,
          blacklist: data.blacklist.split(","),
          whitelist: data.whitelist.split(",")
        }

        await invoke('toggle_live_scraper_update_settings', { token: settings.access_token, settings: { ...settings, ...settingsData, in_game_name: user.ingame_name } });
        updateSettings(settingsData)
      })}>
        <Group grow >
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
          <NumberInput
            required
            label={useTranslateSettingsModal('price_shift_threshold')}
            value={roleForm.values.price_shift_threshold}
            description={useTranslateSettingsModal('price_shift_threshold_description')}
            onChange={(value) => roleForm.setFieldValue('price_shift_threshold', Number(value))}
            error={roleForm.errors.price_shift_threshold && 'Invalid Price Shift Threshold'}
          />
          <Button type="submit" variant="light" color="blue">
            {useTranslateSettingsModal('save')}
          </Button>
        </Group>
      </form>
    </Box>
  );
}