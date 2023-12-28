import { createContext, useContext, useEffect, useState } from "react";
import { AppInfo, RustError, Settings } from '$types/index';
import { OnTauriEvent, OnTauriUpdateDataEvent, SendNotificationToWindow, SendTauriEvent, SendTauriUpdateDataEvent } from "../utils";
import { useQuery } from "@tanstack/react-query";
import api from "../api";
import { useTranslateGeneral, useTranslateRustError } from "@hooks/index";
import { SplashScreen } from "../components/splashScreen";
import { notifications } from "@mantine/notifications";
import { Button, Text } from "@mantine/core";
import { open } from '@tauri-apps/api/shell';

type AppContextProps = {
  settings: Settings | undefined;
  app_info: AppInfo | undefined;
}

type AppContextProviderProps = {
  children: React.ReactNode;
}

export const AppContext = createContext<AppContextProps>({
  settings: undefined,
  app_info: undefined,
});

export const useAppContext = () => useContext(AppContext);

export const AppContextProvider = ({ children }: AppContextProviderProps) => {
  const [settings, setSettings] = useState<Settings | undefined>(undefined);
  const [appInfo, setAppInfo] = useState<AppInfo | undefined>(undefined);
  const [initializstatus, setInitializstatus] = useState<string>("Initializing..");



  // Fetch data from rust side
  const { isFetching } = useQuery({
    queryKey: ['init'],
    queryFn: () => api.auth.init(),
    onSuccess(data) {
      SendTauriUpdateDataEvent("user", { data: data.user, operation: "SET" })
      SendTauriEvent("Cache:Update:Items", data.items)
      SendTauriEvent("Cache:Update:RivenTypes", data.riven_items)
      SendTauriEvent("Cache:Update:RivenAttributes", data.riven_attributes)
      SendTauriEvent("PriceScraper:Initialize", { last_run: data.price_scraper_last_run == null ? null : new Date(data.price_scraper_last_run) })

      // Stock Context
      SendTauriUpdateDataEvent("StockItems", { data: data.stock_items, operation: "SET" })
      SendTauriUpdateDataEvent("StockRivens", { data: data.stock_rivens, operation: "SET" })
      SendTauriUpdateDataEvent("transactions", { data: data.transactions, operation: "SET" })
      if (data.valid) {
        SendTauriUpdateDataEvent("orders", { data: data.orders, operation: "SET" })
        SendTauriUpdateDataEvent("auctions", { data: data.auctions, operation: "SET" })
        SendTauriUpdateDataEvent("ChatMessages", { data: data.chats, operation: "SET" })
      }
      setSettings({ ...data.settings })
      setAppInfo(data.app_info);

      if (!data.app_info.app_version.update_available) return;
      notifications.show({
        title: useTranslateGeneral("new_release_label", { v: data.app_info.app_version.version }),
        message: <>
          <Text>{data.app_info.app_version.release_notes}</Text>
          <Button style={{ width: '100%' }} onClick={async () => {
            if (data.app_info.app_version.download_url)
              await open(data.app_info.app_version.download_url);
          }}>{useTranslateGeneral('new_release_message')}</Button>
        </>,
        autoClose: false
      });
    },
    onError(error: RustError) {
      SendNotificationToWindow(useTranslateRustError("title", { component: error.component }), useTranslateRustError("message", { loc: error.component }));
    }
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
    <AppContext.Provider value={{ settings, app_info: appInfo }}>
      <SplashScreen opened={isFetching} text={initializstatus} />
      {children}
    </AppContext.Provider>
  )
}