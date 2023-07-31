import { Box } from "@mantine/core";
import { createContext, useContext, useState } from "react";
import { Wfm, Settings } from '$types/index';
import useStorage from "../hooks/useStorage.hook";
type TauriContextProps = {
  loading: boolean;
  user: Wfm.User;
  updateUser: (user: Wfm.User) => void;
  settings: Settings;
  updateSettings: (user: Settings) => void;
}
type TauriContextProviderProps = {
  children: React.ReactNode;
}

export const TauriContext = createContext<TauriContextProps>({
  loading: true,
  user: {
    banned: false,
    id: '',
    avatar: '',
    ingame_name: '',
    locale: 'en',
    platform: 'pc',
    region: 'en',
    role: 'user',
  },
  updateUser: () => { },
  settings: {
    mastery_rank: 2, // Trading is unlocked at MR2
    user_email: '',
    user_password: '',
    access_token: undefined,
    budget: 0,
    current_plat: 0,
  },
  updateSettings: () => { },
});

export const useTauriContext = () => useContext(TauriContext);

export const TauriContextProvider = ({ children }: TauriContextProviderProps) => {
  const [loading] = useState(true);
  const [user, setUser] = useStorage<Wfm.User>("user", useContext(TauriContext).user);
  const [settings, setSettings] = useStorage<Settings>("settings", useContext(TauriContext).settings);

  const handleUpdateUser = (userData: Partial<Wfm.User>) => {
    setUser({ ...user, ...userData });
  }

  const handleUpdateSettings = (settingsData: Partial<Settings>) => {
    setSettings({ ...settings, ...settingsData });
  }
  return (
    <TauriContext.Provider value={{ loading, user, updateUser: handleUpdateUser, settings, updateSettings: handleUpdateSettings }}>
      <Box>
        {children}
      </Box>
    </TauriContext.Provider>
  )
}