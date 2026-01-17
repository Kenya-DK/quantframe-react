import { useContext } from "react";
import { AppContext } from "@contexts/app.context";
export const useIsDev = () => {
  const appState = useContext(AppContext);
  return appState.app_info?.is_dev ?? false;
};
