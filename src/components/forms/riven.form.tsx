import { forwardRef, useEffect, useState } from 'react';
import { useForm, } from '@mantine/form';
import { ActionIcon, Button, Group, NumberInput, Select, Title, Tooltip, Text } from "@mantine/core";

import { Wfm } from '../../types';
import { useTranslateForm } from '../../hooks';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faPlus } from '@fortawesome/free-solid-svg-icons';
import { RivenPreview } from '../rivenPreview';
import SvgIcon, { SvgType } from '../SvgIcon';




interface ItemProps extends React.ComponentPropsWithoutRef<'div'> {
  label: string;
  value: string;
}
const SelectItem = forwardRef<HTMLDivElement, ItemProps>(
  ({ value, label, ...others }: ItemProps, ref) => (
    <div ref={ref} {...others}>
      <Group noWrap>
        <SvgIcon svgProp={{ width: 16, height: 16, }} iconType={SvgType.Polaritys} iconName={value} />
        <div>
          <Text size="sm">{label}</Text>
        </div>
      </Group>
    </div>
  )
);

interface FormPropsProps {
  riven?: Wfm.RivenItemDto | undefined | null;
  availableAttributes: Wfm.RivenAttributeInfoDto[],
  availableRivens: Wfm.RivenItemTypeDto[],
  onSubmit: (user: Wfm.RivenItemDto & { price: number }) => void;
  onCancel?: () => void;
}

