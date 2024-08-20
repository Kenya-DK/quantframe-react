import { createContext, useContext, useEffect, useState } from "react";
import { useQuery } from "@tanstack/react-query";
import api, { OnTauriDataEvent, OnTauriEvent } from "@api/index";
import { useTranslateContexts, useTranslateNotifications } from "@hooks/useTranslate.hook";
import { notifications } from "@mantine/notifications";
import { Box, Button, Group } from "@mantine/core";
import { checkUpdate, installUpdate } from '@tauri-apps/api/updater'
import { relaunch } from "@tauri-apps/api/process";
import { AppInfo, QfSocketEvent, QfSocketEventOperation, ResponseError, Settings } from "@api/types";
import { AuthContextProvider } from "./auth.context";
import { SplashScreen } from "@components/SplashScreen";
import { TextTranslate } from "@components/TextTranslate";
import { modals } from "@mantine/modals";
import { RichTextEditor } from '@mantine/tiptap';
import { useEditor } from '@tiptap/react';
import StarterKit from '@tiptap/starter-kit';
import { Markdown } from 'tiptap-markdown';

type NotificationPayload = {
  i18n_key_title: string;
  i18n_key_message: string;
  values: { [key: string]: string | number };
}

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
  const useTranslateNewUpdateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateNewUpdate(`buttons.${key}`, { ...context }, i18Key);

  // Fetch data from rust side
  const { isFetching, error } = useQuery({
    queryKey: ['app_init'],
    queryFn: () => api.app.init(),
    retry: 0,
    enabled: !window.location.href.includes('controls'),
  })

  const editor = useEditor({
    extensions: [
      StarterKit,
      Markdown,
    ],
    content: ``,
  })
  useEffect(() => {
    setIsControl(window.location.href.includes('controls'));

    OnTauriEvent(QfSocketEvent.OnInitialize, (i18Key: string) => setI18Key(i18Key));

    checkUpdate().then(({ shouldUpdate, manifest }) => {
      if (!shouldUpdate || !manifest || !editor)
        return;
      editor.commands.setContent(manifest?.body);
      console.log(manifest);
      modals.open({
        title: useTranslateNewUpdate("title", { version: manifest?.version }),
        size: 'lg',
        children: (<>
          <RichTextEditor editor={editor}>
            <RichTextEditor.Content />
          </RichTextEditor>
          <Group grow justify="space-between" mt={"md"}>
            <Button onClick={async () => {
              // Install the update. This will also restart the app on Windows!
              await installUpdate();

              // On macOS and Linux you will need to restart the app manually.
              // You could use this step to display another confirmation dialog.
              await relaunch();
            }}>{useTranslateNewUpdateButtons('install')}</Button>
            <Button onClick={async () => {
              window.open(`https://github.com/Kenya-DK/quantframe-react/releases/tag/v${manifest.version}`, '_blank');
            }}>{useTranslateNewUpdateButtons('read_more')}</Button>
          </Group>
        </>),
      });

    }).catch((e) => {
      console.error(e);
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
  const handleNotification = (payload: NotificationPayload, color: string) => {
    notifications.show({
      title: useTranslateNotifications(payload.i18n_key_title, payload.values),
      message: <Box>
        <TextTranslate i18nKey={useTranslateNotifications(payload.i18n_key_message, undefined, true)} values={payload.values} />
      </Box>,
      color: color,
    });
  }
  // Hook on tauri events from rust side
  useEffect(() => {
    OnTauriDataEvent<Settings>(QfSocketEvent.UpdateSettings, ({ data, operation }) => handleUpdateSettings(operation, data));
    OnTauriDataEvent<AppInfo>(QfSocketEvent.UpdateAppInfo, ({ data, operation }) => handleUpdateAppInfo(operation, data));
    OnTauriEvent<ResponseError>(QfSocketEvent.UpdateAppError, (data) => setAppError(data));
    OnTauriEvent<NotificationPayload>(QfSocketEvent.OnNotificationError, (data) => handleNotification(data, 'red.7'));
    OnTauriEvent<NotificationPayload>(QfSocketEvent.OnNotificationWarning, (data) => handleNotification(data, 'yellow.7'));
    OnTauriEvent<NotificationPayload>(QfSocketEvent.OnNotificationSuccess, (data) => handleNotification(data, 'green.7'));
    return () => { }
  }, []);

  return (
    <AppContext.Provider value={{ settings, app_info: appInfo, app_error: appError }}>
      {!isControl && <SplashScreen opened={isFetching} text={useTranslateEvents(i18Key)} />}
      <AuthContextProvider>
        {children}
      </AuthContextProvider>
    </AppContext.Provider>
  )
}