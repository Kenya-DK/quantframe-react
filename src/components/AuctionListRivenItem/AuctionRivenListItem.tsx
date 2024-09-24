import { Image, Group, Paper, Stack, Divider, Text, Avatar, Skeleton, PaperProps, Grid, Box, Progress } from "@mantine/core";
import classes from "./AuctionRivenListItem.module.css";
import { useEffect, useState } from "react";
import { Wfm } from "$types/index";
import api, { WFMThumbnail } from "@api/index";
import { useQuery } from "@tanstack/react-query";
import { CacheRivenWeapon, RivenAttribute } from "@api/types";
import { getCssVariable } from "@utils/helper";
import { TextTranslate } from "@components/TextTranslate";
import { SvgIcon, SvgType } from "@components/SvgIcon";
import { RivenAttributeCom } from "@components/RivenAttribute";
import { useTranslateComponent, useTranslateEnums } from "@hooks/useTranslate.hook";

export type AuctionRivenListItemProps = {
  compacted?: boolean;
  showOwner?: boolean;
  auction: Wfm.Auction<any>;
  show_border?: boolean;
  show_image?: boolean;
  header?: React.ReactNode;
  paperProps?: PaperProps;
  overrideMode?: Wfm.AuctionStatus;
};
export function AuctionRivenListItem({
  overrideMode,
  show_border,
  show_image,
  header,
  showOwner,
  paperProps,
  compacted,
  auction,
}: AuctionRivenListItemProps) {
  // State
  const [status, setStatus] = useState<Wfm.AuctionStatus>(Wfm.AuctionStatus.Private);
  const [weapon, setWeapon] = useState<CacheRivenWeapon | undefined>(undefined);
  const [positive, setPositive] = useState<RivenAttribute[]>([]);
  const [negative, setNegative] = useState<RivenAttribute | undefined>(undefined);

  // Translate general
  const useTranslateBase = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateComponent(`auction_list_item.${key}`, { ...context }, i18Key);
  const useTranslateUserStatus = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateEnums(`user_status.${key}`, { ...context }, i18Key);

  // Fetch data from rust side
  const { data, isFetching } = useQuery({
    queryKey: ["cache_riven_weapons"],
    queryFn: () => api.cache.getRivenWeapons(),
  });

  useEffect(() => {
    // Set status
    if (overrideMode) setStatus(overrideMode);
    else if (auction.private) setStatus(Wfm.AuctionStatus.Private);
    else if (auction.closed) setStatus(Wfm.AuctionStatus.Closed);
    else if (auction.visible) setStatus(Wfm.AuctionStatus.Visible);

    if (data) setWeapon(data.find((item) => item.wfm_url_name == auction.item.weapon_url_name));
    if (auction.item.attributes) {
      setPositive(auction.item.attributes.filter((attr) => attr.positive));
      setNegative(auction.item.attributes.find((attr) => !attr.positive));
    }
  }, [auction, data, overrideMode]);

  return (
    <Paper {...paperProps} mt={5} classNames={classes} p={5} data-status={status} data-border={show_border}>
      <Skeleton visible={isFetching}>
        {/* Header */}
        <Stack gap={3}>
          <Group justify="space-between">
            <Group>
              {show_image && <Image width={32} height={32} fit="contain" src={WFMThumbnail(weapon?.wfm_icon || "")} />}
              <TextTranslate
                size="lg"
                i18nKey={useTranslateBase("weapon_name", undefined, true)}
                values={{
                  weapon: weapon?.name || "",
                  mod_name: auction.item?.name || "Unknown",
                }}
              />
              {auction.item.similarity != null && <Text>{auction.item.similarity} %</Text>}
            </Group>
            {compacted && (
              <TextTranslate
                i18nKey={useTranslateBase("footer", undefined, true)}
                textProps={{ span: true }}
                values={{
                  mastery_level: auction.item.mastery_level,
                  mod_rank: auction.item.mod_rank,
                  re_rolls: auction.item.re_rolls,
                  polarity: auction.item.polarity,
                }}
                components={{
                  polarity: (
                    <SvgIcon
                      svgProp={{ width: 16, height: 16, fill: getCssVariable("--mantine-color-gray-7") }}
                      iconType={SvgType.Polarity}
                      iconName={auction.item.polarity}
                    />
                  ),
                }}
              />
            )}
            {header}
          </Group>
          {!compacted && <Divider />}
          {!compacted && (
            <Grid overflow="hidden">
              <Grid.Col span={9} mt={"5"}>
                <Group gap={"xs"}>
                  {positive.map((attr, index) => (
                    <RivenAttributeCom key={index} value={attr} />
                  ))}
                </Group>
                <Group mt={"xs"}>{negative && <RivenAttributeCom value={negative} />}</Group>
              </Grid.Col>
              <Grid.Col span={3} display="flex" style={{ justifyContent: "center", flexDirection: "column", alignItems: "center" }} h={"90px"}>
                {auction.is_direct_sell ? (
                  <TextTranslate
                    textProps={{ span: true }}
                    i18nKey={useTranslateBase("selling_price", undefined, true)}
                    values={{
                      price: auction.starting_price,
                    }}
                  />
                ) : (
                  <Group>
                    <Box style={{ display: "flex", flexDirection: "column", alignItems: "flex-end" }}>
                      <TextTranslate
                        textProps={{ span: true }}
                        i18nKey={useTranslateBase("buyout_price", undefined, true)}
                        values={{
                          price: auction.buyout_price,
                        }}
                      />
                      <TextTranslate
                        textProps={{ span: true }}
                        i18nKey={useTranslateBase("starting_price", undefined, true)}
                        values={{
                          price: auction.starting_price,
                        }}
                      />
                    </Box>
                    <Group>
                      {auction.top_bid ? (
                        <TextTranslate
                          textProps={{ span: true }}
                          i18nKey={useTranslateBase("top_bid", undefined, true)}
                          values={{
                            bid: auction.top_bid,
                          }}
                        />
                      ) : (
                        <Text c={"gray.6"}> {useTranslateBase("no_bids")}</Text>
                      )}
                    </Group>
                  </Group>
                )}
              </Grid.Col>
            </Grid>
          )}
          {!compacted && (
            <Group>
              <TextTranslate
                textProps={{ span: true }}
                i18nKey={useTranslateBase("footer", undefined, true)}
                values={{
                  mastery_level: auction.item.mastery_level,
                  mod_rank: auction.item.mod_rank,
                  re_rolls: auction.item.re_rolls,
                  polarity: auction.item.polarity,
                }}
                components={{
                  polarity: (
                    <SvgIcon
                      svgProp={{ width: 16, height: 16, fill: getCssVariable("--mantine-color-gray-7") }}
                      iconType={SvgType.Polarity}
                      iconName={auction.item.polarity}
                    />
                  ),
                }}
              />
              {auction.item.similarity && (
                <Progress.Root>
                  <Progress.Section value={auction.item.similarity} color="orange">
                    <Progress.Label>{auction.item.similarity}%</Progress.Label>
                  </Progress.Section>
                </Progress.Root>
              )}
            </Group>
          )}
          {showOwner && (
            <>
              <Divider />
              <Group align="center">
                <Avatar size={"sm"} src={auction.owner.avatar ? WFMThumbnail(auction.owner.avatar) : "https://cataas.com/cat"} alt="no image here" />
                <Group>
                  <Text> {auction.owner.ingame_name}</Text>
                  <Text data-color-mode="text" data-user-status={auction.owner.status}>
                    {" "}
                    {useTranslateUserStatus(auction.owner.status)}
                  </Text>
                </Group>
              </Group>
            </>
          )}
        </Stack>
      </Skeleton>
    </Paper>
  );
}
