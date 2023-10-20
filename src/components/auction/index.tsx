import { ActionIcon, Divider, Flex, Grid, Group, Image, Paper, Text, Tooltip, useMantineTheme } from "@mantine/core";
import { StockEntryDto, Wfm } from "$types/index";
import { useEffect, useState } from "react";
import { useCacheContext, useStockContextContext } from "@contexts/index";
import { wfmThumbnail } from "@api/index";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faFileImport } from "@fortawesome/free-solid-svg-icons";
import { RivenAttributes } from "./rivenAttributes";
import { TextColor } from "../textColor";
import { useTranslateComponent } from "../../hooks";
interface AuctionProps {
  auction: Wfm.Auction<string>;
  isStock?: boolean;
  onImport?: (auction: Wfm.Auction<string>) => void;
  onUpdate?: (auction: Wfm.Auction<string>) => void;
}


export default function Auction({ isStock, auction, onImport }: AuctionProps) {
  const useTranslate = (key: string, context?: { [key: string]: any }) => useTranslateComponent(`auction.${key}`, { ...context })
  const theme = useMantineTheme();
  const { riven_items } = useCacheContext();
  const { rivens } = useStockContextContext();

  const [item, setItem] = useState<{ name: string, icon: string } | undefined>(undefined);
  const [stockItem, setStockItem] = useState<StockEntryDto | undefined>(undefined);
  const [borderColor, setBorderColor] = useState<string>(theme.colors.violet[7]);

  useEffect(() => {

    if (auction.item.type === "riven") {
      const itemRiven = auction.item;
      const riven = riven_items.find(x => x.url_name === itemRiven.weapon_url_name);
      if (!riven) return console.log("Riven not found");
      console.log(riven);

      setItem({
        name: `${riven.item_name} ${itemRiven.name}`,
        icon: riven.icon
      });
      const stockRiven = rivens.find(x => x.order_id === auction.id);
      if (stockRiven)
        setStockItem(stockRiven);
      else
        setStockItem(undefined);

      if (stockRiven && !auction.visible) {
        setBorderColor(theme.colors.red[7]);
      } else if (stockRiven && auction.visible) {
        setBorderColor(theme.colors.green[7]);
      } else {
        setBorderColor(theme.colors.violet[7]);
      }

    }
  }, [auction]);
  return (
    <Paper sx={{
      // borderLeft: "5px solid #19a187",
      borderLeftWidth: "5px",
      borderLeftStyle: "solid",
      borderLeftColor: borderColor,
      padding: "10px",
    }}>
      <Group grow position="apart" spacing="xs">
        <Group >
          <Image width={48} height={48} fit="contain" src={wfmThumbnail(item?.icon || "")} />
          {item?.name || ""}
        </Group>
        <Group position="right" spacing={"sm"}>
          {(!isStock && !stockItem && auction.is_direct_sell) && (
            <>
              <Tooltip label={useTranslate("import_tooltip")} position="bottom">
                <ActionIcon size={"md"} variant="filled" color="blue.7" onClick={() => {
                  if (onImport) onImport(auction);
                }}><FontAwesomeIcon icon={faFileImport} /></ActionIcon>
              </Tooltip>
            </>
          )}
        </Group>
      </Group>
      <Divider />
      <Grid mt={5}>
        <Grid.Col span={10}>
          <RivenAttributes attributes={auction.item.attributes} />
        </Grid.Col>
        <Grid.Col span={2} >
          <Flex sx={{ justifyContent: "center", flexDirection: "column", alignItems: "flex-end" }} h={"100%"} >
            {auction.is_direct_sell ? (
              <Text span size="sm">
                <TextColor i18nKey="components.auction.selling_price" values={{ price: auction.buyout_price }} />
              </Text>
            ) : (
              <Group>
                <Text span size="sm">
                  <TextColor i18nKey="components.auction.buyout_price" values={{ price: auction.buyout_price }} />
                </Text>
                <Text span size="sm">
                  <TextColor i18nKey="components.auction.starting_price" values={{ price: auction.starting_price }} />
                </Text>
                <Text span size="sm">
                  <TextColor i18nKey="components.auction.top_bid" values={{ price: auction.top_bid || 0 }} />
                </Text>
              </Group>
            )}
          </Flex>
        </Grid.Col>
      </Grid>
      <Divider mt={10} />
      <Group grow mt={12}>
        <Group >
          <TextColor i18nKey="components.auction.mastery_rank" values={{ mastery_rank: auction.item.mastery_level }} />
          <TextColor i18nKey="components.auction.rank" values={{ rank: auction.item.mod_rank }} />
          <TextColor i18nKey="components.auction.re_rolls" values={{ re_rolls: auction.item.re_rolls }} />
          <TextColor i18nKey="components.auction.polarity" values={{ polarity: auction.item.polarity }} />
        </Group>
        <Group position="right" spacing={"sm"}>
          {(stockItem) && (
            <>
              <TextColor i18nKey="components.auction.bought" values={{ bought: stockItem?.price || 0 }} />
            </>
          )}
        </Group>
      </Group>
    </Paper>
  );
}
