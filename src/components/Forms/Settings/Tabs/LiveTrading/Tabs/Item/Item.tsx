import { TauriTypes } from "$types";
import { useTranslateForms } from "@hooks/useTranslate.hook";
import { Accordion } from "@mantine/core";
import { WTBItemAccordion } from "./Accordion/WTB";
import { WTSItemAccordion } from "./Accordion/WTS";

export type ItemPanelProps = {
  value: TauriTypes.SettingsStockItem;
  onSubmit: (value: TauriTypes.SettingsStockItem) => void;
};

export const ItemPanel = ({ value, onSubmit }: ItemPanelProps) => {
  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`settings.${key}`, { ...context }, i18Key);
  const useTranslateTabs = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`tabs.${key}`, { ...context }, i18Key);

  const tabs = [
    {
      label: useTranslateTabs("live_trading.item.wtb.title"),
      component: <WTBItemAccordion value={value} onSubmit={(v) => onSubmit(v)} />,
      id: "wtb",
    },
    {
      label: useTranslateTabs("live_trading.item.wts.title"),
      component: <WTSItemAccordion value={value} onSubmit={(v) => onSubmit(v)} />,
      id: "wts",
    },
  ];

  return (
    <Accordion defaultValue={tabs[0].id}>
      {tabs.map((tab) => (
        <Accordion.Item value={tab.id} key={tab.id}>
          <Accordion.Control>{tab.label}</Accordion.Control>
          <Accordion.Panel>{tab.component}</Accordion.Panel>
        </Accordion.Item>
      ))}
    </Accordion>
  );
};
