import { Box } from "@mantine/core";
import { createContext, useContext, useEffect, useState } from "react";
import { Wfm, Settings } from '$types/index';
import { useStorage, settings as sStore, user as uStore } from "../hooks/useStorage.hook";
type TauriContextProps = {
  loading: boolean;
  user: Wfm.UserDto;
  updateUser: (user: Wfm.UserDto) => void;
  settings: Settings;
  updateSettings: (user: Settings) => void;
}
type TauriContextProviderProps = {
  children: React.ReactNode;
}

export const TauriContext = createContext<TauriContextProps>({
  loading: true,
  user: uStore.defaults,
  updateUser: () => { },
  settings: sStore.defaults,
  updateSettings: () => { },
});

export const useTauriContext = () => useContext(TauriContext);

export const TauriContextProvider = ({ children }: TauriContextProviderProps) => {
  const [loading] = useState(true);
  const [user, setUser] = useStorage<Wfm.UserDto>(uStore.name, useContext(TauriContext).user);
  const [settings, setSettings] = useStorage<Settings>(sStore.name, useContext(TauriContext).settings);

  const handleUpdateUser = (userData: Partial<Wfm.UserDto>) => {
    setUser({ ...user, ...userData });
  }

  const handleUpdateSettings = (settingsData: Partial<Settings>) => {
    setSettings({ ...settings, ...settingsData });
  }

  useEffect(() => {
    setInterval(async () => {
      handleUpdateUser({
        ingame_name: Math.random().toString(36).substring(7),
      })
    }, 1000)
  }, [])

  return (
    <TauriContext.Provider value={{ loading, user, updateUser: handleUpdateUser, settings, updateSettings: handleUpdateSettings }}>
      <Box>
        {children}
        <pre>{
          JSON.stringify(user, null, 2)
        }</pre>
      </Box>
    </TauriContext.Provider>
  )
}