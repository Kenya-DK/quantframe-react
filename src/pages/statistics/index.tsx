import { Grid } from "@mantine/core";
import { RivenForm } from "../../components/forms/riven.form";
import { Wfm } from "../../types";

export default function StatisticsPage() {
  return (
    <Grid>
      <Grid.Col md={12}>
        <RivenForm onSubmit={function (): void {
          throw new Error("Function not implemented.");
        }} />
      </Grid.Col>
    </Grid>
  );
}
