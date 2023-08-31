import { Text, Card, Group, Button, Grid } from "@mantine/core";
import { useMutation } from "@tanstack/react-query";
import api from "@api/index";

export const ResetData = () => {
  const resetDataMutation = useMutation((data: { type: string }) => api.debug.reset_data(data.type), {
    onSuccess: async () => {
      window.location.reload();
    },
    onError: () => {

    },
  })
  const handleImportWarframeAlgoTraderData = async (type: string) => {
    await resetDataMutation.mutateAsync({ type })
  };
  return (
    <Card h={180}>
      <Group position="apart" mb="xs">
        <Text weight={500}>Reset Data</Text>
      </Group>
      <Grid>
        <Grid.Col span={6}>
          <Group>
            <Button variant="light" color="blue" radius="md" onClick={() => handleImportWarframeAlgoTraderData("transactions")}>
              Reset Transactions
            </Button>
            <Button variant="light" color="blue" radius="md" onClick={() => handleImportWarframeAlgoTraderData("inventory")}>
              Reset Inventory
            </Button>
          </Group>
        </Grid.Col>
      </Grid>
    </Card>
  );
}