import { Box } from "@mantine/core";
import { TauriTypes } from "$types";

interface PurchasePanelProps {
  value: TauriTypes.WFGDPRAccount | null;
}

export const PurchasePanel = ({}: PurchasePanelProps) => {
  return <Box p={"md"} h={"85vh"}></Box>;
};
