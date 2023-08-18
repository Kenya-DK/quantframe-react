import { createContext, useContext, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { settings, user } from "@store/index";
import { OnTauriEvent } from "../utils";
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
    const data = await settings.get();
    const { ingame_name } = await user.get();
    const running = !isRunning;
    setIsRunning(running);
    await invoke("toggle_live_scraper", {
      token: data.access_token,
      settings: {
        ...data,
        in_game_name: ingame_name
      }
    })
  }

  useEffect(() => {
    OnTauriEvent("live_scraper_error", (data: any) => {
      console.log(data);
      setIsRunning(false)
    });
    return () => { }
  }, []);

  return (
    <LiveScraperContext.Provider value={{ isRunning, toggle: handleToggle }}>
      {children}
    </LiveScraperContext.Provider>
  )
}