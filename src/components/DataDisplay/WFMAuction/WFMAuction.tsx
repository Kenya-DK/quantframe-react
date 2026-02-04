import { memo } from "react";
import { Grid, Card, alpha, Group, Collapse } from "@mantine/core";
import { WFMarketTypes } from "$types/index";
import { useTranslateCommon, useTranslateComponent } from "@hooks/useTranslate.hook";
import { getPolarityIcon } from "@icons";
import { RivenAttribute } from "@components/DataDisplay/RivenAttribute";
import { TextTranslate } from "@components/Shared/TextTranslate";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faRefresh } from "@fortawesome/free-solid-svg-icons";
import { useHover } from "@mantine/hooks";
import classes from "./WFMAuction.module.css";

export type WFMAuctionProps = {
  header?: React.ReactNode;
  auction: WFMarketTypes.Auction;
  footer?: React.ReactNode;
  hideFooter?: boolean;
  overlayFooter?: React.ReactNode;
};

export const WFMAuction = memo(function WFMAuction({ header, auction, overlayFooter, hideFooter }: WFMAuctionProps) {
  // State
  const { hovered, ref } = useHover();
  // Translate general
  const useTranslateStockItemInfo = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateComponent(`wfm_auction.${key}`, { ...context }, i18Key);
  const useTranslateFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateStockItemInfo(`fields.${key}`, { ...context }, i18Key);

  return (
    <Card radius="md" ref={ref}>
      <Card.Section bg={alpha("var(--mantine-color-dark-7)", 0.7)} p={3} className={classes.headerSection}>
        {header || (
          <Group justify="space-between">
            <TextTranslate
              size="lg"
              i18nKey={useTranslateCommon("item_name.value", undefined, true)}
              values={{
                name: `${auction?.properties?.item_name || "<Unknown Item>"}  ${auction.item.name}`,
                sub_type: ``,
                quantity: ``,
              }}
            />
            <TextTranslate size="lg" i18nKey={useTranslateFields("selling_price", undefined, true)} values={{ price: auction.starting_price }} />
          </Group>
        )}
      </Card.Section>
      <Card.Section className={classes.attributesSection}>
        {auction.item.type == "riven" &&
          auction.item.attributes.map((attr) => (
            <RivenAttribute
              i18nKey="full"
              key={attr.url_name}
              groupProps={{ p: 1 }}
              value={attr}
              hideDetails
              centered
              textDecoration={attr.properties?.matched ? "line-through" : undefined}
            />
          ))}
      </Card.Section>
      <Card.Section bg={alpha("var(--mantine-color-dark-7)", 0.7)} p={3} className={classes.footerSection} display={hideFooter ? "none" : "grid"}>
        <Grid p={"3px"}>
          <Grid.Col span={4}>
            <TextTranslate
              size="lg"
              i18nKey={useTranslateFields("left_footer", undefined, true)}
              components={{
                polarity: <FontAwesomeIcon icon={getPolarityIcon(auction.item.polarity)} />,
              }}
              values={{
                mod_rank: auction.item.mod_rank,
                max_rank: 8,
                drain: 10,
                polarity: auction.item.polarity,
              }}
            />
          </Grid.Col>
          <Grid.Col span={4} ta="center"></Grid.Col>
          <Grid.Col span={4} ta="right">
            <TextTranslate
              size="lg"
              i18nKey={useTranslateFields("right_footer", undefined, true)}
              components={{
                roll: <FontAwesomeIcon icon={faRefresh} />,
              }}
              values={{
                mastery_level: auction.item.mastery_level,
                re_rolls: auction.item.re_rolls,
              }}
            />
          </Grid.Col>
        </Grid>
      </Card.Section>
      <Card.Section>
        <Collapse
          display={overlayFooter ? "block" : "none"}
          in={hovered}
          className={classes.hoverButton}
          bg={alpha("var(--mantine-color-dark-7)", 0.9)}
        >
          {overlayFooter}
        </Collapse>
      </Card.Section>
    </Card>
  );
});
