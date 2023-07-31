import { Navigate, Outlet } from "react-router-dom";
import { useIsAuthenticated } from "@hooks/index";

type Props = {
  RenderError?: React.ComponentType,
  exclude?: boolean,
  goTo?: string,
  children?: JSX.Element,
};
const AuthenticatedGate: React.FC<Props> = ({
  children = undefined,
  exclude = false,
  RenderError = () => <></>,
  goTo,
}) => {
  const isAuthenticated: boolean = exclude ? !useIsAuthenticated() : useIsAuthenticated();
  if (!isAuthenticated) return goTo ? <Navigate to={goTo} /> : <RenderError /> || <></>;
  return children ? children : <Outlet />;
};
export default AuthenticatedGate;