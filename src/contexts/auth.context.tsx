import { createContext, useContext, useEffect } from 'react';
import { Box } from '@mantine/core';
import { Wfm } from '$types/index';
import useStorage from '../hooks/useStorage.hook';

type AuthContextProps = {
  user: Wfm.User;
  updateUser: (user: Wfm.User) => void;
  logOut: () => void;
}
type AuthContextProviderProps = {
  children: React.ReactNode;
}

export const AuthContext = createContext<AuthContextProps>({
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
  logOut: () => { },
});

export const AuthContextProvider = ({ children }: AuthContextProviderProps) => {
  const [user, setUser] = useStorage<Wfm.User>("user", useContext(AuthContext).user);

  const logOut = async () => { }


  useEffect(() => {
    // setInterval(async () => {
    //   handleUpdateUser({
    //     ingame_name: Math.random().toString(36).substring(7),
    //   })
    // }, 1000)
  }, [])

  useEffect(() => {
    console.log('storeUser', user.ingame_name);
  }, [user])

  const handleUpdateUser = (userData: Partial<Wfm.User>) => {
    setUser({ ...user, ...userData });
  }

  return (
    <AuthContext.Provider value={{ user, updateUser: handleUpdateUser, logOut }}>
      <Box>
        {children}
        <pre>{
          JSON.stringify(user, null, 2)
        }</pre>
      </Box>
    </AuthContext.Provider>
  )
}