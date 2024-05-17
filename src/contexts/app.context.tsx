import { createContext, useContext, useEffect, useState } from "react";
import { useQuery } from "@tanstack/react-query";
import api, { OnTauriDataEvent, OnTauriEvent } from "@api/index";
import { useTranslateContexts } from "@hooks/index";
import { notifications } from "@mantine/notifications";
import { Button, Group, Text } from "@mantine/core";
import {
  checkUpdate, installUpdate,
  // installUpdate,
  // onUpdaterEvent,
} from '@tauri-apps/api/updater'
import { relaunch } from "@tauri-apps/api/process";
import { AppInfo, QfSocketEvent, QfSocketEventOperation, ResponseError, Settings } from "@api/types";
import { AuthContextProvider } from "./auth.context";
import { QFSocketContextProvider } from "./qfSocket.context";
import { SplashScreen } from "../components/SplashScreen";

export type AppContextProps = {
  settings: Settings | undefined;
  app_info: AppInfo | undefined;
  app_error?: ResponseError;
}

export type AppContextProviderProps = {
  children: React.ReactNode;
}

export const AppContext = createContext<AppContextProps>({
  settings: undefined,
  app_info: undefined,
  app_error: undefined,
});

export const useAppContext = () => useContext(AppContext);

export function AppContextProvider({ children }: AppContextProviderProps) {
  const [settings, setSettings] = useState<Settings | undefined>(undefined);
  const [appInfo, setAppInfo] = useState<AppInfo | undefined>(undefined);
  const [i18Key, setI18Key] = useState<string>('cache');
  const [isControl, setIsControl] = useState<boolean>(false);
  const [appError, setAppError] = useState<ResponseError | undefined>(undefined);
  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateContexts(`app.${key}`, { ...context }, i18Key)
  const useTranslateEvents = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslate(`loading_events.${key}`, { ...context }, i18Key)
  const useTranslateNewUpdate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslate(`new_update.${key}`, { ...context }, i18Key)
  const useTranslateNewUpdateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateNewUpdate(`buttons.${key}`, { ...context }, i18Key)



  // Fetch data from rust side
  const { isFetching, error } = useQuery({
    queryKey: ['app_init'],
    queryFn: () => api.app.init(),
    enabled: !window.location.href.includes('controls'),
  })


  useEffect(() => {
    setIsControl(window.location.href.includes('controls'));

    OnTauriEvent(QfSocketEvent.OnInitialize, (i18Key: string) => setI18Key(i18Key));

    checkUpdate().then(({ shouldUpdate, manifest }) => {
      if (!shouldUpdate)
        return;

      notifications.show({
        title: useTranslateNewUpdate("title", { version: manifest?.version }),
        message: <>
          <Text truncate="end">{manifest?.body}</Text>
          <Group grow justify="space-between">
            <Button onClick={async () => {
              // Install the update. This will also restart the app on Windows!
              await installUpdate();

              // On macOS and Linux you will need to restart the app manually.
              // You could use this step to display another confirmation dialog.
              await relaunch();
            }}>{useTranslateNewUpdateButtons('install')}</Button>
            <Button onClick={async () => {
              window.open("https://github.com/Kenya-DK/quantframe-react/releases", '_blank');
            }}>{useTranslateNewUpdateButtons('read_more')}</Button>
          </Group>
        </>,
        autoClose: false
      });

    }).catch((e) => {
      console.log(e);
    })
  }, [])


  useEffect(() => {
    if (error == undefined) return;
    setAppError(error as ResponseError);
  }, [error])
  // Handle update, create, delete
  const handleUpdateSettings = (operation: QfSocketEventOperation, data: Settings) => {
    switch (operation) {
      case QfSocketEventOperation.CREATE_OR_UPDATE:
        setSettings((settings) => ({ ...settings, ...data }));
        break;
      case QfSocketEventOperation.SET:
        setSettings(data);
        break;
    }
  }
  const handleUpdateAppInfo = (operation: QfSocketEventOperation, data: AppInfo) => {
    switch (operation) {
      case QfSocketEventOperation.CREATE_OR_UPDATE:
        setAppInfo((settings) => ({ ...settings, ...data }));
        break;
      case QfSocketEventOperation.SET:
        setAppInfo(data);
        break;
    }
  }

  // Hook on tauri events from rust side
  useEffect(() => {
    OnTauriDataEvent<Settings>(QfSocketEvent.UpdateSettings, ({ data, operation }) => handleUpdateSettings(operation, data));
    OnTauriDataEvent<AppInfo>(QfSocketEvent.UpdateAppInfo, ({ data, operation }) => handleUpdateAppInfo(operation, data));
    OnTauriEvent<ResponseError>(QfSocketEvent.UpdateAppError, (data) => setAppError(data));
    return () => { }
  }, []);

  return (
    <AppContext.Provider value={{ settings, app_info: appInfo, app_error: appError }}>
      {!isControl && <SplashScreen opened={isFetching} text={useTranslateEvents(i18Key)} />}
      <AuthContextProvider>
        <QFSocketContextProvider>
          {children}
        </QFSocketContextProvider>
      </AuthContextProvider>
    </AppContext.Provider>
  )
}