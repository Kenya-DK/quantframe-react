import { useGetUser } from "./useGetUser.hook";

export const useIsAuthenticated = () => {
  const user = useGetUser();
  return user && !user.anonymous && user.verification ? true : false;
};
