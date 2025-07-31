import { TauriTypes } from "$types";
import { useQuery } from "@tanstack/react-query";
import { resolveResource } from "@tauri-apps/api/path";
import { readDir, readTextFile } from "@tauri-apps/plugin-fs";
import { useTheme } from "../../../../../contexts/theme.context";
import { Box, SimpleGrid } from "@mantine/core";
import { ThemePreview } from "../../../../DataDisplay/ThemePreview";
import { LiveThemeEditor } from "../../../../ThemeEditor";

interface Theme {
  name: string;
  author: string;
  fileName: string;
  icon: string;
  properties: Record<string, any>;
}

async function getAllFilesInResourceFolder(): Promise<Theme[]> {
  try {
    // 1. Resolve path to your bundled resource folder
    const resourcePath = await resolveResource("resources/themes");

    // 2. Read directory contents recursively
    const files = await readDir(resourcePath);

    // 3. Map out file paths
    let themes = [];
    for (const file of files) {
      const content = await readTextFile(`resources/themes/${file.name}`);
      themes.push({
        ...JSON.parse(content),
        fileName: file.name,
      } as Theme);
    }
    return themes;
  } catch (error) {
    console.error("Failed to read resource files:", error);
  }
  return [];
}

export type ThemesPanelProps = {
  value: TauriTypes.Settings;
  onSubmit: (value: TauriTypes.Settings) => void;
};

export const ThemesPanel = ({}: ThemesPanelProps) => {
  const { switchTheme } = useTheme();
  const { data } = useQuery({
    queryKey: ["themes"],
    queryFn: getAllFilesInResourceFolder,
    refetchOnWindowFocus: false,
    refetchOnReconnect: false,
    refetchOnMount: false,
    refetchInterval: false,
    refetchIntervalInBackground: false,
  });
  return (
    <Box>
      <LiveThemeEditor />
      <SimpleGrid cols={3} spacing="md">
        {data?.map((theme) => (
          <ThemePreview
            key={theme.fileName}
            fileName={theme.fileName}
            theme={theme.properties}
            name={theme.name}
            author={theme.author}
            onClick={() => switchTheme(theme.fileName)}
          />
        ))}
      </SimpleGrid>
    </Box>
  );
};
