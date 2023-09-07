import { createContext, useContext, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { OnTauriEvent } from "../utils";
import { useTauriContext } from ".";
import { useTranslateContext } from "../hooks";
type LiveScraperContextProps = {
  isRunning: boolean;
  toggle: () => void;
}
type LiveScraperContextProviderProps = {
  children: React.ReactNode;
}

export const LiveScraperContext = createContext<LiveScraperContextProps>({
  isRunning: false,
  toggle: () => { },
});

export const useLiveScraperContext = () => useContext(LiveScraperContext);

export const LiveScraperContextProvider = ({ children }: LiveScraperContextProviderProps) => {
  const [isRunning, setIsRunning] = useState(false)
  const useTranslateLiveScraper = (key: string, context?: { [key: string]: any }) => useTranslateContext(`live_scraper.${key}`, { ...context })
  const { sendNotification } = useTauriContext()
  const handleToggle = async () => {
    const running = !isRunning;
    setIsRunning(running);
    await invoke("toggle_live_scraper")
  }

  useEffect(() => {
    OnTauriEvent("live_scraper_error", () => {
      setIsRunning(false)
      sendNotification(useTranslateLiveScraper("error_title"), (useTranslateLiveScraper("error_message")));
    });
    return () => { }
  }, []);

  return (
    <LiveScraperContext.Provider value={{ isRunning, toggle: handleToggle }}>
      {children}
    </LiveScraperContext.Provider>
  )
}