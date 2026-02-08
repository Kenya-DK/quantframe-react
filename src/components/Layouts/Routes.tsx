import { BrowserRouter, Route, Routes } from "react-router-dom";
import { useAppContext } from "@contexts/app.context";
import { useAuthContext } from "@contexts/auth.context";
import { lazy } from "react";
import { routeLoaders } from "./routeLoaders";

// Layouts
import { LogInLayout } from "./LogIn";
import { LogOutLayout } from "./LogOut";
import { CleanLayout } from "./Clean";

// Permissions Gate
import AuthenticatedGate from "../AuthenticatedGate";

// Lazy loaded pages for code splitting

// Home Routes
const PHome = lazy(routeLoaders.home);

// Auth Routes
const PLogin = lazy(routeLoaders.login);

// Debug Routes
const PDebug = lazy(routeLoaders.debug);

// Error Routes
const PError = lazy(routeLoaders.error);

// Banned Routes
const PBanned = lazy(routeLoaders.banned);

// Live Scraper
const PLiveScraper = lazy(routeLoaders.liveScraper);

// Trading Analytics
const TradingAnalyticsPage = lazy(routeLoaders.tradingAnalytics);

// Warframe Market
const PWarframeMarket = lazy(routeLoaders.warframeMarket);
const PWarframeMarketChat = lazy(routeLoaders.chat);

// Trade messages
const PTradeMessages = lazy(routeLoaders.tradeMessages);

// About Page
const AboutPage = lazy(routeLoaders.about);

// Clean Pages
const CleanPage = lazy(routeLoaders.clean);

export function AppRoutes() {
  const { app_error } = useAppContext();
  const { user } = useAuthContext();

  const ShowErrorPage = () => {
    if (!window.location.href.includes("clean")) return false;
    if (!app_error) return false;
    if (app_error.isWebSocket()) return false; // Show error page only for non-WebSocket errors
    return true;
  };

  const IsUserBanned = () => {
    if (!window.location.href.includes("clean")) return false;
    if (!user) return false;
    if (user.anonymous) return false;
    if (user.qf_banned || user.wfm_banned) return true;
    return false;
  };

  const ShowCleanLayout = () => {
    if (window.location.href.includes("clean")) return true;
    return false;
  };

  return (
    <BrowserRouter>
      <Routes>
        {ShowCleanLayout() && (
          <Route path="clean" element={<CleanLayout />}>
            <Route index element={<CleanPage />} />
          </Route>
        )}
        {!ShowErrorPage() && !IsUserBanned() && !ShowCleanLayout() && (
          <>
            <Route element={<AuthenticatedGate exclude goTo="/" />}>
              <Route path="/auth" element={<LogOutLayout />}>
                <Route path="login" element={<PLogin />} />
              </Route>
            </Route>
            <Route path="/" element={<LogInLayout />}>
              <Route element={<AuthenticatedGate goTo="/auth/login" />}>
                <Route path="/" element={<PHome />} />
                <Route path="debug">
                  <Route index element={<PDebug />} />
                </Route>
                <Route path="live_scraper" element={<PLiveScraper />} />
                <Route path="warframe-market" element={<PWarframeMarket />} />
                <Route path="chat" element={<PWarframeMarketChat />} />
                <Route path="trading_analytics" element={<TradingAnalyticsPage />} />
                <Route path="trade_messages" element={<PTradeMessages />} />
                <Route path="about" element={<AboutPage />} />
              </Route>
              <Route path="*" element={<PHome />} />
            </Route>
          </>
        )}
        {ShowErrorPage() && !ShowCleanLayout() && (
          <Route path="*" element={<LogOutLayout />}>
            <Route path="*" element={<PError />} />
          </Route>
        )}
        {IsUserBanned() && !ShowCleanLayout() && (
          <Route path="*" element={<LogOutLayout />}>
            <Route path="*" element={<PBanned />} />
          </Route>
        )}
      </Routes>
    </BrowserRouter>
  );
}
