import { Grid } from "@mantine/core";
import { Inventory } from "../../components/inventory";
import { TransactionControl } from "../../components/transactionControl";

export default function LiveTradingPage() {
  return (
    <>
      <Grid>
        <Grid.Col md={12}>
          <TransactionControl />
        </Grid.Col>
      </Grid>
      <Grid>
        <Grid.Col md={12}>
          <Inventory />
        </Grid.Col>
      </Grid>
    </>
  );
}
