import { createContext, useContext, useEffect, useState } from "react";
import { Wfm } from '$types/index';
import { OnTauriUpdateDataEvent } from "../utils";

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
      case "create":
      case "update":
        setUser((user) => ({ ...user, ...data }));
        break;
      case "delete":
        setUser(undefined);
        break;
      case "set":
        setUser(data);
        break;
    }
  }

  // Hook on tauri events from rust side
  useEffect(() => {
    OnTauriUpdateDataEvent<Wfm.UserDto>("user", ({ data, operation }) => handleUpdateUser(operation, data));
    return () => { }
  }, []);

  return (
    <AuthContext.Provider value={{ user }}>
      {children}
    </AuthContext.Provider>
  )
}