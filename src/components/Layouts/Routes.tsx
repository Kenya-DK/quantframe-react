import { BrowserRouter, Route, Routes } from "react-router-dom";

// Layouts
import { LogInLayout } from "./LogIn";
import { LogOutLayout } from "./LogOut";

// Permissions Gate
import AuthenticatedGate from "../AuthenticatedGate";

// Home Routes
import PHome from "@pages/home";

// Auth Routes
import PLogin from "@pages/auth/login";

// Live Trading routes
import PLiveTrading from "@pages/liveTrading";

// Live Trading routes
import PWarframeMarket from "@pages/warframeMarket";

// Debug Routes
import PDebug from "@pages/debug";

// Error Routes
import PError from "@pages/error";

// Banned Routes
import PBanned from "@pages/banned";

// Banned Routes
import PAbout from "@pages/about";

// Chats Routes
import PChats from "@pages/chats";

// Banned Routes
import PTest from "@pages/test";

// Control Routes
import { LiveTradingControl } from "../LiveTradingControl";

export function AppRoutes() {
  return (
    <BrowserRouter>
      <Routes>
        <Route element={<AuthenticatedGate exclude goTo="/" />}>
          <Route path="/auth" element={<LogOutLayout />}>
            <Route path="login" element={<PLogin />} />
          </Route>
        </Route>
        <Route path="/" element={<LogInLayout />}>
          <Route element={<AuthenticatedGate goTo="/auth/login" />}>
            <Route index element={<PHome />} />
            <Route path="live-trading">
              <Route index element={<PLiveTrading />} />
            </Route>
            <Route path="chats">
              <Route index element={<PChats />} />
            </Route>
            <Route path="debug">
              <Route index element={<PDebug />} />
            </Route>
            <Route path="warframe-market">
              <Route index element={<PWarframeMarket />} />
            </Route>
            <Route path="about">
              <Route index element={<PAbout />} />
            </Route>
            <Route path="test">
              <Route index element={<PTest />} />
            </Route>
          </Route>
        </Route>
        <Route path="controls">
          <Route path="live-trading" element={<LiveTradingControl />} />
        </Route>
        <Route path="/error" element={<LogOutLayout />}>
          <Route index element={<PError />} />
          <Route path="banned" element={<PBanned />} />
        </Route>
        <Route path="*" element={<PHome />} />
      </Routes>
    </BrowserRouter>
  );
}
