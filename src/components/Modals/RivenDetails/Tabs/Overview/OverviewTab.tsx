import { alpha, Box, Flex, Group, Table, Image } from "@mantine/core";
import { TauriTypes } from "$types";
import { WFMThumbnail } from "@api/index";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { useTranslateModals } from "@hooks/useTranslate.hook";
import { TextTranslate } from "@components/Shared/TextTranslate";
import { faCheckCircle, faClose } from "@fortawesome/free-solid-svg-icons";
import { PriceHistoryPopover } from "@components/DataDisplay/PriceHistoryPopover";
import { TimerStamp } from "@components/Shared/TimerStamp";
import classes from "./OverviewTab.module.css";
import { ItemName } from "@components/DataDisplay/ItemName";
import { RivenGrade } from "../../../../DataDisplay/RivenGrade";

interface Properties {
  image: string;
  disposition_rank: number;
  [key: string]: any;
}

export type OverviewTabProps = {
  value: TauriTypes.StockRiven<Properties> | TauriTypes.WishListItem<Properties> | undefined;
};

export function OverviewTab({ value }: OverviewTabProps) {
  // Translate general
  const useTranslateTab = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateModals(`riven_details.tabs.overview.${key}`, { ...context }, i18Key);
  const GetCellValue = (
    cells: {
      i18nKey: string;
      hidden?: boolean;
      value?: Record<string, string | number>;
      components?: Record<string, React.ReactNode>;
      attributes?: { [key: string]: any };
    }[],
  ) => {
    return (
      <Table.Tr>
        {cells
          .filter((cell) => !cell.hidden)
          .map((cell, index) => (
            <Table.Td key={index}>
              <TextTranslate
                size="md"
                color="gray.5"
                i18nKey={useTranslateTab(cell.i18nKey, undefined, true)}
                values={cell.value || {}}
                components={cell.components}
              />
            </Table.Td>
          ))}
      </Table.Tr>
    );
  };

  if (!value) return <></>;

  const GetProperty = (key: string) => {
    return value[key as keyof typeof value] ?? value.properties?.[key];
  };

  return (
    <Group align="flex-start">
      <Group gap="xs" style={{ flex: 1 }} align="flex-start">
        <Box
          className={classes.thumbnail_wrapper}
          style={{
            backgroundColor: alpha("var(--mantine-color-dark-9)", 0.5),
          }}
        >
          <Box className={classes.thumbnail_inner}>
            <Image className={classes.thumbnail_image} src={WFMThumbnail(GetProperty("image"))} />
          </Box>
        </Box>

        <Flex direction="column">
          <Group gap={3}>
            <PriceHistoryPopover histories={value.price_history} status={value.status} size="2x" />
            <ItemName
              hideQuantity
              value={value}
              color="white"
              fw={"var(--mantine-h2-font-weight)"}
              fz={"var(--mantine-h2-font-size)"}
              lh={"var(--mantine-h2-line-height)"}
            />
          </Group>
          <Table withRowBorders={false} withColumnBorders={false} striped={false} verticalSpacing={0}>
            <Table.Tbody>
              {GetCellValue([
                {
                  i18nKey: "labels.created_at",
                  value: {},
                  hidden: GetProperty("created_at") === undefined && GetProperty("created") === undefined,
                  components: { date: <TimerStamp date={new Date(GetProperty("created_at") ?? GetProperty("created"))} /> },
                },
                {
                  i18nKey: "labels.updated_at",
                  value: {},
                  hidden: GetProperty("updated_at") === undefined && GetProperty("updated") === undefined,
                  components: { date: <TimerStamp date={new Date(GetProperty("updated_at") ?? GetProperty("updated"))} /> },
                },
              ])}
              {GetCellValue([
                {
                  i18nKey: "labels.list_price",
                  value: { price: GetProperty("list_price") ?? GetProperty("starting_price") },
                },
                {
                  i18nKey: "labels.bought",
                  hidden: GetProperty("bought") === undefined,
                  value: { price: GetProperty("bought") },
                },
                {
                  i18nKey: "labels.potential_profit",
                  hidden: GetProperty("potential_profit") === undefined,
                  value: { price: GetProperty("potential_profit") ?? "N/A" },
                },
              ])}
              {GetCellValue([
                {
                  i18nKey: "labels.minimum_profit",
                  hidden: GetProperty("minimum_profit") === undefined,
                  value: { price: GetProperty("minimum_profit") ?? "N/A" },
                },
                {
                  i18nKey: "labels.minimum_sma",
                  hidden: GetProperty("minimum_sma") === undefined,
                  value: { price: GetProperty("minimum_sma") ?? "N/A" },
                },
              ])}
              {GetCellValue([
                {
                  i18nKey: "labels.kuva",
                  hidden: GetProperty("kuva") === undefined,
                  value: { kuva: GetProperty("kuva") ?? "N/A" },
                },
                {
                  i18nKey: "labels.endo",
                  hidden: GetProperty("endo") === undefined,
                  value: { endo: GetProperty("endo") ?? "N/A" },
                },
              ])}
              {GetCellValue([
                {
                  i18nKey: "labels.quantity",
                  hidden: GetProperty("quantity") === undefined && GetProperty("owned") === undefined,
                  value: { quantity: GetProperty("owned") ?? GetProperty("quantity") ?? 0 },
                },
                {
                  i18nKey: "labels.is_hidden",
                  hidden: GetProperty("is_hidden") === undefined && GetProperty("visible") === undefined,
                  components: {
                    hidden: (
                      <FontAwesomeIcon
                        icon={(GetProperty("is_hidden") ?? GetProperty("visible")) ? faCheckCircle : faClose}
                        color={(GetProperty("is_hidden") ?? !GetProperty("visible")) ? "green" : "red"}
                      />
                    ),
                  },
                },
              ])}
            </Table.Tbody>
          </Table>
        </Flex>
      </Group>
      {GetProperty("grade") !== undefined && (
        <Group gap="md" style={{ flex: "0 0 auto" }}>
          <RivenGrade value={GetProperty("grade") ?? "unknown"} size={64} />
        </Group>
      )}
    </Group>
  );
}
