import { Center, Text, Divider, Grid, Group, Select, Stack, TextInput, Title, Box, Flex } from "@mantine/core";
import { TauriTypes } from "$types";
import { useTranslateEnums, useTranslateModals } from "@hooks/useTranslate.hook";
import dayjs from "dayjs";
import { PriceHistoryListItem } from "@components/DataDisplay/PriceHistoryListItem";
import { RivenAttribute } from "@components/DataDisplay/RivenAttribute";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import faPolarityMadurai from "../../../../../icons/faPolarityMadurai";
import faPolarityNaramon from "../../../../../icons/faPolarityNaramon";
import faPolarityVazarin from "../../../../../icons/faPolarityVazarin";

export type OverviewTabProps = {
  value: TauriTypes.StockRivenDetails | undefined;
};
const icons: Record<string, React.ReactNode> = {
  madurai: <FontAwesomeIcon icon={faPolarityMadurai} />,
  naramon: <FontAwesomeIcon icon={faPolarityNaramon} />,
  vazarin: <FontAwesomeIcon icon={faPolarityVazarin} />,
};
export function OverviewTab({ value }: OverviewTabProps) {
  // Translate general
  const useTranslateTab = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateModals(`stock_riven_details.tabs.overview.${key}`, { ...context }, i18Key);
  const useTranslateFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateTab(`fields.${key}`, { ...context }, i18Key);
  const useTranslateStockStatus = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateEnums(`stock_status.${key}`, { ...context }, i18Key);
  if (!value) return <></>;
  return (
    <Box p={"md"}>
      <Group mb="md">
        <Box
          style={{
            width: 120,
            height: 120,
            backgroundImage: `url("https://warframe.market/static/assets/items/images/en/kulstar.92736ca911a3b84f99bc9e50f24369f0.png")`,
            borderRadius: "100px",
            border: "3px solid #252859",
            backgroundColor: "#2528598f",
            backgroundPosition: "center",
            backgroundSize: "80%",
            backgroundRepeat: "no-repeat",
          }}
        />
        <Flex direction="column">
          <Group justify="left">
            <Text w={90}>Rank: {value.stock.sub_type?.rank || "0"}/8</Text>
            <Text>Rerolls: {value.stock.re_rolls || "0"}</Text>
          </Group>
          <Group justify="left">
            <Text w={90}>
              Drain: {"10"} {icons[value.stock.polarity || ""]}
            </Text>
            <Text>Min MR: {value.stock.mastery_rank || "0"}</Text>
          </Group>
        </Flex>
        {/* <Group justify="left" grow>
          <Text>Rank: {value.stock.sub_type?.rank || "0"}</Text>
          <Text>Rank: {value.stock.sub_type?.rank || "0"}</Text>
        </Group>
        <Text size="sm" fw={500}>
          Rerolls: {value.stock.re_rolls || "0"}
        </Text>
        <Text size="sm" fw={500}>
          Min MR: {value.stock.mastery_rank || "0"}
        </Text>
        <Text size="sm" fw={500}>
          Drain: {"10"}
        </Text> */}
        <Group gap="xs">asds</Group>
      </Group>
      <Group justify="space-between" mb="md">
        <Text size="sm" fw={500}>
          Modifiers:
        </Text>
        <Group gap="md">
          <Group gap="xs">
            <Select label="Weapon" size="xs" w={120} data={["Exergis", "Weapon 2", "Weapon 3"]} defaultValue="Exergis" variant="default" />
          </Group>
          <Group gap="xs">
            <Select label="Level" size="xs" w={80} data={["0", "1", "2", "3", "4", "5", "6", "7", "8"]} defaultValue="0" variant="default" />
          </Group>
        </Group>
      </Group>

      <Stack gap="xs">
        {value.stock.attributes.map((modifier, index) => (
          <RivenAttribute key={index} value={{ ...modifier, letterGrade: "A", minValue: -200, maxValue: 500 }} hideDetails={false} />
        ))}
      </Stack>
      <Grid>
        <Grid.Col span={6}>
          <Group grow>
            <TextInput label={useTranslateFields("created_at")} value={dayjs(value.stock.created_at).format("DD/MM/YYYY HH:mm:ss")} readOnly />
            <TextInput label={useTranslateFields("updated_at")} value={dayjs(value.stock.updated_at).format("DD/MM/YYYY HH:mm:ss")} readOnly />
          </Group>
          <Group grow>
            <TextInput
              label={useTranslateFields("status")}
              data-stock-status={value.stock.status}
              data-color-mode="text"
              value={useTranslateStockStatus(value.stock.status)}
              readOnly
            />
            <TextInput label={useTranslateFields("minimum_price")} value={value.stock.minimum_price || "N/A"} readOnly />
          </Group>
          <Group grow>
            <TextInput label={useTranslateFields("bought")} value={value.stock.bought} readOnly />
            <TextInput label={useTranslateFields("list_price")} value={value.stock.list_price || "N/A"} readOnly />
            <TextInput label={useTranslateFields("profit")} value={value.stock_profit || "N/A"} readOnly />
          </Group>
          <Divider mt={"md"} />
          <Group grow>
            {/* <TextInput label={useTranslateFields("highest_price")} value={value.order_info?.properties?.highest_price || "N/A"} readOnly />
          <TextInput label={useTranslateFields("lowest_price")} value={value.order_info?.properties?.lowest_price || "N/A"} readOnly /> */}
          </Group>
        </Grid.Col>
        <Grid.Col span={6}>
          <Title order={3}>{useTranslateFields("listed")}</Title>
          {value.stock.price_history.length <= 0 && (
            <Center h={"90%"}>
              <Title order={3}>{useTranslateFields("no_listed")}</Title>
            </Center>
          )}
          {value.stock.price_history.length > 0 &&
            value.stock.price_history
              .sort((a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime())
              .slice(0, 5)
              .map((price, index) => <PriceHistoryListItem key={index} history={price} />)}
        </Grid.Col>
      </Grid>
    </Box>
  );
}
