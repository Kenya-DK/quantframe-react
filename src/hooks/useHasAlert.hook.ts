import { useContext } from "react";
import { AppContext } from "@contexts/app.context";
export const useHasAlert = () => {
  const authState = useContext(AppContext);
  return authState.alerts.length > 0;
};
