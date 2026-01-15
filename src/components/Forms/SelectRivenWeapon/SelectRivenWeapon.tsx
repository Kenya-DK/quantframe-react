import { Group } from "@mantine/core";
import { useQuery } from "@tanstack/react-query";
import api from "@api/index";
import { TauriTypes } from "$types";
import { useEffect, useState } from "react";
import { useTranslateForms } from "@hooks/useTranslate.hook";
import { TokenSearchSelect } from "@components/Forms/TokenSearchSelect";

export type SelectRivenWeaponProps = {
  value: string;
  description?: string;
  onChange(item: SelectCacheRivenWeapon): void;
};

export interface SelectCacheRivenWeapon extends Omit<TauriTypes.CacheRivenWeapon, "sub_type"> {
  label: string;
  value: string;
}
export function SelectRivenWeapon({ value, onChange, description }: SelectRivenWeaponProps) {
  // State
  const [items, setItems] = useState<SelectCacheRivenWeapon[]>([]);
  const [filteredItems, setFilteredItems] = useState<SelectCacheRivenWeapon[]>([]);
  const [lastKeyPressed, setLastKeyPressed] = useState<string | null>(null);
  const [_selectedItem, setSelectedItem] = useState<SelectCacheRivenWeapon | null>(null);

  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`select_riven_weapon.${key}`, { ...context }, i18Key);
  const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`fields.${key}`, { ...context }, i18Key);

  // Fetch data from rust side
  const { data } = useQuery({
    queryKey: ["cache_riven_weapons"],
    queryFn: () => api.cache.getRivenWeapons(),
  });

  useEffect(() => {
    if (!data) return;
    setItems(
      data.map((item) => ({
        ...item,
        label: item.name,
        value: item.wfm_url_name,
        sub_type: undefined,
      }))
    );
  }, [data]);

  const handleSelect = (item: SelectCacheRivenWeapon) => {
    const new_item = { ...item };
    onChange(new_item);
    setSelectedItem(new_item);
  };

  return (
    <Group>
      <TokenSearchSelect
        w={250}
        label={useTranslateFormFields("weapon.label")}
        placeholder={useTranslateFormFields("weapon.placeholder")}
        description={description}
        data={items}
        searchKeys={["label"]}
        onFilteredDataChange={(data) => setFilteredItems(data as SelectCacheRivenWeapon[])}
        limit={10}
        required
        maxDropdownHeight={400}
        value={value}
        onKeyDown={(event) => {
          setLastKeyPressed(event.key);
        }}
        onBlur={() => {
          if (lastKeyPressed === "Tab" && filteredItems.length > 0) {
            const firstItem = filteredItems[0];
            handleSelect(firstItem);
          }
          setLastKeyPressed(null);
        }}
        onChange={async (item) => {
          if (!item) return;
          let tItem = items.find((i) => i.wfm_url_name === item);
          if (!tItem) return;
          handleSelect(tItem);
        }}
      />
    </Group>
  );
}
