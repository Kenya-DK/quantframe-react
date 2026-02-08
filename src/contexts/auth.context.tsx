import { createContext, useContext, useEffect, useMemo, useState } from "react";
import { OffTauriDataEvent, OnTauriDataEvent } from "@api/index";
import { TauriTypes } from "$types";
import api from "@api/index";
import { useQuery } from "@tanstack/react-query";
export type AuthContextProps = {
  user: TauriTypes.User | undefined;
};
export type TauriContextProviderProps = {
  children: React.ReactNode;
};

export const AuthContext = createContext<AuthContextProps>({
  user: undefined,
});

export const useAuthContext = () => useContext(AuthContext);

export function AuthContextProvider({ children }: TauriContextProviderProps) {
  // States
  const [user, setUser] = useState<TauriTypes.User | undefined>(undefined);

  // Fetch data from rust side
  const { data } = useQuery({
    queryKey: ["auth_me"],
    queryFn: () => api.auth.me(),
    retry: 0,
  });
  useEffect(() => setUser(data), [data]);

  const handleUpdateUser = (operation: string, data: TauriTypes.User) => {
    switch (operation) {
      case TauriTypes.EventOperations.CREATE_OR_UPDATE:
        setUser((user) => ({ ...user, ...data }));
        break;
      case TauriTypes.EventOperations.DELETE:
        setUser(undefined);
        break;
      case TauriTypes.EventOperations.SET:
        setUser(data);
        break;
    }
  };

  // Hook on tauri events from rust side
  useEffect(() => {
    OnTauriDataEvent<TauriTypes.User>(TauriTypes.Events.UpdateUser, ({ data, operation }) => handleUpdateUser(operation, data));
    return () => {
      OffTauriDataEvent<TauriTypes.User>(TauriTypes.Events.UpdateUser, ({ data, operation }) => handleUpdateUser(operation, data));
    };
  }, []);

  const contextValue = useMemo(() => ({ user }), [user]);

  return <AuthContext.Provider value={contextValue}>{children}</AuthContext.Provider>;
}
