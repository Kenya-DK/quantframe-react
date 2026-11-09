import { useAppContext } from "@contexts/app.context";
import { useAuthContext } from "@contexts/auth.context";
import { BrowserRouter, RouteObject, useRoutes } from "react-router-dom";
import { appShellRoutes } from "./routes/app";
import { bannedRoutes, cleanRoutes, errorRoutes } from "./routes/fallbacks";
import { publicRoutes } from "./routes/public";
import { wfInventoryRoutes } from "./routes/wfInventory";

/** Which route tree is rendered — decided once, used instead of scattered booleans. */
type AppMode = "app" | "clean" | "error" | "banned" | "wf_inventory";

/** clean > banned > error > app (mirrors the old precedence of the boolean mix). */
const useAppMode = (): AppMode => {
  const { app_error } = useAppContext();
  const { user } = useAuthContext();

  if (window.location.href.includes("clean")) return "clean";
  if (window.location.href.includes("wf_inventory")) return "wf_inventory";
  if (!!user && !user.anonymous && (user.qf_banned || user.wfm_banned)) return "banned";
  if (app_error && app_error.error.component !== "WebSocket") return "error";
  return "app";
};

export function AppRoutes() {
  return (
    <BrowserRouter>
      <ModeRoutes />
    </BrowserRouter>
  );
}

/** Runs inside the router so `useRoutes` has its context. */
function ModeRoutes() {
  const mode = useAppMode();
  return useRoutes(modeByRouteTree[mode]);
}

const modeByRouteTree: Record<AppMode, RouteObject[]> = {
  clean: cleanRoutes,
  error: errorRoutes,
  banned: bannedRoutes,
  wf_inventory: wfInventoryRoutes,
  app: [...publicRoutes, ...appShellRoutes],
};
