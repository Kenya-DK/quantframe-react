import { Box } from "@mantine/core";
import { createContext, useContext, useState } from "react";
type StatsScraperContextProps = {
  isRunning: boolean;
}
type StatsScraperContextProviderProps = {
  children: React.ReactNode;
}

export const StatsScraperContext = createContext<StatsScraperContextProps>({
  isRunning: false,
});

export const useStatsScraperContext = () => useContext(StatsScraperContext);

export const StatsScraperContextProvider = ({ children }: StatsScraperContextProviderProps) => {
  const [isRunning] = useState(false);
  return (
    <StatsScraperContext.Provider value={{ isRunning }}>
      <Box>
        {children}
      </Box>
    </StatsScraperContext.Provider>
  )
}