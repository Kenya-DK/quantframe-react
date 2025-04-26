import { createContext, useContext, useEffect, useState } from "react";
import { ResponseError, TauriTypes } from "$types";
import { OffTauriEvent, OnTauriEvent } from "../api";
import { useTranslateContexts } from "@hooks/useTranslate.hook";
import { notifications } from "@mantine/notifications";
export type LiveScraperContextProps = {
  is_running: boolean;
  can_run: boolean;
  message: { i18nKey: string; values: { [key: string]: number | string } } | undefined;
  error: ResponseError | null;
};
export type LiveScraperContextProviderProps = {
  children: React.ReactNode;
};

export type LiveScraperMessage = {
  i18nKey: string;
  values: { [key: string]: number | string };
};

export const LiveScraperContext = createContext<LiveScraperContextProps>({
  is_running: false,
  can_run: true,
  message: undefined,
  error: null,
});

export const useLiveScraperContext = () => useContext(LiveScraperContext);

export function LiveScraperContextProvider({ children }: LiveScraperContextProviderProps) {
  const [is_running, setIsRunning] = useState(false);
  const [error, setError] = useState<ResponseError | null>(null);
  const [message, setMessage] = useState<LiveScraperMessage | undefined>(undefined);
  const [can_run, setCanRun] = useState(true);
  // Translate general
  const useTranslateContext = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateContexts(`live_scraper.${key}`, { ...context }, i18Key);
  const useTranslateErrors = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateContext(`errors.${key}`, { ...context }, i18Key);

  const OnUpdateRunningState = async (enable: boolean) => setIsRunning(enable);
  const OnUpdateError = async (error: ResponseError) => {
    notifications.show({
      title: useTranslateErrors("run.title"),
      message: useTranslateErrors("run.message", {
        component: error.component,
        backtrace: error.backtrace,
        message: error.message,
      }),
      color: "red.7",
    });
    setError(error);
    setMessage(undefined);
  };
  const OnUpdateMessage = (messageIn: LiveScraperMessage) => setMessage(messageIn);
  const OnToggleControl = (messageIn: TauriTypes.OnToggleControlPayload) => {
    if (messageIn.id === "live_trading") setCanRun(messageIn.state);
  };

  useEffect(() => {
    OnTauriEvent<boolean>(TauriTypes.Events.UpdateLiveTradingRunningState, OnUpdateRunningState);
    OnTauriEvent<TauriTypes.OnToggleControlPayload>(TauriTypes.Events.OnToggleControl, OnToggleControl);
    OnTauriEvent<ResponseError>(TauriTypes.Events.OnLiveTradingError, OnUpdateError);
    OnTauriEvent<LiveScraperMessage>(TauriTypes.Events.OnLiveTradingMessage, OnUpdateMessage);
    return () => {
      OffTauriEvent<boolean>(TauriTypes.Events.UpdateLiveTradingRunningState, OnUpdateRunningState);
      OffTauriEvent<ResponseError>(TauriTypes.Events.OnLiveTradingError, OnUpdateError);
      OffTauriEvent<LiveScraperMessage>(TauriTypes.Events.OnLiveTradingMessage, OnUpdateMessage);
    };
  }, []);

  return <LiveScraperContext.Provider value={{ is_running, error, message, can_run }}>{children}</LiveScraperContext.Provider>;
}
