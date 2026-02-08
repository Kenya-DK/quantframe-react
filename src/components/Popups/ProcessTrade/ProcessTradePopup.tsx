import { Box, Button, Table, Text, NumberInput, Divider, Group } from "@mantine/core";
import dayjs from "dayjs";
import { DataTable } from "mantine-datatable";
import { useEffect, useState } from "react";
import { ActionWithTooltip } from "@components/Shared/ActionWithTooltip";
import { faPlay } from "@fortawesome/free-solid-svg-icons";
import { useForm } from "@mantine/form";
import { ItemName } from "@components/DataDisplay/ItemName";
import { StatsWithSegments } from "@components/Shared/StatsWithSegments";
import { useTranslateCommon, useTranslateEnums, useTranslatePages } from "@hooks/useTranslate.hook";
import { TauriTypes } from "$types";
import { useMutations } from "./mutations";
import { listen, emit } from "@tauri-apps/api/event";

enum TradeProcessingStep {
  View = "view",
  Validate = "validate",
}

export interface PlayerTrade {
  credits: number;
  items: TradeItem[];
  platinum: number;
  playerName: string;
  tradeTime: string;
  type: string;
}

export interface TradeItem {
  price: number;
  quantity: number;
  wfm_url: string;
  sub_type?: TauriTypes.SubType;
}

export interface TradeItemProperties {
  price: number;
  wfm_url: string;
}

