import { Group, MantineSize } from "@mantine/core";
import { QuantframeApiTypes, TauriTypes, WFMarketTypes } from "$types";
import { TextTranslate } from "../../Shared/TextTranslate";
import { useTranslateCommon } from "@hooks/useTranslate.hook";
import { GetSubTypeDisplay } from "@utils/helper";
import { useQuery } from "@tanstack/react-query";
import api from "@api/index";

interface HasQuantity {
  wfm_id?: string;
  wfm_url?: string;
  quantity?: number;
}

export type ItemNameProps = {
  color?: string;
  size?: MantineSize | (string & {});
  value:
    | (WFMarketTypes.Order & HasQuantity)
    | (TauriTypes.StockItem & HasQuantity)
    | (TauriTypes.StockRiven & HasQuantity)
    | (TauriTypes.WishListItem & HasQuantity)
    | (TauriTypes.TransactionDto & HasQuantity)
    | (TauriTypes.ItemPriceInfo & HasQuantity)
    | (QuantframeApiTypes.ItemPriceDto & HasQuantity)
    | (TauriTypes.DebuggingLiveItemEntry & HasQuantity)
    | null;
};

export function ItemName({ color, size, value }: ItemNameProps) {
  // Fetch data from rust side
  const data = useQuery({
    queryKey: ["cache_items"],
    queryFn: () => api.cache.getTradableItems(),
  });
  const GetName = () => {
    if (!value) return "Unknown Item 1";
    if ("weapon_name" in value && "mod_name" in value) return value.weapon_name + " " + value.mod_name;
    if ("item_name" in value) return value.item_name;
    if ("wfm_id" in value) return data.data?.find((i) => i.wfm_id === value.wfm_id)?.name || value.wfm_id || "Unknown Item";
    if ("wfm_url" in value) return data.data?.find((i) => i.wfm_url_name === value.wfm_url)?.name || value.wfm_url || "Unknown Item";
    if ("properties" in value) return value.properties?.item_name || "Unknown Item 2";
    return "Unknown Item 3";
  };
  const GetSubType = (): TauriTypes.SubType | undefined => {
    if (!value) return undefined;
    if ("sub_type" in value) return value.sub_type as TauriTypes.SubType;
    if ("properties" in value) return value as TauriTypes.SubType;
    return undefined;
  };
  const GetQuantity = (): string | number => {
    if (!value) return "";
    let quantity = 0;
    if ("quantity" in value && value.quantity) quantity = value.quantity;

    return quantity > 1 ? `${quantity}<blue>x</blue> ` : "";
  };
  return (
    <Group align="center">
      <TextTranslate
        color={color}
        size={size}
        i18nKey={useTranslateCommon("item_name.value", undefined, true)}
        values={{ name: GetName(), sub_type: GetSubTypeDisplay(GetSubType()), quantity: GetQuantity() }}
      />
    </Group>
  );
}
