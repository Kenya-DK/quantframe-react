import { Container, ScrollArea, Text, SimpleGrid, Box } from "@mantine/core";
import { useStockContextContext } from "../../contexts/stock.context";

export default function TestPage() {
  const { rivens } = useStockContextContext();
  return (
    <Container size={"100%"}>
      <ScrollArea mt={"md"} h={"calc(100vh - 300px)"}>
        <SimpleGrid cols={{ base: 1, sm: 2, lg: 4 }} spacing="lg">
          {rivens.map((order, i) => (
            <Box
              key={i}
              w={"100%"}
              p={"md"}
              flex={1}
              style={{
                backgroundColor: "#000",
                borderRadius: "15px",
              }}
            >
              <Box mb="xs" p={"3"} bg={"blue.9"}>
                <Text fw={500}>
                  {order.weapon_name} {order.mod_name}
                </Text>
              </Box>
              {order.attributes.map((attribute, i) => (
                <Text key={i} size="sm" c={!attribute.positive ? "red" : "green"}>
                  {attribute.url_name}: {attribute.value}
                </Text>
              ))}
            </Box>
          ))}
        </SimpleGrid>
      </ScrollArea>
    </Container>
  );
}
