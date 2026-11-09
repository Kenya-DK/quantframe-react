import { lazy } from "react";
import { RouteObject } from "react-router-dom";
import { CleanLayout } from "../Clean";
import { LogOutLayout } from "../LogOut";
import { routeLoaders } from "../routeLoaders";

const PError = lazy(routeLoaders.error);
const PBanned = lazy(routeLoaders.banned);
const CleanPage = lazy(routeLoaders.clean);

/** Chrome-free minimal page (the same app, no navigation UI). */
export const cleanRoutes: RouteObject[] = [
  {
    path: "clean",
    element: <CleanLayout />,
    children: [{ index: true, element: <CleanPage /> }],
  },
];

/** Blocks everything but a full-page error screen. */
export const errorRoutes: RouteObject[] = [
  {
    path: "*",
    element: <LogOutLayout />,
    children: [{ path: "*", element: <PError /> }],
  },
];

/** Blocks everything but a full-page banned screen. */
export const bannedRoutes: RouteObject[] = [
  {
    path: "*",
    element: <LogOutLayout />,
    children: [{ path: "*", element: <PBanned /> }],
  },
];
