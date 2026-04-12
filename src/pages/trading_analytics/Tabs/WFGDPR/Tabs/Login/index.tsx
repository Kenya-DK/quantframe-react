import { Box } from "@mantine/core";
import { TauriTypes } from "$types";

interface LoginPanelProps {
  value: TauriTypes.WFGDPRAccount | null;
}

export const LoginPanel = ({}: LoginPanelProps) => {
  return <Box p={"md"} h={"85vh"}></Box>;
};
