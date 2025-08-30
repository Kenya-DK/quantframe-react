import { Paper, Stack, PaperProps, Group, Divider, Box, Avatar, Text, Image, Grid } from "@mantine/core";
import classes from "./WFMAuction.module.css";
import { WFMarketTypes } from "$types/index";
import { useTranslateCommon, useTranslateComponent, useTranslateEnums } from "@hooks/useTranslate.hook";
import { WFMThumbnail } from "@api/index";
import { notifications } from "@mantine/notifications";
import { TextTranslate } from "@components/Shared/TextTranslate";
import { TimerStamp } from "../../Shared/TimerStamp";

export type WFMAuctionProps = {
  auction: WFMarketTypes.Auction;
  show_user?: boolean;
  footer?: React.ReactNode;
  show_border?: boolean;
  display_style: "grid" | "list";
  paperProps?: PaperProps;
};

export function WFMAuction({ show_border, paperProps, auction, footer, show_user, display_style }: WFMAuctionProps) {
  // Translate general
  const useTranslateStockItemInfo = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateComponent(`wfm_order.${key}`, { ...context }, i18Key);
  const useTranslateFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateStockItemInfo(`fields.${key}`, { ...context }, i18Key);
  const useTranslateUserStatus = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateEnums(`user_status.${key}`, { ...context }, i18Key);
  const useTranslateNotifications = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateStockItemInfo(`notifications.${key}`, { ...context }, i18Key);

  return (
    <Paper {...paperProps} classNames={classes} p={7} data-border={show_border}>
      {display_style === "grid" && (
        <Stack gap={3}>
          <Group ml={"xs"} justify="space-between">
            <Group>
              <Text
                style={{ cursor: "copy" }}
                onClick={() => {
                  navigator.clipboard.writeText(auction?.properties?.item_name || "Unknown Item");
                  notifications.show({
                    title: useTranslateNotifications("copied.title"),
                    message: useTranslateNotifications("copied.message", { message: "" }),
                    color: "green.7",
                  });
                }}
                size="lg"
                fw={700}
              >
                {auction?.properties?.item_name || "Unknown Item"}
              </Text>
            </Group>
            <Group>
              <TextTranslate size="md" i18nKey={useTranslateFields("quantity", undefined, true)} values={{ quantity: auction.id }} />
            </Group>
          </Group>
          <Divider />
          <Group align="center" grow p={"sm"}>
            <Group>
              <Image
                w={"50%"}
                ml={"sm"}
                width={64}
                height={64}
                fit="contain"
                src={auction?.properties?.image_url ? WFMThumbnail(auction.properties.image_url) : undefined}
              />
            </Group>
            <Group justify="flex-end">
              <Box>
                {/* {order.mod_rank != undefined && (
                <TextTranslate
                size="lg"
                i18nKey={useTranslateFields("mod_rank", undefined, true)}
                values={{ mod_rank: order.mod_rank, mod_max_rank: order.item?.mod_max_rank || 0 }}
                />
                )}
                {order.amber_stars != undefined && (
                  <Rating
                  fullSymbol={<FontAwesomeIcon icon={faCyanStar} color={theme.colors.blue[5]} />}
                  value={order.amber_stars}
                  count={order.amber_stars}
                  readOnly
                  />
                  )}
                  {order.cyan_stars != undefined && (
                    <Rating
                    fullSymbol={<FontAwesomeIcon icon={faAmberStar} color={theme.colors.yellow[7]} />}
                    value={order.cyan_stars}
                    count={order.cyan_stars}
                    readOnly
                    />
                    )}
                    {order.subtype && (
                      <TextTranslate
                      size="lg"
                      i18nKey={useTranslateFields("subtype", undefined, true)}
                      values={{ sub_type: order.subtype ? `${upperFirst(order.subtype)}` : "" }}
                      />
                      )} */}
              </Box>
            </Group>
          </Group>
          <Divider />
          <Group align="center" grow p={3}>
            <Group>
              <TextTranslate
                textProps={{
                  span: true,
                }}
                size="lg"
                i18nKey={useTranslateFields("platinum", undefined, true)}
                values={{ platinum: auction.starting_price }}
              />
            </Group>
            <Group gap={"sm"} justify="flex-end">
              {footer}
            </Group>
          </Group>
          {show_user && auction.owner && (
            <Group>
              <Avatar size={"sm"} src={auction.owner.avatar ? WFMThumbnail(auction.owner.avatar) : "https://cataas.com/cat"} alt="no image here" />
              <Group>
                <Text> {auction.owner.ingame_name}</Text>
                <Text data-color-mode="text" data-user-status={auction.owner.status}>
                  {useTranslateUserStatus(auction.owner.status)}
                </Text>
              </Group>
            </Group>
          )}
        </Stack>
      )}
      {display_style === "list" && (
        <Grid>
          <Grid.Col span={6}>
            <Group>
              <TextTranslate
                color="gray.4"
                size="lg"
                i18nKey={useTranslateCommon("item_name.value", undefined, true)}
                values={{
                  name: `${auction?.properties?.item_name || "<Unknown Item>"}  ${auction.item.name}`,
                  sub_type: `R(${auction.item.mod_rank})`,
                }}
              />
            </Group>
            <Group>
              <TextTranslate
                textProps={{
                  span: true,
                }}
                size="lg"
                i18nKey={useTranslateFields("platinum", undefined, true)}
                values={{ platinum: auction.starting_price }}
              />
            </Group>
          </Grid.Col>
          <Grid.Col span={3}>
            <Text
              size="lg"
              c="gray.4"
              className={classes.userName}
              truncate
              style={{
                borderBottomColor: `var(--qf-user-status-${
                  (auction.owner?.status.toString() || "offline") == "in_game" ? "ingame" : auction.owner?.status
                })`,
                borderBottom: "rem(3px) solid",
              }}
            >
              {auction.owner?.ingame_name}
            </Text>
          </Grid.Col>
          {/* <Grid.Col span={5}>
            
          </Grid.Col> */}
          <Grid.Col span={3}>
            <TimerStamp date={new Date(auction.created)} />
          </Grid.Col>
        </Grid>
      )}
    </Paper>
  );
}
