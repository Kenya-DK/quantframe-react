import React, { createContext, useEffect, useState } from "react";
import { wfmSocket } from "@models/index";
import { useAuthContext } from "./auth.context";
import { Wfm } from "../types";
import api from "../api/index";
export type WFMSocketContextProps = {
  // Socket State's
  isConnected: boolean;
  total_users: number;
  registered_users: number;
  inErrorState: boolean;
}

export type WFMSocketContextProviderProps = {
  children: React.ReactNode;
}

export const WFMSocketContext = createContext<WFMSocketContextProps>({
  isConnected: false,
  total_users: 0,
  registered_users: 0,
  inErrorState: false
});

export const useWFMSocketContext = () => React.useContext(WFMSocketContext);

export function WFMSocketContextProvider({ children }: WFMSocketContextProviderProps) {
  const [isConnected, setIsConnected] = useState<boolean>(wfmSocket.isConnected());
  const [total_users, setTotalUsers] = useState<number>(0);
  const [registered_users, setRegisteredUsers] = useState<number>(0);
  const [inErrorState, setInErrorState] = useState<boolean>(false);

  const { user } = useAuthContext();

  useEffect(() => {
    if (user?.wfm_access_token)
      wfmSocket.updateToken(user.wfm_access_token);
  }, [user?.wfm_access_token]);

  // Socket event listeners
  useEffect(() => {
    const OnConnect = () => {
      setIsConnected(true);
      setInErrorState(false);
      if (user)
        api.auth.update_status(user?.status);
    }

    const OnDisconnect = () => {
      setIsConnected(false);
      setTotalUsers(0);
      setRegisteredUsers(0);
      setInErrorState(true);
    }

    const OnOnlineCount = (payload: { total_users: number, registered_users: number }) => {
      setTotalUsers(payload.total_users);
      setRegisteredUsers(payload.registered_users);
    }

    const OnError = () => {
      setInErrorState(true);
      setIsConnected(false);
    }

    wfmSocket.on('connect', OnConnect);
    wfmSocket.on('disconnect', OnDisconnect);
    wfmSocket.on(Wfm.SocketEvent.OnError, OnError);
    wfmSocket.on(Wfm.SocketEvent.OnUserCountChange, OnOnlineCount);

    return () => {
      wfmSocket.off('connect', OnConnect);
      wfmSocket.off('disconnect', OnDisconnect);
      wfmSocket.off(Wfm.SocketEvent.OnError, OnError);
      wfmSocket.off(Wfm.SocketEvent.OnUserCountChange, OnOnlineCount);
    };
  }, []);


  return (
    <WFMSocketContext.Provider value={{ isConnected, total_users, registered_users, inErrorState }}>
      {children}
    </WFMSocketContext.Provider>
  )
}


