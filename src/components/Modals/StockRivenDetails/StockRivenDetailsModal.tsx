import {
  Text,
  Group,
  Select,
  Stack,
  Title,
  Box,
  Flex,
  Image,
  Rating,
  Table,
  NumberFormatter,
  Card,
  Badge,
  alpha,
  SimpleGrid,
  ScrollArea,
  Grid,
  Divider,
  Popover,
} from "@mantine/core";
import { useQuery } from "@tanstack/react-query";
import api, { WFMThumbnail } from "@api/index";
import { Loading } from "@components/Shared/Loading";
import { upperFirst, useDisclosure } from "@mantine/hooks";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faCircle, faGlobe } from "@fortawesome/free-solid-svg-icons";
import { PriceHistoryListItem } from "../../DataDisplay/PriceHistoryListItem";
import { RivenAttribute as RivenAttributeCon } from "../../DataDisplay/RivenAttribute";
import { WFMAuction } from "../../DataDisplay/WFMAuction";
import { DisplayPlatinum } from "../../DataDisplay/DisplayPlatinum";
import { ActionWithTooltip } from "../../Shared/ActionWithTooltip";
import { TransactionListItem } from "../../DataDisplay/TransactionListItem";
import { StatsWithSegments } from "../../Shared/StatsWithSegments";
import { faWarframeMarket } from "@icons";
import { LocalizedDynamicMessage } from "../../Shared/LocalizedDynamicMessage";
import { RivenAttribute, TauriTypes } from "../../../types";
import { getPolarityIcon } from "@icons";
import { useState, useEffect } from "react";
import { useTranslateEnums, useTranslateModals } from "@hooks/useTranslate.hook";

const colors = {
  mandatory: alpha("var(--mantine-color-grape-7)", 0.55),
  optional: alpha("var(--mantine-color-dark-4)", 0.5),
  matchText: alpha("var(--mantine-color-blue-2)", 0.85),
};

const RenderAttribute = (key: React.Key, label: string, match: boolean) => {
  return (
    <LocalizedDynamicMessage
      key={key}
      textProps={{
        fw: 700,
        ["data-match"]: match,
        c: match ? colors.matchText : "inherit",
      }}
      tokens={[
        {
          pattern: /<([A-Z0-9_]+)>/,
          render: (m) => <Image src={`/damageTypes/${m[1]}.png`} h={16} w="auto" fit="contain" mr={2} />,
        },
      ]}
      message={label}
    />
  );
};
const grades: Record<string, React.ReactNode> = {
  perfect: <Image src="/grades/gradePerfect.png" h={64} w="auto" fit="contain" />,
  good: <Image src="/grades/gradeGreen.png" h={64} w="auto" fit="contain" />,
  has_potential: <Image src="/grades/gradeYellow.png" h={64} w="auto" fit="contain" />,
  bad: <Image src="/grades/gradeRed.png" h={64} w="auto" fit="contain" />,
  unknown: <Image src="/question.png" h={64} w="auto" fit="contain" />,
};

export type StockRivenDetailsModalProps = {
  value: number;
};

