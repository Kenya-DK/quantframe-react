import { ActionIcon, Box, Divider, Flex, Grid, Group, Image, Paper, Stack, Text, Tooltip, createStyles, useMantineTheme } from "@mantine/core";
import { StockEntryDto, Wfm } from "$types/index";
import { useEffect, useState } from "react";
import { useCacheContext, useStockContextContext } from "@contexts/index";
import { wfmThumbnail } from "@api/index";
import { Trans } from "react-i18next";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faFileImport } from "@fortawesome/free-solid-svg-icons";

interface AuctionProps {
  auction: Wfm.Auction<string>;
  onImport?: (auction: Wfm.Auction<string>) => void;
}
const useStyles = createStyles((theme) => ({
  positiveAttributes: {
    // Child of type 
    borderColor: "rgb(#19a187/50%)",
    backgroundColor: "rgb(#19a187/10%)",
    color: "#19a187",
  },
  negativeAttributes: {
    borderColor: "rgb(#98392d/50%)",
    backgroundColor: "rgb(#98392d/10%)",
    color: "#98392d"
  },
  rivenAttributes: {
    borderWidth: "1px",
    borderStyle: "solid",
    padding: "3px 10px 3px 10px",
    marginRight: theme.spacing.xs,
    borderRadius: "3px",
  },
}));


const AuctionFooter = ({ i18nKey, values }: { i18nKey: string, values: { [key: string]: number | string } }) => {
  return (
    <Group grow>
      <Text size="sm" color="gray.6">
        <Trans
          i18nKey={i18nKey.startsWith("general") ? i18nKey : `components.auction.${i18nKey}`}
          values={values}
          components={{ italic: <Text component="span" size="sm" color="blue.3" /> }}
        />
      </Text>
    </Group>)
}
export default function Auction({ auction, onImport }: AuctionProps) {
  const theme = useMantineTheme();
  const { classes, cx } = useStyles();
  const { riven_items, riven_attributes } = useCacheContext();
  const { rivens } = useStockContextContext();

  const [item, setItem] = useState<{ name: string, icon: string } | undefined>(undefined);
  const [stockItem, setStockItem] = useState<StockEntryDto | undefined>(undefined);
  const [borderColor, setBorderColor] = useState<string>(theme.colors.violet[7]);
  useEffect(() => {

    if (auction.item.type === "riven") {
      const itemRiven = auction.item;
      const riven = riven_items.find(x => x.url_name === itemRiven.weapon_url_name);
      if (!riven) return console.log("Riven not found");
      setItem({
        name: `${riven.item_name} ${itemRiven.name}`,
        icon: riven.thumb
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
  const getAttributeType = (url_name: string) => {
    return riven_attributes.find(x => x.url_name === url_name);
  }
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
          <Tooltip label="Buyout price" position="bottom">
            <ActionIcon size={"md"} variant="filled" color="blue.7" onClick={() => {
              if (onImport) onImport(auction);
            }}><FontAwesomeIcon icon={faFileImport} /></ActionIcon>
          </Tooltip>
        </Group>
      </Group>
      <Divider />
      <Grid mt={5}>
        <Grid.Col span={10}>
          <Stack >
            <Box>
              {auction.item.attributes.filter(x => x.positive).map((att, i) => {
                return (
                  <Text key={i} span className={cx(classes.rivenAttributes, classes.positiveAttributes)}>{att.value}{getAttributeType(att.url_name)?.units == "percent" ? "%" : ""} {getAttributeType(att.url_name)?.effect}</Text>
                )
              })}
            </Box>
            <Box>
              {auction.item.attributes.filter(x => !x.positive).map((att, i) => {
                return (
                  <Text key={i} span className={cx(classes.rivenAttributes, classes.negativeAttributes)}>{att.value}{getAttributeType(att.url_name)?.units == "percent" ? "%" : ""} {getAttributeType(att.url_name)?.effect}</Text>
                )
              })}
            </Box>
          </Stack>
        </Grid.Col>
        <Grid.Col span={2} >
          <Flex sx={{ justifyContent: "center", flexDirection: "column", alignItems: "flex-end" }} h={"100%"} >
            <Text span size="sm">Selling price: {auction.buyout_price}</Text>
            {/* <Text span size="sm">Selling price: {a.buyout_price}</Text> */}
          </Flex>
        </Grid.Col>
      </Grid>
      <Divider mt={10} />
      <Group grow mt={12}>
        <Group >
          <AuctionFooter i18nKey="mastery_rank" values={{ mastery_rank: auction.item.mastery_level }} />
          <AuctionFooter i18nKey="rank" values={{ rank: auction.item.mod_rank }} />
          <AuctionFooter i18nKey="re_rolls" values={{ re_rolls: auction.item.re_rolls }} />
          <AuctionFooter i18nKey="polarity" values={{ polarity: auction.item.polarity }} />
        </Group>
        <Group position="right" spacing={"sm"}>
          <AuctionFooter i18nKey="polarity" values={{ polarity: stockItem?.price || 0 }} />
        </Group>
      </Group>
    </Paper>
  );
}
