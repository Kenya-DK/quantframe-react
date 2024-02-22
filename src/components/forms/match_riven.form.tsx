import { useEffect, useState } from 'react';
import { useForm, } from '@mantine/form';
import { Button, Checkbox, Collapse, Group, NumberInput, Select, Stack, Switch, Tooltip } from "@mantine/core";

import { CacheDataId, MatchRivenAttributeDto, MatchRivenDto, Wfm } from '$types/index';
import { MinMaxField } from '../MinMaxField';
import SvgIcon, { SvgType } from '../SvgIcon';
import { useLocalStorage } from '@mantine/hooks';


interface FormPropsProps {
  height?: number | string;
  match?: MatchRivenDto
  weapon?: Wfm.RivenItemTypeDto | undefined
  canToggle?: boolean;
  onSubmit: (match: MatchRivenDto) => void;
  onCancel?: () => void;
}

export const MatchRivenForm = ({ onSubmit, canToggle, height, weapon, match }: FormPropsProps) => {
  const [riven_items] = useLocalStorage<Wfm.RivenItemTypeDto[]>({ key: CacheDataId.RivenItems, defaultValue: [] });
  const [riven_attributes] = useLocalStorage<Wfm.RivenAttributeInfoDto[]>({ key: CacheDataId.RivenAttributes, defaultValue: [] });
  const [, setDisabled] = useState(false);
  const [attributeOptions, setAttributeOptions] = useState<{ value: string, label: string }[]>([]);

  // Set Form Values on load
  useEffect(() => {
    if (weapon)
      userForm.setFieldValue('weapon', weapon)
    // Check the type of match
    if (!match) return;

    const filteredObject = Object.fromEntries(
      Object.entries(match).filter(([_, value]) => value !== null)
    );

    if (Object.keys(filteredObject).length === 0) {
      setDisabled(true);
    }

    filteredObject.attributes = [null, null, null, null];

    console.log("filteredObject", filteredObject);

    const positive = match.attributes?.filter((item) => item != null && !item.is_negative);
    if (positive)
      for (let i = 0; i < positive.length; i++)
        filteredObject.attributes[i] = positive[i];

    const negative = match.attributes?.filter((item) => item != null && item.is_negative);
    if (negative && negative.length > 0)
      filteredObject.attributes[3] = negative[0];
    userForm.setFieldValue("match", { ...userForm.values.match, ...filteredObject, });
  }, [weapon, match]);


  useEffect(() => {
    if (!riven_attributes) return;
    setAttributeOptions([{ value: "", label: "-" }, ...riven_attributes.map((item) => { return { value: item.url_name, label: item.effect }; })]);
  }, [riven_attributes]);

  useEffect(() => {
    if (!match) return;
    const filteredObject = Object.fromEntries(
      Object.entries(match).filter(([_, value]) => value !== null)
    );
    filteredObject.attributes = [null, null, null, null];

    const positive = match.attributes?.filter((item) => !item?.is_negative);
    if (positive)
      for (let i = 0; i < positive.length; i++)
        filteredObject.attributes[i] = positive[i];

    const negative = match.attributes?.filter((item) => item?.is_negative);
    if (negative && negative.length > 0)
      filteredObject.attributes[3] = negative[0];
    userForm.setFieldValue("match", { ...userForm.values.match, ...filteredObject, });
  }, [match]);

  const userForm = useForm({

    initialValues: {
      weapon: weapon,
      match: {
        enabled: false,
        rank: { min: 0, max: "" as number | "" },
        mastery_rank: { min: 0, max: "" as number | "" },
        re_rolls: { min: 0, max: "" as number | "" },
        polarity: '',
        similarity: 0,
        attributes: [null, null, null, null] as Array<MatchRivenAttributeDto | null>,
        required_negative: false,
      },
    },
    validate: {},
  });

  // const GetAttributes = (_index: number) => {
  //   let attributes: Wfm.RivenAttributeInfoDto[] = riven_attributes;
  //   // Filter exclusive attributes.
  //   if (userForm.values.weapon) {
  //     const exclusive = userForm.values.weapon.exclusive_to;
  //     if (exclusive) {
  //       attributes = attributes.filter((item) => {
  //         return !exclusive.includes(item.url_name) || exclusive == null;
  //       });
  //     }
  //   }
  //   return riven_attributes.map((item) => { return { value: item.url_name, label: item.effect }; });
  // }

  return (
    <form style={{
      height: height,
    }} method="post" onSubmit={userForm.onSubmit(async (data) => {
      const match: MatchRivenDto = {
        ...data.match,
        rank: { min: data.match.rank.min, max: data.match.rank.max || 0 },
        mastery_rank: { min: data.match.mastery_rank.min, max: data.match.mastery_rank.max || 0 },
        re_rolls: { min: data.match.re_rolls.min, max: data.match.re_rolls.max || 0 },
        attributes: data.match.attributes.filter((item) => item != null) as MatchRivenAttributeDto[],
      };
      onSubmit(match)
    })}>
      {canToggle &&
        <Checkbox
          mt={30}
          size={"md"}
          label={('Enabled')}
          checked={userForm.values.match.enabled || false}
          onChange={(event) => userForm.setFieldValue('match.enabled', event.currentTarget.checked)}
        />
      }
      <Collapse in={userForm.values.match.enabled || !canToggle}>
        <Stack mt={15}>
          <Select
            searchable
            clearable
            value={userForm.values.weapon?.url_name || ""}
            onChange={(event) => {
              if (!riven_items) return;
              userForm.setFieldValue('weapon', riven_items.find(x => x.url_name === event))
            }}
            data={riven_items?.map((item) => { return { value: item.url_name, label: item.item_name }; }) || []}
          />
          {Array.from({ length: 4 }, (_, i) => {
            return (
              <Select
                key={i}
                searchable
                clearable
                limit={5}
                value={userForm.values.match.attributes[i]?.url_name || ""}
                onChange={(event) => {
                  const attributes = userForm.values.match.attributes;

                  if (event === "" || event === null) {
                    attributes[i] = null;
                  } else {
                    attributes[i] = { ...attributes[i] || { is_negative: i >= 3, is_required: false }, url_name: event }
                  }
                  userForm.setFieldValue(`match.attributes`, attributes)
                }}
                styles={(_theme) => ({
                  input: {
                    backgroundColor: i >= 3 ? "darkred" : "darkgreen",
                  },
                })}
                rightSection={
                  <Tooltip label='Is required'>
                    <span>
                      <Switch disabled={userForm.values.match.attributes[i] === null}
                        mr={20}
                        checked={userForm.values.match.attributes[i]?.is_required || false}
                        onChange={() => {
                          const attributes = userForm.values.match.attributes;

                          if (attributes[i] === null) return;
                          attributes[i]!.is_required = !attributes[i]?.is_required
                          userForm.setFieldValue(`match.attributes`, attributes)
                        }}
                      />
                    </span>
                  </Tooltip>
                }
                // data={[{ value: "", label: "-" }, ...riven_attributes.map((item) => { return { value: item.url_name, label: item.effect }; })]}
                data={attributeOptions}
              />
            )
          })}
          <Group grow>
            <MinMaxField size={"xs"} maxAllowed={16} width={65} min={userForm.values.match.mastery_rank?.min || 0} max={userForm.values.match.mastery_rank?.max || 0} label='MR' onChange={(min: number | "", max: number | "") => {
              userForm.setFieldValue('match.mastery_rank.min', min)
              userForm.setFieldValue('match.mastery_rank.max', max)
            }} />
            <MinMaxField size={"xs"} width={75} minAllowed={0} min={userForm.values.match.re_rolls?.min || 0} max={userForm.values.match.re_rolls?.max || 0} label='Re Rolls' onChange={(min: number | "", max: number | "") => {
              userForm.setFieldValue('match.re_rolls.min', min)
              userForm.setFieldValue('match.re_rolls.max', max)
            }} />
            <MinMaxField size={"xs"} width={75} maxAllowed={8} min={userForm.values.match.rank?.min || 0} max={userForm.values.match.rank?.max || 0} label='Rank' onChange={(min: number | "", max: number | "") => {
              userForm.setFieldValue('match.rank.min', min)
              userForm.setFieldValue('match.rank.max', max)
            }} />
            <Select
              w={150}
              size={"xs"}
              label={("polarity")}
              value={userForm.values.match.polarity || ""}
              onChange={(event) => userForm.setFieldValue('match.polarity', event === "" ? undefined : event)}
              icon={userForm.values.match.polarity ? <SvgIcon svgProp={{ width: 16, height: 16, }} iconType={SvgType.Polaritys} iconName={userForm.values.match.polarity || ""} /> : undefined}
              data={[
                { value: "", label: "Any" },
                { value: "madurai", label: "Madurai" },
                { value: "naramon", label: "Naramon" },
                { value: "vazarin", label: "Vazarin" },
              ]}
            />
            <NumberInput
              w={150}
              required
              label={("Minimum similarity")}
              min={0}
              size={"xs"}
              max={100}
              formatter={(value) => value + "%"}
              value={userForm.values.match.similarity}
              onChange={(value) => {
                const similarity = Number(value)
                if (similarity > 0)
                  userForm.setFieldValue('match.similarity', Number(value))
                else
                  userForm.setFieldValue('match.similarity', undefined)
              }}
              error={userForm.errors.semlartiy && 'Invalid identifier'}
            />
            <Checkbox
              mt={30}
              size={"md"}
              label={('Required negative')}
              checked={userForm.values.match.required_negative || false}
              onChange={(event) => userForm.setFieldValue('match.required_negative', event.currentTarget.checked)}
            />
          </Group>
        </Stack>
      </Collapse>
      <Group position="right" mt={10} sx={{
        position: "absolute",
        bottom: 15,
        right: 15,
      }}>
        <Button type="submit" variant="light" color="blue">
          {('save')}
        </Button>
      </Group>
    </form>
  );
}