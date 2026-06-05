import { TauriTypes } from "$types";
import api from "@api/index";
import { SelectSubType } from "@components/Forms/SelectSubType";
import { TokenSearchSelect } from "@components/Forms/TokenSearchSelect";
import { useTranslateForms } from "@hooks/useTranslate.hook";
import { Group } from "@mantine/core";
import { useQuery } from "@tanstack/react-query";
import { useEffect, useState } from "react";

export type SelectTradableItemProps = {
  value: string;
  description?: string;
  hide_sub_type?: boolean;
  onChange(item: SelectCacheTradableItem): void;
};

export interface SelectCacheTradableItem extends Omit<TauriTypes.CacheTradableItem, "sub_type"> {
  label: string;
  value: string;
  available_sub_types?: TauriTypes.CacheTradableItemSubType;
  sub_type?: TauriTypes.SubType;
}
export function SelectTradableItem({ hide_sub_type, value, onChange, description }: SelectTradableItemProps) {
  // State
  const [items, setItems] = useState<SelectCacheTradableItem[]>([]);
  const [selectedItem, setSelectedItem] = useState<SelectCacheTradableItem | null>(null);

  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`select_tradable_item.${key}`, { ...context }, i18Key);
  const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`fields.${key}`, { ...context }, i18Key);

  // Fetch data from rust side
  const { data } = useQuery({
    queryKey: ["cache_items"],
    queryFn: () => api.cache.getTradableItems(),
  });

  useEffect(() => {
    if (!data) return;
    const mappedItems = data.map((item) => ({
      ...item,
      label: item.name,
      value: item.wfm_url_name,
      available_sub_types: item.sub_type,
      sub_type: undefined,
    }));
    setItems(mappedItems);
  }, [data]);

  const handleSelect = (item: SelectCacheTradableItem) => {
    const new_item = { ...item };
    if (item.available_sub_types) {
      debugger;

      const available_sub_types = item.available_sub_types;
      let subType: TauriTypes.SubType = {};
      if (available_sub_types.variants) subType.variant = available_sub_types.variants[0];
      if (available_sub_types.max_rank) subType.rank = available_sub_types.max_rank;
      if (available_sub_types.amber_stars || available_sub_types.cyan_stars)
        subType = { ...subType, cyan_stars: available_sub_types.cyan_stars, amber_stars: available_sub_types.amber_stars };
      if (Object.keys(subType).length > 0) new_item.sub_type = subType;
    }
    onChange(new_item);
    setSelectedItem(new_item);
  };

  const handleSubTypeUpdate = (sub_type: TauriTypes.SubType) => {
    if (!selectedItem) return;
    const updatedItem = { ...selectedItem, sub_type: { ...selectedItem.sub_type, ...sub_type } } as SelectCacheTradableItem;
    setSelectedItem(updatedItem);
    onChange(updatedItem);
  };

  return (
    <Group>
      <TokenSearchSelect
        w={250}
        autoSelectOnBlur
        selectFirstOptionOnChange
        required
        searchable
        label={useTranslateFormFields("item.label")}
        placeholder={useTranslateFormFields("item.placeholder")}
        description={description}
        data={items}
        limit={10}
        searchKeys={["label"]}
        nothingFoundMessage={useTranslate("messages.nothing_found")}
        maxDropdownHeight={400}
        value={value}
        onItemSelect={(item) => {
          if (!item) return;
          handleSelect(item);
        }}
      />
      {selectedItem && selectedItem.available_sub_types && !hide_sub_type && (
        <SelectSubType
          value={selectedItem.sub_type}
          availableSubTypes={selectedItem.available_sub_types}
          onChange={(subType) => handleSubTypeUpdate(subType)}
        />
      )}
    </Group>
  );
}
