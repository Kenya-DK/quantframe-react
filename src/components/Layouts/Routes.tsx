import { BrowserRouter, Route, Routes } from "react-router-dom";

// Layouts
import { LogInLayout } from "./LogIn";
import { LogOutLayout } from "./LogOut";

// Permissions Gate
import AuthenticatedGate from "../AuthenticatedGate";

// Auth Routes
import PLogin from "@pages/auth/login";

// Debug Routes
import PDebug from "@pages/debug";

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
            <Route path="debug">
              <Route index element={<PDebug />} />
            </Route>
          </Route>
        </Route>
        <Route path="/error" element={<LogOutLayout />}></Route>
      </Routes>
    </BrowserRouter>
  );
}
