import { createContext, useContext, useEffect, useState } from "react";
import { OnTauriEvent, SendNotificationToWindow } from "@utils/index";
import { RustError, ScraperState } from "../types";
import { useTranslateContext } from "../hooks";

type WhisperScraperContextProps = ScraperState & {

}
type WhisperScraperContextProviderProps = {
  children: React.ReactNode;
}

export const WhisperScraperContext = createContext<WhisperScraperContextProps>({
  is_running: false,
  last_run: null,
  error: null,
});

export const useWhisperScraperContext = () => useContext(WhisperScraperContext);

export const WhisperScraperContextProvider = ({ children }: WhisperScraperContextProviderProps) => {
  const [is_running, setIsRunning] = useState(false);
  const [error, setError] = useState<RustError | null>(null);
  const useTranslateWhisper = (key: string, context?: { [key: string]: any }) => useTranslateContext(`wisper.${key}`, { ...context })

  useEffect(() => {
    OnTauriEvent("WhisperScraper:Toggle", () => {
      setIsRunning((is_running) => !is_running)
    });
    OnTauriEvent("WhisperScraper:Error", (error: RustError) => {
      setIsRunning(false)
      setError(error)
    });
    OnTauriEvent("WhisperScraper:ReceivedMessage", (data: { name: string }) => {
      const { name } = data;
      SendNotificationToWindow(useTranslateWhisper("title"), (useTranslateWhisper("message", { name })));
    });
    return () => { }
  }, []);
  return (
    <WhisperScraperContext.Provider value={{ is_running, last_run: null, error }}>
      {children}
    </WhisperScraperContext.Provider>
  )
}