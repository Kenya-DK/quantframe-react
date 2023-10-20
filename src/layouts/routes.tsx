import { BrowserRouter, Route, Routes } from 'react-router-dom'

// Layouts
import MainLayout from './main.layout'
import AuthLayout from './auth.layout'

// Base routes
import PHome from '../pages/home'
import PNotFoundPage from './notFound'

// Auth routes
import PLogin from '../pages/auth/login'
import AuthenticatedGate from '../components/AuthenticatedGate'

// Live Trading routes
import PLiveTrading from '../pages/liveTrading'
import PWTBMessage from '../pages/liveTrading/tabs/rivens/wtbMessage';



// Statistics routes
import PStatistics from '../pages/statistics'

// Warframe Market routes
import PWarframeMarket from '../pages/warframeMarket'


// Debug routes
import PDebug from '../pages/debug'
export default function AppRoutes() {
  return (
    <BrowserRouter>
      <Routes>
        <Route element={<AuthenticatedGate exclude goTo="/" />} >
          <Route path="/auth" element={<AuthLayout />}>
            <Route path="login" element={<PLogin />} />
          </Route>
        </Route>
        <Route path="/" element={<MainLayout />}>
          <Route element={<AuthenticatedGate goTo="/auth/login" />} >
            <Route index element={<PHome />} />
            <Route path="live-trading" >
              <Route index element={<PLiveTrading />} />
              <Route path="riven_wtb_message" element={<PWTBMessage />} />
            </Route>
            <Route path="statistics" element={<PStatistics />} />
            <Route path="warframe-market" element={<PWarframeMarket />} />
            <Route path="debug" element={<PDebug />} />
          </Route>
        </Route>
        <Route path="*" element={<PNotFoundPage />} />
      </Routes>
    </BrowserRouter>
  )
}