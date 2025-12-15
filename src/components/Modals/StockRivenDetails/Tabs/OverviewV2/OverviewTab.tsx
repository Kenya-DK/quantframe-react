import { Text, Group, Select, Stack, Title, Box, Flex, Image, Rating, Table, NumberFormatter, Card, Badge } from "@mantine/core";
import { TauriTypes } from "$types";
import { RivenAttribute as RivenAttributeCom } from "@components/DataDisplay/RivenAttribute";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import faPolarityMadurai from "../../../../../icons/faPolarityMadurai";
import faPolarityNaramon from "../../../../../icons/faPolarityNaramon";
import faPolarityVazarin from "../../../../../icons/faPolarityVazarin";
import { faCircle } from "@fortawesome/free-solid-svg-icons";
import { useForm } from "@mantine/form";
import api, { WFMThumbnail } from "@api/index";
import { useQuery } from "@tanstack/react-query";
import { useEffect, useState } from "react";
import { upperFirst } from "@mantine/hooks";

export type OverviewTabProps = {
  value: TauriTypes.StockRivenDetails | undefined;
};
const icons: Record<string, React.ReactNode> = {
  madurai: <FontAwesomeIcon icon={faPolarityMadurai} />,
  naramon: <FontAwesomeIcon icon={faPolarityNaramon} />,
  vazarin: <FontAwesomeIcon icon={faPolarityVazarin} />,
};
const grades: Record<string, React.ReactNode> = {
  perfect: <Image src="/grades/gradePerfect.png" h={64} w="auto" fit="contain" />,
  good: <Image src="/grades/gradeGreen.png" h={64} w="auto" fit="contain" />,
  has_potential: <Image src="/grades/gradeYellow.png" h={64} w="auto" fit="contain" />,
  bad: <Image src="/grades/gradeRed.png" h={64} w="auto" fit="contain" />,
  unknown: <Image src="/question.png" h={64} w="auto" fit="contain" />,
};
export function OverviewTabV2({ value }: OverviewTabProps) {
  // Translate general

  if (!value) return <></>;

  const form = useForm({
    initialValues: {
      rank: value.riven_summary.rank.toString() || "0",
      selectedWeapon: value.riven_summary.stat_with_weapons[0],
    },
  });
  // Fetches detailed attribute metadata (like unit types) from the cache.
  // This query runs once and its data is cached by React Query.
  const { data: cacheAttributes } = useQuery<TauriTypes.CacheRivenAttribute[]>({
    queryKey: ["cache_riven_attributes"],
    queryFn: () => api.cache.getRivenAttributes(),
    enabled: true, // Always load metadata for consistent formatting.
  });

  const [urlMapper, setUrlMapper] = useState<{ [key: string]: TauriTypes.CacheRivenAttribute }>({});

  useEffect(() => {
    if (!cacheAttributes) return;
    setUrlMapper(() =>
      cacheAttributes.reduce((acc, attr) => {
        acc[attr.uniqueName.split("/").pop() || ""] = attr;
        return acc;
      }, {} as { [key: string]: TauriTypes.CacheRivenAttribute })
    );
  }, [cacheAttributes]);
  const GetCellValue = (cells: { label: React.ReactNode; value: React.ReactNode }[]) => {
    return (
      <Table.Tr>
        {cells.map((cell, index) => (
          <Table.Td key={index}>
            <Text>
              {cell.label} <strong>{cell.value}</strong>
            </Text>
          </Table.Td>
        ))}
      </Table.Tr>
    );
  };

  return (
    <Box p={"md"}>
      <Group mb="md" align="flex-start">
        <Group gap="xs" style={{ flex: 1 }} align="flex-start">
          <Box
            style={{
              width: 120,
              height: 120,
              backgroundImage: `url("${WFMThumbnail(value.riven_summary.image)}")`,
              borderRadius: "100px",
              border: "3px solid #252859",
              backgroundColor: "#2528598f",
              backgroundPosition: "center",
              backgroundSize: "80%",
              backgroundRepeat: "no-repeat",
            }}
          />
          <Flex direction="column">
            <Title order={2}>
              {value.riven_summary.weapon_name} {upperFirst(value.riven_summary.sub_name)}
            </Title>
            <Rating
              value={form.values.selectedWeapon.disposition_rank || 0}
              emptySymbol={<FontAwesomeIcon icon={faCircle} color="gray" />}
              fullSymbol={<FontAwesomeIcon icon={faCircle} color="#a802b1" />}
              readOnly
            />
            <Table mt="sm" withRowBorders={false} withColumnBorders={false} striped={false} verticalSpacing={0}>
              <Table.Tbody>
                {GetCellValue([
                  { label: "Rank:", value: <strong>{value.riven_summary.rank}/8</strong> },
                  { label: "Rerolls:", value: <strong>{value.riven_summary.rerolls}</strong> },
                ])}
                {GetCellValue([
                  {
                    label: "Drain:",
                    value: (
                      <>
                        {"10"}
                        {icons[value.riven_summary.polarity]}
                      </>
                    ),
                  },
                  {
                    label: "Min MR:",
                    value: value.riven_summary.mastery_rank,
                  },
                ])}
                {GetCellValue([
                  { label: "Total Endo:", value: value.riven_summary.endo },
                  {
                    label: "Spand Kuva:",
                    value: <NumberFormatter value={value.riven_summary.kuva} thousandsGroupStyle="thousand" thousandSeparator="," />,
                  },
                ])}
              </Table.Tbody>
            </Table>
          </Flex>
        </Group>
        <Group gap="md" style={{ flex: "0 0 auto" }}>
          {grades[value.riven_summary.grade || "unknown"]}
        </Group>
      </Group>
      <Group justify="space-between" mb="md">
        <Text size="sm" fw={500}>
          Modifiers:
        </Text>
        <Group gap="md">
          <Group gap="xs">
            <Select
              label="Weapon"
              size="xs"
              w={120}
              data={value.riven_summary.stat_with_weapons.map((weapon) => weapon.name)}
              value={form.values.selectedWeapon.name}
              onChange={(val) => {
                const selected = value.riven_summary.stat_with_weapons.find((weapon) => weapon.name === val);
                if (selected) form.setFieldValue("selectedWeapon", selected);
              }}
              variant="default"
            />
          </Group>
          <Group gap="xs">
            <Select
              label="Level"
              size="xs"
              w={80}
              data={["0", "1", "2", "3", "4", "5", "6", "7", "8"]}
              value={form.values.rank.toString()}
              onChange={(val) => {
                if (!val) return;
                form.setFieldValue("rank", val);
              }}
              variant="default"
            />
          </Group>
        </Group>
      </Group>

      <Stack gap="xs">
        {form.values.selectedWeapon.by_level[form.values.rank].map((modifier, index) => (
          <RivenAttributeCom key={index} value={modifier} hideDetails={false} />
        ))}
      </Stack>
      <Box>
        <Group justify="space-between" align="flex-start" mb="md">
          <Text size="lg" fw={600}>
            Best attributes:
          </Text>
          <Group gap="xs">
            <Badge color="grape" variant="filled" tt="uppercase">
              Mandatory
            </Badge>
            <Badge color="dark" variant="outline" tt="uppercase">
              Optional
            </Badge>
          </Group>
        </Group>
        <Group justify="space-between" bg={"gray"}>
          <Flex direction="row" gap="md" justify="center" align="center" flex={1}>
            {value.riven_summary.good_rolls?.good_rolls.map((rollSet) => (
              <Box>
                <Flex
                  direction="column"
                  align="center"
                  gap={0}
                  bg="grape.7"
                  p={"sm"}
                  style={{ borderRadius: "5px" }}
                  display={rollSet.required.length == 0 ? "none" : ""}
                >
                  {rollSet.required.map((attr, idx) => (
                    <Text fw={700} key={idx}>
                      {urlMapper[attr]?.name || attr}
                    </Text>
                  ))}
                </Flex>
                <Flex
                  direction="column"
                  align="center"
                  gap={0}
                  bg="gray.7"
                  p={"sm"}
                  style={{ borderRadius: "5px" }}
                  mt={"sm"}
                  display={rollSet.optional.length == 0 ? "none" : ""}
                >
                  {rollSet.optional.map((attr, idx) => (
                    <Text fw={700} key={idx}>
                      {urlMapper[attr]?.name || attr}
                    </Text>
                  ))}
                </Flex>
              </Box>
            ))}
          </Flex>
          <Card radius="md">
            <Text fw={700} mb="sm">
              Best bad attrs:
            </Text>
            <Flex align="center" direction="column" gap={0} bg="gray.7" p={"sm"} style={{ borderRadius: "5px" }}>
              {value.riven_summary.good_rolls?.negative_attributes.map((attr, idx) => (
                <Text fw={700} key={idx}>
                  {urlMapper[attr]?.name || attr}
                </Text>
              ))}
            </Flex>
          </Card>
        </Group>
      </Box>
    </Box>
  );
}
