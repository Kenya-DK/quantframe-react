import { Box, Group, Text, Badge, Flex, Card, alpha, Image } from "@mantine/core";
import { TauriTypes } from "$types";
import { useTranslateModals } from "@hooks/useTranslate.hook";
import { LocalizedDynamicMessage } from "@components/Shared/LocalizedDynamicMessage";

const colors = {
  mandatory: alpha("var(--mantine-color-grape-7)", 0.55),
  optional: alpha("var(--mantine-color-dark-4)", 0.5),
  matchText: alpha("var(--mantine-color-blue-2)", 0.85),
};
const RenderAttribute = (key: React.Key, label: string, match: boolean) => {
  return (
    <LocalizedDynamicMessage
      key={key}
      textProps={{
        fw: 700,
        ["data-match"]: match,
        c: match ? colors.matchText : "inherit",
      }}
      tokens={[
        {
          pattern: /<([A-Z0-9_]+)>/,
          render: (m) => <Image src={`/damageTypes/${m[1]}.png`} h={16} w="auto" fit="contain" mr={2} />,
        },
      ]}
      message={label}
    />
  );
};
interface ToleratedNegativeAttribute {
  label: string;
  matches: boolean;
}

interface ValidRoll {
  optional: ToleratedNegativeAttribute[];
  required: ToleratedNegativeAttribute[];
}

interface RollEvaluation {
  tolerated_negative_attributes: ToleratedNegativeAttribute[];
  valid_rolls: ValidRoll[];
}
interface Properties {
  roll_evaluation: RollEvaluation;
}

export type RollEvaluationTabProps = {
  value: TauriTypes.StockRiven<Properties> | undefined;
};

export function RollEvaluationTab({ value }: RollEvaluationTabProps) {
  if (!value) return <></>;
  // Translate general
  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateModals(`riven_details.tabs.roll_evaluation.${key}`, { ...context }, i18Key);

  return (
    <Box mt="lg">
      <Group justify="space-between" align="flex-start" mb="md">
        <Text size="lg" fw={600}>
          {useTranslate("labels.best_attributes")}
        </Text>
        <Group gap="xs">
          <Badge size="lg" color={colors.mandatory} c={"white"} variant="filled" tt="none">
            {useTranslate("labels.mandatory")}
          </Badge>
          <Badge size="lg" color={colors.optional} c={"white"} variant="filled" tt="none">
            {useTranslate("labels.optional")}
          </Badge>
        </Group>
      </Group>
      <Group justify="space-between" bg={"dark.8"} p={"sm"}>
        <Flex direction="row" gap="md" justify="center" align="center" flex={1}>
          {value.properties.roll_evaluation?.valid_rolls.map((rollSet, i) => (
            <Box key={i}>
              <Flex
                direction="column"
                align="center"
                gap={0}
                bg={colors.mandatory}
                p={"sm"}
                style={{ borderRadius: "5px" }}
                display={rollSet.required.length == 0 ? "none" : ""}
              >
                {rollSet.required.map((attr, idx) => RenderAttribute(idx, attr.label, attr.matches))}
              </Flex>
              <Flex
                direction="column"
                align="center"
                gap={0}
                bg={colors.optional}
                p={"sm"}
                style={{ borderRadius: "5px" }}
                mt={"sm"}
                display={rollSet.optional.length == 0 ? "none" : ""}
              >
                {rollSet.optional.map((attr, idx) => RenderAttribute(idx, attr.label, attr.matches))}
              </Flex>
            </Box>
          ))}
        </Flex>
        <Card radius="md" bg={alpha("var(--mantine-color-red-9)", 0.35)}>
          <Text fw={700} mb="sm">
            {useTranslate("labels.best_bad_attributes")}
          </Text>
          <Flex align="center" direction="column" gap={0} bg={alpha("var(--mantine-color-red-9)", 0.45)} p={"sm"} style={{ borderRadius: "5px" }}>
            {value.properties.roll_evaluation?.tolerated_negative_attributes.map((attr, idx) => RenderAttribute(idx, attr.label, attr.matches))}
          </Flex>
        </Card>
      </Group>
    </Box>
  );
}
