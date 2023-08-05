import { Grid } from "@mantine/core";
import { Inventory } from "../../components/inventory";
import { TransactionControl } from "../../components/transactionControl";

export default function HomePage() {
  return (
    <Grid>
      <Grid.Col md={6}>
        <Inventory />
      </Grid.Col>
      <Grid.Col md={6}>
        <TransactionControl />
      </Grid.Col>
    </Grid>
  );
}
