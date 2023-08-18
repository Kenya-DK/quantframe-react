import { createContext, useContext, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useTauriContext } from ".";
import { useTranslateContext } from "@hooks/index";
import { OnTauriEvent } from "@utils/index";

type WhisperScraperContextProps = {
  isRunning: boolean;
  isStarting?: boolean;
  toggle: () => void;
}
type WhisperScraperContextProviderProps = {
  children: React.ReactNode;
}

export const WhisperScraperContext = createContext<WhisperScraperContextProps>({
  isRunning: false,
  isStarting: false,
  toggle: () => { },
});

export const useWhisperScraperContext = () => useContext(WhisperScraperContext);

export const WhisperScraperContextProvider = ({ children }: WhisperScraperContextProviderProps) => {
  const useTranslateWhisper = (key: string, context?: { [key: string]: any }) => useTranslateContext(`wisper.${key}`, { ...context })
  const [isRunning, setIsRunning] = useState(false)
  const [isStarting, setIsStarting] = useState(false)
  const { sendNotification } = useTauriContext()
  const handleToggle = async () => {
    const running = !isRunning;
    setIsStarting(true)
    setIsRunning(running);
    await invoke("toggle_whisper_scraper")
    setIsStarting(false)
    // await invoke("toggle_live_scraper")
  }
  useEffect(() => {
    OnTauriEvent("whisper_scraper_mesage_from_player", (data: { name: string }) => {
      const { name } = data;
      sendNotification(useTranslateWhisper("title"), (useTranslateWhisper("message", { name })));

    });
    OnTauriEvent("whisper_scraper_error", (data: any) => {
      console.log(data);
      setIsStarting(false)
    });
    return () => { }
  }, []);
  return (
    <WhisperScraperContext.Provider value={{ isStarting, isRunning, toggle: handleToggle }}>
      {children}
    </WhisperScraperContext.Provider>
  )
}