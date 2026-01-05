import { Box } from "@mantine/core";

interface TransactionPanelProps {
  isActive?: boolean;
  wasInitialized?: boolean;
}

export const TransactionPanel = ({}: TransactionPanelProps = {}) => {
  return <Box h={"85vh"}></Box>;
};
