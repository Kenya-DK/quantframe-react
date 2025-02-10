import { hasPermission } from "$lib/utils";
import { useContext } from "react";
import { AuthContext } from "@contexts/auth.context";
import { useGetUser } from "./useGetUser.hook";

export const useHasPermissions = (perms: string) => {
  const authState = useContext(AuthContext);
  if (!perms) return true;
  const permsArr = perms.split(",");
  if (permsArr.length === 0) return true;
  const permissions = (authState?.user?.role?.permissions || "").split(",");
  return authState?.user ? hasPermission(permissions, perms) : false;
};

export const useIsAuthenticated = () => {
  const user = useGetUser();
  return (user && user.role) ? true : false;
}