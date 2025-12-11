import { Text, Group, Select, Stack, Title, Box, Flex, Image, Rating, Table } from "@mantine/core";
import { TauriTypes } from "$types";
import { RivenAttribute as RivenAttributeCom } from "@components/DataDisplay/RivenAttribute";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import faPolarityMadurai from "../../../../../icons/faPolarityMadurai";
import faPolarityNaramon from "../../../../../icons/faPolarityNaramon";
import faPolarityVazarin from "../../../../../icons/faPolarityVazarin";
import { faCircle } from "@fortawesome/free-solid-svg-icons";
import { useForm } from "@mantine/form";

export type OverviewTabProps = {
  value: TauriTypes.StockRivenDetails | undefined;
};
const icons: Record<string, React.ReactNode> = {
  madurai: <FontAwesomeIcon icon={faPolarityMadurai} />,
  naramon: <FontAwesomeIcon icon={faPolarityNaramon} />,
  vazarin: <FontAwesomeIcon icon={faPolarityVazarin} />,
};
export function OverviewTab({ value }: OverviewTabProps) {
  // Translate general

  if (!value) return <></>;

  const form = useForm({
    initialValues: {
      rank: value.riven_summary.rank.toString() || "0",
      selectedWeapon: value.riven_summary.stat_with_weapons[0],
    },
  });

  return (
    <Box p={"md"}>
      <Group mb="md" align="flex-start">
        <Group gap="xs" style={{ flex: 1 }} align="flex-start">
          <Box
            style={{
              width: 120,
              height: 120,
              backgroundImage: `url("https://warframe.market/static/assets/items/images/en/kulstar.92736ca911a3b84f99bc9e50f24369f0.png")`,
              borderRadius: "100px",
              border: "3px solid #252859",
              backgroundColor: "#2528598f",
              backgroundPosition: "center",
              backgroundSize: "80%",
              backgroundRepeat: "no-repeat",
            }}
          />
          <Flex direction="column">
            <Title order={2}>{value.stock.weapon_name || "Unknown Weapon"} </Title>
            <Rating
              value={4}
              emptySymbol={<FontAwesomeIcon icon={faCircle} color="gray" />}
              fullSymbol={<FontAwesomeIcon icon={faCircle} color="#a802b1" />}
              readOnly
            />
            <Table mt="sm">
              <tbody>
                <tr>
                  <td>
                    <Text>
                      Rank: <strong>{value.riven_summary.rank}/8</strong>
                    </Text>
                  </td>
                  <td>
                    <Text>
                      Rerolls: <strong>{value.riven_summary.rerolls}</strong>
                    </Text>
                  </td>
                </tr>
                <tr>
                  <td>
                    <Text>
                      Drain: <strong>{"10"}</strong> {icons[value.riven_summary.polarity]}
                    </Text>
                  </td>
                  <td>
                    <Text>
                      Min MR: <strong>{value.riven_summary.mastery_rank}</strong>
                    </Text>
                  </td>
                </tr>
                <tr>
                  <td>
                    <Text>
                      Total Endo: <strong>{value.riven_summary.endo}</strong>
                    </Text>
                  </td>
                  <td>
                    <Text>
                      Spand Kuvan: <strong>{value.riven_summary.kuva}</strong>
                    </Text>
                  </td>
                </tr>
              </tbody>
            </Table>
          </Flex>
        </Group>
        <Group gap="md" style={{ flex: "0 0 auto" }}>
          <Image src="/grades/gradePerfect.png" h={64} w="auto" fit="contain" />
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
          <RivenAttributeCom key={index} value={{ ...modifier, grade: "A" }} hideDetails={false} />
        ))}
      </Stack>
    </Box>
  );
}
