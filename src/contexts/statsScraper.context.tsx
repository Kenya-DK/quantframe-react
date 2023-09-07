import { createContext, useContext, useEffect } from "react";
import { invoke } from "@tauri-apps/api";
import { useTauriContext } from ".";
import { OnTauriEvent } from "../utils";
import { useTranslateContext } from "../hooks";
type StatsScraperContextProps = {
  isRunning: boolean;
  run: (days: number) => void;
}
type StatsScraperContextProviderProps = {
  children: React.ReactNode;
}

export const StatsScraperContext = createContext<StatsScraperContextProps>({
  isRunning: false,
  run: () => { },
});

export const useStatsScraperContext = () => useContext(StatsScraperContext);

export const StatsScraperContextProvider = ({ children }: StatsScraperContextProviderProps) => {
  const handleRun = async (days: number) => {
    // await refetch();
    await invoke("generate_price_history", {
      platform: "pc",
      days
    })
  }
  const useTranslatePriceScraper = (key: string, context?: { [key: string]: any }) => useTranslateContext(`price_scraper.${key}`, { ...context })
  const { sendNotification } = useTauriContext()

  useEffect(() => {
    OnTauriEvent("price_scraper_error", () => {
      sendNotification(useTranslatePriceScraper("error_title"), (useTranslatePriceScraper("error_message")));
    });
    return () => { }
  }, []);
  return (
    <StatsScraperContext.Provider value={{ isRunning: false, run: handleRun }}>
      {children}
    </StatsScraperContext.Provider>
  )
}