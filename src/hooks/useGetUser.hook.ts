import { useContext } from "react";
import { AuthContext } from "@contexts/auth.context";
export const useGetUser = () => {
  const authState = useContext(AuthContext)
  return authState.user
}
