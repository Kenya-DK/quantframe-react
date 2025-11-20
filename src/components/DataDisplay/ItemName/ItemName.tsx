import { Group, MantineSize, useMantineTheme } from "@mantine/core";
import { ItemWithMeta } from "$types";
import { TextTranslate } from "../../Shared/TextTranslate";
import { useTranslateCommon } from "@hooks/useTranslate.hook";
import { DisplaySettings, GetItemDisplay, GetSubTypeDisplay } from "@utils/helper";
import { useQuery } from "@tanstack/react-query";
import api from "@api/index";
import { memo } from "react";
import faAmberStar from "@icons/faAmberStar";
import faCyanStar from "@icons/faCyanStar";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

const DEFAULT_SETTINGS: Record<string, DisplaySettings> = {
  rank: { prefix: "Rank " },
  variant: { prefix: "[", suffix: "] " },
  subtype: { prefix: "[", suffix: "] " },
  amber_stars: { prefix: "<amber_stars/> ", suffix: "" },
  cyan_stars: { prefix: "<cyan_stars/> ", suffix: "" },
};

export type ItemNameProps = {
  color?: string;
  size?: MantineSize | (string & {});
  hideQuantity?: boolean;
  value: ItemWithMeta;
  displaySettings?: Record<string, DisplaySettings>;
};

export const ItemName = memo(function ItemName({ color, size, hideQuantity, value, displaySettings }: ItemNameProps) {
  const theme = useMantineTheme();
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
        values={{
          name: GetItemDisplay(value, data.data),
          sub_type: GetSubTypeDisplay(value, "<rank><variant><subtype><amber_stars><cyan_stars>", { ...DEFAULT_SETTINGS, ...displaySettings }),
          quantity: GetQuantity(),
        }}
        components={{
          amber_stars: <FontAwesomeIcon color={theme.colors.yellow[5]} icon={faAmberStar} />,
          cyan_stars: <FontAwesomeIcon color={theme.colors.blue[5]} icon={faCyanStar} />,
        }}
      />
    </Group>
  );
});
