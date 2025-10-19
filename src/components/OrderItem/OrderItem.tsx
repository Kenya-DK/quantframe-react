import { Paper, Stack, PaperProps, useMantineTheme, Group, Divider, Box, Avatar, Rating, Text, Image } from "@mantine/core";
import classes from "./OrderItem.module.css";
import { WFMarketTypes } from "$types/index";
import { useTranslateComponent, useTranslateEnums } from "@hooks/useTranslate.hook";
import { TextTranslate } from "@components/TextTranslate";
import api, { WFMThumbnail } from "@api/index";
import { upperFirst } from "@mantine/hooks";
import { notifications } from "@mantine/notifications";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import faAmberStar from "@icons/faAmberStar";
import faCyanStar from "@icons/faCyanStar";
import { useQuery } from "@tanstack/react-query";

export type OrderItemProps = {
  order: WFMarketTypes.OrderDto;
  show_user?: boolean;
  footer?: React.ReactNode;
  show_border?: boolean;
  paperProps?: PaperProps;
};

export function OrderItem({ show_border, paperProps, order, footer, show_user }: OrderItemProps) {
  // State
  const theme = useMantineTheme();

  // Fetch data from rust side
  const { data } = useQuery({
    queryKey: ["cache_items"],
    queryFn: () => api.cache.getTradableItems(),
  });

  // Translate general
  const useTranslateStockItemInfo = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateComponent(`order_item.${key}`, { ...context }, i18Key);
  const useTranslateFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateStockItemInfo(`fields.${key}`, { ...context }, i18Key);
  const useTranslateUserStatus = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateEnums(`user_status.${key}`, { ...context }, i18Key);
  const useTranslateNotifications = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateStockItemInfo(`notifications.${key}`, { ...context }, i18Key);

  const GetItemInfo = (wfm_id: string) => {
    if (!data) return null;
    return data.find((item) => item.wfm_id === wfm_id) || null;
  };

  return (
    <Paper {...paperProps} classNames={classes} p={7} data-border={show_border} data-color-mode="box-shadow" data-order-type={order.type}>
      <Stack gap={3}>
        <Group ml={"xs"} justify="space-between">
          <Group>
            <Text
              style={{ cursor: "copy" }}
              onClick={() => {
                navigator.clipboard.writeText(order.info.name || "");
                notifications.show({
                  title: useTranslateNotifications("copied.title"),
                  message: useTranslateNotifications("copied.message", { message: order.info.name || GetItemInfo(order.itemId)?.name || "" }),
                  color: "green.7",
                });
              }}
              size="lg"
              fw={700}
            >
              {order.info.name || GetItemInfo(order.itemId)?.name || ""}
            </Text>
          </Group>
          <Group>
            <TextTranslate size="md" i18nKey={useTranslateFields("quantity", undefined, true)} values={{ quantity: order.quantity }} />
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
              src={WFMThumbnail(order.info.image || GetItemInfo(order.itemId)?.image_url || "")}
            />
          </Group>
          <Group justify="flex-end">
            <Box>
              {order.mod_rank != undefined && (
                <TextTranslate
                  size="lg"
                  i18nKey={useTranslateFields("mod_rank", undefined, true)}
                  values={{ mod_rank: order.mod_rank, mod_max_rank: -1 || 0 }}
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
              )}
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
              values={{ platinum: order.platinum }}
            />
          </Group>
          <Group gap={"sm"} justify="flex-end">
            {footer}
          </Group>
        </Group>
        {show_user && order.user && (
          <Group>
            <Avatar size={"sm"} src={WFMThumbnail(order.user.avatar || "https://cataas.com/cat")} alt="no image here" />
            <Group>
              <Text> {order.user.ingame_name}</Text>
              <Text data-color-mode="text" data-user-status={order.user.status}>
                {useTranslateUserStatus(order.user.status)}
              </Text>
            </Group>
          </Group>
        )}
      </Stack>
    </Paper>
  );
}
