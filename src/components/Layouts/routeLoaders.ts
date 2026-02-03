type RouteLoader = () => Promise<unknown>;

export const routeLoaders = {
  home: () => import("@pages/home"),
  login: () => import("@pages/auth/login"),
  debug: () => import("@pages/debug"),
  error: () => import("@pages/error"),
  banned: () => import("@pages/banned"),
  liveScraper: () => import("@pages/live_scraper"),
  tradingAnalytics: () => import("@pages/trading_analytics"),
  warframeMarket: () => import("@pages/warframe_market"),
  chat: () => import("@pages/chat"),
  tradeMessages: () => import("@pages/trade_messages"),
  about: () => import("@pages/about"),
} as const satisfies Record<string, RouteLoader>;

export type RouteLoaderKey = keyof typeof routeLoaders;

const prefetched = new Set<RouteLoaderKey>();

export const prefetchRoute = (key: RouteLoaderKey) => {
  if (prefetched.has(key)) return;
  prefetched.add(key);
  routeLoaders[key]().catch(() => {
    prefetched.delete(key);
  });
};

export const prefetchRoutes = (keys: RouteLoaderKey[]) => {
  keys.forEach(prefetchRoute);
};

export const prefetchLoggedInRoutes = () => {
  prefetchRoutes([
    "home",
    "liveScraper",
    "tradingAnalytics",
    "tradeMessages",
    "warframeMarket",
    "chat",
    "about",
    "login",
    "error",
    "banned",
  ]);
};
