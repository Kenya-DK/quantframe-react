import { createContext, useContext, useEffect, useState } from "react";
import api, { OnTauriDataEvent, OnTauriEvent } from "@api/index";
import { QfSocketEvent, QfSocketEventOperation, User, UserStatus } from "@api/types";
import wfmSocket from "@models/wfmSocket";
import { Wfm } from "../types";
export type AuthContextProps = {
  user: User | undefined;
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
  const [user, setUser] = useState<User | undefined>(undefined);

  // Handle update, create, delete transaction
  const handleUpdateUser = (operation: string, data: User) => {
    window.data = data;
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
  };

  const OnUserStatusChange = async (status: UserStatus) => {
    // Update user status in backend
    await api.auth.set_status(status);
    setUser((user) => {
      if (!user) return user;
      return { ...user, status };
    });
  };

  const handleAddUnreadMessages = (count: number) => {
    setUser((user) => {
      if (!user) return user;
      return { ...user, unread_messages: user.unread_messages + count };
    });
  };

  const handleSubtractUnreadMessages = (count: number) => {
    setUser((user) => {
      if (!user) return user;
      return { ...user, unread_messages: user.unread_messages - count };
    });
  };

  const handleSetUnreadMessages = (count: number) => {
    setUser((user) => {
      if (!user) return user;
      return { ...user, unread_messages: count };
    });
  };

  // Hook on tauri events from rust side
  useEffect(() => {
    wfmSocket.on(Wfm.SocketEvent.OnUserStatusChange, OnUserStatusChange);
    OnTauriDataEvent<User>(QfSocketEvent.UpdateUser, ({ data, operation }) => handleUpdateUser(operation, data));
    OnTauriEvent<number>(QfSocketEvent.AddUnreadMessages, (data) => handleAddUnreadMessages(data));
    OnTauriEvent<number>(QfSocketEvent.SubtractUnreadMessages, (data) => handleSubtractUnreadMessages(data));
    OnTauriEvent<number>(QfSocketEvent.SetUnreadMessages, (data) => handleSetUnreadMessages(data));
    return () => {
      wfmSocket.off(Wfm.SocketEvent.OnUserStatusChange, OnUserStatusChange);
    };
  }, []);

  return <AuthContext.Provider value={{ user }}>{children}</AuthContext.Provider>;
}
