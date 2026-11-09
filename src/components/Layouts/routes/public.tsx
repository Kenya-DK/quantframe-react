import { lazy } from "react";
import { RouteObject } from "react-router-dom";
import AuthenticatedGate from "../../AuthenticatedGate";
import { LogOutLayout } from "../LogOut";
import { routeLoaders } from "../routeLoaders";

const PLogin = lazy(routeLoaders.login);

/**
 * Public routes: the login page. Wrapped in a gate that bounces already-authenticated users back to the app.
 */
export const publicRoutes: RouteObject[] = [
  {
    element: <AuthenticatedGate exclude goTo="/" />,
    children: [
      {
        path: "/auth",
        element: <LogOutLayout />,
        children: [{ path: "login", element: <PLogin /> }],
      },
    ],
  },
];
