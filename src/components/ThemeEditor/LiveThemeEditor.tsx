import { Group, Text, ColorInput, Accordion, Box, Card, TextInput, Button } from "@mantine/core";
import { useTheme } from "@contexts/theme.context";
import { useTranslateComponent, useTranslateEnums } from "@hooks/useTranslate.hook";
import { TauriTypes, UserStatus } from "$types";
import api from "../../api";
import { useForm } from "@mantine/form";
import { useMutation } from "@tanstack/react-query";
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
export function LiveThemeEditor() {
  // Context
  const { currentThemeData, updateThemeProperty } = useTheme();

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

  const handlePropertyChange = (path: string, value: any) => {
    updateThemeProperty(path, value);
  };

  const export_form = useForm({
    initialValues: {
      name: "",
      author: "",
    },
    validate: {
      name: (value) => (value.length < 3 ? useTranslateFormFields("export_name.error") : null),
      author: (value) => (value.length < 3 ? useTranslateFormFields("export_author.error") : null),
    },
  });

  const exportTheme = useMutation({
    mutationFn: (data: { name: string; author: string; properties: any }) => api.cache.createTheme(data.name, data.author, data.properties),
    onSuccess: () => {
      console.log("Theme exported successfully");
      // Handle success, e.g., show a notification
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

          <ColorPaletteEditor
            colorName={useTranslateEditor("color_palettes.stock_status")}
            colors={Object.values(TauriTypes.StockStatus).map((status, i) => ({
              name: useTranslateEnums(`stock_status.${status}`),
              value: colors["stock-status"][i],
            }))}
            onColorChange={(index, color) => handleColorChange("stock-status", index, color)}
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
            <ColorPaletteEditor
              colorName={useTranslateEditor("color_palettes.user_status")}
              colors={Object.values(UserStatus).map((status, i) => ({
                name: useTranslateEnums(`user_status.${status}`),
                value: colors["user-status"][i],
              }))}
              onColorChange={(index, color) => handleColorChange("user-status", index, color)}
            />
            <ColorPaletteEditor
              colorName={useTranslateEditor("color_palettes.transaction_type")}
              colors={["purchase", "sale"].map((status, i) => ({
                name: useTranslateEnums(`transaction_type.${status}`),
                value: colors["transaction-type"][i],
              }))}
              onColorChange={(index, color) => handleColorChange("transaction-type", index, color)}
            />
            <ColorPaletteEditor
              colorName={useTranslateEditor("color_palettes.item_type")}
              colors={["item", "riven"].map((status, i) => ({
                name: useTranslateEnums(`item_type.${status}`),
                value: colors["item-type"][i],
              }))}
              onColorChange={(index, color) => handleColorChange("item-type", index, color)}
            />
          </Group>
        </Accordion.Panel>
      </Accordion.Item>
      {/* Sharing */}
      <Accordion.Item value="sharing">
        <Accordion.Control>{useTranslateEditor("sharing.title")}</Accordion.Control>
        <Accordion.Panel>
          <Group>
            <Button onClick={() => {}}>{useTranslateButtons("copy_to_clipboard")}</Button>
            <Button onClick={() => {}}>{useTranslateButtons("import_from_clipboard")}</Button>
            <Button onClick={() => {}}>{useTranslateButtons("open_themes_folder")}</Button>
          </Group>
          <Group mt={"md"}>
            <Button
              mt={30}
              onClick={async () => {
                export_form.validate();
                if (!export_form.isValid()) return;
                exportTheme.mutateAsync({ ...export_form.values, properties: currentThemeData });
              }}
            >
              {useTranslateButtons("export_theme")}
            </Button>
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
            />
          </Group>
        </Accordion.Panel>
      </Accordion.Item>
    </Accordion>
  );
}
