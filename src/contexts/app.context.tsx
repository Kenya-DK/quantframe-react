import { createContext, useContext, useEffect, useState } from "react";
import { Settings } from '$types/index';
import { OnTauriEvent, OnTauriUpdateDataEvent, SendTauriEvent, SendTauriUpdateDataEvent } from "../utils";
import { useQuery } from "@tanstack/react-query";
import api from "../api";
import { useTranslateGeneral } from "@hooks/index";
import { SplashScreen } from "../components/splashScreen";
import { notifications } from "@mantine/notifications";
import { Button, Text } from "@mantine/core";
import { open } from '@tauri-apps/api/shell';

type AppContextProps = {
  settings: Settings | undefined;
  version: string;
}

type AppContextProviderProps = {
  children: React.ReactNode;
}

export const AppContext = createContext<AppContextProps>({
  settings: undefined,
  version: "0.0.0"
});

export const useAppContext = () => useContext(AppContext);

export const AppContextProvider = ({ children }: AppContextProviderProps) => {
  const [settings, setSettings] = useState<Settings | undefined>(undefined);
  const [initializstatus, setInitializstatus] = useState<string>("Initializing..");
  const [version, setVersion] = useState<string>("0.0.0");


  // Fetch data from rust side
  const { isFetching } = useQuery({
    queryKey: ['init'],
    queryFn: () => api.auth.init(),
    onSuccess(data) {
      SendTauriUpdateDataEvent("user", { data: data.user, operation: "SET" })
      SendTauriUpdateDataEvent("transactions", { data: data.transactions, operation: "SET" })
      // Stock Context
      SendTauriUpdateDataEvent("StockItems", { data: data.stock_items, operation: "SET" })
      SendTauriUpdateDataEvent("StockRivens", { data: data.stock_rivens, operation: "SET" })

      SendTauriEvent("Cache:Update:Items", data.items)
      SendTauriEvent("Cache:Update:RivenTypes", data.riven_items)
      SendTauriEvent("Cache:Update:RivenAttributes", data.riven_attributes)
      SendTauriEvent("PriceScraper:Initialize", { last_run: data.price_scraper_last_run == null ? null : new Date(data.price_scraper_last_run) })
      if (data.valid) {
        SendTauriUpdateDataEvent("orders", { data: data.orders, operation: "SET" })
        SendTauriUpdateDataEvent("auctions", { data: data.auctions, operation: "SET" })

      }
      setSettings({ ...data.settings })
      setVersion(data.update_state.current_version);

      if (!data.update_state.update_available) return;
      notifications.show({
        title: useTranslateGeneral("new_release_label", { v: data.update_state.version }),
        message: <>
          <Text>{data.update_state.release_notes}</Text>
          <Button style={{ width: '100%' }} onClick={async () => {
            if (data.update_state.download_url)
              await open(data.update_state.download_url);
          }}>{useTranslateGeneral('new_release_message')}</Button>
        </>,
        autoClose: false
      });
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
    <AppContext.Provider value={{ settings, version }}>
      <SplashScreen opened={isFetching} text={initializstatus} />
      {children}
    </AppContext.Provider>
  )
}