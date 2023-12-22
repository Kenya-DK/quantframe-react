import { createContext, useContext, useEffect, useState } from "react";
import { OnTauriEvent } from "../utils";
import { RustError, ScraperState, ScraperMessage } from "../types";
type LiveScraperContextProps = ScraperState & {

}
type LiveScraperContextProviderProps = {
  children: React.ReactNode;
}

export const LiveScraperContext = createContext<LiveScraperContextProps>({
  is_running: false,
  last_run: null,
  message: undefined,
  error: null,
});

export const useLiveScraperContext = () => useContext(LiveScraperContext);

export const LiveScraperContextProvider = ({ children }: LiveScraperContextProviderProps) => {
  const [is_running, setIsRunning] = useState(false);
  const [error, setError] = useState<RustError | null>(null);
  const [message, setMessage] = useState<ScraperMessage | undefined>(undefined);

  useEffect(() => {
    OnTauriEvent("LiveScraper:UpdateMessage", (e: ScraperMessage) => {
      setMessage(e)
    });
    OnTauriEvent("LiveScraper:Toggle", () => {
      setIsRunning((is_running) => !is_running)
    });
    OnTauriEvent("LiveScraper:Error", (error: RustError) => {
      setIsRunning(false)
      setError(error)
    });
    return () => {

    }
  }, []);

  return (
    <LiveScraperContext.Provider value={{ is_running, last_run: null, error, message }}>
      {children}
    </LiveScraperContext.Provider>
  )
}