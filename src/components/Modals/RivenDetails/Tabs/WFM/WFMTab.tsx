import { Box, ScrollArea, SimpleGrid, Text } from "@mantine/core";
import { TauriTypes, WFMarketTypes } from "$types";
import { PreviewCard } from "@components/Shared/PreviewCard/PreviewCard";
import { ActionWithTooltip } from "@components/Shared/ActionWithTooltip";
import { RivenAttribute } from "@components/DataDisplay/RivenAttribute";
import { useTranslateModals } from "@hooks/useTranslate.hook";
import { open } from "@tauri-apps/plugin-shell";
import { faGlobe } from "@fortawesome/free-solid-svg-icons";
export type WFMTabProps = {
  value: TauriTypes.StockItem<{
    auctions?: WFMarketTypes.Auction[];
    [key: string]: any;
  }>;
};

export function WFMTab({ value }: WFMTabProps) {
  // Translate general
  const useTranslateTab = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateModals(`riven_details.tabs.wfm.${key}`, { ...context }, i18Key);

  return (
    <Box>
      <Text size="lg" fw={600}>
        {useTranslateTab("title")}
      </Text>
      <ScrollArea mt={"md"} h={"35vh"}>
        <SimpleGrid cols={{ base: 4 }} spacing="sm">
          {value.properties.auctions?.map((riven, i) => (
            <PreviewCard
              key={i}
              pos={"relative"}
              value={riven}
              headerLeft={{
                fz: "lg",
                i18nKey: useTranslateTab("labels.auction_header_left", undefined, true),
                values: {
                  similarity: ((riven.item.similarity.score || 0) * 100).toFixed(0),
                },
              }}
              headerRight={{
                fz: "lg",
                i18nKey: useTranslateTab("labels.auction_header_right", undefined, true),
                values: {
                  price: riven.starting_price || 0,
                },
              }}
              footerShowOnHover
              footerRight={
                <ActionWithTooltip
                  icon={faGlobe}
                  tooltip={useTranslateTab("buttons.view_on_wfm")}
                  onClick={() => open(`https://warframe.market/auction/${riven.id}`)}
                />
              }
              renderBody={() =>
                riven.item.attributes.map((attr) => (
                  <RivenAttribute
                    key={attr.url_name}
                    i18nKey="full"
                    groupProps={{ p: 1 }}
                    value={attr}
                    hideDetails
                    centered
                    hideGrade
                    textDecoration={
                      [...(riven.item.similarity.missing ?? []), ...(riven.item.similarity.extra ?? [])].includes(`${attr.url_name}:${attr.positive}`)
                        ? "line-through"
                        : undefined
                    }
                  />
                ))
              }
            />
          ))}
        </SimpleGrid>
      </ScrollArea>
    </Box>
  );
}
