import { createContext, useContext, useEffect, useState } from "react";
import { TauriTypes } from "$types";
import api, { OffTauriEvent, OnTauriEvent } from "../api";
import { invoke } from "@tauri-apps/api/core";
export type LiveScraperContextProps = {
  is_running: boolean;
  message?: { i18nKey: string; values: {} };
};
export type LiveScraperContextProviderProps = {
  children: React.ReactNode;
};

export const LiveScraperContext = createContext<LiveScraperContextProps>({
  is_running: false,
  message: undefined,
});

export const useLiveScraperContext = () => useContext(LiveScraperContext);

export function LiveScraperContextProvider({ children }: LiveScraperContextProviderProps) {
  const [is_running, setIsRunning] = useState(false);
  const [message, setMessage] = useState<{ i18nKey: string; values: {} } | undefined>(undefined);

  const InitializeApp = async () => {
    const data = await api.live_scraper.get_state();
    setIsRunning(data.is_running);
  };

  // Hook on tauri events from rust side
  useEffect(() => {
    invoke("was_initialized")
      .then((wasInitialized) => wasInitialized && InitializeApp())
      .catch((e) => console.error("Error checking initialization:", e));
    OnTauriEvent<boolean>(TauriTypes.Events.UpdateLiveScraperRunningState, (data) => setIsRunning(data));
    OnTauriEvent<{ i18nKey: string; values: {} } | undefined>(TauriTypes.Events.OnLiveScraperMessage, (data) => setMessage(data));
    return () => {
      OffTauriEvent<boolean>(TauriTypes.Events.UpdateLiveScraperRunningState, (data) => setIsRunning(data));
      OffTauriEvent<{ i18nKey: string; values: {} } | undefined>(TauriTypes.Events.OnLiveScraperMessage, (data) => setMessage(data));
    };
  }, []);

  return <LiveScraperContext.Provider value={{ is_running, message }}>{children}</LiveScraperContext.Provider>;
}
