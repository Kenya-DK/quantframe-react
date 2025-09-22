import { BrowserRouter, Route, Routes } from "react-router-dom";
import { useAppContext } from "@contexts/app.context";
import { useAuthContext } from "@contexts/auth.context";

// Layouts
import { LogInLayout } from "./LogIn";
import { LogOutLayout } from "./LogOut";

// Permissions Gate
import AuthenticatedGate from "../AuthenticatedGate";

// Home Routes
import PHome from "@pages/home";

// Auth Routes
import PLogin from "@pages/auth/login";

// Debug Routes
import PDebug from "@pages/debug";

// Error Routes
import PError from "@pages/error";

// Banned Routes
import PBanned from "@pages/banned";

// Live Scraper
import PLiveScraper from "@pages/live_scraper";

// Warframe Market
import PWarframeMarket from "@pages/warframe_market";
import PWarframeMarketChat from "@pages/chat";

export function AppRoutes() {
  const { app_error } = useAppContext();
  const { user } = useAuthContext();

  const ShowErrorPage = () => {
    if (!app_error) return false;
    if (app_error.isWebSocket()) return false; // Show error page only for non-WebSocket errors
    return true;
  };

  const IsUserBanned = () => {
    if (!user) return false;
    if (user.anonymous) return false;
    if (user.qf_banned || user.wfm_banned) return true;
    return false;
  };

  return (
    <BrowserRouter>
      <Routes>
        {!ShowErrorPage() && !IsUserBanned() && (
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
              </Route>
              <Route path="*" element={<PHome />} />
            </Route>
          </>
        )}
        {ShowErrorPage() && (
          <Route path="*" element={<LogOutLayout />}>
            <Route path="*" element={<PError />} />
          </Route>
        )}
        {IsUserBanned() && (
          <Route path="*" element={<LogOutLayout />}>
            <Route path="*" element={<PBanned />} />
          </Route>
        )}
      </Routes>
    </BrowserRouter>
  );
}
