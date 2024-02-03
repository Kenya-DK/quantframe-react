import { createContext, useContext, useEffect, useState } from "react";
import { OnTauriEvent, SendNotificationToWindow } from "../utils";
import { RustError, ScraperState, ScraperMessage } from "../types";
import { useTranslateContext } from "../hooks";
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
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateContext(`live_scraper.${key}`, { ...context }, i18Key);
  const [is_running, setIsRunning] = useState(false);
  const [error, setError] = useState<RustError | null>(null);
  const [message, setMessage] = useState<ScraperMessage | undefined>(undefined);

  useEffect(() => {
    OnTauriEvent("LiveScraper:UpdateMessage", (e: ScraperMessage) => {
      if (e.i18n_key == "")
        setMessage(undefined);
      else
        setMessage({ ...e, i18n_key: `live_scraper.${e.i18n_key}` })
    });
    OnTauriEvent("LiveScraper:Toggle", () => {
      setIsRunning((is_running) => !is_running)
    });
    OnTauriEvent("LiveScraper:Error", (error: RustError) => {
      setIsRunning(false)
      setError(error)
      SendNotificationToWindow(useTranslate("error_title"), useTranslate("error_message"));
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