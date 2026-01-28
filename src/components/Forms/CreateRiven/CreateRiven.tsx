import { Box, Button, Grid, Group, NumberInput, Select, Text, SelectProps, Flex } from "@mantine/core";
import { useTranslateForms } from "@hooks/useTranslate.hook";
import { useForm } from "@mantine/form";
import { TauriTypes } from "$types";
import api from "@api/index";
import { useQuery } from "@tanstack/react-query";
import { RivenPreview } from "@components/DataDisplay/RivenPreview";
import { CreateRivenAttributes } from "../CreateRivenAttributes";
import { useEffect, useState } from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { SelectRivenWeapon } from "../SelectRivenWeapon";
import { getPolarityIcon } from "@icons";
import { TokenSearchSelect } from "@components/Forms/TokenSearchSelect";

export type CreateRivenProps = {
  value?: TauriTypes.StockRiven;
  onSubmit: (values: TauriTypes.StockRiven) => void;
};

const renderSelectOption: SelectProps["renderOption"] = ({ option, checked }) => (
  <Group flex="1" gap="xs" style={{ fontWeight: checked ? 700 : 400 }}>
    {<FontAwesomeIcon icon={getPolarityIcon(option.value)} />}
    {option.label}
  </Group>
);
export function CreateRiven({ value, onSubmit }: CreateRivenProps) {
  // State
  const [availableAttributes, setAvailableAttributes] = useState<TauriTypes.CacheRivenAttribute[]>([]);
  const [modNames, setModNames] = useState<string[]>([]);

  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`create_riven.${key}`, { ...context }, i18Key);
  const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`fields.${key}`, { ...context }, i18Key);
  const useTranslateButton = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`buttons.${key}`, { ...context }, i18Key);

  // Fetch data from rust side
  const { data: weapons } = useQuery({
    queryKey: ["cache_riven_weapons"],
    queryFn: () => api.cache.getRivenWeapons(),
  });

  const { data: attributes } = useQuery({
    queryKey: ["cache_riven_attributes"],
    queryFn: () => api.cache.getRivenAttributes(),
  });

  // User form
  const form = useForm({
    initialValues: {
      ...value,
      id: value?.id || 0,
      mastery_rank: value?.mastery_rank || 8,
      re_rolls: value?.re_rolls || 0,
      sub_type: {
        ...value?.sub_type,
        rank: value?.sub_type?.rank || 0,
      },
      polarity: value?.polarity || "madurai",
    },
    validate: {
      attributes: (value?: TauriTypes.StockRiven["attributes"]) => {
        if (!value || value.length === 0) return null;
        const hasZeroValue = value.some((item) => item && item.url_name && item.url_name !== "N/A" && item.value === 0);
        if (hasZeroValue) return useTranslateFormFields("attribute.error");
        return null;
      },
    },
  });

  const resetFormForWeapon = (weaponUrl: string) => {
    form.reset();
    form.setFieldValue("wfm_weapon_url", weaponUrl);
    setModNames([]);
  };

  // Effects
  useEffect(() => {
    if (!attributes) return;

    const weapon = weapons?.find((item) => item.wfm_url_name == form.values.wfm_weapon_url);

    const avAttributes = attributes.filter((item) => !item.exclusiveTo || item.exclusiveTo.includes(weapon?.riven_type || ""));
    setAvailableAttributes(avAttributes);
  }, [form.values, attributes]);
  useEffect(() => {
    const rivenIds: { [key: string]: TauriTypes.CacheRivenAttribute } = {};
    const filteredArray = form.values.attributes?.filter((entry) => entry !== null && entry.positive);
    if (!filteredArray || filteredArray.length === 0) return;
    availableAttributes.forEach((item) => {
      rivenIds[item.url_name] = { ...item };
    });
    function generatePermutations(inputArray: string[]): string[][] {
      let currentIndex: string, swapIndex: number;
      const arrayLength = inputArray.length;
      let permutations = [inputArray.slice()];
      const counters = new Array(arrayLength).fill(0);

      for (let index = 1; index < arrayLength; ) {
        if (counters[index] < index) {
          swapIndex = index % 2 ? counters[index] : 0;

          // Swap elements
          currentIndex = inputArray[index];
          inputArray[index] = inputArray[swapIndex];
          inputArray[swapIndex] = currentIndex;

          counters[index]++;
          index = 1;
          permutations.push(inputArray.slice());
        } else {
          counters[index] = 0;
          index++;
        }
      }

      return permutations;
    }
    let selectedIds = generatePermutations(filteredArray.map((item) => item.url_name));

    let modNames: string[] = [];
    selectedIds.forEach((item) => {
      if (2 === item.length) {
        modNames.push(`${rivenIds[item[0]].prefix}${rivenIds[item[1]].suffix.toLowerCase()}`);
      } else if (3 === item.length) {
        modNames.push(`${rivenIds[item[0]].prefix}-${rivenIds[item[1]].prefix.toLowerCase()}${rivenIds[item[2]].suffix.toLowerCase()}`);
      }
    });
    setModNames(modNames);
  }, [form.values.attributes, availableAttributes]);

  return (
    <Box w={"100%"}>
      <form
        onSubmit={form.onSubmit((data) => {
          onSubmit(data as TauriTypes.StockRiven);
        })}
      >
        <Grid mb={75}>
          <Grid.Col span={4} p={0}>
            <RivenPreview riven={form.values as TauriTypes.StockRiven} />
          </Grid.Col>
          <Grid.Col span={8}>
            <Group gap="md" grow>
              <SelectRivenWeapon
                value={form.values.wfm_weapon_url || ""}
                onChange={(item) => {
                  if (item.wfm_url_name === form.values.wfm_weapon_url) return;
                  resetFormForWeapon(item.wfm_url_name);
                }}
              />
            </Group>
            <Group>
              <CreateRivenAttributes
                key={form.values.wfm_weapon_url || "no-weapon"}
                maxNegative={1}
                maxPositive={3}
                attributes={availableAttributes}
                value={form.values.attributes || []}
                onSubmit={(values) => form.setFieldValue("attributes", values)}
              />
            </Group>
            {form.errors.attributes && <Text c="red">{form.errors.attributes}</Text>}
            <Group grow>
              <TokenSearchSelect
                key={`mod-name-${form.values.wfm_weapon_url || "no-weapon"}`}
                size="sm"
                autoSelectOnBlur
                selectFirstOptionOnChange
                required
                allowDeselect={false}
                label={useTranslateFormFields("mod_name.label")}
                value={form.values.mod_name}
                onChange={(event) => form.setFieldValue("mod_name", event || "")}
                error={form.errors.mod_name && useTranslateFormFields("mod_name.error")}
                data={modNames}
              />
            </Group>
            <Flex gap="md">
              <NumberInput
                size="sm"
                required
                w={250}
                max={16}
                min={8}
                label={useTranslateFormFields("mastery_rank.label")}
                placeholder={useTranslateFormFields("mastery_rank.placeholder")}
                value={form.values.mastery_rank}
                onChange={(event) => form.setFieldValue("mastery_rank", Number(event))}
                error={form.errors.mastery_rank && useTranslateFormFields("mastery_rank.error")}
                radius="md"
              />
              <NumberInput
                required
                size="sm"
                min={0}
                label={useTranslateFormFields("re_rolls.label")}
                placeholder={useTranslateFormFields("re_rolls.placeholder")}
                value={form.values.re_rolls}
                onChange={(event) => form.setFieldValue("re_rolls", Number(event))}
                error={form.errors.re_rolls && useTranslateFormFields("re_rolls.error")}
                radius="md"
              />
              <NumberInput
                required
                size="sm"
                min={0}
                label={useTranslateFormFields("bought.label")}
                placeholder={useTranslateFormFields("bought.placeholder")}
                value={form.values.bought || 0}
                onChange={(event) => form.setFieldValue("bought", Number(event))}
                error={form.errors.bought && useTranslateFormFields("bought.error")}
                radius="md"
              />
              <NumberInput
                required
                size="sm"
                max={8}
                min={0}
                label={useTranslateFormFields("rank.label")}
                placeholder={useTranslateFormFields("rank.placeholder")}
                value={form.values.sub_type?.rank || 0}
                onChange={(event) => form.setFieldValue("sub_type.rank", Number(event))}
                error={form.errors.rank && useTranslateFormFields("rank.error")}
                radius="md"
              />
              <Select
                required
                size="sm"
                w={350}
                allowDeselect={false}
                label={useTranslateFormFields("polarity.label")}
                value={form.values.polarity}
                onChange={(event) => form.setFieldValue("polarity", event || "")}
                leftSection={<FontAwesomeIcon icon={getPolarityIcon(form.values.polarity)} />}
                data={[
                  { value: "madurai", label: "Madurai" },
                  { value: "naramon", label: "Naramon" },
                  { value: "vazarin", label: "Vazarin" },
                ]}
                renderOption={renderSelectOption}
              />
            </Flex>
          </Grid.Col>
        </Grid>
        <Group
          justify="flex-end"
          style={{
            position: "absolute",
            bottom: 25,
            right: 25,
          }}
        >
          <Button type="submit" variant="light" color="blue">
            {useTranslateButton("submit.label")}
          </Button>
        </Group>
      </form>
    </Box>
  );
}
