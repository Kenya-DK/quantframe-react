import { createContext, useContext, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { useTauriContext } from ".";
import { useTranslateContext } from "../hooks";

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
    //listen to a event
    const unlisten = listen("newWhisper", (data: { payload: { name: string } }) => {
      const { name } = data.payload;
      sendNotification(useTranslateWhisper("title"), (useTranslateWhisper("message", { name })));
    });

    // invoke a Rust function to start a loop for periodically emitting event.
    // start_backend_emitting_loop();

    return () => {
      unlisten.then(f => f());
    }
  }, []);
  return (
    <WhisperScraperContext.Provider value={{ isStarting, isRunning, toggle: handleToggle }}>
      {children}
    </WhisperScraperContext.Provider>
  )
}