import { createContext, useContext } from "react";
import { user } from "@store/index";
import { invoke } from "@tauri-apps/api";
type StatsScraperContextProps = {
  isRunning: boolean;
  run: () => void;
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
  // const { isFetching, refetch } = useQuery({
  //   queryKey: ['statsScraper'],
  //   queryFn: () => api.itemprices.updatePriceHistory(7),
  //   enabled: false,
  // })

  const handleRun = async () => {
    // await refetch();
    const { platform } = await user.get();;
    await invoke("generate_price_history", {
      platform
    })
  }

  return (
    <StatsScraperContext.Provider value={{ isRunning: false, run: handleRun }}>
      {children}
    </StatsScraperContext.Provider>
  )
}