import { useGetUser } from ".";

export const useIsAuthenticated = () => {
  const user = useGetUser();
  return (user && !user.anonymous && user.verification) ? true : false;
}