import { createContext, useContext, useEffect, useState } from "react";
import { QfSocketEvent, ResponseError } from "@api/types";
import { OnTauriEvent } from "../api";
export type LiveScraperContextProps = {
  is_running: boolean;
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
  message: undefined,
  error: null,
});

export const useLiveScraperContext = () => useContext(LiveScraperContext);

export function LiveScraperContextProvider({ children }: LiveScraperContextProviderProps) {
  const [is_running, setIsRunning] = useState(false);
  const [error, setError] = useState<ResponseError | null>(null);
  const [message, setMessage] = useState<LiveScraperMessage | undefined>(undefined);

  const OnUpdateRunningState = async (enable: boolean) => setIsRunning(enable);
  const OnUpdateError = async (error: ResponseError) => setError(error);
  const OnUpdateMessage = (messageIn: LiveScraperMessage) => setMessage(messageIn);


  useEffect(() => {
    OnTauriEvent<boolean>(QfSocketEvent.UpdateLiveTradingRunningState, OnUpdateRunningState)
    OnTauriEvent<ResponseError>(QfSocketEvent.OnLiveTradingError, OnUpdateError)
    OnTauriEvent<LiveScraperMessage>(QfSocketEvent.OnLiveTradingMessage, OnUpdateMessage)
    return () => {
      OnTauriEvent<boolean>(QfSocketEvent.UpdateLiveTradingRunningState, OnUpdateRunningState)
      OnTauriEvent<ResponseError>(QfSocketEvent.OnLiveTradingError, OnUpdateError)
      OnTauriEvent<LiveScraperMessage>(QfSocketEvent.OnLiveTradingMessage, OnUpdateMessage)
    }
  }, []);

  return (
    <LiveScraperContext.Provider value={{ is_running, error, message }}>
      {children}
    </LiveScraperContext.Provider>
  )
}