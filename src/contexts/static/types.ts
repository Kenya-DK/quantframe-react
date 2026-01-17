import { createTheme, CSSVariablesResolver } from "@mantine/core";

export interface ThemeContextType {
  theme: ReturnType<typeof createTheme>;
  resolver: CSSVariablesResolver;
  currentThemeData: any;
  switchTheme: (properties: Record<string, any>) => void;
  updateThemeProperty: (path: string, value: any) => void;
  resetTheme: () => void;
  exportTheme: () => string;
  importTheme: (themeData: string) => void;
}
