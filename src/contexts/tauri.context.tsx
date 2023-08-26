import { createContext, useContext, useEffect, useState } from "react";
import { Wfm, Settings, TransactionEntryDto, InventoryEntryDto } from '$types/index';
import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/api/notification';
import api from "../api";
import { SplashScreen } from "../components/splashScreen";
import { useQuery } from "@tanstack/react-query";
import { notifications } from "@mantine/notifications";
import { OnTauriEvent, OnTauriUpdateDataEvent } from "../utils";
let permissionGranted = await isPermissionGranted();
if (!permissionGranted) {
  const permission = await requestPermission();
  permissionGranted = permission === 'granted';
}


type TauriContextProps = {
  user: Wfm.UserDto | undefined;
  tradable_items: Wfm.ItemDto[];
  transactions: TransactionEntryDto[];
  inventorys: InventoryEntryDto[];
  updateUser: (user: Partial<Wfm.UserDto>) => void;
  settings: Settings | undefined;
  updateSettings: (user: Partial<Settings>) => void;
  sendNotification: (title: string, body: string) => void;
}
type TauriContextProviderProps = {
  children: React.ReactNode;
}

export const TauriContext = createContext<TauriContextProps>({
  user: undefined,
  tradable_items: [],
  transactions: [],
  inventorys: [],
  updateUser: () => { },
  settings: undefined,
  updateSettings: () => { },
  sendNotification: () => { },
});

export const useTauriContext = () => useContext(TauriContext);

export const TauriContextProvider = ({ children }: TauriContextProviderProps) => {
  const [user, setUser] = useState<Wfm.UserDto | undefined>(undefined);
  const [settings, setSettings] = useState<Settings | undefined>(undefined);
  const [tradable_items, setTradableItems] = useState<Wfm.ItemDto[]>([]);
  const [transactions, setTransactions] = useState<TransactionEntryDto[]>([]);
  const [inventorys, setInventorys] = useState<InventoryEntryDto[]>([]);

  const { isFetching } = useQuery({
    queryKey: ['validate'],
    queryFn: () => api.auth.validate(),
    onSuccess(data) {
      console.log(data);

      if (!data.valid) {
        notifications.show({
          title: 'Session Expired',
          message: 'Please login again',
          color: 'red',
          autoClose: 5000,
        });
      } else
        setUser({ ...data.user })
      setSettings({ ...data.settings })
      setInventorys([...data.inventorys])
      setTransactions([...data.transactions])
    },
  })

  const handleUpdateUser = (userData: Partial<Wfm.UserDto>) => {
    if (!user) return;
    setUser({ ...user, ...userData });
  }

  const handleUpdateSettings = async (settingsData: Partial<Settings>) => {
    if (!settings) return;
    const data = { ...settings, ...settingsData } as Settings;
    setSettings(data);
    await api.base.updatesettings(data as any); // add 'as any' to avoid type checking
  }
  const handleSendNotification = async (title: string, body: string) => {
    if (permissionGranted) {
      sendNotification({ title: title, body: body });
    }
  }

  const handleUpdateInventory = (operation: string, data: InventoryEntryDto) => {
    switch (operation) {
      case "create":
        setInventorys((inventorys) => [...inventorys, data]);
        break;
      case "update":
        setInventorys((inventorys) => [...inventorys.filter((item) => item.id !== data.id), data]);
        break;
      case "delete":
        console.log("Delete", data);
        setInventorys((inventorys) => [...inventorys.filter((item) => item.id !== data.id)]);
        break;
    }
  }

  useEffect(() => {
    OnTauriEvent("update_tradable_items", (data: Wfm.ItemDto[]) => {
      setTradableItems(data);
    });
    OnTauriUpdateDataEvent<InventoryEntryDto>("inventorys", ({ data, operation }) => handleUpdateInventory(operation, data));
    return () => { }
  }, []);

  return (
    <TauriContext.Provider value={{ user, transactions, inventorys, tradable_items, updateUser: handleUpdateUser, settings, updateSettings: handleUpdateSettings, sendNotification: handleSendNotification }}>
      <SplashScreen opened={isFetching} />
      {children}
    </TauriContext.Provider>
  )
}