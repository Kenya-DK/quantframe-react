import { createContext, useContext, useEffect, useState } from "react";
import { Settings } from '$types/index';
import { OnTauriEvent, OnTauriUpdateDataEvent, SendTauriUpdateDataEvent } from "../utils";
import { useQuery } from "@tanstack/react-query";
import api from "../api";
import { SplashScreen } from "../components/splashScreen";

type AppContextProps = {
  settings: Settings | undefined;
}

type AppContextProviderProps = {
  children: React.ReactNode;
}

export const AppContext = createContext<AppContextProps>({
  settings: undefined,
});

export const useAppContext = () => useContext(AppContext);

export const AppContextProvider = ({ children }: AppContextProviderProps) => {
  const [settings, setSettings] = useState<Settings | undefined>(undefined);
  const [initializstatus, setInitializstatus] = useState<string>("Initializing..");

  const { isFetching } = useQuery({
    queryKey: ['init'],
    queryFn: () => api.auth.validate(),
    onSuccess(data) {
      if (!data.valid) {

      } else {
        SendTauriUpdateDataEvent("user", { data: data.user, operation: "set" })
        SendTauriUpdateDataEvent("inventorys", { data: data.inventorys, operation: "set" })
        SendTauriUpdateDataEvent("transactions", { data: data.transactions, operation: "set" })
        SendTauriUpdateDataEvent("orders", { data: data.orders, operation: "set" })
      }
      setSettings({ ...data.settings })
    },
  })

  // Handle update, create, delete transaction
  const handleUpdateSettings = (operation: string, data: Settings) => {
    switch (operation) {
      case "update":
        setSettings((settings) => ({ ...settings, ...data }));
        break;
      case "set":
        setSettings(data);
        break;
    }
  }

  // Hook on tauri events from rust side
  useEffect(() => {
    OnTauriEvent("set_initializstatus", (data: { status: string }) => setInitializstatus(data.status));
    OnTauriUpdateDataEvent<Settings>("settings", ({ data, operation }) => handleUpdateSettings(operation, data));
    return () => { }
  }, []);

  return (
    <AppContext.Provider value={{ settings }}>
      <SplashScreen opened={isFetching} text={initializstatus} />
      {children}
    </AppContext.Provider>
  )
}