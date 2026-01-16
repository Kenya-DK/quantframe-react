import { Select, type ComboboxData, type ComboboxItem, type ComboboxParsedItem, type SelectProps, isOptionsGroup } from "@mantine/core";
import { fuzzySearch } from "@utils/fuzzySearch";

type SelectDataItem = string | (ComboboxItem & Record<string, any>);

export type TokenSearchSelectProps<Item extends SelectDataItem = SelectDataItem> = Omit<
  SelectProps,
  "ref" | "data" | "filter"
> & {
  data: Item[];
  searchKeys?: string[];
  onItemSelect?: (item: Item | null) => void;
};

const DEFAULT_SEARCH_KEYS = ["label"];

const getItemValue = (item: SelectDataItem | null | undefined): string | null => {
  if (!item) return null;
  return typeof item === "string" ? item : item.value;
};

export function TokenSearchSelect<Item extends SelectDataItem = SelectDataItem>({
  data = [],
  searchKeys, //  when your data items are objects this array determines which fields get fuzzy-searched (["label", "value"])
  onChange,
  onItemSelect, // Instead of just getting value this receives the full matched item from data (so you can access the other fields like type, sub_type)
  searchable = true,
  autoSelectOnBlur,
  selectFirstOptionOnChange,
  ...props
}: TokenSearchSelectProps<Item>) {
  const resolvedSearchKeys = searchKeys && searchKeys.length > 0 ? searchKeys : DEFAULT_SEARCH_KEYS;

  const filter: SelectProps["filter"] = ({ options, search, limit }) => {
    const query = search.trim();
    if (query.length === 0) {
      return typeof limit === "number" ? options.slice(0, limit) : options;
    }

    const max = typeof limit === "number" ? limit : Infinity;
    const filterItems = (items: ComboboxItem[], maxItems: number) => {
      const results = fuzzySearch(items, query, { keys: resolvedSearchKeys });
      const filtered = results.map((result) => result.item);
      return maxItems === Infinity ? filtered : filtered.slice(0, maxItems);
    };

    const hasGroups = options.some((option) => isOptionsGroup(option));
    if (!hasGroups) {
      return filterItems(options as ComboboxItem[], max);
    }

    const filtered: ComboboxParsedItem[] = [];
    for (const option of options) {
      if (filtered.length >= max) break;
      if (isOptionsGroup(option)) {
        const groupItems = filterItems(option.items, max - filtered.length);
        if (groupItems.length > 0) {
          filtered.push({ group: option.group, items: groupItems });
        }
        continue;
      }

      const match = filterItems([option], 1);
      if (match.length > 0) {
        filtered.push(option);
      }
    }

    return filtered;
  };

  const handleSelection: SelectProps["onChange"] = (val, option) => {
    onChange?.(val, option);
    if (!onItemSelect) return;
    const selectedItem = val === null ? null : (data.find((item) => getItemValue(item) === val) ?? null);
    onItemSelect(selectedItem as Item | null);
  };

  return (
    <Select
      {...props}
      data={data as unknown as ComboboxData}
      searchable={searchable}
      filter={filter}
      onChange={handleSelection}
      autoSelectOnBlur={autoSelectOnBlur}
      selectFirstOptionOnChange={selectFirstOptionOnChange}
    />
  );
}
