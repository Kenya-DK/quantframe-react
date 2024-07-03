import { useGetUser } from ".";

export const useIsAuthenticated = () => {
  const user = useGetUser();
  return (user && user.authorized) ? true : false;
}
