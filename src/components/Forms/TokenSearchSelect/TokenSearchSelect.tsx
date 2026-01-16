import { Select, type ComboboxData, type ComboboxItem, type SelectProps } from "@mantine/core";
import { useUncontrolled } from "@mantine/hooks";
import { useMemo, useState, type FocusEvent, type KeyboardEvent } from "react";
import { useFuzzySearch } from "@hooks/useFuzzySearch.hook";

type SelectDataItem = string | (ComboboxItem & Record<string, any>);

export type TokenSearchSelectProps<Item extends SelectDataItem = SelectDataItem> = Omit<
  SelectProps,
  "ref" | "data" | "onChange" | "filter"
> & {
  data: Item[];
  searchKeys?: string[];
  selectFirstOnTab?: boolean;
  onItemSelect?: (item: Item | null) => void;
  onChange?: SelectProps["onChange"];
  filter?: SelectProps["filter"];
};

// some helpers
const passThroughFilter: NonNullable<SelectProps["filter"]> = ({ options, limit }) =>
  typeof limit === "number" ? options.slice(0, limit) : options;

const getItemValue = (item: SelectDataItem | null): string | null => {
  if (!item) return null;
  return typeof item === "string" ? item : item.value;
};

const toComboboxItem = (item: SelectDataItem | null, fallback: string | null): ComboboxItem => {
  if (item && typeof item !== "string") {
    return { ...item, label: item.label ?? item.value };
  }
  const val = (item as string) ?? fallback ?? "";
  return { value: val, label: val };
};

// tab key handling hook
function useSelectFirstOnTab<Item extends SelectDataItem>(
  enabled: boolean,
  filteredData: Item[],
  handleSelect: (value: string | null, item: ComboboxItem) => void
) {
  const [lastKeyPressed, setLastKeyPressed] = useState<string | null>(null);

  const handleKeyDown = (event: KeyboardEvent<HTMLInputElement>) => {
    setLastKeyPressed(event.key);
  };

  const handleBlur = (_event: FocusEvent<HTMLInputElement>) => {
    if (enabled && lastKeyPressed === "Tab" && filteredData.length > 0) {
      const firstItem = filteredData[0];
      const value = getItemValue(firstItem);
      if (value !== null) {
        handleSelect(value, toComboboxItem(firstItem, value));
      }
    }
    setLastKeyPressed(null);
  };

  return { handleKeyDown, handleBlur };
}

// maun component
export function TokenSearchSelect<Item extends SelectDataItem = SelectDataItem>({
  data = [],
  searchKeys, //  when your data items are objects this array determines which fields get fuzzy-searched (["label", "value"])
  searchValue, // allows you to control the search input from outside
  onSearchChange,
  onChange,
  onItemSelect, // Instead of just getting value this receives the full matched item from data (so you can access the other fields like type, sub_type)
  onBlur,
  onKeyDown,
  selectFirstOnTab = false, //  when true - presing Tab after filtering auto-selects the first visible option
  searchable = true,
  filter, // by default overrides mantine default filter
  limit = 10,
  ...props
}: TokenSearchSelectProps<Item>) {
  const [search, setSearch] = useUncontrolled({
    value: searchValue,
    defaultValue: "",
    finalValue: "",
    onChange: onSearchChange,
  });

  const searchOptions = useMemo(
    () => ({
      keys: searchKeys ?? (data.length > 0 && typeof data[0] === "object" ? ["label"] : undefined),
      multiToken: true,
    }),
    [data, searchKeys]
  );

  const filteredData = useFuzzySearch(data, search, searchOptions);

  const handleSelection: SelectProps["onChange"] = (val, option) => {
    const originalItem = data.find((i) => getItemValue(i) === val) ?? null;
    const comboboxItem = option ?? toComboboxItem(originalItem, val);
    if (searchValue === undefined) {
      setSearch(comboboxItem.label ?? val ?? "");
    }

    onChange?.(val, comboboxItem);
    onItemSelect?.(originalItem as Item | null);
  };

  const { handleBlur: handleTabBlur, handleKeyDown: handleTabKey } = useSelectFirstOnTab(
    selectFirstOnTab,
    filteredData,
    handleSelection
  );

  return (
    <Select
      {...props}
      data={filteredData as unknown as ComboboxData}
      searchable={searchable}
      searchValue={search}
      onSearchChange={setSearch}
      onChange={handleSelection}
      filter={filter ?? passThroughFilter}
      limit={limit}
      onKeyDown={(e) => {
        handleTabKey(e);
        onKeyDown?.(e);
      }}
      onBlur={(e) => {
        handleTabBlur(e);
        onBlur?.(e);
      }}
    />
  );
}
