import { createContext, useContext, useEffect, useState } from "react";
import api, { OffTauriEvent, OnTauriEvent } from "@api/index";
import { QuantframeApiTypes, ResponseError, TauriTypes } from "$types";
import { AuthContextProvider } from "./auth.context";
import { AppError } from "../model/appError";
import { useQuery } from "@tanstack/react-query";

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
  const { data: app_info } = api.app.get_app_info();

  const handleAppError = (error: ResponseError | undefined) => setError(error ? new AppError(error) : undefined);

  const handleUpdateSettings = () => refetchSettings();

  // Fetch data from rust side
  const { data: alerts, refetch: refetchAlerts } = useQuery({
    queryKey: ["alerts"],
    queryFn: () => api.alert.get_alerts(),
    retry: 0,
  });
  useEffect(() => {
    // 10 Minutes interval to keep the app alive
    setInterval(async () => {
      await refetchAlerts();
    }, 1 * 60 * 1000);
  }, []);

  // Hook on tauri events from rust side
  useEffect(() => {
    OnTauriEvent<ResponseError | undefined>(TauriTypes.Events.OnError, (data) => handleAppError(data));
    OnTauriEvent<undefined>(TauriTypes.Events.RefreshSettings, () => handleUpdateSettings());
    return () => {
      OffTauriEvent<ResponseError | undefined>(TauriTypes.Events.OnError, (data) => handleAppError(data));
      OffTauriEvent<undefined>(TauriTypes.Events.RefreshSettings, () => handleUpdateSettings());
    };
  }, []);
  return (
    <AppContext.Provider value={{ settings, alerts: alerts?.results || [], app_info: app_info, app_error: error }}>
      <AuthContextProvider>{children}</AuthContextProvider>
    </AppContext.Provider>
  );
}
