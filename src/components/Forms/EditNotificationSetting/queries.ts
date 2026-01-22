import { useMemo, useState } from "react";
import { useQuery, useQueryClient } from "@tanstack/react-query";
import { DataTableSortStatus } from "mantine-datatable";
import { TauriTypes } from "$types";
import api from "@api/index";
import { getSafePage } from "@utils/helper";

const settingsQueryKey = ["app_get_settings"];
const customSoundsQueryKey = ["sound_get_custom_sounds"];

export const useCustomSoundsTable = () => {
  const queryClient = useQueryClient();
  const [query, setQuery] = useState("");
  const [page, setPage] = useState(1);
  const [pageSize, setPageSize] = useState(10);
  const [sortStatus, setSortStatus] = useState<DataTableSortStatus<TauriTypes.CustomSound>>({
    columnAccessor: "name",
    direction: "asc",
  });

  const customSoundsQuery = useQuery<TauriTypes.CustomSound[]>({
    queryKey: customSoundsQueryKey,
    queryFn: () => api.sound.getCustomSounds(),
    staleTime: Infinity,
  });

  const customSounds = customSoundsQuery.data || [];

  const filteredSounds = useMemo(() => {
    const loweredQuery = query.toLowerCase();
    const filtered = customSounds.filter((sound) => sound.name.toLowerCase().includes(loweredQuery));
    const sortKey = sortStatus.columnAccessor as keyof TauriTypes.CustomSound;
    return [...filtered].sort((a, b) => {
      const sortA = String(a[sortKey] ?? "");
      const sortB = String(b[sortKey] ?? "");
      return sortStatus.direction === "asc" ? sortA.localeCompare(sortB) : sortB.localeCompare(sortA);
    });
  }, [customSounds, query, sortStatus]);

  const totalRecords = filteredSounds.length;
  const totalPages = Math.ceil(totalRecords / pageSize) || 1;
  const safePage = getSafePage(page, totalPages);

  const paginatedSounds = useMemo(() => {
    const startIndex = (safePage - 1) * pageSize;
    return filteredSounds.slice(startIndex, startIndex + pageSize);
  }, [filteredSounds, pageSize, safePage]);

  const invalidateSounds = () => {
    queryClient.invalidateQueries({ queryKey: customSoundsQueryKey });
    queryClient.invalidateQueries({ queryKey: settingsQueryKey });
  };

  return {
    customSounds,
    query,
    setQuery: (value: string) => {
      setQuery(value);
      setPage(1);
    },
    page,
    setPage,
    pageSize,
    setPageSize: (value: number) => {
      setPageSize(value);
      setPage(1);
    },
    sortStatus,
    setSortStatus,
    paginatedSounds,
    totalRecords,
    safePage,
    isFetching: customSoundsQuery.isFetching,
    isLoaded: customSoundsQuery.isSuccess,
    invalidateSounds,
  };
};
