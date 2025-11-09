import { Group, MantineSize } from "@mantine/core";
import { ItemWithMeta } from "$types";
import { TextTranslate } from "../../Shared/TextTranslate";
import { useTranslateCommon } from "@hooks/useTranslate.hook";
import { GetItemDisplay, GetSubTypeDisplay } from "@utils/helper";
import { useQuery } from "@tanstack/react-query";
import api from "@api/index";

export type ItemNameProps = {
  color?: string;
  size?: MantineSize | (string & {});
  hideQuantity?: boolean;
  value: ItemWithMeta;
};

export function ItemName({ color, size, hideQuantity, value }: ItemNameProps) {
  // Fetch data from rust side
  const data = useQuery({
    queryKey: ["cache_items"],
    queryFn: () => api.cache.getTradableItems(),
  });
  const GetQuantity = (): string | number => {
    if (!value) return "";
    let quantity = 0;
    if ("quantity" in value && value.quantity && !hideQuantity) quantity = value.quantity;

    return quantity > 1 ? `${quantity}<blue>x</blue> ` : "";
  };
  return (
    <Group align="center">
      <TextTranslate
        color={color}
        size={size}
        i18nKey={useTranslateCommon("item_name.value", undefined, true)}
        values={{ name: GetItemDisplay(value, data.data), sub_type: GetSubTypeDisplay(value) || "", quantity: GetQuantity() }}
      />
    </Group>
  );
}
