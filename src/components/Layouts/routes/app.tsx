import { lazy } from "react";
import { RouteObject } from "react-router-dom";
import AuthenticatedGate from "../../AuthenticatedGate";
import { LogInLayout } from "../LogIn";
import { routeLoaders } from "../routeLoaders";

const PHome = lazy(routeLoaders.home);
const PDebug = lazy(routeLoaders.debug);
const PLiveScraper = lazy(routeLoaders.liveScraper);
const PWarframeMarket = lazy(routeLoaders.warframeMarket);
const PWarframeMarketChat = lazy(routeLoaders.chat);
const TradingAnalyticsPage = lazy(routeLoaders.tradingAnalytics);
const PTradeMessages = lazy(routeLoaders.tradeMessages);
const AboutPage = lazy(routeLoaders.about);

/**
 * The main app shell: every authenticated page rendered inside the LogInLayout
 * (sidebar navigation, ticker, user menu).
 */
export const appShellRoutes: RouteObject[] = [
  {
    path: "/",
    element: <LogInLayout />,
    children: [
      {
        element: <AuthenticatedGate goTo="/auth/login" />,
        children: [
          { path: "/", element: <PHome /> },
          { path: "debug", element: <PDebug /> },
          { path: "live_scraper", element: <PLiveScraper /> },
          { path: "warframe-market", element: <PWarframeMarket /> },
          { path: "chat", element: <PWarframeMarketChat /> },
          { path: "trading_analytics", element: <TradingAnalyticsPage /> },
          { path: "trade_messages", element: <PTradeMessages /> },
          { path: "about", element: <AboutPage /> },
        ],
      },
      { path: "*", element: <PHome /> },
    ],
  },
];
