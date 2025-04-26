import { createContext, useContext, useEffect, useState } from "react";
import { useQuery } from "@tanstack/react-query";
import api, { OnTauriDataEvent, OnTauriEvent } from "@api/index";
import { useTranslateContexts, useTranslateNotifications, useTranslateModals } from "@hooks/useTranslate.hook";
import { notifications } from "@mantine/notifications";
import { Box, Button, Group } from "@mantine/core";
import { check } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";
import { TauriTypes, QuantframeApiTypes, ResponseError } from "$types";
import { AuthContextProvider } from "./auth.context";
import { SplashScreen } from "@components/SplashScreen";
import { TextTranslate } from "@components/TextTranslate";
import { modals } from "@mantine/modals";
import { RichTextEditor } from "@mantine/tiptap";
import { useEditor } from "@tiptap/react";
import StarterKit from "@tiptap/starter-kit";
import { Markdown } from "tiptap-markdown";
import { TermsAndConditions } from "@components/Modals/TermsAndConditions";
import { open } from "@tauri-apps/plugin-shell";
import { resolveResource } from "@tauri-apps/api/path";
import { readTextFile } from "@tauri-apps/plugin-fs";

type NotificationPayload = {
  i18n_key_title: string;
  i18n_key_message: string;
  autoClose?: boolean | number;
  values: { [key: string]: string | number };
};

export type AppContextProps = {
  settings: TauriTypes.Settings | undefined;
  alerts: QuantframeApiTypes.Alert[];
  app_info: TauriTypes.AppInfo | undefined;
  app_error?: ResponseError;
};

export type AppContextProviderProps = {
  children: React.ReactNode;
};
interface Entity {
  id: string | number;
}
export const AppContext = createContext<AppContextProps>({
  settings: undefined,
  app_info: undefined,
  app_error: undefined,
  alerts: [],
});
type SetDataFunction<T> = React.Dispatch<React.SetStateAction<T>>;
export const useAppContext = () => useContext(AppContext);

