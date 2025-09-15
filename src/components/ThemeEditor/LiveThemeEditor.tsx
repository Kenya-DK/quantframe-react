import { Group, Text, ColorInput, Accordion, Box, Card, TextInput, Button } from "@mantine/core";
import { useTheme } from "@contexts/theme.context";
import { useTranslateComponent, useTranslateEnums } from "@hooks/useTranslate.hook";
import api from "../../api";
import { useForm } from "@mantine/form";
import { useMutation } from "@tanstack/react-query";
import { notifications } from "@mantine/notifications";
import { ActionWithTooltip } from "../Shared/ActionWithTooltip";
import { faPlus } from "@fortawesome/free-solid-svg-icons";
import { writeText, readText } from "@tauri-apps/plugin-clipboard-manager";
interface ChartColorPaletteEditorProps {
  colorName: string;
  colors: { [key: string]: string };
  onColorChange: (property: string, color: string) => void;
  baseTranslation: (key: string, context?: { [key: string]: any }, i18Key?: boolean) => string;
}
function ChartColorPaletteEditor({ colorName, colors, onColorChange, baseTranslation }: ChartColorPaletteEditorProps) {
  return (
    <Box mb="md">
      <Text size="md" fw={500}>
        {colorName}
      </Text>
      <Card shadow="sm">
        <Group gap="xs">
          {[...Object.keys(colors)].map((key) => (
            <ColorInput
              key={key}
              label={baseTranslation(`${key}`, { colorName })}
              size="xs"
              value={colors[key as keyof typeof colors]}
              onChangeEnd={(value) => onColorChange(key, value)}
              withEyeDropper={false}
            />
          ))}
        </Group>
      </Card>
    </Box>
  );
}
interface ColorPaletteEditorProps {
  colorName: string;
  colors: { name?: string; value: string }[];
  onColorChange: (index: number, color: string) => void;
}
function ColorPaletteEditor({ colorName, colors, onColorChange }: ColorPaletteEditorProps) {
  return (
    <Box mb="md">
      <Text size="md" fw={500}>
        {colorName}
      </Text>
      <Card shadow="sm">
        <Group gap="xs">
          {colors.map((item, index) => (
            <Box key={index} style={{ display: "flex", flexDirection: "column", alignItems: "center", gap: 4 }}>
              <ColorInput
                label={item.name}
                size="xs"
                value={item.value}
                onChangeEnd={(value) => onColorChange(index, value)}
                w={100}
                withEyeDropper={false}
              />
            </Box>
          ))}
        </Group>
      </Card>
    </Box>
  );
}

