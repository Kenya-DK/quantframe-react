import { Box } from "@mantine/core";
import { TauriTypes } from "$types";

interface TransactionPanelProps {
  value: TauriTypes.WFGDPRAccount | null;
}

export const TransactionPanel = ({}: TransactionPanelProps) => {
  return <Box p={"md"} h={"85vh"}></Box>;
};
