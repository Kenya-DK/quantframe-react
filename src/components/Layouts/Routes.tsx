import { BrowserRouter, Route, Routes } from 'react-router-dom'

// Layouts
import { LiveTradingControl, LogOutLayout, LogInLayout } from '@components'

// Permissions Gate
import AuthenticatedGate from '../AuthenticatedGate'


// Home Routes
import PHome from '@pages/home'

// Auth Routes
import PLogin from '@pages/auth/login'

// Live Trading routes
import PLiveTrading from '@pages/liveTrading'

// Live Trading routes
import PWarframeMarket from '@pages/warframeMarket'

// Debug Routes
import PDebug from '@pages/debug'

// Error Routes
import PError from '@pages/error'

export function AppRoutes() {
  return (
    <BrowserRouter>
      <Routes>
        <Route element={<AuthenticatedGate exclude goTo="/" />} >
          <Route path="/auth" element={<LogOutLayout />}>
            <Route path="login" element={<PLogin />} />
          </Route>
        </Route>
        <Route path="/" element={<LogInLayout />}>
          <Route element={<AuthenticatedGate goTo="/auth/login" />} >
            <Route index element={<PHome />} />
            <Route path="live-trading" >
              <Route index element={<PLiveTrading />} />
            </Route>
            <Route path="debug" >
              <Route index element={<PDebug />} />
            </Route>
            <Route path="warframe-market" >
              <Route index element={<PWarframeMarket />} />
            </Route>
          </Route>
        </Route>
        <Route path="controls" >
          <Route path='live-trading' element={<LiveTradingControl />} />
        </Route>
        <Route path="/error" element={<LogOutLayout />}>
          <Route index element={<PError />} />
        </Route>
        <Route path="*" element={<PHome />} />
      </Routes>
    </BrowserRouter>
  )
}