interface LiveThemeEditorProps {
  onNewTheme?: () => void;
}
export function LiveThemeEditor({ onNewTheme }: LiveThemeEditorProps) {
  // Context
  const { currentThemeData, updateThemeProperty, importTheme } = useTheme();

  // Translate general
  const useTranslateEditor = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateComponent(`theme_editor.${key}`, { ...context }, i18Key);
  const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateEditor(`fields.${key}`, { ...context }, i18Key);
  const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateEditor(`buttons.${key}`, { ...context }, i18Key);
  const handleColorChange = (colorName: string, index: number, value: string) => {
    updateThemeProperty(`colors.${colorName}.${index}`, value);
  };

  const handlePropertyChange = (path: string, value: any) => updateThemeProperty(path, value);

  const export_form = useForm({
    initialValues: {
      name: "" as string,
      author: "" as string,
    },
    validate: {
      name: (value: string) => (value.length < 3 ? useTranslateFormFields("export_name.error") : null),
      author: (value: string) => (value.length < 3 ? useTranslateFormFields("export_author.error") : null),
    },
  });

  const exportTheme = useMutation({
    mutationFn: (data: { name: string; author: string; properties: any }) => api.cache.createTheme(data.name, data.author, data.properties),
    onSuccess: () => {
      onNewTheme?.();
      notifications.show({
        title: useTranslateEditor("export_success.title"),
        message: useTranslateEditor("export_success.message"),
        color: "green",
      });
    },
    onError: (error) => {
      console.error("Export theme error:", error);
      notifications.show({
        title: useTranslateEditor("export_error.title"),
        message: useTranslateEditor("export_error.message", { error: error.message }),
        color: "red",
      });
    },
  });

  const colors = currentThemeData?.colors || {};
  const other = currentThemeData?.other || {};

  return (
    <Accordion variant="separated">
      {/* Color Palettes */}
      <Accordion.Item value="color_palettes">
        <Accordion.Control>{useTranslateEditor("color_palettes.title")}</Accordion.Control>
        <Accordion.Panel>
          <ColorPaletteEditor
            colorName={useTranslateEditor("color_palettes.base_colors")}
            colors={Object.values(colors["dark"] as string[]).map((c, _) => ({ value: c }))}
            onColorChange={(index, color) => handleColorChange(`dark`, index, color)}
          />
          <ChartColorPaletteEditor
            colorName={useTranslateEditor("color_palettes.stock_status")}
            baseTranslation={(key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
              useTranslateEnums(`stock_status.${key}`, { ...context }, i18Key)
            }
            colors={other.stockStatus}
            onColorChange={(property, color) => handlePropertyChange(`other.stockStatus.${property}`, color)}
          />
          <ChartColorPaletteEditor
            colorName={useTranslateEditor("color_palettes.transaction_type")}
            baseTranslation={(key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
              useTranslateEnums(`transaction_type.${key}`, { ...context }, i18Key)
            }
            colors={other.transactionType}
            onColorChange={(property, color) => handlePropertyChange(`other.transactionType.${property}`, color)}
          />
          <ChartColorPaletteEditor
            colorName={useTranslateEditor("color_palettes.alert_type")}
            baseTranslation={(key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
              useTranslateEnums(`alert_type.${key}`, { ...context }, i18Key)
            }
            colors={other.alertType}
            onColorChange={(property, color) => handlePropertyChange(`other.alertType.${property}`, color)}
          />
          <ChartColorPaletteEditor
            colorName={useTranslateEditor("color_palettes.user_status")}
            baseTranslation={(key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
              useTranslateEnums(`user_status.${key}`, { ...context }, i18Key)
            }
            colors={other.userStatus}
            onColorChange={(property, color) => handlePropertyChange(`other.userStatus.${property}`, color)}
          />
          <ChartColorPaletteEditor
            colorName={useTranslateEditor("color_palettes.item_type")}
            baseTranslation={(key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
              useTranslateEnums(`item_type.${key}`, { ...context }, i18Key)
            }
            colors={other.itemType}
            onColorChange={(property, color) => handlePropertyChange(`other.itemType.${property}`, color)}
          />
          <Group>
            <ColorInput
              label={useTranslateEditor("color_palettes.logo_color")}
              size="xs"
              value={other.logoColor}
              onChangeEnd={(value) => handlePropertyChange("other.logoColor", value)}
              w={100}
              withEyeDropper={false}
            />
            <ColorInput
              label={useTranslateEditor("color_palettes.positive")}
              size="xs"
              value={other.positiveColor}
              onChangeEnd={(value) => handlePropertyChange("other.positiveColor", value)}
              w={100}
              withEyeDropper={false}
            />
            <ColorInput
              label={useTranslateEditor("color_palettes.negative")}
              size="xs"
              value={other.negativeColor}
              onChangeEnd={(value) => handlePropertyChange("other.negativeColor", value)}
              w={100}
              withEyeDropper={false}
            />
            <ColorInput
              label={useTranslateEditor("color_palettes.profit")}
              size="xs"
              value={other.profit}
              onChangeEnd={(value) => handlePropertyChange("other.profit", value)}
              w={100}
              withEyeDropper={false}
            />
          </Group>
        </Accordion.Panel>
      </Accordion.Item>
      {/* Chart Styles */}
      <Accordion.Item value="chart_styles">
        <Accordion.Control>{useTranslateEditor("chart_styles.title")}</Accordion.Control>
        <Accordion.Panel>
          <Group>
            <ChartColorPaletteEditor
              colorName={useTranslateEditor("chart_styles.total")}
              baseTranslation={(key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
                useTranslateEditor(`chart_styles.fields.${key}`, { ...context }, i18Key)
              }
              colors={other.chartStyles.total}
              onColorChange={(property, color) => handlePropertyChange(`other.chartStyles.total.${property}`, color)}
            />
            <ChartColorPaletteEditor
              colorName={useTranslateEditor("chart_styles.today")}
              baseTranslation={(key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
                useTranslateEditor(`chart_styles.fields.${key}`, { ...context }, i18Key)
              }
              colors={other.chartStyles.today}
              onColorChange={(property, color) => handlePropertyChange(`other.chartStyles.today.${property}`, color)}
            />
            <ChartColorPaletteEditor
              colorName={useTranslateEditor("chart_styles.last_days")}
              baseTranslation={(key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
                useTranslateEditor(`chart_styles.fields.${key}`, { ...context }, i18Key)
              }
              colors={other.chartStyles.lastDays}
              onColorChange={(property, color) => handlePropertyChange(`other.chartStyles.lastDays.${property}`, color)}
            />
          </Group>
        </Accordion.Panel>
      </Accordion.Item>
      {/* Sharing */}
      <Accordion.Item value="sharing">
        <Accordion.Control>{useTranslateEditor("sharing.title")}</Accordion.Control>
        <Accordion.Panel>
          <Group>
            <Button
              onClick={() => {
                writeText(JSON.stringify(currentThemeData, null, 2));
                notifications.show({
                  title: useTranslateButtons("copy_to_clipboard.title"),
                  message: useTranslateButtons("copy_to_clipboard.message"),
                  color: "green",
                });
              }}
            >
              {useTranslateButtons("copy_to_clipboard")}
            </Button>
            <Button
              onClick={() => {
                readText().then((text) => {
                  if (text) {
                    importTheme(text);
                    notifications.show({
                      title: useTranslateButtons("import_from_clipboard.title"),
                      message: useTranslateButtons("import_from_clipboard.message"),
                      color: "green",
                    });
                  }
                });
              }}
            >
              {useTranslateButtons("import_from_clipboard")}
            </Button>
            <Button onClick={() => api.cache.openThemeFolder()}>{useTranslateButtons("open_themes_folder")}</Button>
          </Group>
          <Group mt={"md"}>
            <TextInput
              required
              label={useTranslateFormFields("export_name.label")}
              value={export_form.values.name}
              onChange={(e) => export_form.setFieldValue("name", e.target.value)}
              error={export_form.errors.name}
              placeholder={useTranslateFormFields("export_name.placeholder")}
            />
            <TextInput
              required
              label={useTranslateFormFields("export_author.label")}
              value={export_form.values.author}
              onChange={(e) => export_form.setFieldValue("author", e.target.value)}
              error={export_form.errors.author}
              placeholder={useTranslateFormFields("export_author.placeholder")}
              rightSection={
                <ActionWithTooltip
                  tooltip={useTranslateFormFields("export_author.tooltip")}
                  icon={faPlus}
                  onClick={() => {
                    export_form.validate();
                    if (!export_form.isValid()) return;
                    exportTheme.mutateAsync({ ...export_form.values, properties: currentThemeData });
                  }}
                />
              }
            />
          </Group>
        </Accordion.Panel>
      </Accordion.Item>
    </Accordion>
  );
}
