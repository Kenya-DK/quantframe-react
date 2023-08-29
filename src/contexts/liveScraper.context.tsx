import { createContext, useContext, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { OnTauriEvent } from "../utils";
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
  const handleToggle = async () => {
    const running = !isRunning;
    setIsRunning(running);
    await invoke("toggle_live_scraper")
  }

  useEffect(() => {
    OnTauriEvent("live_scraper_error", () => {
      setIsRunning(false)
    });
    return () => { }
  }, []);

  return (
    <LiveScraperContext.Provider value={{ isRunning, toggle: handleToggle }}>
      {children}
    </LiveScraperContext.Provider>
  )
}