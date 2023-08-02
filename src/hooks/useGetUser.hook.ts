import { useContext } from "react";
import { TauriContext } from "@contexts/index";
export const useGetUser = () => {
  const authState = useContext(TauriContext)
  return authState.user
}
