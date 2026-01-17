import { DEFAULT_THEME, Divider, Group, Image, MantineColorsTuple, Paper, Stack, Text } from "@mantine/core";
import { createTheme } from "@mantine/core";

export type ThemePreviewProps = {
  name: string;
  author: string;
  fileName?: string;
  icon: string;
  selected?: boolean;
  onClick?: (fileName: string) => void;
  theme: ReturnType<typeof createTheme>;
};

export function ThemePreview({ fileName, theme, author, name, icon, onClick }: ThemePreviewProps) {
  const colors = theme.colors || DEFAULT_THEME.colors;
  const primaryColor = theme.primaryColor || "blue";

  // Get primary color hex value
  const primaryColorArray = colors[primaryColor as keyof typeof colors] as MantineColorsTuple;

  const fontFamily = theme.fontFamily || DEFAULT_THEME.fontFamily;

  const lightMode = false;

  const dark = colors.dark as MantineColorsTuple;

  const fontColor = lightMode ? "black" : dark ? dark[0] : DEFAULT_THEME.colors.dark[0];
  const backgroundColor = lightMode ? "white" : dark ? dark[7] : DEFAULT_THEME.colors.dark[7];

  return (
    <Paper
      w={"250px"}
      p={"xs"}
      withBorder
      onClick={() => onClick?.(fileName || "")}
      style={{
        backgroundColor,
        fontFamily,
        color: fontColor,
        borderColor: primaryColorArray ? primaryColorArray[6] : DEFAULT_THEME.colors.blue[6],
        cursor: "pointer",
      }}
    >
      <Group gap={5}>
        <Image m={0} radius="md" h={48} w="auto" fit="contain" src={`data:image/png;base64,${icon}`} alt={name} />
        <Divider orientation="vertical" />
        <Stack w={"100%"} gap={2} style={{ flex: 1 }}>
          <Group>
            <Text style={{ color: fontColor }}>{name}</Text>
          </Group>
          <Divider />
          <Group>
            <Text size="xs" c="dimmed">
              {author}
            </Text>
          </Group>
        </Stack>
      </Group>
    </Paper>
  );
}
