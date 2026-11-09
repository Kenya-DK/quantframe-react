import { lazy } from "react";
import { RouteObject } from "react-router-dom";
import AuthenticatedGate from "../../AuthenticatedGate";
import { WFInventoryLayout } from "../WFInventory";
import { routeLoaders } from "../routeLoaders";

const WfInventoryModsPage = lazy(routeLoaders.wfInventoryMods);
const WfInventoryRivenPage = lazy(routeLoaders.wfInventoryRiven);

/**
 * Full-screen feature layouts that get their own shell (no app sidebar):
 * one gate + one layout block per feature. Add new feature layouts here.
 */
export const fullScreenRoutes: RouteObject[] = [
  {
    element: <AuthenticatedGate goTo="/auth/login" />,
    children: [
      {
        element: <WFInventoryLayout />,
        children: [
          { path: "wf_inventory/mods", element: <WfInventoryModsPage /> },
          { path: "wf_inventory/riven", element: <WfInventoryRivenPage /> },
        ],
      },
    ],
  },
];
