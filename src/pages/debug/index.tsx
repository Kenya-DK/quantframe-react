import { Button, Group, Tabs } from "@mantine/core";
export default function DebugPage() {
  return (
    <Tabs defaultValue="first">
      <Tabs.List>
        <Tabs.Tab value="reststorecache">Rest Store Cache</Tabs.Tab>
        <Tabs.Tab value="second">Second tab</Tabs.Tab>
      </Tabs.List>

      <Tabs.Panel value="reststorecache">
        <Group position="center">
          <Button onClick={async () => {
            window.location.reload()
          }}>
            Rest Settings Cache
          </Button>
          <Button onClick={async () => {
            window.location.reload()
          }}>
            Rest All Cache
          </Button>
        </Group>
      </Tabs.Panel>
      <Tabs.Panel value="second">Second panel</Tabs.Panel>
    </Tabs>
  );
}
