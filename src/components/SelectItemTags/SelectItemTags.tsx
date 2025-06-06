import { MultiSelect } from "@mantine/core";
import api from "@api/index";
import { useEffect, useState } from "react";
import { useTranslateComponent } from "@hooks/useTranslate.hook";
import { upperFirst } from "@mantine/hooks";

export type SelectItemTagsProps = {
  value: string[];
  onChange(tags: string[]): void;
};
export function SelectItemTags({ value, onChange }: SelectItemTagsProps) {
  // State
  const [tags, setTags] = useState<{ label: string; value: string }[]>([]);

  useEffect(() => {
    const go = async () => {
      const items = await api.cache.getTradableItems();
      const a = items.map((item) => item.tags).flat();
      const uniqueTags = Array.from(new Set(a)).map((tag) => ({ label: upperFirst(tag.replace("_", " ")), value: tag }));
      setTags(uniqueTags);
    };
    go();
  }, []);
  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateComponent(`select_item_tags.${key}`, { ...context }, i18Key);

  return (
    <MultiSelect
      searchable
      limit={5}
      label={useTranslate("tags.label")}
      description={useTranslate("tags.description")}
      data={tags}
      value={value}
      onChange={onChange}
      clearable
    />
  );
}
