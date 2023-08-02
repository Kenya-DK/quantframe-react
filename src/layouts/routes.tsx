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
          </Route>
        </Route>
        <Route path="*" element={<PNotFoundPage />} />
      </Routes>
    </BrowserRouter>
  )
}