export function StockRivenDetailsModal({ value }: StockRivenDetailsModalProps) {
  const [opened, { close, open }] = useDisclosure(false);
  const { data, isLoading } = useQuery({
    queryKey: ["stock_riven", value],
    queryFn: () => api.stock_riven.getById(value, "summary"),
  });
  const [selectedWeapon, setSelectedWeapon] = useState<TauriTypes.StatWithWeapon | null>(null);
  const [rank, setRank] = useState<number>(0);

  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateModals(`stock_riven_details.${key}`, { ...context }, i18Key);

  // Translate general

  // Set initial selectedWeapon when data loads
  useEffect(() => {
    if (data?.stat_with_weapons?.[0]) {
      setSelectedWeapon(data.stat_with_weapons[0]);
    }
  }, [data]);

  if (isLoading || !data)
    return (
      <Box p={"lg"} h={"50vh"}>
        <Loading text="Loading..." />
      </Box>
    );
  const GetCellValue = (cells: { label: React.ReactNode; value: React.ReactNode }[]) => {
    return (
      <Table.Tr>
        {cells.map((cell, index) => (
          <Table.Td key={index}>
            <Text>
              {cell.label} <strong>{cell.value}</strong>
            </Text>
          </Table.Td>
        ))}
      </Table.Tr>
    );
  };

  return (
    <Box p={"md"}>
      <Group align="flex-start">
        <Group gap="xs" style={{ flex: 1 }} align="flex-start">
          <Box
            style={{
              width: 120,
              height: 120,
              backgroundImage: `url("${WFMThumbnail(data.image)}")`,
              borderRadius: "100px",
              // border: "3px solid #252859",
              border: `3px solid ${alpha("var(--mantine-color-dark-4)", 0.3)}`,
              backgroundColor: alpha("var(--mantine-color-dark-9)", 0.5),
              backgroundPosition: "center",
              backgroundSize: "80%",
              backgroundRepeat: "no-repeat",
            }}
          />
          <Flex direction="column">
            <Title order={2}>
              <Popover position="bottom" opened={opened}>
                <Popover.Target>
                  <FontAwesomeIcon
                    onMouseEnter={open}
                    onMouseLeave={close}
                    size={"sm"}
                    icon={faWarframeMarket}
                    data-stock-status={data.stock_status || "live"}
                    data-color-mode="text"
                    style={{ marginRight: "3px" }}
                  />
                </Popover.Target>
                <Popover.Dropdown style={{ pointerEvents: "none" }}>
                  <Group justify="space-between" mb="sm">
                    <Title order={4} mb="sm">
                      {useTranslate("labels.price_history")}
                    </Title>
                    <Title order={4} mb="sm" data-stock-status={data.stock_status || "live"} data-color-mode="text">
                      {useTranslateEnums(`stock_status.${data.stock_status || "live"}`)}
                    </Title>
                  </Group>
                  {data.price_history
                    .sort((a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime())
                    .slice(0, 5)
                    .map((price, index) => (
                      <PriceHistoryListItem key={index} history={price} />
                    ))}
                </Popover.Dropdown>
              </Popover>
              {data.weapon_name} {upperFirst(data.mod_name)}
            </Title>
            <Rating
              value={selectedWeapon?.disposition_rank || 0}
              emptySymbol={<FontAwesomeIcon icon={faCircle} color="gray" />}
              fullSymbol={<FontAwesomeIcon icon={faCircle} color="#a802b1" />}
              readOnly
            />
            <Table mt="sm" withRowBorders={false} withColumnBorders={false} striped={false} verticalSpacing={0}>
              <Table.Tbody>
                {GetCellValue([
                  { label: useTranslate("labels.rank"), value: <strong>{data.rank}/8</strong> },
                  { label: useTranslate("labels.rerolls"), value: <strong>{data.rerolls}</strong> },
                ])}
                {GetCellValue([
                  {
                    label: useTranslate("labels.drain"),
                    value: (
                      <>
                        {"10"}
                        <FontAwesomeIcon icon={getPolarityIcon(data.polarity)} />
                      </>
                    ),
                  },
                  {
                    label: useTranslate("labels.mastery_rank"),
                    value: data.mastery_rank,
                  },
                ])}
                {GetCellValue([
                  {
                    label: useTranslate("labels.total_endo"),
                    value: <NumberFormatter value={data.endo} thousandsGroupStyle="thousand" thousandSeparator="," />,
                  },
                  {
                    label: useTranslate("labels.spent_kuva"),
                    value: <NumberFormatter value={data.kuva} thousandsGroupStyle="thousand" thousandSeparator="," />,
                  },
                ])}
                {GetCellValue([
                  { label: useTranslate("labels.bought_price"), value: data.financial_summary.bought_price },
                  {
                    label: useTranslate("labels.potential_profit"),
                    value: (
                      <NumberFormatter
                        data-color-mode="text"
                        data-positive={data.financial_summary.potential_profit > 0 ? "positive" : "negative"}
                        value={data.financial_summary.potential_profit || 0}
                        color="blue"
                        thousandsGroupStyle="thousand"
                        thousandSeparator=","
                        style={{
                          color: data.financial_summary.potential_profit > 0 ? "var(--qf-positive-color)" : "var(--qf-negative-color)",
                        }}
                      />
                    ),
                  },
                ])}
              </Table.Tbody>
            </Table>
          </Flex>
        </Group>
        <Group gap="md" style={{ flex: "0 0 auto" }}>
          {grades[data.grade || "unknown"]}
        </Group>
      </Group>
      <Divider mt={"md"} />
      <Group justify="space-between" mb="md">
        <Text size="lg" fw={600}>
          {useTranslate("titles.modifiers")}
        </Text>
        <Group gap="md">
          <Group gap="xs">
            <Select
              label={useTranslate("labels.weapon")}
              size="xs"
              w={120}
              data={data.stat_with_weapons.map((weapon) => weapon.name)}
              value={selectedWeapon?.name || ""}
              onChange={(val) => {
                const selected = data.stat_with_weapons.find((weapon) => weapon.name === val);
                if (selected) setSelectedWeapon(selected);
              }}
              variant="default"
            />
          </Group>
          <Group gap="xs">
            <Select
              label={useTranslate("labels.rank")}
              size="xs"
              w={80}
              data={["0", "1", "2", "3", "4", "5", "6", "7", "8"]}
              value={rank.toString()}
              onChange={(val) => {
                if (!val) return;
                setRank(parseInt(val, 10));
              }}
              variant="default"
            />
          </Group>
        </Group>
      </Group>

      <Stack gap="xs">
        {selectedWeapon?.by_level[rank].map((modifier: RivenAttribute, index: number) => (
          <RivenAttributeCon key={index} value={modifier} hideDetails={false} />
        ))}
      </Stack>
      <Divider mt={"md"} />
      <Box mt="lg">
        <Group justify="space-between" align="flex-start" mb="md">
          <Text size="lg" fw={600}>
            {useTranslate("titles.similar_auctions")}
          </Text>
        </Group>
        <ScrollArea.Autosize mah={"60vh"} style={{ width: "100%" }} scrollbarSize={3}>
          <SimpleGrid cols={{ base: 4, xl: 5 }} spacing="md">
            {data.similarly_auctions.map((auction) => (
              <WFMAuction
                hideFooter
                key={auction.id}
                auction={auction}
                header={
                  <Group justify="space-between" align="center">
                    <Text>{(auction.item.similarity.score * 100).toFixed(0) || 0}%</Text>
                    <DisplayPlatinum value={auction.starting_price} />
                  </Group>
                }
                overlayFooter={
                  <Group gap={"xs"} p={3} justify="space-between">
                    <ActionWithTooltip
                      icon={faGlobe}
                      onClick={(e) => {
                        e.stopPropagation();
                        // open(`https://warframe.market/auction/${auction.id}`);
                      }}
                    />
                  </Group>
                }
              />
            ))}
          </SimpleGrid>
        </ScrollArea.Autosize>
      </Box>
      <Divider mt={"md"} />
      <Box mt="lg">
        <Group justify="space-between" align="flex-start" mb="md">
          <Text size="lg" fw={600}>
            {useTranslate("labels.best_attributes")}
          </Text>
          <Group gap="xs">
            <Badge size="lg" color={colors.mandatory} c={"white"} variant="filled" tt="none">
              {useTranslate("labels.mandatory")}
            </Badge>
            <Badge size="lg" color={colors.optional} c={"white"} variant="filled" tt="none">
              {useTranslate("labels.optional")}
            </Badge>
          </Group>
        </Group>
        <Group justify="space-between" bg={"dark.8"}>
          <Flex direction="row" gap="md" justify="center" align="center" flex={1}>
            {data.roll_evaluation?.valid_rolls.map((rollSet) => (
              <Box>
                <Flex
                  direction="column"
                  align="center"
                  gap={0}
                  bg={colors.mandatory}
                  p={"sm"}
                  style={{ borderRadius: "5px" }}
                  display={rollSet.required.length == 0 ? "none" : ""}
                >
                  {rollSet.required.map((attr, idx) => RenderAttribute(idx, attr.label, attr.matches))}
                </Flex>
                <Flex
                  direction="column"
                  align="center"
                  gap={0}
                  bg={colors.optional}
                  p={"sm"}
                  style={{ borderRadius: "5px" }}
                  mt={"sm"}
                  display={rollSet.optional.length == 0 ? "none" : ""}
                >
                  {rollSet.optional.map((attr, idx) => RenderAttribute(idx, attr.label, attr.matches))}
                </Flex>
              </Box>
            ))}
          </Flex>
          <Card radius="md" bg={alpha("var(--mantine-color-red-9)", 0.35)}>
            <Text fw={700} mb="sm">
              {useTranslate("labels.best_bad_attributes")}
            </Text>
            <Flex align="center" direction="column" gap={0} bg={alpha("var(--mantine-color-red-9)", 0.45)} p={"sm"} style={{ borderRadius: "5px" }}>
              {data.roll_evaluation?.tolerated_negative_attributes.map((attr, idx) => RenderAttribute(idx, attr.label, attr.matches))}
            </Flex>
          </Card>
        </Group>
      </Box>
      <Divider mt={"md"} />
      <Grid mt="lg">
        <Grid.Col span={3.5}>
          <Title order={3} mb="md">
            {useTranslate("titles.last_transactions")}
          </Title>
          <ScrollArea>
            {data.financial_summary.last_transactions.map((transaction, index) => (
              <TransactionListItem key={index} transaction={transaction} orientation="vertical" />
            ))}
          </ScrollArea>
        </Grid.Col>
        <Grid.Col span={8.5}>
          <Title order={3} mb="md">
            {useTranslate("titles.financial_summary")}
          </Title>
          <StatsWithSegments
            p={0}
            orientation="vertical"
            h={330}
            segments={[
              {
                label: useTranslate("labels.total_profit"),
                count: data.financial_summary.total_profit,
                color: "teal",
                hideInProgress: true,
                tooltip: useTranslate("tooltips.roi"),
                part: data.financial_summary.roi,
                suffix: " $",
                decimalScale: 2,
              },
              { label: useTranslate("labels.expenses"), count: data.financial_summary.expenses, color: "red" },
              { label: useTranslate("labels.revenue"), count: data.financial_summary.revenue, color: "green" },
            ]}
            showPercent
            percentSymbol="%"
            footer={
              <Stack>
                <StatsWithSegments
                  p={0}
                  hidePercentBar
                  segments={[
                    {
                      label: useTranslate("labels.total_transactions"),
                      count: data.financial_summary.total_transactions,
                      color: "orange",
                      tooltip: useTranslate("tooltips.average_transaction_value"),
                      part: data.financial_summary.average_transaction,
                      decimalScale: 2,
                    },
                    {
                      label: useTranslate("labels.profit_margin"),
                      count: Math.round(data.financial_summary.profit_margin * 100) / 100,
                      color: data.financial_summary.profit_margin >= 0 ? "var(--qf-positive-color)" : "var(--qf-negative-color)",
                      tooltip: useTranslate("tooltips.average_profit_per_transaction"),
                      part: data.financial_summary.average_profit,
                      suffix: " P",
                      decimalScale: 2,
                    },
                    {
                      label: useTranslate("labels.sales_count"),
                      count: data.financial_summary.sale_count,
                      color: "green",
                      tooltip: useTranslate("tooltips.average_revenue_per_transaction"),
                      part: data.financial_summary.average_revenue,
                      suffix: " P",
                      decimalScale: 2,
                    },
                    {
                      label: useTranslate("labels.purchases_count"),
                      count: data.financial_summary.purchases_count,
                      color: "red",
                      tooltip: useTranslate("tooltips.average_expense_per_transaction"),
                      part: data.financial_summary.average_expense,
                      suffix: " P",
                      decimalScale: 2,
                    },
                  ]}
                  showPercent
                />
                <StatsWithSegments
                  p={0}
                  hidePercentBar
                  segments={[
                    {
                      label: useTranslate("labels.highest_revenue"),
                      count: data.financial_summary.highest_revenue,
                      color: "green",
                      tooltip: useTranslate("tooltips.lowest_revenue"),
                      part: data.financial_summary.lowest_revenue,
                      decimalScale: 2,
                    },
                    {
                      label: useTranslate("labels.highest_expense"),
                      count: data.financial_summary.highest_expense,
                      color: "red",
                      tooltip: useTranslate("tooltips.lowest_expense"),
                      part: data.financial_summary.lowest_expense,
                      decimalScale: 2,
                    },
                  ]}
                  showPercent
                />
              </Stack>
            }
          />
        </Grid.Col>
      </Grid>
    </Box>
  );
}
