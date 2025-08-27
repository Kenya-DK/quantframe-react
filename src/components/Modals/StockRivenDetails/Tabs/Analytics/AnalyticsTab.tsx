import { Grid } from "@mantine/core";
import { TauriTypes } from "$types";

export type AnalyticsTabProps = {
  value: TauriTypes.StockRivenDetails | undefined;
};

export function AnalyticsTab({ value }: AnalyticsTabProps) {
  if (!value) return <></>;
  // Translate general
  // const useTranslateTab = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
  //   useTranslateModals(`stock_item_details.tabs.analytics.${key}`, { ...context }, i18Key);

  return <Grid>TODO</Grid>;
}
