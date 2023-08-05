import { createContext, useContext, useState } from "react";
import { liveScraper } from "../store";
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
    if (running)
      liveScraper.start();
    else
      liveScraper.stop();
  }

  return (
    <LiveScraperContext.Provider value={{ isRunning, toggle: handleToggle }}>
      {children}
    </LiveScraperContext.Provider>
  )
}