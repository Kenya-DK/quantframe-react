import { useContext } from "react";
import { AppContext } from "@contexts/app.context";
import { AuthContext } from "@contexts/auth.context";
export const useGetPatreonInfo = () => {
  const appState = useContext(AppContext);
  const authState = useContext(AuthContext);
  console.log("Patreon usernames:", appState.app_info?.patreon_usernames);
  return {
    is_dev: appState.app_info?.is_dev ?? false,
    user: authState.user,
    user_names: appState.app_info?.patreon_usernames ?? [],
  };
};
