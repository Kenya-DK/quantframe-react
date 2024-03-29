import { Text, Card, Divider, Group, ScrollArea, SimpleGrid, Stack, Center, useMantineTheme, Button } from "@mantine/core";
import { useCacheContext, useStockContextContext } from "../../../contexts";
import { StockRivenDto, Wfm } from "../../../types";
import { useEffect, useState } from "react";
import { InfoBox } from "../../../components/InfoBox";
import api from "../../../api";

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
  const theme = useMantineTheme();
  useEffect(() => {
    setStock(rivens.find(x => x.weapon_url == "ceramic_dagger"));
  }, [rivens])
  const getColors = () => {
    let colors: { name: string, color: string }[] = [];
    // Loop through object
    for (const key in theme.colors) {
      let colorIndex = theme.colors[key].length;
      for (let i = 0; i < colorIndex; i++)
        colors.push({
          name: `${key}.${i}`,
          color: theme.colors[key][i]
        });
    }
    return colors;
  }
  return (

    <>
      <ScrollArea mt={25} h={"calc(100vh - 243px)"} pr={15} pl={15}>
        <Button onClick={() => api.debug.test("test_df", {
          iterations: 3
        })}>Test DF</Button>
        <Button onClick={() => api.debug.test("compare_live", {
          item_name: "blaze",
          item_id: "660190daa9630600f6bf1a2e",
          item_rank: 3.0,
          closed_avg: 525.0,
        })}>Compare Live</Button>
        <SimpleGrid
          cols={4}
          spacing="lg"
          breakpoints={[
            { maxWidth: '80rem', cols: 5, spacing: 'lg' },
            { maxWidth: '62rem', cols: 5, spacing: 'md' },
            { maxWidth: '48rem', cols: 5, spacing: 'sm' },
            { maxWidth: '36rem', cols: 1, spacing: 'sm' },
          ]}
        >
          {stock?.trades?.map((riven, i) => {
            return <AttributeText key={i} auction={riven} />
          })}
          <Card>

            <Group position="apart" mr="md" ml="xs">
              {getColors().map((color, i) => <InfoBox key={i} text={color.name} color={color.color} />)}
            </Group>
          </Card>
        </SimpleGrid>
      </ScrollArea>
    </>
  );
}