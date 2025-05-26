import { createContext, useContext, useEffect, useState } from "react";
import api, { OnTauriDataEvent } from "@api/index";
import { TauriTypes, UserStatus } from "$types";
import wfmSocket from "@models/wfmSocket";
import { WFMarketTypes } from "../types";
import { useAppContext } from "./app.context";
export type AuthContextProps = {
  user: TauriTypes.User | undefined;
  patreon_link?: string;
};
export type TauriContextProviderProps = {
  children: React.ReactNode;
};

export const AuthContext = createContext<AuthContextProps>({
  user: undefined,
  patreon_link: undefined,
});

export const useAuthContext = () => useContext(AuthContext);

export function AuthContextProvider({ children }: TauriContextProviderProps) {
  // Context
  const { app_info } = useAppContext();

  // States
  const [user, setUser] = useState<TauriTypes.User | undefined>(undefined);
  const [patreon_link, setPatreonLink] = useState<string | undefined>(undefined);

  useEffect(() => {
    if (!app_info) return;
    setPatreonLink(
      `https://www.patreon.com/oauth2/authorize?response_type=code&client_id=6uDrK7uhMBAidiAvzQd7ukmHFz4NUXO1wocruae24C4_04rXrUMSvCzC9RKbQpmN&scope=identity%20identity%5Bemail%5D&redirect_uri=${
        app_info?.is_development ? "http://localhost:6969/auth/patreon/link" : "https://api.quantframe.app/auth/patreon/link"
      }&state=${user?.id}|${user?.check_code}`
    );
  }, [app_info, user]);
  const handleUpdateUser = (operation: string, data: TauriTypes.User) => {
    window.data = data;
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

  const OnUserStatusChange = async (status: UserStatus) => {
    // Update user status in backend
    await api.auth.set_status(status);
    setUser((user) => {
      if (!user) return user;
      return { ...user, status };
    });
  };

  // Hook on tauri events from rust side
  useEffect(() => {
    wfmSocket.on(WFMarketTypes.SocketEvent.OnUserStatusChange, OnUserStatusChange);
    OnTauriDataEvent<TauriTypes.User>(TauriTypes.Events.UpdateUser, ({ data, operation }) => handleUpdateUser(operation, data));
    return () => {
      wfmSocket.off(WFMarketTypes.SocketEvent.OnUserStatusChange, OnUserStatusChange);
    };
  }, []);

  return <AuthContext.Provider value={{ user, patreon_link }}>{children}</AuthContext.Provider>;
}
