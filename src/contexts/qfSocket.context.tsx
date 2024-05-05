import React, { createContext, useEffect, useState } from "react";
import { qfSocket } from "@models/index";
import api from "@api/index";
import { CreateStockRiven } from "@api/types";
import { useTranslateSockets } from "@hooks/index";
import { notifications } from "@mantine/notifications";

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

  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateSockets(`qf_socket.${key}`, { ...context }, i18Key);
  const useTranslateEvents = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslate(`events.${key}`, { ...context }, i18Key);

  useEffect(() => {
    const OnConnect = async () => setIsConnected(true);
    const onDisconnect = async () => setIsConnected(false);
    const AddRivenFromAlecaFrame = async (stockRiven: CreateStockRiven) => {
      const cacheRivens = await api.cache.getRivenWeapons();
      const weapon = cacheRivens.find((x) => x.i18n["en"].name === stockRiven.wfm_url);
      if (!weapon) {
        notifications.show({
          title: useTranslateEvents("OnAddRivenAlecaFrame.errors.title"),
          message: useTranslateEvents("OnAddRivenAlecaFrame.errors.weapon_not_found", { name: stockRiven.wfm_url }),
          color: "red",
        });
        return;
      }
      stockRiven.wfm_url = weapon.wfm_url_name;
      const cacheRivenAttributes = await api.cache.getWeaponUpgrades(weapon.uniqueName);

      for (const attribute of stockRiven.attributes) {
        const attr = Object.values(cacheRivenAttributes).find((x) => x.shortString.replace(/<[^>]*>/g, '') === attribute.url_name);
        if (!attr) {
          notifications.show({
            title: useTranslateEvents("OnAddRivenAlecaFrame.errors.title"),
            message: useTranslateEvents("OnAddRivenAlecaFrame.errors.attribute_not_found", { name: attribute.url_name }),
            color: "red",
          });
          return;
        }
        attribute.url_name = attr.wfm_id;
      }
      await api.stock.riven.create(stockRiven);
      notifications.show({
        title: useTranslateEvents("OnAddRivenAlecaFrame.success.title"),
        message: useTranslateEvents("OnAddRivenAlecaFrame.success.message", { name: `${weapon.i18n["en"].name} ${stockRiven.mod_name}` }),
        color: "green",
      });
    };
    qfSocket.on('connect', OnConnect);
    qfSocket.on('disconnect', onDisconnect);
    qfSocket.on('OnAddRivenAlecaFrame', AddRivenFromAlecaFrame);
    return () => {
      qfSocket.off('connect', OnConnect);
      qfSocket.off('disconnect', onDisconnect);
      qfSocket.off('OnAddRivenAlecaFrame', AddRivenFromAlecaFrame);
    };
  }, []);


  return (
    <QfSocketContext.Provider value={{ isConnected }}>
      {children}
    </QfSocketContext.Provider>
  )
}


