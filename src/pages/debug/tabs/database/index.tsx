import { SimpleGrid } from "@mantine/core";
import { RestCard } from "./cards/Rest";
import { MigrateCard } from "./cards/Migrate";
import { ImportATraderCard } from "./cards/ImportAlgoTrader";

interface DataBasePanelProps {
}
export const DataBasePanel = ({ }: DataBasePanelProps) => {
  return (
    <SimpleGrid cols={3} p={15}>
      <RestCard />
      <MigrateCard />
      <ImportATraderCard />
    </SimpleGrid>
  );
};