import React, { createContext, useEffect, useState } from "react";
import { qfSocket } from "@models/index";
export type QFSocketContextProps = {
  isConnected: boolean;
}

export type QFSocketContextProviderProps = {
  children: React.ReactNode;
}

export const QfSocketContext = createContext<QFSocketContextProps>({
  isConnected: false,
});

export const useQfSocketContext = () => React.useContext(QfSocketContext);

export function QFSocketContextProvider({ children }: QFSocketContextProviderProps) {
  const [isConnected, setIsConnected] = useState<boolean>(qfSocket.isConnected());

  useEffect(() => {
    const OnConnect = async () => setIsConnected(true);
    const onDisconnect = async () => setIsConnected(false);

    qfSocket.on('connect', OnConnect);
    qfSocket.on('disconnect', onDisconnect);

    return () => {
      qfSocket.off('connect', OnConnect);
      qfSocket.off('disconnect', onDisconnect);
    };
  }, []);


  return (
    <QfSocketContext.Provider value={{ isConnected }}>
      {children}
    </QfSocketContext.Provider>
  )
}


