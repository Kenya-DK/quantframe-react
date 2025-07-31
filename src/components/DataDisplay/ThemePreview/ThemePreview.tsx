import { Group, Text, Box, DEFAULT_THEME, MantineColorsTuple } from "@mantine/core";
import { createTheme } from "@mantine/core";
import classes from "./ThemeSelector.module.css";

export type ThemePreviewProps = {
  name: string;
  author: string;
  fileName: string;
  selected?: boolean;
  onClick?: (fileName: string) => void;
  theme: ReturnType<typeof createTheme>;
};

export function ThemePreview({ name, author, selected, theme, onClick, fileName }: ThemePreviewProps) {
  const colors = theme.colors || DEFAULT_THEME.colors;
  const primaryColor = theme.primaryColor || "blue";

  // Get primary color hex value
  const primaryColorArray = colors[primaryColor as keyof typeof colors] as MantineColorsTuple;
  const hex = primaryColorArray ? primaryColorArray[6] : DEFAULT_THEME.colors.blue[6];

  const fontFamily = theme.fontFamily || DEFAULT_THEME.fontFamily;

  const lightMode = false;

  const dark = colors.dark as MantineColorsTuple;

  const fontColor = lightMode ? "black" : dark ? dark[0] : DEFAULT_THEME.colors.dark[0];
  const backgroundColor = lightMode ? "white" : dark ? dark[7] : DEFAULT_THEME.colors.dark[7];

  return (
    <Box
      p="md"
      className={classes.themePreview}
      onClick={() => onClick?.(fileName)}
      style={{
        backgroundColor,
        fontFamily,
        borderRadius: "8px",
        border: selected ? `2px solid ${hex}` : `1px solid ${dark ? dark[6] : DEFAULT_THEME.colors.dark[6]}`,
        cursor: "pointer",
        minWidth: "200px",
      }}
    >
      <Group justify="space-between" align="center" gap="md">
        <Box style={{ display: "flex", alignItems: "center", gap: "12px" }}>
          <Box
            w={32}
            h={32}
            style={{
              backgroundColor: hex,
              borderRadius: "50%",
              border: "1px solid rgba(255, 255, 255, 0.1)",
              display: "flex",
              alignItems: "center",
              justifyContent: "center",
            }}
          >
            <Text size="sm" fw={600} style={{ color: "white" }}>
              {name.charAt(0).toUpperCase()}
            </Text>
          </Box>
          <Box>
            <Text size="sm" fw={600} style={{ color: fontColor, lineHeight: 1.2 }}>
              {name}
            </Text>
            <Text size="xs" style={{ color: dark ? dark[2] : DEFAULT_THEME.colors.dark[2], lineHeight: 1.2 }}>
              {author}
            </Text>
          </Box>
        </Box>
      </Group>
    </Box>
  );
}
