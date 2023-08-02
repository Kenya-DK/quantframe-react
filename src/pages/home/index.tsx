import { Grid } from "@mantine/core";
import { Inventory } from "../../components/inventory";

export default function HomePage() {
  return (
    <Grid>
      <Grid.Col md={6}>
        <Inventory />
      </Grid.Col>
      <Grid.Col md={6}>
      </Grid.Col>
    </Grid>
  );
}
