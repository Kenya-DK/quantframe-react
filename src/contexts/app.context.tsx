import { createContext, useContext, useEffect, useState } from "react";
import { Settings } from '$types/index';
import { OnTauriEvent, OnTauriUpdateDataEvent, SendTauriEvent, SendTauriUpdateDataEvent } from "../utils";
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
    queryFn: () => api.auth.init(),
    onSuccess(data) {
      console.log(data);

      if (!data.valid) {

      } else {
        SendTauriUpdateDataEvent("user", { data: data.user, operation: "SET" })
        SendTauriUpdateDataEvent("inventorys", { data: data.inventorys, operation: "SET" })
        SendTauriUpdateDataEvent("transactions", { data: data.transactions, operation: "SET" })
        SendTauriUpdateDataEvent("orders", { data: data.orders, operation: "SET" })
        SendTauriEvent("Cache:Update:Items", data.items)
        SendTauriEvent("Cache:Update:RivenTypes", data.riven_items)
        SendTauriEvent("Cache:Update:RivenAttributes", data.riven_attributes)
        SendTauriEvent("PriceScraper:Initialize", { last_run: data.price_scraper_last_run == null ? null : new Date(data.price_scraper_last_run) })

      }
      setSettings({ ...data.settings })
    },
  })

  // Handle update, create, delete transaction
  const handleUpdateSettings = (operation: string, data: Settings) => {
    switch (operation) {
      case "UPDATE":
        setSettings((settings) => ({ ...settings, ...data }));
        break;
      case "SET":
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