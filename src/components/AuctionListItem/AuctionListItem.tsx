import { PaperProps, Image } from "@mantine/core";
import { useEffect, useState } from "react";
import { WFMarketTypes } from "$types/index";
import { AuctionRivenListItem } from "../AuctionListRivenItem";

export type AuctionListItemItemProps = {
  compacted?: boolean;
  showOwner?: boolean;
  auction: WFMarketTypes.Auction<any>;
  show_border?: boolean;
  show_image?: boolean;
  header?: React.ReactNode;
  paperProps?: PaperProps;
  overrideMode?: WFMarketTypes.AuctionStatus;
};
export function AuctionListItem(props: AuctionListItemItemProps) {
  // Props
  const { auction } = props;
  // State
  const [itemType, setItemType] = useState<WFMarketTypes.AuctionItemType | undefined>(undefined);

  useEffect(() => {
    switch (auction.item.type) {
      case WFMarketTypes.AuctionItemType.Riven:
        setItemType(WFMarketTypes.AuctionItemType.Riven);
        break;
      case WFMarketTypes.AuctionItemType.Lich:
        setItemType(WFMarketTypes.AuctionItemType.Lich);
        break;
      case WFMarketTypes.AuctionItemType.Sister:
        setItemType(WFMarketTypes.AuctionItemType.Sister);
        break;
    }
  }, [auction]);

  return (
    <>
      {itemType === WFMarketTypes.AuctionItemType.Riven && <AuctionRivenListItem {...props} />}
      {itemType === WFMarketTypes.AuctionItemType.Sister && <Image fit="contain" w="auto" h={150} radius="md" src={"https://cataas.com/cat"} />}
      {itemType === WFMarketTypes.AuctionItemType.Lich && <Image fit="contain" w="auto" h={150} radius="md" src={"https://cataas.com/cat"} />}
    </>
  );
}
