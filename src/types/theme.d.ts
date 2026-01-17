import { MantineColorsTuple } from "@mantine/core";
import { UserStatus, TauriTypes } from "$types";

declare module "@mantine/core" {
  export interface MantineThemeColorsOverride {
    other: {
      statusColors: Record<UserStatus, string>;
      stockStatus: Record<TauriTypes.StockStatus, string>;
      alertTypes: Record<string, string>;
      itemTypes: Record<TauriTypes.TransactionItemType, string>;
    };
  }
}
