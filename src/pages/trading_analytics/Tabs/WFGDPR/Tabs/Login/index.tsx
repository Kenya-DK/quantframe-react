import { Box } from "@mantine/core";

interface LoginPanelProps {
  isActive?: boolean;
  wasInitialized?: boolean;
}

export const LoginPanel = ({}: LoginPanelProps = {}) => {
  return <Box h={"85vh"}></Box>;
};
