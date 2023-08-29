import { Grid } from "@mantine/core";
import { Inventory } from "../../components/inventory";
import { TransactionControl } from "../../components/transactionControl";

export default function LiveTradingPage() {
  return (
    <Grid>
      <Grid.Col md={8}>
        <Inventory />
      </Grid.Col>
      <Grid.Col md={4}>
        <TransactionControl />
      </Grid.Col>
    </Grid>
  );
}
