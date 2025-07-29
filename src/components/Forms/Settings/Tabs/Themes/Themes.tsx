import { TauriTypes } from "$types";

export type ThemesPanelProps = {
  value: TauriTypes.Settings;
  onSubmit: (value: TauriTypes.Settings) => void;
};

export const ThemesPanel = ({}: ThemesPanelProps) => {
  return <>Themes Panel</>;
};
