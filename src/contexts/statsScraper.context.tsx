import { useQuery } from "@tanstack/react-query";
import { createContext, useContext } from "react";
import api from '@api/index';
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
  const { isFetching, refetch } = useQuery({
    queryKey: ['statsScraper'],
    queryFn: () => api.itemprices.updatePriceHistory(7),
    enabled: false,
  })

  const handleRun = async () => {
    await refetch();
  }

  return (
    <StatsScraperContext.Provider value={{ isRunning: isFetching, run: handleRun }}>
      {children}
    </StatsScraperContext.Provider>
  )
}