import { TauriTypes } from "$types";
import { useTranslateForms } from "@hooks/useTranslate.hook";
import { Accordion } from "@mantine/core";
import { WTBItemAccordion } from "./Accordion/WTB";
import { WTSItemAccordion } from "./Accordion/WTS";
import { SummaryAccordion } from "./Accordion/Summary";
import { UseFormReturnType } from "@mantine/form";

export type ItemPanelProps = {
  form: UseFormReturnType<TauriTypes.Settings>;
};

export const ItemPanel = ({ form }: ItemPanelProps) => {
  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`settings.${key}`, { ...context }, i18Key);
  const useTranslateTabs = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`tabs.${key}`, { ...context }, i18Key);

  const tabs = [
    {
      label: useTranslateTabs("live_scraper.item.wtb.title"),
      component: <WTBItemAccordion form={form} />,
      id: "wtb",
    },
    {
      label: useTranslateTabs("live_scraper.item.wts.title"),
      component: <WTSItemAccordion form={form} />,
      id: "wts",
    },
    {
      label: useTranslateTabs("live_scraper.item.summary.title"),
      component: <SummaryAccordion value={form.values.live_scraper.stock_item} />,
      id: "summary",
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
