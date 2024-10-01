import { Card, Container, ScrollArea, Text, SimpleGrid, Group, Divider } from "@mantine/core";
import { useStockContextContext } from "../../contexts/stock.context";

export default function TestPage() {
  const { rivens } = useStockContextContext();
  return (
    <Container size={"100%"}>
      <ScrollArea mt={"md"} h={"calc(100vh - 300px)"}>
        <SimpleGrid cols={{ base: 1, sm: 2, lg: 4 }} spacing="lg">
          {rivens.map((order, i) => (
            <Card key={i} shadow="xs">
              <Card.Section></Card.Section>

              <Group justify="space-between" mt="md" mb="xs">
                <Text fw={500}>
                  {order.weapon_name} {order.mod_name}
                </Text>
              </Group>
              <Divider />
              {order.attributes.map((attribute, i) => (
                <Text key={i} size="sm" c={!attribute.positive ? "red" : "green"}>
                  {attribute.url_name}: {attribute.value}
                </Text>
              ))}
            </Card>
          ))}
        </SimpleGrid>
      </ScrollArea>
    </Container>
  );
}
