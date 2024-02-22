import { Text, Card, Divider, Group, ScrollArea, SimpleGrid, Stack, Center } from "@mantine/core";
import { useCacheContext, useStockContextContext } from "../../../contexts";
import { StockRivenDto, Wfm } from "../../../types";
import { useEffect, useState } from "react";


interface RivenAttributesProps {
  auction: Wfm.Auction<Wfm.AuctionOwner>
}
const AttributeText = ({ auction }: RivenAttributesProps) => {
  const { riven_attributes } = useCacheContext();
  const getAttributeType = (url_name: string) => {
    return riven_attributes.find(x => x.url_name === url_name);
  }
  const isMissing = (url_name: string) => {
    return auction.item.extra_attributes.find(x => x.url_name === url_name);
  }
  return (
    <Card>
      <Group position="apart" mr="md" ml="xs">
        <Text size="lg" weight={700}>
          {auction.buyout_price}
        </Text>
        <Text color="blue" weight={700} align="center" size="md">
          {Math.round(auction.item.similarity)}%
        </Text>
      </Group>
      <Divider />
      <Center >
        <Stack spacing={2}>
          {auction.item.attributes.map((attr, i) => {
            return (
              <Text key={i} color={attr.positive ? "green" : "red"} td={isMissing(attr.url_name) ? "line-through" : ""}>
                {getAttributeType(attr.url_name)?.effect} {attr.value}{getAttributeType(attr.url_name)?.units == "percent" ? "%" : ""}
              </Text>
            )
          })}
        </Stack>
      </Center>
    </Card>
  )
}

export const Logging = () => {
  const [stock, setStock] = useState<StockRivenDto | undefined>();
  const { rivens } = useStockContextContext();
  useEffect(() => {
    setStock(rivens.find(x => x.weapon_url == "torid"));
  }, [rivens])
  return (

    <>
      <ScrollArea mt={25} h={"calc(100vh - 243px)"} pr={15} pl={15}>
        <SimpleGrid
          cols={4}
          spacing="lg"
          breakpoints={[
            { maxWidth: '80rem', cols: 3, spacing: 'lg' },
            { maxWidth: '62rem', cols: 3, spacing: 'md' },
            { maxWidth: '48rem', cols: 2, spacing: 'sm' },
            { maxWidth: '36rem', cols: 1, spacing: 'sm' },
          ]}
        >
          {stock?.trades?.map((riven, i) => {
            return <AttributeText key={i} auction={riven} />
          })}
        </SimpleGrid>
      </ScrollArea>
    </>
  );
}