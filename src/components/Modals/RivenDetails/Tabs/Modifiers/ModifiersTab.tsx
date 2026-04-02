import { Group, Text, Select, Box, Stack } from "@mantine/core";
import { RivenAttribute, TauriTypes } from "$types";
import { RivenAttribute as RivenAttributeCon } from "@components/DataDisplay/RivenAttribute";
import { useTranslateModals } from "@hooks/useTranslate.hook";
import { useEffect, useState } from "react";

interface Variant {
  disposition: number;
  disposition_rank: number;
  name: string;
  ranks: RivenAttribute[][];
}
interface Properties {
  attributes: RivenAttribute[];
  last_transactions?: TauriTypes.TransactionDto[];
  variants?: Variant[];
  [key: string]: any;
}

export type ModifiersTabProps = {
  value: TauriTypes.StockRiven<Properties> | undefined;
};

export function ModifiersTab({ value }: ModifiersTabProps) {
  if (!value) return <></>;
  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateModals(`riven_details.tabs.modifiers.${key}`, { ...context }, i18Key);

  const [selectedWeapon, setSelectedWeapon] = useState<Variant | null>(null);
  const [rank, setRank] = useState<number>(0);

  useEffect(() => {
    if (!value.properties.variants) {
      setSelectedWeapon({
        name: "",
        disposition: 0,
        disposition_rank: 0,
        ranks: [value.properties.attributes],
      });
      setRank(0);
    } else {
      setSelectedWeapon(value.properties.variants[0]);
      setRank(value.sub_type?.rank ?? 0);
    }
  }, [value]);

  return (
    <Box>
      <Group justify="space-between" mb="md">
        <Text size="lg" fw={600}>
          {useTranslate("title")}
        </Text>
        {value.properties.variants && value.properties.variants.length > 0 && (
          <Group gap="md">
            <Group gap="xs">
              <Select
                label={useTranslate("labels.weapon")}
                size="xs"
                w={120}
                data={value.properties.variants?.map((variant) => variant.name) || []}
                value={selectedWeapon?.name || ""}
                onChange={(val) => {
                  const selected = value.properties.variants?.find((variant) => variant.name === val);
                  if (selected) setSelectedWeapon(selected);
                }}
                variant="default"
              />
            </Group>
            <Group gap="xs">
              <Select
                label={useTranslate("labels.rank")}
                size="xs"
                w={80}
                data={
                  selectedWeapon?.ranks.length ? selectedWeapon.ranks.map((_, index) => ({ label: index.toString(), value: index.toString() })) : []
                }
                value={rank.toString()}
                onChange={(val) => {
                  if (!val) return;
                  setRank(parseInt(val, 10));
                }}
                variant="default"
              />
            </Group>
          </Group>
        )}
      </Group>
      <Stack gap="xs">
        {selectedWeapon?.ranks[rank]?.map((modifier: RivenAttribute, index: number) => (
          <RivenAttributeCon key={index} value={modifier} hideDetails={false} />
        ))}
      </Stack>
    </Box>
  );
}
