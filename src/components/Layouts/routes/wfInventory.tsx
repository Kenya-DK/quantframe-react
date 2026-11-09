import WFInventoryModsPage from "@pages/wf_inventory/mod";
import WFInventoryRivenPage from "@pages/wf_inventory/riven";
import { RouteObject } from "react-router-dom";
import AuthenticatedGate from "../../AuthenticatedGate";
import { WFInventoryLayout } from "../WFInventory";

/**
 * Full-screen feature layouts that get their own shell (no app sidebar):
 * one gate + one layout block per feature. Add new feature layouts here.
 */
export const wfInventoryRoutes: RouteObject[] = [
  {
    element: <AuthenticatedGate goTo="/auth/login" />,
    children: [
      {
        element: <WFInventoryLayout />,
        children: [
          { path: "wf_inventory/mods", element: <WFInventoryModsPage /> },
          { path: "wf_inventory/riven", element: <WFInventoryRivenPage /> },
        ],
      },
    ],
  },
];
