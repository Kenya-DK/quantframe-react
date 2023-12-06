import { createContext, useContext, useEffect, useState } from "react";
import { Wfm } from '$types/index';
import { OnTauriUpdateDataEvent, OnSocketEvent } from "../utils";
import { useMutation } from "@tanstack/react-query";
import api from "@api/index";

type AuthContextProps = {
  user: Wfm.UserDto | undefined;
}
type TauriContextProviderProps = {
  children: React.ReactNode;
}

export const AuthContext = createContext<AuthContextProps>({
  user: undefined,
});

export const useAuthContext = () => useContext(AuthContext);
export const AuthContextProvider = ({ children }: TauriContextProviderProps) => {
  const [user, setUser] = useState<Wfm.UserDto | undefined>(undefined);

  // Handle update, create, delete transaction
  const handleUpdateUser = (operation: string, data: Wfm.UserDto) => {
    switch (operation) {
      case "CREATE":
      case "UPDATE":
        setUser((user) => ({ ...user, ...data }));
        break;
      case "DELETE":
        setUser(undefined);
        break;
      case "SET":
        setUser(data);
        break;
    }
  }

  // Mutations
  const setUserStatus = useMutation((data: Wfm.UserStatus) => api.auth.update_user_status(data), {
    onSuccess: async () => { },
    onError: () => { },
  })

  // Hook on tauri events from rust side
  useEffect(() => {
    OnTauriUpdateDataEvent<Wfm.UserDto>("user", ({ data, operation }) => handleUpdateUser(operation, data));
    OnSocketEvent("USER/SET_STATUS", (data: string) => {
      switch (data) {
        case "online":
          setUserStatus.mutate(Wfm.UserStatus.Online);
          break;
        case "ingame":
          setUserStatus.mutate(Wfm.UserStatus.Ingame);
          break;
        case "invisible":
          setUserStatus.mutate(Wfm.UserStatus.Invisible);
          break;
      }
    });
    return () => { }
  }, []);

  return (
    <AuthContext.Provider value={{ user }}>
      {children}
    </AuthContext.Provider>
  )
}