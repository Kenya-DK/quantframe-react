import { TauriTypes } from "$types";
import { defaultTheme, useTheme } from "@contexts/theme.context";
import { Box, Flex, Title } from "@mantine/core";
import { ThemePreview } from "@components/DataDisplay/ThemePreview";
import { LiveThemeEditor } from "@components/ThemeEditor/LiveThemeEditor";
import api from "@api/index";
import { useTranslateForms } from "@hooks/useTranslate.hook";

export type ThemesPanelProps = {
  value: TauriTypes.Settings;
  onSubmit: (value: TauriTypes.Settings) => void;
};

export const ThemesPanel = ({}: ThemesPanelProps) => {
  const { switchTheme } = useTheme();

  const useTranslateEditor = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`settings.tabs.themes.${key}`, { ...context }, i18Key);

  const { data } = api.cache.getThemePresets();
  return (
    <Box p={"md"}>
      <Title order={4} mb="md">
        {useTranslateEditor("community_themes")}
      </Title>
      <Flex p={"md"} gap="sm" justify="flex-start" align="flex-start" direction="row" wrap="wrap">
        <ThemePreview
          icon={defaultTheme.iconBase64}
          theme={defaultTheme.properties}
          name={defaultTheme.name}
          author={defaultTheme.author}
          onClick={() => switchTheme(defaultTheme.properties)}
        />
        {data?.map((theme, i) => (
          <ThemePreview
            key={i}
            icon={theme.iconBase64}
            fileName={theme.fileName}
            theme={theme.properties}
            name={theme.name}
            author={theme.author}
            onClick={() => switchTheme(theme.properties)}
          />
        ))}
      </Flex>
      <Title order={4} mb="md">
        {useTranslateEditor("theme_configuration")}
      </Title>
      <LiveThemeEditor />
    </Box>
  );
};