export function AppContextProvider({ children }: AppContextProviderProps) {
  const [settings, setSettings] = useState<TauriTypes.Settings | undefined>(undefined);
  const [appInfo, setAppInfo] = useState<TauriTypes.AppInfo | undefined>(undefined);
  const [alerts, setAlerts] = useState<QuantframeApiTypes.Alert[]>([]);
  const [i18Key, setI18Key] = useState<string>("cache");
  const [checkingUpdate, setCheckingUpdate] = useState(true);
  const [isControl, setIsControl] = useState<boolean>(false);
  const [appError, setAppError] = useState<ResponseError | undefined>(undefined);

  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateContexts(`app.${key}`, { ...context }, i18Key);
  const useTranslateEvents = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`loading_events.${key}`, { ...context }, i18Key);
  const useTranslateNewUpdate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`new_update.${key}`, { ...context }, i18Key);
  const useTranslateNewUpdateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateNewUpdate(`buttons.${key}`, { ...context }, i18Key);

  // Fetch data from rust side
  const { isFetching, error } = useQuery({
    queryKey: ["app_init"],
    queryFn: () => api.app.init(),
    retry: 0,
    enabled: !window.location.href.includes("controls") || !checkingUpdate,
  });

  const editor = useEditor({
    extensions: [StarterKit, Markdown],
    content: ``,
  });
  useEffect(() => {
    setIsControl(window.location.href.includes("controls"));

    OnTauriEvent(TauriTypes.Events.OnInitialize, (i18Key: string) => setI18Key(i18Key));

    const checkForUpdates = async () => {
      const update = await check();
      console.log(update);
      setCheckingUpdate(false);
      if (!update || !editor) return;

      editor.commands.setContent(update?.body || "");
      modals.open({
        title: useTranslateNewUpdate("title", { version: update?.version }),
        size: "lg",
        children: (
          <>
            <RichTextEditor editor={editor}>
              <RichTextEditor.Content />
            </RichTextEditor>
            <Group grow justify="space-between" mt={"md"}>
              <Button
                onClick={async () => {
                  let downloaded = 0;
                  let contentLength: number | undefined = 0;
                  await update.downloadAndInstall((event) => {
                    switch (event.event) {
                      case "Started":
                        contentLength = event.data.contentLength;
                        console.log(`started downloading ${event.data.contentLength} bytes`);
                        break;
                      case "Progress":
                        downloaded += event.data.chunkLength;
                        console.log(`downloaded ${downloaded} from ${contentLength}`);
                        break;
                      case "Finished":
                        console.log("download finished");
                        break;
                    }
                  });
                  await relaunch();
                }}
              >
                {useTranslateNewUpdateButtons("install")}
              </Button>
              <Button
                onClick={async () => {
                  open(`https://github.com/Kenya-DK/quantframe-react/releases/tag/v${update.version}`);
                }}
              >
                {useTranslateNewUpdateButtons("read_more")}
              </Button>
            </Group>
          </>
        ),
      });
    };
    checkForUpdates().catch((e) => {
      console.error(e);
    });
  }, []);

  useEffect(() => {
    console.log("App Info Updated", appInfo);
  }, [appInfo]);
  useEffect(() => {
    if (error == undefined) return;
    setAppError(error as ResponseError);
  }, [error]);

  useEffect(() => {
    const OpenTos = async () => {
      const resourcePath = await resolveResource("resources/tos.md");
      const context = await readTextFile(resourcePath);
      // Get Text Between <ID</ID>
      const start = context.indexOf("<ID>") + 4;
      const end = context.indexOf("</ID>");
      const id = context.substring(start, end);

      console.log("OpenTos", settings?.tos_uuid, id);
      if (id == settings?.tos_uuid) return;
      modals.open({
        title: useTranslateModals("tos.title"),
        size: "100%",
        closeOnEscape: false,
        closeOnClickOutside: false,
        withCloseButton: false,
        children: (
          <TermsAndConditions
            content={context}
            onAccept={async () => {
              modals.closeAll();
              if (!settings) return;
              await api.app.updateSettings({ ...settings, tos_uuid: id });
            }}
            onDecline={async () => {
              api.app.exit();
            }}
          />
        ),
      });
    };
    if (!settings) return;
    OpenTos();
  }, [settings]);

  // Handle update, create, delete
  const handleUpdateSettings = (operation: TauriTypes.EventOperations, data: TauriTypes.Settings) => {
    switch (operation) {
      case TauriTypes.EventOperations.CREATE_OR_UPDATE:
        setSettings((settings) => ({ ...settings, ...data }));
        break;
      case TauriTypes.EventOperations.SET:
        setSettings(data);
        break;
    }
  };
  const handleUpdate = <T extends Entity>(operation: TauriTypes.EventOperations, data: T | T[], setData: SetDataFunction<T[]>) => {
    switch (operation) {
      case TauriTypes.EventOperations.CREATE_OR_UPDATE:
        // setData(myState.map(item => item.id === id ? {...item, item.description: "new desc"} : item))
        setData((items) => {
          // Check if the item already exists in the list
          const itemExists = items.some((item) => item.id === (data as T).id);

          // If the item exists, update it; otherwise, add the new item
          if (itemExists) return items.reverse().map((item) => (item.id === (data as T).id ? (data as T) : item));
          else return [data as T, ...items.reverse()];
        });
        break;
      case TauriTypes.EventOperations.DELETE:
        setData((items) => items.filter((item) => item.id !== (data as T).id));
        break;
      case TauriTypes.EventOperations.SET:
        setData(data as T[]);
        break;
    }
  };
  const handleUpdateAppInfo = (operation: TauriTypes.EventOperations, data: TauriTypes.AppInfo) => {
    switch (operation) {
      case TauriTypes.EventOperations.CREATE_OR_UPDATE:
        setAppInfo((settings) => ({ ...settings, ...data }));
        break;
      case TauriTypes.EventOperations.SET:
        setAppInfo(data);
        break;
    }
  };

  const handleNotification = (payload: NotificationPayload, color: string, autoClose: boolean = true) => {
    notifications.show({
      title: useTranslateNotifications(payload.i18n_key_title, payload.values),
      autoClose: payload.autoClose ?? autoClose,
      message: (
        <Box>
          <TextTranslate i18nKey={useTranslateNotifications(payload.i18n_key_message, undefined, true)} values={payload.values} />
        </Box>
      ),
      color: color,
    });
  };
  // Hook on tauri events from rust side
  useEffect(() => {
    OnTauriDataEvent<TauriTypes.Settings>(TauriTypes.Events.UpdateSettings, ({ data, operation }) => handleUpdateSettings(operation, data));
    OnTauriDataEvent<TauriTypes.AppInfo>(TauriTypes.Events.UpdateAppInfo, ({ data, operation }) => handleUpdateAppInfo(operation, data));
    OnTauriDataEvent<any>(TauriTypes.Events.UpdateAlert, ({ data, operation }) => handleUpdate(operation, data, setAlerts));
    OnTauriEvent<ResponseError>(TauriTypes.Events.UpdateAppError, (data) => setAppError(data));
    OnTauriEvent<NotificationPayload>(TauriTypes.Events.OnNotificationError, (data) => handleNotification(data, "red.7", false));
    OnTauriEvent<NotificationPayload>(TauriTypes.Events.OnNotificationWarning, (data) => handleNotification(data, "yellow.7", false));
    OnTauriEvent<NotificationPayload>(TauriTypes.Events.OnNotificationSuccess, (data) => handleNotification(data, "green.7"));
    return () => {};
  }, []);

  return (
    <AppContext.Provider value={{ settings, app_info: appInfo, app_error: appError, alerts }}>
      {!isControl && <SplashScreen opened={isFetching} text={useTranslateEvents(i18Key)} />}
      <AuthContextProvider>{children}</AuthContextProvider>
    </AppContext.Provider>
  );
}
