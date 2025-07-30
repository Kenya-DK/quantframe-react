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
import { UpdateAvailableModal } from "../components/Modals/UpdateAvailable";
import { useTranslateComponent } from "../hooks/useTranslate.hook";

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
    invoke("was_initialized")
      .then((wasInitialized) => (wasInitialized ? InitializeApp() : console.log("App was not initialized")))
      .catch((e) => console.error("Error checking initialization:", e));
    listen("app:ready", () => InitializeApp());
    return () => {
      OffTauriEvent<ResponseError | undefined>(TauriTypes.Events.OnError, (data) => handleAppError(data));
      OffTauriEvent<undefined>(TauriTypes.Events.RefreshSettings, () => refetchSettings());
    };
  }, []);
  return (
    <AppContext.Provider value={{ settings, alerts: alerts?.results || [], app_info: app_info, app_error: error }}>
      <SplashScreen opened={loading} text={"Kenya"} />
      {!loading && <AuthContextProvider>{children}</AuthContextProvider>}
    </AppContext.Provider>
  );
}
