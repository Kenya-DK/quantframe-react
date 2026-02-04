import { createContext, useContext, useMemo } from "react";
import { useQuery } from "@tanstack/react-query";
import api from "@api/index";
import { TauriTypes } from "$types";

export type CacheContextProps = {
    tradableItems: TauriTypes.CacheTradableItem[];
    weapons: TauriTypes.CacheRivenWeapon[];
    isLoading: boolean;
};

export type CacheContextProviderProps = {
    children: React.ReactNode;
};

export const CacheContext = createContext<CacheContextProps>({
    tradableItems: [],
    weapons: [],
    isLoading: true,
});

export const useCacheContext = () => useContext(CacheContext);

export function CacheContextProvider({ children }: CacheContextProviderProps) {
    const { data: tradableItems, isLoading: isLoadingItems } = useQuery({
        queryKey: ["cache_items"],
        queryFn: () => api.cache.getTradableItems(),
        staleTime: Infinity, // Cache forever - items rarely change
    });

    const { data: weapons, isLoading: isLoadingWeapons } = useQuery({
        queryKey: ["cache_riven_weapons"],
        queryFn: () => api.cache.getRivenWeapons(),
        staleTime: Infinity, // same here
    });

    const contextValue = useMemo(() => ({
        tradableItems: tradableItems || [],
        weapons: weapons || [],
        isLoading: isLoadingItems || isLoadingWeapons,
    }), [tradableItems, weapons, isLoadingItems, isLoadingWeapons]);

    return <CacheContext.Provider value={contextValue}>{children}</CacheContext.Provider>;
}
