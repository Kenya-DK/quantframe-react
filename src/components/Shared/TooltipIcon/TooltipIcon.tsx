import { faInfoCircle } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { Center, Text, Tooltip } from "@mantine/core";
import { open } from "@tauri-apps/plugin-shell";

export type TooltipIconProps = {
  link?: string;
  label: string;
};

export function TooltipIcon({ label, link }: TooltipIconProps) {
  return (
    <Tooltip label={label} position="top-end" withArrow transitionProps={{ transition: "pop-bottom-right" }}>
      <Text component="div" c="dimmed" style={{ cursor: "help" }} onClick={() => link && open(link)}>
        <Center>
          <FontAwesomeIcon icon={faInfoCircle} />
        </Center>
      </Text>
    </Tooltip>
  );
}
