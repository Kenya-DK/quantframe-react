import { createContext, useContext } from "react";
import { invoke } from "@tauri-apps/api";
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
  return (
    <StatsScraperContext.Provider value={{ isRunning: false, run: handleRun }}>
      {children}
    </StatsScraperContext.Provider>
  )
}