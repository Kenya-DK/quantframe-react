import { useState } from "react";
import {
  Box,
  Stack,
  Group,
  Text,
  ColorInput,
  TextInput,
  Button,
  Textarea,
  Accordion,
  NumberInput,
  Select,
  Paper,
  Divider,
  ActionIcon,
  Tooltip,
} from "@mantine/core";
import { useTheme } from "../../contexts/theme.context";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faDownload, faRotateRight } from "@fortawesome/free-solid-svg-icons";
import { notifications } from "@mantine/notifications";

interface ColorPaletteEditorProps {
  colorName: string;
  colors: string[];
  onColorChange: (index: number, color: string) => void;
}

function ColorPaletteEditor({ colorName, colors, onColorChange }: ColorPaletteEditorProps) {
  return (
    <Stack gap="xs">
      <Text size="sm" fw={500}>
        {colorName}
      </Text>
      <Group gap="xs">
        {colors.map((color, index) => (
          <Box key={index} style={{ display: "flex", flexDirection: "column", alignItems: "center", gap: 4 }}>
            <ColorInput size="xs" value={color} onChange={(value) => onColorChange(index, value)} style={{ width: 60 }} withEyeDropper={false} />
            <Text size="xs" c="dimmed">
              {index}
            </Text>
          </Box>
        ))}
      </Group>
    </Stack>
  );
}

export function LiveThemeEditor() {
  const { theme, currentThemeData, updateThemeProperty, resetTheme, exportTheme, importTheme } = useTheme();
  const [importData, setImportData] = useState("");

  const handleColorChange = (colorName: string, index: number, value: string) => {
    updateThemeProperty(`colors.${colorName}.${index}`, value);
  };

  const handlePropertyChange = (path: string, value: any) => {
    updateThemeProperty(path, value);
  };

  const handleExport = () => {
    const themeJson = exportTheme();
    navigator.clipboard.writeText(themeJson);
    notifications.show({
      title: "Theme Exported",
      message: "Theme data copied to clipboard",
      color: "green",
    });
  };

  const handleImport = () => {
    if (importData.trim()) {
      importTheme(importData);
      setImportData("");
      notifications.show({
        title: "Theme Imported",
        message: "Theme has been imported successfully",
        color: "green",
      });
    }
  };

  const colors = currentThemeData?.colors || {};

  return (
    <Paper p="md" withBorder>
      <Stack gap="md">
        <Group justify="space-between">
          <Text size="lg" fw={600}>
            Live Theme Editor
          </Text>
          <Group gap="xs">
            <Tooltip label="Export Theme">
              <ActionIcon variant="light" onClick={handleExport}>
                <FontAwesomeIcon icon={faDownload} size="sm" />
              </ActionIcon>
            </Tooltip>
            <Tooltip label="Reset Theme">
              <ActionIcon variant="light" color="red" onClick={resetTheme}>
                <FontAwesomeIcon icon={faRotateRight} size="sm" />
              </ActionIcon>
            </Tooltip>
          </Group>
        </Group>

        <Accordion variant="separated">
          {/* Color Palettes */}
          <Accordion.Item value="colors">
            <Accordion.Control>Color Palettes</Accordion.Control>
            <Accordion.Panel>
              <Stack gap="lg">
                {Object.entries(colors).map(([colorName, colorArray]) => {
                  if (Array.isArray(colorArray)) {
                    return (
                      <ColorPaletteEditor
                        key={colorName}
                        colorName={colorName}
                        colors={colorArray as string[]}
                        onColorChange={(index, color) => handleColorChange(colorName, index, color)}
                      />
                    );
                  }
                  return null;
                })}
              </Stack>
            </Accordion.Panel>
          </Accordion.Item>

          {/* Primary Color */}
          <Accordion.Item value="primary">
            <Accordion.Control>Primary Settings</Accordion.Control>
            <Accordion.Panel>
              <Stack gap="md">
                <Select
                  label="Primary Color"
                  value={currentThemeData?.primaryColor || "blue"}
                  onChange={(value) => handlePropertyChange("primaryColor", value)}
                  data={Object.keys(colors)}
                />
                <Select
                  label="Color Scheme"
                  value={currentThemeData?.colorScheme || "dark"}
                  onChange={(value) => handlePropertyChange("colorScheme", value)}
                  data={["light", "dark", "auto"]}
                />
              </Stack>
            </Accordion.Panel>
          </Accordion.Item>

          {/* Typography */}
          <Accordion.Item value="typography">
            <Accordion.Control>Typography</Accordion.Control>
            <Accordion.Panel>
              <Stack gap="md">
                <TextInput
                  label="Font Family"
                  value={currentThemeData?.fontFamily || ""}
                  onChange={(e) => handlePropertyChange("fontFamily", e.target.value)}
                  placeholder="Inter, system-ui, sans-serif"
                />
                <NumberInput
                  label="Font Size (rem)"
                  value={currentThemeData?.fontSizes?.md || 14}
                  onChange={(value) => handlePropertyChange("fontSizes.md", value)}
                  step={0.1}
                  min={0.5}
                  max={2}
                  decimalScale={1}
                />
              </Stack>
            </Accordion.Panel>
          </Accordion.Item>

          {/* Spacing */}
          <Accordion.Item value="spacing">
            <Accordion.Control>Spacing</Accordion.Control>
            <Accordion.Panel>
              <Stack gap="md">
                <NumberInput
                  label="Default Spacing (rem)"
                  value={currentThemeData?.spacing?.md || 16}
                  onChange={(value) => handlePropertyChange("spacing.md", value)}
                  step={1}
                  min={1}
                  max={50}
                />
                <NumberInput
                  label="Border Radius (px)"
                  value={currentThemeData?.radius?.md || 8}
                  onChange={(value) => handlePropertyChange("radius.md", value)}
                  step={1}
                  min={0}
                  max={20}
                />
              </Stack>
            </Accordion.Panel>
          </Accordion.Item>

          {/* Import/Export */}
          <Accordion.Item value="import-export">
            <Accordion.Control>Import/Export</Accordion.Control>
            <Accordion.Panel>
              <Stack gap="md">
                <Textarea
                  label="Import Theme JSON"
                  value={importData}
                  onChange={(e) => setImportData(e.target.value)}
                  placeholder="Paste theme JSON here..."
                  minRows={4}
                  maxRows={8}
                />
                <Group>
                  <Button onClick={handleImport} disabled={!importData.trim()}>
                    Import Theme
                  </Button>
                  <Button variant="light" onClick={handleExport}>
                    Copy Current Theme
                  </Button>
                </Group>

                <Divider my="md" />

                <Text size="sm" fw={500}>
                  Current Theme Preview:
                </Text>
                <Textarea value={exportTheme()} readOnly minRows={6} maxRows={12} style={{ fontFamily: "monospace", fontSize: "12px" }} />
              </Stack>
            </Accordion.Panel>
          </Accordion.Item>
        </Accordion>

        {/* Live Preview */}
        <Box>
          <Text size="sm" fw={500} mb="xs">
            Live Preview
          </Text>
          <Paper p="md" withBorder style={{ backgroundColor: theme.colors?.dark?.[7] }}>
            <Stack gap="md">
              <Text style={{ color: theme.colors?.dark?.[0] }}>Sample text in current theme</Text>
              <Group>
                <Button>Primary Button</Button>
                <Button variant="light">Light Button</Button>
                <Button variant="outline">Outline Button</Button>
              </Group>
            </Stack>
          </Paper>
        </Box>
      </Stack>
    </Paper>
  );
}
