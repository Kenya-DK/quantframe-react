import { Navigate, Outlet } from "react-router-dom";
import { useIsAuthenticated } from "@hooks/useIsAuthenticated.hook";

type Props = {
  RenderError?: React.ComponentType;
  exclude?: boolean;
  goTo?: string;
  children?: JSX.Element;
};
const AuthenticatedGate: React.FC<Props> = ({ children = undefined, exclude = false, RenderError = undefined, goTo }) => {
  const isAuthenticated: boolean = exclude ? !useIsAuthenticated() : useIsAuthenticated();
  if (!isAuthenticated) {
    if (goTo) return <Navigate to={goTo} />;
    if (RenderError) return <RenderError />;
    return <></>;
  }
  return children ? children : <Outlet />;
};
export default AuthenticatedGate;
