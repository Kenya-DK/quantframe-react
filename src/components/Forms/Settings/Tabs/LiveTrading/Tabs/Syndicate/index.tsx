import { TauriTypes } from "$types";
import { useTranslateForms } from "@hooks/useTranslate.hook";
import { Accordion } from "@mantine/core";
import { UseFormReturnType } from "@mantine/form";
import { WTSItemAccordion } from "./Accordion/WTS";

export type SyndicatePanelProps = {
  form: UseFormReturnType<TauriTypes.Settings>;
};

export const SyndicatePanel = ({ form }: SyndicatePanelProps) => {
  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`settings.${key}`, { ...context }, i18Key);
  const useTranslateTabs = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`tabs.${key}`, { ...context }, i18Key);

  const tabs = [
    {
      label: useTranslateTabs("live_scraper.syndicate.wts.title"),
      component: <WTSItemAccordion form={form} />,
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
