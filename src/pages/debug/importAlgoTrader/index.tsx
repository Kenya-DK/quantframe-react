import { Text, Card, Group, Button, TextInput, Select } from "@mantine/core";
import { useLocalStorage } from "@mantine/hooks";
import { useMutation } from "@tanstack/react-query";
import { RustError } from "$types/index";
import { SendNotificationToWindow } from "@utils/index";
import { useTranslateRustError } from "@hooks/index";
import api from "@api/index";

export const ImportAlgoTrader = () => {
  const [dbPath, setDbPath] = useLocalStorage<string>({ key: "dbPath", defaultValue: "" });
  const [type, setType] = useLocalStorage<string>({ key: "type", defaultValue: "" });

  const importWarframeAlgoTraderDataMutation = useMutation((data: { type: string, path: string }) => api.debug.importWarframeAlgoTraderData(data.path, data.type), {
    onSuccess: async () => {
      window.location.reload();
    },
    onError(error: RustError) {
      SendNotificationToWindow(useTranslateRustError("title", { component: error.component }), useTranslateRustError("message", { loc: error.component }));
    }
  })
  const handleImportWarframeAlgoTraderData = async () => {
    await importWarframeAlgoTraderDataMutation.mutateAsync({ path: dbPath, type })
  };
  return (
    <Card h={180}>
      <Group position="apart" mb="xs">
        <Text weight={500}>Warefream Import Algo Trader</Text>
      </Group>
      <Group grow mt="xs">
        <TextInput
          placeholder="Enter your database path"
          value={dbPath}
          onChange={(event) => setDbPath(event.currentTarget.value)}
        />
        <Select
          data={[
            { value: "transactions", label: "Transactions" },
            { value: "inventory", label: "Inventory" },
          ]}
          placeholder="Select type"
          value={type}
          onChange={(value) => {
            if (!value) return;
            setType(value)
          }}
        />
        <Button variant="light" color="blue" radius="md" onClick={() => handleImportWarframeAlgoTraderData()}>
          Import
        </Button>
      </Group>
    </Card>
  );
}