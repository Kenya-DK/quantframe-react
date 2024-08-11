import { createContext, useContext, useEffect, useState } from "react";
import { OnToggleControlPayload, QfSocketEvent, ResponseError } from "@api/types";
import { OffTauriEvent, OnTauriEvent } from "../api";
import { useTranslateContexts } from "@hooks/useTranslate.hook";
import { notifications } from "@mantine/notifications";
export type LiveScraperContextProps = {
  is_running: boolean;
  can_run: boolean;
  message: { i18nKey: string, values: { [key: string]: number | string } } | undefined;
  error: ResponseError | null;
}
export type LiveScraperContextProviderProps = {
  children: React.ReactNode;
}

export type LiveScraperMessage = {
  i18nKey: string;
  values: { [key: string]: number | string }
}

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
  const [can_run, setCanRun] = useState(false);
  // Translate general
  const useTranslateContext = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateContexts(`live_scraper.${key}`, { ...context }, i18Key)
  const useTranslateErrors = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateContext(`errors.${key}`, { ...context }, i18Key)

  const OnUpdateRunningState = async (enable: boolean) => setIsRunning(enable);
  const OnUpdateError = async (error: ResponseError) => {
    notifications.show({
      title: useTranslateErrors("run.title"),
      message: useTranslateErrors("run.message", {
        component: error.component,
        backtrace: error.backtrace,
        message: error.message
      }), color: "red.7"
    });
    setError(error);
    setMessage(undefined);
  }
  const OnUpdateMessage = (messageIn: LiveScraperMessage) => setMessage(messageIn);
  const OnToggleControl = (messageIn: OnToggleControlPayload) => {
    if (messageIn.id === "live_trading")
      setCanRun(messageIn.state);
  }

  useEffect(() => {
    OnTauriEvent<boolean>(QfSocketEvent.UpdateLiveTradingRunningState, OnUpdateRunningState)
    OnTauriEvent<OnToggleControlPayload>(QfSocketEvent.OnToggleControl, OnToggleControl)
    OnTauriEvent<ResponseError>(QfSocketEvent.OnLiveTradingError, OnUpdateError)
    OnTauriEvent<LiveScraperMessage>(QfSocketEvent.OnLiveTradingMessage, OnUpdateMessage)
    return () => {
      OffTauriEvent<boolean>(QfSocketEvent.UpdateLiveTradingRunningState, OnUpdateRunningState)
      OffTauriEvent<ResponseError>(QfSocketEvent.OnLiveTradingError, OnUpdateError)
      OffTauriEvent<LiveScraperMessage>(QfSocketEvent.OnLiveTradingMessage, OnUpdateMessage)
    }
  }, []);

  return (
    <LiveScraperContext.Provider value={{ is_running, error, message, can_run }}>
      {children}
    </LiveScraperContext.Provider>
  )
}