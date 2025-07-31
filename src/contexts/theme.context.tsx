import React, { createContext, useContext, useCallback } from "react";
import { createTheme } from "@mantine/core";
import { useLocalStorage } from "@mantine/hooks";
import { readTextFile } from "@tauri-apps/plugin-fs";

interface ThemeContextType {
  theme: ReturnType<typeof createTheme>;
  currentThemeData: any;
  switchTheme: (theme: string) => void;
  updateThemeProperty: (path: string, value: any) => void;
  resetTheme: () => void;
  exportTheme: () => string;
  importTheme: (themeData: string) => void;
}

const ThemeContext = createContext<ThemeContextType | undefined>(undefined);

export const themes: { [key: string]: ReturnType<typeof createTheme> } = {
  default: createTheme({
    colors: {
      // override dark colors to change them for all components
      dark: ["#d5d7e0", "#acaebf", "#8c8fa3", "#666980", "#4d4f66", "#34354a", "#2b2c3d", "#1d1e30", "#0c0d21", "#01010a"],
    },
  }),
};

interface ThemeProviderProps {
  children: React.ReactNode;
}

export const ThemeProvider: React.FC<ThemeProviderProps> = ({ children }) => {
  const [currentTheme, setCurrentTheme] = useLocalStorage<object>({
    key: "theme",
    defaultValue: {
      colors: {
        // override dark colors to change them for all components
        dark: ["#d5d7e0", "#acaebf", "#8c8fa3", "#666980", "#4d4f66", "#34354a", "#2b2c3d", "#1d1e30", "#0c0d21", "#01010a"],
      },
    },
  });

  const switchTheme = useCallback(
    async (theme: string) => {
      const content = await readTextFile(`resources/themes/${theme}`);
      setCurrentTheme(JSON.parse(content).properties);
    },
    [setCurrentTheme]
  );

  const updateThemeProperty = useCallback(
    (path: string, value: any) => {
      setCurrentTheme((prevTheme) => {
        const newTheme = JSON.parse(JSON.stringify(prevTheme)); // Deep clone
        const pathArray = path.split(".");
        let current: any = newTheme;

        // Navigate to the parent of the target property
        for (let i = 0; i < pathArray.length - 1; i++) {
          const key = pathArray[i];
          if (!current[key]) {
            current[key] = {};
          }
          current = current[key];
        }

        // Set the final property
        const finalKey = pathArray[pathArray.length - 1];
        current[finalKey] = value;

        return newTheme;
      });
    },
    [setCurrentTheme]
  );

  const resetTheme = useCallback(() => {
    setCurrentTheme({
      colors: {
        dark: ["#d5d7e0", "#acaebf", "#8c8fa3", "#666980", "#4d4f66", "#34354a", "#2b2c3d", "#1d1e30", "#0c0d21", "#01010a"],
      },
    });
  }, [setCurrentTheme]);

  const exportTheme = useCallback(() => {
    return JSON.stringify(currentTheme, null, 2);
  }, [currentTheme]);

  const importTheme = useCallback(
    (themeData: string) => {
      try {
        const parsedTheme = JSON.parse(themeData);
        setCurrentTheme(parsedTheme);
      } catch (error) {
        console.error("Failed to import theme:", error);
      }
    },
    [setCurrentTheme]
  );

  const GetTheme = useCallback(() => {
    return createTheme(currentTheme);
  }, [currentTheme]);

  const value: ThemeContextType = {
    theme: GetTheme(),
    currentThemeData: currentTheme,
    switchTheme,
    updateThemeProperty,
    resetTheme,
    exportTheme,
    importTheme,
  };
  return <ThemeContext.Provider value={value}>{children}</ThemeContext.Provider>;
};

export const useTheme = (): ThemeContextType => {
  const context = useContext(ThemeContext);
  if (context === undefined) {
    throw new Error("useTheme must be used within a ThemeProvider");
  }
  return context;
};
