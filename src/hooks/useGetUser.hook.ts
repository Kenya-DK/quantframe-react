import { useContext } from "react";
import { AuthContext } from "$contexts/index";
export const useGetUser = () => {
  const authState = useContext(AuthContext)
  return authState.user
}
