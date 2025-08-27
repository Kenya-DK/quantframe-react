import { Grid } from "@mantine/core";
import { TauriTypes } from "$types";

export type WFMTabProps = {
  value: TauriTypes.WishListItemDetails | undefined;
};

export function WFMTab({ value }: WFMTabProps) {
  // Translate general
  // const useTranslateTab = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
  //   useTranslateModals(`stock_item_details.tabs.wfm.${key}`, { ...context }, i18Key);

  if (!value) return <></>;
  return <Grid>TODO</Grid>;
}
