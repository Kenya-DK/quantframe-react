import { createContext, useContext, useEffect, useState } from "react";
import api, { OnTauriDataEvent } from "@api/index";
import { QfSocketEvent, QfSocketEventOperation, User, UserStatus } from "@api/types";
import { wfmSocket } from "@models/index";
import { Wfm } from "../types";
export type AuthContextProps = {
  user: User | undefined;
}
export type TauriContextProviderProps = {
  children: React.ReactNode;
}

export const AuthContext = createContext<AuthContextProps>({
  user: undefined,
});

export const useAuthContext = () => useContext(AuthContext);

export function AuthContextProvider({ children }: TauriContextProviderProps) {
  const [user, setUser] = useState<User | undefined>(undefined);

  // Handle update, create, delete transaction
  const handleUpdateUser = (operation: string, data: User) => {
    switch (operation) {
      case QfSocketEventOperation.CREATE_OR_UPDATE:
        setUser((user) => ({ ...user, ...data }));
        break;
      case QfSocketEventOperation.DELETE:
        setUser(undefined);
        break;
      case QfSocketEventOperation.SET:
        setUser(data);
        break;
    }
  }

  const OnUserStatusChange = async (status: UserStatus) => {
    // Update user status in backend
    await api.auth.set_status(status);
    setUser((user) => {
      if (!user)
        return user;
      return { ...user, status };
    });
  };
  // Hook on tauri events from rust side
  useEffect(() => {
    wfmSocket.on(Wfm.SocketEvent.OnUserStatusChange, OnUserStatusChange);
    OnTauriDataEvent<User>(QfSocketEvent.UpdateUser, ({ data, operation }) => handleUpdateUser(operation, data));
    return () => {
      wfmSocket.off(Wfm.SocketEvent.OnUserStatusChange, OnUserStatusChange);
    }
  }, []);

  return (
    <AuthContext.Provider value={{ user }}>
      {children}
    </AuthContext.Provider>
  )
}