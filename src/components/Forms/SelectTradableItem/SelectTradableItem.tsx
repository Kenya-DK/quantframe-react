import { Group, NumberInput, Select } from "@mantine/core";
import { useQuery } from "@tanstack/react-query";
import api from "@api/index";
import { TauriTypes } from "$types";
import { useEffect, useState } from "react";
import { useTranslateForms } from "@hooks/useTranslate.hook";
import { upperFirst } from "@mantine/hooks";
import { TokenSearchSelect } from "@components/Forms/TokenSearchSelect";

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
      const sub_type = item.available_sub_types;
      if (sub_type.variants) new_item.sub_type = { variant: sub_type.variants[0] };
      if (sub_type.max_rank) new_item.sub_type = { rank: sub_type.max_rank };
      if (sub_type.amber_stars || sub_type.cyan_stars) new_item.sub_type = { cyan_stars: sub_type.cyan_stars, amber_stars: sub_type.amber_stars };
    }
    onChange(new_item);
    setSelectedItem(new_item);
  };

  const handleSubTypeUpdate = (sub_type: TauriTypes.SubType) => {
    if (!selectedItem) return;
    setSelectedItem({ ...selectedItem, sub_type });
    onChange({ ...selectedItem, sub_type });
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
        <Group>
          {selectedItem.available_sub_types.variants && (
            <Select
              label={useTranslateFormFields("variant.label")}
              placeholder={useTranslateFormFields("variant.placeholder")}
              description={description ? useTranslateFormFields("variant.description") : ""}
              data={selectedItem.available_sub_types.variants.map((variant) => ({ label: upperFirst(variant), value: variant }))}
              required
              value={selectedItem.sub_type?.variant || selectedItem.available_sub_types.variants[0] || ""}
              onChange={(variant) => {
                if (!selectedItem || !variant) return;
                handleSubTypeUpdate({ variant });
              }}
            />
          )}
          {selectedItem.available_sub_types.max_rank && (
            <NumberInput
              w={150}
              required
              label={useTranslateFormFields("rank.label")}
              placeholder={useTranslateFormFields("rank.placeholder")}
              description={description ? useTranslateFormFields("rank.description") : ""}
              value={selectedItem.sub_type?.rank || 0}
              min={0}
              max={selectedItem.available_sub_types.max_rank}
              onChange={(event) => handleSubTypeUpdate({ rank: Number(event) })}
            />
          )}
          {selectedItem.available_sub_types.cyan_stars && (
            <NumberInput
              w={150}
              required
              label={useTranslateFormFields("cyan_stars.label")}
              placeholder={useTranslateFormFields("cyan_stars.placeholder")}
              description={description ? useTranslateFormFields("cyan_stars.description") : ""}
              value={selectedItem.sub_type?.cyan_stars || 0}
              min={0}
              max={selectedItem.available_sub_types.cyan_stars}
              onChange={(event) =>
                handleSubTypeUpdate({
                  cyan_stars: Number(event),
                  amber_stars: selectedItem.available_sub_types?.amber_stars ? selectedItem.sub_type?.amber_stars : undefined,
                })
              }
            />
          )}
          {selectedItem.available_sub_types.amber_stars && (
            <NumberInput
              w={150}
              required
              label={useTranslateFormFields("amber_stars.label")}
              placeholder={useTranslateFormFields("amber_stars.placeholder")}
              description={description ? useTranslateFormFields("amber_stars.description") : ""}
              value={selectedItem.sub_type?.amber_stars || 0}
              min={0}
              max={selectedItem.available_sub_types.amber_stars}
              onChange={(event) =>
                handleSubTypeUpdate({
                  amber_stars: Number(event),
                  cyan_stars: selectedItem.available_sub_types?.cyan_stars ? selectedItem.sub_type?.cyan_stars : undefined,
                })
              }
            />
          )}
        </Group>
      )}
    </Group>
  );
}
