import { Box } from "@mantine/core";

interface PurchasePanelProps {
  isActive?: boolean;
  wasInitialized?: boolean;
}

export const PurchasePanel = ({}: PurchasePanelProps = {}) => {
  return <Box h={"85vh"}></Box>;
};
