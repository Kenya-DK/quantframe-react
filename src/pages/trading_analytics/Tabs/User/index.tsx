import { Box } from "@mantine/core";
import { PatreonOverlay } from "@components/Shared/PatreonOverlay/PatreonOverlay";
import { TauriTypes } from "$types";
interface UserPanelProps {
  isActive?: boolean;
}

export const UserPanel = ({}: UserPanelProps = {}) => {
  return (
    <Box p={"md"} pos={"relative"}>
      <PatreonOverlay permission={TauriTypes.PermissionsFlags.WFM_USER_ACTIVE_HISTORY} tier="T1+" />
    </Box>
  );
};
