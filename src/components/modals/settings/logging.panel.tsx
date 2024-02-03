import { Button, Group, MultiSelect } from "@mantine/core";
import { Settings } from "$types/index";
import { useForm } from "@mantine/form";
import { useEffect } from "react";
import { useTranslateModal } from "../../../hooks";
interface LoggingProps {
  settings: Settings | undefined;
  updateSettings: (user: Partial<Settings>) => void;
}

export function LoggingPanel({ updateSettings, settings }: LoggingProps) {
  const roleForm = useForm({
    initialValues: {
      debug: [] as string[],
    },
    validate: {},
  });


  const useTranslateSettingsModal = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateModal(`settings.panels.logging.${key}`, { ...context }, i18Key)
  const useTranslateFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateSettingsModal(`fields.${key}`, { ...context }, i18Key)

  useEffect(() => {
    if (!settings) return;
    // Set Settings from live Scraper
    roleForm.setFieldValue("debug", settings.debug);
  }, [settings]);

  return (
    <form method="post" onSubmit={roleForm.onSubmit(async (data) => {
      console.log(data);

      updateSettings({ debug: data.debug })
    })}>
      <MultiSelect
        label={useTranslateFields("log.label")}
        description={useTranslateFields("log.description")}
        placeholder={useTranslateFields("log.placeholder")}
        data={
          [
            { label: useTranslateFields("log.options.all"), value: "*" },
            { label: useTranslateFields("log.options.wfm_client_auth"), value: "wfm_client_auth" },
            { label: useTranslateFields("log.options.wfm_client_order"), value: "wfm_client_order" },
            { label: useTranslateFields("log.options.wfm_client_item"), value: "wfm_client_item" },
            { label: useTranslateFields("log.options.wfm_client_auction"), value: "wfm_client_auction" },
            { label: useTranslateFields("log.options.wfm_client_chat"), value: "wfm_client_chat" },
          ]
        }
        value={roleForm.values.debug}
        onChange={(value) => {
          roleForm.setFieldValue("debug", value);
          console.log(value);
        }}
        clearable
        searchable
        maw={400}
      />
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