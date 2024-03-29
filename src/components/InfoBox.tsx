
import { Box, Group, Text, Tooltip } from "@mantine/core";
interface InfoBoxProps {
  text: string;
  color: string;
  tooltip?: string;
}
export const InfoBox = ({ tooltip, text, color }: InfoBoxProps) => {
  return (
    <Tooltip disabled={!tooltip} label={tooltip} withArrow>
      <Group spacing={"sm"}>
        <Box w={16} h={16} sx={{
          backgroundColor: color,
          borderRadius: "3px",
        }} />
        <Text size="sm">{text}</Text>
      </Group>
    </Tooltip>
  );
}
