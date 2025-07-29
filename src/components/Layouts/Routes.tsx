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

// Debug Routes
import PDebug from "@pages/debug";

// Error Routes
import PError from "@pages/error";

// Banned Routes
import { useAppContext } from "../../contexts/app.context";

export function AppRoutes() {
  const { app_error } = useAppContext();

  const ShowErrorPage = () => {
    if (!app_error) return false;
    if (app_error.isWebSocket()) return false; // Show error page only for non-WebSocket errors
    return true;
  };

  return (
    <BrowserRouter>
      <Routes>
        {!ShowErrorPage() && (
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
              </Route>
            </Route>
          </>
        )}
        {ShowErrorPage() && (
          <Route path="*" element={<LogOutLayout />}>
            <Route path="*" element={<PError />} />
          </Route>
        )}
        {/* <Route path="/info" element={<LogOutLayout />}>
          <Route path="banned" element={<PBanned />} />
          <Route path="error" element={<PError />} />
        </Route> */}
      </Routes>
    </BrowserRouter>
  );
}
