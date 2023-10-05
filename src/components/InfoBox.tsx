
import { Box, Group, Text } from "@mantine/core";
interface InfoBoxProps {
  text: string;
  color: string;
}
export const InfoBox = ({ text, color }: InfoBoxProps) => {
  return (
    <Group spacing={"sm"}>
      <Box w={16} h={16} sx={{
        backgroundColor: color,
        borderRadius: "3px",
      }} />
      <Text size="sm">{text}</Text>
    </Group>);
}
