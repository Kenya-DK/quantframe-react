import { createContext, useContext, useEffect, useState } from "react";
import api, { OffTauriEvent, OnTauriEvent } from "@api/index";
import { QuantframeApiTypes, ResponseError, TauriTypes } from "$types";
import { AuthContextProvider } from "./auth.context";
import { AppError } from "../model/appError";
import { SplashScreen } from "@components/Layouts/Shared/SplashScreen";
import { useQuery } from "@tanstack/react-query";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { check } from "@tauri-apps/plugin-updater";
import { modals } from "@mantine/modals";
import { UpdateAvailableModal } from "@components/Modals/UpdateAvailable";
import { TermsAndConditions } from "@components/Modals/TermsAndConditions";
import { useTranslateCommon, useTranslateComponent, useTranslateContexts } from "@hooks/useTranslate.hook";
import { resolveResource } from "@tauri-apps/api/path";
import { readTextFile } from "@tauri-apps/plugin-fs";
import { LiveScraperContextProvider } from "./liveScraper.context";
import { notifications } from "@mantine/notifications";
import { TextTranslate } from "../components/Shared/TextTranslate";

export type AppContextProps = {
  app_info: TauriTypes.AppInfo | undefined;
  app_error: AppError | undefined;
  alerts: QuantframeApiTypes.AlertDto[];
  settings: TauriTypes.Settings | undefined;
};

export type AppContextProviderProps = {
  children: React.ReactNode;
};
export const AppContext = createContext<AppContextProps>({
  settings: undefined,
  app_info: undefined,
  alerts: [],
  app_error: undefined,
});

export const useAppContext = () => useContext(AppContext);
export const useIsDev = () => {
  const { app_info } = useAppContext();
  return app_info?.is_dev ?? false;
};
export const useAppError = () => {
  const { app_error } = useAppContext();
  return app_error;
};

export function AppContextProvider({ children }: AppContextProviderProps) {
  const [error, setError] = useState<AppError | undefined>(undefined);
  const [startingUp, setStartingUp] = useState<{ i18n_key: string; values: {} }>({
    i18n_key: "starting_up",
    values: {},
  });

  const { data: settings, refetch: refetchSettings } = api.app.get_settings();
  const { data: app_info, refetch: refetchAppInfo } = api.app.get_app_info();
  const [loading, setLoading] = useState(true);

  const handleAppError = (error: ResponseError | undefined) => {
    // setError(error ? new AppError(error) : undefined);
    setError((prevError) => {
      if (prevError && !prevError.isWebSocket()) return prevError; // No error to set
      return error ? new AppError(error) : undefined;
    });
  };

  const handleOnNotify = (data: { i18n_key: string; color: string; type: string; values: Record<string, any> }) => {
    notifications.show({
      title: useTranslateCommon(`notifications.${data.i18n_key}.${data.type}.title`, data.values),
      color: data.color,
      message: (
        <TextTranslate i18nKey={useTranslateCommon(`notifications.${data.i18n_key}.${data.type}.message`, undefined, true)} values={data.values} />
      ),
    });
  };

  const checkForUpdates = async (info: TauriTypes.AppInfo) => {
    const update = await check();
    if (!update) return;
    modals.open({
      title: useTranslateComponent("modals.update_available.title", { version: update.version }),
      withCloseButton: false,
      closeOnClickOutside: false,
      closeOnEscape: false,
      size: "75%",
      children: <UpdateAvailableModal updater={update} app_info={info} context={update.body || ""} />,
    });
  };

  const checkForTosUpdates = async (info: TauriTypes.AppInfo) => {
    const resourcePath = await resolveResource("resources/tos.md");
    const context = await readTextFile(resourcePath);
    // Get Text Between <ID</ID>
    const start = context.indexOf("<ID>") + 4;
    const end = context.indexOf("</ID>");
    const id = context.substring(start, end);

    if (id == info?.tos_uuid) return;
    const modalId = modals.open({
      title: useTranslateComponent("modals.tos.title", { version: id }),
      withCloseButton: false,
      closeOnClickOutside: false,
      closeOnEscape: false,
      size: "75%",
      children: (
        <TermsAndConditions
          content={context}
          onAccept={async () => {
            await api.app.accept_tos(id);
            modals.close(modalId);
          }}
          onDecline={async () => {
            api.app.exit();
          }}
        />
      ),
    });
  };

  // Fetch data from rust side
  const {
    data: alerts,
    error: alertsError,
    refetch: refetchAlerts,
  } = useQuery({
    queryKey: ["alerts"],
    queryFn: () => api.alert.get_alerts(),
    retry: 0,
    enabled: false, // Disable automatic fetching
  });

  useEffect(() => {
    // 10 Minutes interval to keep the app alive
    setInterval(async () => {
      await refetchAlerts();
    }, 10 * 60 * 1000);
  }, []);

  useEffect(() => {
    if (!app_info) return;
    checkForUpdates(app_info);
    checkForTosUpdates(app_info);
  }, [app_info]);

  const InitializeApp = async () => {
    await refetchAppInfo();
    await refetchAlerts();
    await refetchSettings();
    setLoading(false);
  };

  useEffect(() => {
    if (alertsError) handleAppError(alertsError as ResponseError);
  }, [alertsError]);

  // Hook on tauri events from rust side
  useEffect(() => {
    OnTauriEvent<ResponseError | undefined>(TauriTypes.Events.OnError, (data) => handleAppError(data));
    OnTauriEvent<undefined>(TauriTypes.Events.RefreshSettings, () => refetchSettings());
    OnTauriEvent<any>(TauriTypes.Events.OnStartingUp, (data) => setStartingUp(data));
    OnTauriEvent<any>(TauriTypes.Events.OnNotify, (data) => handleOnNotify(data));
    invoke("was_initialized")
      .then((wasInitialized) => (wasInitialized ? InitializeApp() : console.log("App was not initialized")))
      .catch((e) => console.error("Error checking initialization:", e));
    listen("app:ready", () => InitializeApp());
    return () => {
      OffTauriEvent<ResponseError | undefined>(TauriTypes.Events.OnError, (data) => handleAppError(data));
      OffTauriEvent<undefined>(TauriTypes.Events.RefreshSettings, () => refetchSettings());
      OffTauriEvent<any>(TauriTypes.Events.OnStartingUp, (data) => setStartingUp(data));
      OffTauriEvent<any>(TauriTypes.Events.OnNotify, (data) => handleOnNotify(data));
    };
  }, []);
  return (
    <AppContext.Provider value={{ settings, alerts: alerts?.results || [], app_info: app_info, app_error: error }}>
      <SplashScreen opened={loading} text={useTranslateContexts(`app.${startingUp.i18n_key}`, startingUp.values)} />
      {!loading && (
        <AuthContextProvider>
          <LiveScraperContextProvider>{children}</LiveScraperContextProvider>
        </AuthContextProvider>
      )}
    </AppContext.Provider>
  );
}
