import { createContext, useContext, useEffect, useState } from "react";
import { TauriTypes } from "$types";
import api from "../api";
import { invoke } from "@tauri-apps/api/core";
import { useTauriEvent } from "../hooks/useTauriEvent.hook";
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

  const handleMessage = (message: { i18nKey: string; values: Record<string, any> } | undefined) => {
    if (message && message.i18nKey.endsWith("rate_limited")) {
      let seconds = parseInt(message.values.seconds, 0);
      let intervalId = setInterval(() => {
        setMessage({ ...message, values: { ...message.values, seconds: seconds-- } });
        if (seconds <= 1) clearInterval(intervalId);
      }, 1000);
    }
    setMessage(message);
  };

  const handleRunningState = (isRunning: boolean) => {
    setIsRunning(isRunning);
  };

  // Hook on tauri events from rust side
  useTauriEvent(TauriTypes.Events.OnLiveScraperMessage, handleMessage, []);
  useTauriEvent(TauriTypes.Events.UpdateLiveScraperRunningState, handleRunningState, []);

  // Get initial state
  useEffect(() => {
    invoke("was_initialized")
      .then((wasInitialized) => wasInitialized && InitializeApp())
      .catch((e) => console.error("Error checking initialization:", e));
  }, []);

  return <LiveScraperContext.Provider value={{ is_running, message }}>{children}</LiveScraperContext.Provider>;
}