export function ProcessTradePopup() {
  // Stats
  const currentTradeForm = useForm({
    initialValues: undefined as PlayerTrade | undefined,
    onValuesChange: (values) => {
      if (!values) return;
      const itemsTotal = values.items.reduce((acc, item) => acc + (item.price || 0) * item.quantity, 0);
      setCalculatedPrice(itemsTotal);
    },
  });
  const [trades, setTrades] = useState<PlayerTrade[]>([]);
  const [currentStep, setCurrentStep] = useState<TradeProcessingStep>(TradeProcessingStep.View);
  const [calculated_price, setCalculatedPrice] = useState<number>(0);

  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`process_trade.${key}`, { ...context }, i18Key);
  const useTranslateDataGridColumns = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`datatable.columns.${key}`, { ...context }, i18Key);
  const useTranslateButton = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`buttons.${key}`, { ...context }, i18Key);
  const useTranslateStats = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslate(`stats.${key}`, { ...context }, i18Key);

  // Mutations
  const { createMutation } = useMutations({ refetchQueries: () => {}, setLoadingRows: () => {} });

  // Handle New Trade
  useEffect(() => {
    listen("add_trade", ({ payload }: { payload: PlayerTrade }) => setTrades((prevTrades) => [...prevTrades, payload]));
    emit("initialize");
  }, []);

  // Helper Methods
  const GetDifferenceColor = (difference: number) => {
    if (difference > 0) return "var(--qf-positive-color)";
    else if (difference < 0) return "var(--qf-negative-color)";
    else return "var(--mantine-color-blue-filled)";
  };

  const CreateItems = async () => {
    let items = currentTradeForm.values?.items.map((item) => ({
      ...item,
      user_name: currentTradeForm.values?.playerName,
      order_type: currentTradeForm.values?.type === "purchase" ? "buy" : "sell",
      operation_set: [`SetDate:${currentTradeForm.values?.tradeTime}`],
    }));
    await createMutation.mutateAsync(items || []);
    setCurrentStep(TradeProcessingStep.View);
    setTrades((prevTrades) => prevTrades.filter((trade) => trade.tradeTime !== currentTradeForm.values?.tradeTime));
  };
  return (
    <Box>
      {currentStep === TradeProcessingStep.View && (
        <DataTable
          height={"100vh"}
          records={trades}
          withColumnBorders
          striped
          idAccessor="tradeTime"
          customRowAttributes={(record: any) => {
            return {
              "data-color-mode": "box-shadow",
              "data-transaction-type": record.type,
            };
          }}
          highlightOnHover
          columns={[
            {
              accessor: "playerName",
              title: useTranslateDataGridColumns("user_name"),
            },
            {
              accessor: "platinum",
              title: useTranslateDataGridColumns("platinum"),
              render: ({ platinum }) => <Text fw={600}>{platinum}</Text>,
            },
            {
              accessor: "tradeTime",
              title: useTranslateDataGridColumns("date"),
              render: ({ tradeTime }) => <Text>{dayjs(tradeTime).format("DD.MM.YYYY HH:mm")}</Text>,
            },
            {
              accessor: "action",
              title: useTranslateCommon("datatable_columns.actions.title"),
              width: "120px",
              render: (row) => (
                <ActionWithTooltip
                  tooltip={useTranslateButton("process_trade_tooltip")}
                  onClick={() => {
                    setCurrentStep(TradeProcessingStep.Validate);
                    currentTradeForm.setValues(row);
                  }}
                  icon={faPlay}
                />
              ),
            },
          ]}
        />
      )}
      {currentStep === TradeProcessingStep.Validate && currentTradeForm.values !== undefined && (
        <Box p={"md"}>
          <StatsWithSegments
            p={0}
            hidePercentBar
            showPercent
            orientation="vertical"
            segments={[
              {
                label: useTranslateStats("user_name"),
                count: currentTradeForm.values.playerName,
                color: "orange",
                tooltip: useTranslateStats("date_of_trade"),
                part: dayjs(currentTradeForm.values.tradeTime).format("DD.MM.YYYY HH:mm"),
              },
              {
                label: useTranslateStats("trade_type"),
                count: useTranslateEnums(`transaction_type.${currentTradeForm.values.type}`),
                color: `var(--qf-transaction-type-${currentTradeForm.values.type})`,
              },
            ]}
            footer={
              <StatsWithSegments
                p={0}
                hidePercentBar
                showPercent
                segments={[
                  {
                    label: useTranslateStats("trade_price"),
                    count: currentTradeForm.values.platinum,
                    color: "var(--mantine-color-blue-filled)",
                  },
                  {
                    label: useTranslateStats("price"),
                    count: calculated_price,
                    color: "var(--mantine-color-blue-filled)",
                  },
                  {
                    label: useTranslateStats("difference"),
                    count: calculated_price - currentTradeForm.values.platinum,
                    color: GetDifferenceColor(calculated_price - currentTradeForm.values.platinum),
                  },
                ]}
              />
            }
          />
          <Divider my="md" />
          <Table>
            <Table.Thead>
              <Table.Tr>
                <Table.Th>{useTranslateDataGridColumns("item_name")}</Table.Th>
                <Table.Th>{useTranslateDataGridColumns("quantity")}</Table.Th>
                <Table.Th>{useTranslateDataGridColumns("price")}</Table.Th>
                <Table.Th>{useTranslateDataGridColumns("total")}</Table.Th>
              </Table.Tr>
            </Table.Thead>
            <Table.Tbody>
              {currentTradeForm.values.items.map((item: any, index: number) => (
                <Table.Tr key={`${item.wfm_url}-${index}`}>
                  <Table.Td>
                    <ItemName value={item} />
                  </Table.Td>
                  <Table.Td>{item.quantity}</Table.Td>
                  <Table.Td>
                    <NumberInput
                      size="xs"
                      value={item.price}
                      onChange={(value) => {
                        currentTradeForm.setFieldValue(`items.${index}.price`, Number(value) || 0);
                      }}
                      min={0}
                      w={100}
                    />
                  </Table.Td>
                  <Table.Td>
                    <Text fw={600}>{item.price * item.quantity}</Text>
                  </Table.Td>
                </Table.Tr>
              ))}
            </Table.Tbody>
          </Table>
        </Box>
      )}
      {currentStep === TradeProcessingStep.Validate && (
        <Group justify="space-between" p={"md"}>
          <Button onClick={() => setCurrentStep(TradeProcessingStep.View)}>{useTranslateButton("back_to_list")}</Button>

          <Button onClick={() => CreateItems()}>{useTranslateButton("confirm_trade")}</Button>
        </Group>
      )}
    </Box>
  );
}
