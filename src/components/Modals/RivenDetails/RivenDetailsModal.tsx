import { Box, Container, Divider } from "@mantine/core";
import { AnalyticsTab, EditTab, OverviewTab, WFMTab, ModifiersTab, RollEvaluationTab } from "./Tabs/index";
import api from "@api/index";
import { useQuery } from "@tanstack/react-query";
import { Loading } from "@components/Shared/Loading";
import { TauriTypes } from "$types";

export enum Operations {
  MarketInfo = "MarketInfo",
  TransactionInfo = "TransactionInfo",
  EditForm = "EditForm",
  VariantInfo = "VariantInfo",
  GradeInfo = "GradeInfo",
  RollEvaluation = "RollEvaluation",
  EndoInfo = "EndoInfo",
  ProfitabilityInfo = "ProfitabilityInfo",
  KuvaInfo = "KuvaInfo",
}

export type RivenDetailsModalProps = {
  lookup: "stock_riven" | "auction";
  operations: Operations[];
  value: number | string;
  onSave?: (item: TauriTypes.UpdateStockItem | TauriTypes.UpdateWishListItem) => void;
};
export function RivenDetailsModal({ lookup, operations, value, onSave }: RivenDetailsModalProps) {
  // Don't cache the result of this query
  const { data: dataStockRiven } = useQuery({
    queryKey: ["stock_riven", value],
    queryFn: () => api.stock_riven.getById<{ ui_operations: string[] }>(value as number, operations),
    enabled: lookup === "stock_riven",
    gcTime: 0,
  });

  const { data: dataAuction } = useQuery({
    queryKey: ["auction", value],
    queryFn: () => api.auction.getById<{ ui_operations: string[] }>(value as string, operations),
    enabled: lookup === "auction",
    gcTime: 0,
  });

  const data = lookup === "stock_riven" ? dataStockRiven : dataAuction;

  if (!data)
    return (
      <Box p={"lg"} h={"50vh"}>
        <Loading text="Loading..." />
      </Box>
    );

  return (
    <Container fluid p={0}>
      <OverviewTab value={data as any} />
      <Divider my="sm" />
      <ModifiersTab value={data as any} />
      <Divider my="md" hidden={!operations.includes(Operations.MarketInfo)} />
      {operations.includes(Operations.MarketInfo) && <WFMTab value={data as any} />}
      <Divider my="md" hidden={!operations.includes(Operations.RollEvaluation)} />
      {operations.includes(Operations.RollEvaluation) && <RollEvaluationTab value={data as any} />}
      <Divider my="md" hidden={!operations.includes(Operations.TransactionInfo)} />
      {operations.includes(Operations.TransactionInfo) && <AnalyticsTab value={data as any} />}
      <Divider my="md" hidden={!operations.includes(Operations.EditForm)} />
      {operations.includes(Operations.EditForm) && <EditTab lookup={lookup} value={data as any} onSave={onSave} />}
    </Container>
  );
}
