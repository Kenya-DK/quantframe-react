import { Grid } from "@mantine/core";
import { TauriTypes } from "$types";

export type OverviewTabProps = {
  value: TauriTypes.StockRivenDetails | undefined;
};

export function OverviewTab({ value }: OverviewTabProps) {
  // Translate general
  // const useTranslateTab = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
  //   useTranslateModals(`stock_item_details.tabs.overview.${key}`, { ...context }, i18Key);

  if (!value) return <></>;
  return <Grid>TODO</Grid>;
}