export const RivenForm = ({ onSubmit, availableAttributes, availableRivens, riven }: FormPropsProps) => {
  const [currentRivenType, setCurrentRivenType] = useState<Wfm.RivenItemTypeDto | undefined>(undefined);
  const [attributeCount, setAttributeCount] = useState<number>(2);
  const [modNames, setModNames] = useState<string[]>([]);


  const useTranslateUserForm = (key: string, context?: { [key: string]: any }) => useTranslateForm(`riven.${key}`, { ...context })

  useEffect(() => {
    if (!riven) return;
    setCurrentRivenType(availableRivens.find((item) => item.url_name === riven.url_name))
    userForm.setFieldValue("url_name", riven.url_name)
    userForm.setFieldValue("mod_name", riven.mod_name)
    userForm.setFieldValue("mod_rank", riven.mod_rank)
    userForm.setFieldValue("mastery_rank", riven.mastery_rank)
    userForm.setFieldValue("attributes", riven.attributes)
    userForm.setFieldValue("re_rolls", riven.re_rolls)
    userForm.setFieldValue("polarity", riven.polarity)
  }, [riven]);

  const userForm = useForm({
    initialValues: {
      url_name: "",
      weapon_name: "",
      mod_name: "",
      mod_rank: 0,
      mastery_rank: 7,
      price: 0,
      attributes: [] as Array<Wfm.RivenAttributeDto | null>,
      re_rolls: 0,
      polarity: "madurai",
    },
    validate: {},
  });

  // Handle attribute permutations
  useEffect(() => {
    // Filter out null entries
    const filteredArray = userForm.values.attributes.filter(entry => entry !== null && entry.positive) as Wfm.RivenAttributeInfoDto[];
    function generatePermutations(inputArray: string[]): string[][] {
      let currentIndex: string, swapIndex: number;
      const arrayLength = inputArray.length;
      let permutations = [inputArray.slice()];
      const counters = new Array(arrayLength).fill(0);

      for (let index = 1; index < arrayLength;) {
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
    const rivenIds: { [key: string]: Wfm.RivenAttributeInfoDto } = {};
    availableAttributes.forEach((item) => { rivenIds[item.url_name] = { ...item } });
    if (availableAttributes.length == 0)
      return;
    let selectedIds = generatePermutations(filteredArray.map((item) => item.url_name));

    let modNames: string[] = [];
    selectedIds.forEach((item) => {
      if (2 === item.length) {
        modNames.push(`${rivenIds[item[0]].prefix}${rivenIds[item[1]].suffix.toLowerCase()}`);
      } else if (3 === item.length) {
        modNames.push(`${rivenIds[item[0]].prefix}-${rivenIds[item[1]].prefix.toLowerCase()}${rivenIds[item[2]].suffix.toLowerCase()}`);
      }
    })
    setModNames(modNames);
  }, [userForm.values.attributes])

  return (
    <form method="post" onSubmit={userForm.onSubmit(async (data) => {

      onSubmit({
        ...data,
        attributes: data.attributes.filter(x => x != null).map((item) => {
          if (!item) return item;
          return {
            ...item,
          }
        }) as Wfm.RivenAttributeDto[]
      })
    })}>

      <RivenPreview riven={{
        ...userForm.values,
        weapon_name: currentRivenType?.item_name || "",
        attributes: userForm.values.attributes.filter(x => x != null).map((item) => {
          if (!item) return item;
          return {
            ...item,
            value: item.positive ? item.value : -item.value,
          }
        }) as Wfm.RivenAttributeDto[]
      }} />

      <Group grow >
        <Select
          label={useTranslateUserForm("weapon_name")}
          value={userForm.values.url_name}
          onChange={(event) => {
            setCurrentRivenType(availableRivens.find((item) => item.url_name === event))
            userForm.setFieldValue('url_name', event || "")
          }}
          searchable
          clearable
          limit={5}
          data={availableRivens.map((item: Wfm.RivenItemTypeDto) => {
            return {
              value: item.url_name,
              label: item.item_name,
            };
          })}
        />
      </Group>
      <Title order={5} style={{ marginBottom: 10 }}>{useTranslateUserForm("attributes")}</Title>
      {Array.from(Array(attributeCount).keys()).map(() => {
        return (
          <></>
          // <RiveAttributeForm
          //   onClear={() => {
          //     const attributes = [...userForm.values.attributes];
          //     attributes[i] = null;
          //     userForm.setFieldValue('attributes', attributes)
          //   }}
          //   onRemove={() => setAttributeCount(attributeCount - 1)}
          //   key={i}
          //   canRemove={i >= 2}
          //   availableAttributes={getFilterAttributes()}
          //   attribute={userForm.values.attributes[i] == null ? undefined : userForm.values.attributes[i] as Wfm.RivenAttributeDto}
          //   index={i}
          //   onChange={(attribute) => {
          //     const attributes = [...userForm.values.attributes];
          //     if (attribute)
          //       attributes[i] = attribute;
          //     else
          //       attributes.splice(i, 1);
          //     userForm.setFieldValue('attributes', attributes)
          //   }}
          // />
        )
      })}
      {attributeCount < 4 &&
        <Tooltip label={useTranslateUserForm("add_attribute")}>
          <ActionIcon mt={4} color="blue.7" onClick={() => {
            setAttributeCount(attributeCount + 1)
          }} >
            <FontAwesomeIcon icon={faPlus} />
          </ActionIcon>
        </Tooltip>
      }
      <Group grow >
        <Select
          label={useTranslateUserForm("mod_name")}
          disabled={modNames.length == 0}
          value={userForm.values.mod_name}
          onChange={(event) => userForm.setFieldValue('mod_name', event || "")}
          searchable
          clearable
          limit={5}
          data={modNames}
        />
      </Group>
      <Group >
        <NumberInput
          required
          label={useTranslateUserForm("mastery_rank")}
          min={7}
          max={16}
          value={userForm.values.mastery_rank}
          onChange={(value) => userForm.setFieldValue('mastery_rank', Number(value))}
          error={userForm.errors.mastery_rank && 'Invalid identifier'}
        />
        <NumberInput
          required
          label={useTranslateUserForm("mod_rank")}
          min={0}
          max={8}
          value={userForm.values.mod_rank}
          onChange={(value) => userForm.setFieldValue('mod_rank', Number(value))}
          error={userForm.errors.mod_rank && 'Invalid identifier'}
        />
        <NumberInput
          required
          label={useTranslateUserForm("re_rolls")}
          min={0}
          value={userForm.values.re_rolls}
          onChange={(value) => userForm.setFieldValue('re_rolls', Number(value))}
          error={userForm.errors.re_rolls && 'Invalid identifier'}
        />
        <NumberInput
          required
          label={useTranslateUserForm("bought")}
          min={0}
          value={userForm.values.price}
          onChange={(value) => userForm.setFieldValue('price', Number(value))}
          error={userForm.errors.price && 'Invalid identifier'}
        />
        <Select
          label={useTranslateUserForm("polarity")}
          value={userForm.values.polarity}
          onChange={(event) => userForm.setFieldValue('polarity', event || "")}
          itemComponent={SelectItem}
          icon={<SvgIcon svgProp={{ width: 16, height: 16, }} iconType={SvgType.Polaritys} iconName={userForm.values.polarity} />}
          data={[
            { value: "madurai", label: "Madurai" },
            { value: "naramon", label: "Naramon" },
            { value: "vazarin", label: "Vazarin" },
          ]}
        />
      </Group>
      <Group position="right" mt={10} sx={{
        position: "absolute",
        bottom: 15,
        right: 15,
      }}>
        <Button type="submit" variant="light" color="blue">
          {useTranslateUserForm('save')}
        </Button>
      </Group>
    </form>
  );
}