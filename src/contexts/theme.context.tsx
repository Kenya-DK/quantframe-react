import React, { createContext, useContext, useCallback } from "react";
import { createTheme, CSSVariablesResolver } from "@mantine/core";
import { useLocalStorage } from "@mantine/hooks";
import { defaultTheme, generateCSSVariables, ThemeContextType } from "./static";
import { notifications } from "@mantine/notifications";

const ThemeContext = createContext<ThemeContextType | undefined>(undefined);

interface ThemeProviderProps {
  children: React.ReactNode;
}

const validateTheme = (theme: Record<string, any>) => {
  const properties = defaultTheme.properties;
  for (const key in properties.colors) if (theme.colors[key] == undefined) theme.colors[key] = properties.colors[key];
  for (const key in properties.other) {
    const typedKey = key as keyof typeof properties.other;
    if (theme.other == undefined) theme.other = {};
    if (theme.other[typedKey] === undefined) theme.other[typedKey] = properties.other[typedKey];
  }
  return theme;
};

export const ThemeProvider: React.FC<ThemeProviderProps> = ({ children }) => {
  const [currentTheme, setCurrentTheme] = useLocalStorage<object>({
    key: "theme",
    defaultValue: defaultTheme.properties,
  });

  const switchTheme = useCallback(
    async (properties: Record<string, any>) => {
      setCurrentTheme(properties);
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
    setCurrentTheme(defaultTheme);
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
        notifications.show({
          title: "Import Error",
          message: "Failed to import theme. Please ensure the JSON format is correct.",
          color: "red",
        });
        console.error("Failed to import theme:", error);
      }
    },
    [setCurrentTheme]
  );

  const resolver: CSSVariablesResolver = (theme) => ({
    variables: {
      ...generateCSSVariables(theme.other),
    },
    light: {},
    dark: {},
  });

  const GetTheme = useCallback(() => {
    return createTheme(validateTheme(currentTheme));
  }, [currentTheme]);

  const value: ThemeContextType = {
    theme: GetTheme(),
    resolver,
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
