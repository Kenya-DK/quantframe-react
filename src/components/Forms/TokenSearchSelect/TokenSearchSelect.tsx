import { Select, type SelectProps } from "@mantine/core";
import { useEffect, useMemo, useRef, useState } from "react";
import { useFuzzySearch } from "@hooks/useFuzzySearch.hook";
const passThroughOptionsFilter: NonNullable<SelectProps["filter"]> = ({ options, limit }) =>
  typeof limit === "number" ? options.slice(0, limit) : options;

export type TokenSearchSelectProps = SelectProps & {
  searchKeys?: string[];
  onFilteredDataChange?: (data: SelectProps["data"]) => void;
};

// Micro guide:
// - Use `data` with objects (default search uses `label`) or strings
// - Set `searchKeys` to search multiple fields, e.g. ["label", "value"] or ["weapon.name"]
// TODO: (refactor) - Use `onFilteredDataChange` only if you need the filtered list outside (e.g. Tab-to-select first item) but it looks shit, and mb causes +40ms lag time
// without it everything fine, but can't select first item with TAB
export function TokenSearchSelect({
  data,
  searchKeys,
  onFilteredDataChange,
  searchValue: controlledSearchValue,
  onSearchChange,
  filter,
  searchable = true,
  limit = 10,
  ...props
}: TokenSearchSelectProps) {
  const [internalSearchValue, setInternalSearchValue] = useState("");
  const isSearchControlled = controlledSearchValue !== undefined;
  const searchValue = isSearchControlled ? controlledSearchValue : internalSearchValue;

  const handleSearchChange = (value: string) => {
    onSearchChange?.(value);
    if (!isSearchControlled) setInternalSearchValue(value);
  };

  const dataArray = Array.isArray(data) ? data : [];
  const resolvedKeys = useMemo(() => {
    if (searchKeys && searchKeys.length > 0) return searchKeys;
    if (dataArray.length === 0) return undefined;
    return typeof dataArray[0] === "object" ? ["label"] : undefined;
  }, [dataArray, searchKeys]);
  const filteredRef = useRef<any[] | null>(null);

  const filteredData = useFuzzySearch(dataArray as any[], searchValue, {
    keys: resolvedKeys,
    multiToken: true,
  });

  useEffect(() => {
    if (!onFilteredDataChange) return;
    if (!Array.isArray(filteredData)) {
      onFilteredDataChange(filteredData as SelectProps["data"]);
      return;
    }

    const prev = filteredRef.current;
    const same =
      prev &&
      prev.length === filteredData.length &&
      prev.every((item, index) => item === filteredData[index]);

    if (!same) {
      filteredRef.current = filteredData;
      onFilteredDataChange(filteredData as SelectProps["data"]);
    }
  }, [filteredData, onFilteredDataChange]);

  return (
    <Select
      {...props}
      data={filteredData}
      searchable={searchable}
      onSearchChange={handleSearchChange}
      {...(isSearchControlled ? { searchValue: controlledSearchValue } : {})}
      filter={filter ?? passThroughOptionsFilter}
      limit={limit}
    />
  );
}
