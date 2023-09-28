import { useEffect, useState } from 'react';
import { useForm, } from '@mantine/form';
import { ActionIcon, Group, NumberInput, Select, Title } from "@mantine/core";

import { Wfm } from '../../types';
import { useTranslateForm } from '../../hooks';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faArrowsRotate, faClose, faPercent } from '@fortawesome/free-solid-svg-icons';
import { useCacheContext } from '../../contexts';
import { RivenPreview } from '../rivenPreview';


interface RiveAttributeFormProps {
  availableAttributes: Wfm.RivenAttributeInfoDto[],
  attribute: Wfm.RivenAttributeDto | undefined,
  index: number,
  onChange?: (riven: Wfm.RivenAttributeDto | undefined) => void
  canRemove?: boolean,
  onRemove?: () => void
  onClear?: () => void
}
const RiveAttributeForm = ({ availableAttributes, canRemove, attribute, onRemove, onChange, onClear }: RiveAttributeFormProps) => {
  const [attributeType, setAttributeType] = useState<Wfm.RivenAttributeInfoDto | undefined>(undefined);
  const [attributeValue, setAttributeValue] = useState<number>(0);


  useEffect(() => {
    if (attribute) {
      setAttributeType(availableAttributes.find((item) => item.url_name === attribute.url_name))
      setAttributeValue(attribute.value)
    }
    else
      setAttributeType(undefined)

  }, [attribute])

  useEffect(() => {
    if (attributeType)
      onChange?.({ ...attributeType, value: attributeValue, positive: attributeType.positive_is_negative ? attributeValue < 0 : attributeValue > 0 })
  }, [attributeType, attributeValue])
  return (

    <Group grow mt={5}>
      <Select
        value={attribute?.url_name || ""}
        onChange={(event) => setAttributeType(availableAttributes.find((item) => item.url_name === event))}
        searchable
        limit={5}
        rightSectionWidth={45}
        rightSection={
          <>
            {attributeType &&
              <ActionIcon color="red.7" onClick={async () => { onClear?.() }} >
                <FontAwesomeIcon icon={faClose} />
              </ActionIcon>
            }
          </>
        }
        data={availableAttributes.map((item: Wfm.RivenAttributeInfoDto) => {
          return {
            value: item.url_name,
            label: item.effect,
          };
        })}
      />
      <NumberInput
        required
        disabled={!attribute}
        value={attribute?.value || 0}
        max={400}
        onChange={(value) => setAttributeValue(Number(value))}
        rightSectionWidth={45}
        rightSection={<FontAwesomeIcon icon={faPercent} />}
      />
      {canRemove &&
        <ActionIcon color="red.7" onClick={() => {
          onRemove?.()
        }} >
          <FontAwesomeIcon icon={faClose} />
        </ActionIcon>
      }
    </Group>
  );
}

interface FormPropsProps {
  riven?: Wfm.RivenItemDto | undefined | null;
  onSubmit: (user: Wfm.RivenItemDto) => void;
  onCancel?: () => void;
}

export const RivenForm = ({ riven }: FormPropsProps) => {
  const [rivenTypes, setRivenTypes] = useState<Wfm.RivenItemTypeDto[]>([]);
  const [currentRivenType, setCurrentRivenType] = useState<Wfm.RivenItemTypeDto | undefined>(undefined);
  const [rivenAttributes, setRivenAttributes] = useState<Wfm.RivenAttributeInfoDto[]>([]);
  const [attributeCount, setAttributeCount] = useState<number>(2);
  const [modNames, setModNames] = useState<string[]>([]);


  const { riven_attributes, riven_items } = useCacheContext();
  useEffect(() => {
    setRivenTypes(riven_items)
    setRivenAttributes(riven_attributes)
  }, [riven_items, riven_attributes]);
  const useTranslateUserForm = (key: string, context?: { [key: string]: any }) => useTranslateForm(`riven.${key}`, { ...context })

  useEffect(() => {
    if (!riven) return;
    setCurrentRivenType(rivenTypes.find((item) => item.url_name === riven.url_name))
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
      attributes: [] as Array<Wfm.RivenAttributeDto | null>,
      re_rolls: 0,
      polarity: "madurai",
    },
    validate: {},
  });
  const getFilterAttributes = (attribute: string) => {
    if (currentRivenType) {
      const attributes = userForm.values.attributes.map((item) => item?.url_name);
      return rivenAttributes.filter((item) => item.exclusive_to == null || item.exclusive_to.includes(currentRivenType.riven_type) && (!attributes.includes(item.url_name) || item.url_name == attribute));
    }
    else
      return rivenAttributes;
  };

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
    rivenAttributes.forEach((item) => { rivenIds[item.url_name] = { ...item } });
    if (rivenAttributes.length == 0)
      return;
    let selectedIds = generatePermutations(filteredArray.map((item) => item.url_name));
    console.log(selectedIds);

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
      console.log(data)
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
            setCurrentRivenType(rivenTypes.find((item) => item.url_name === event))
            userForm.setFieldValue('url_name', event || "")
          }}
          searchable
          clearable
          limit={5}
          data={rivenTypes.map((item: Wfm.RivenItemTypeDto) => {
            return {
              value: item.url_name,
              label: item.item_name,
            };
          })}
        />
      </Group>
      <Title order={5} style={{ marginBottom: 10 }}>{useTranslateUserForm("attributes")}</Title>
      {Array.from(Array(attributeCount).keys()).map((i) => {
        return (
          <RiveAttributeForm
            onClear={() => {
              const attributes = [...userForm.values.attributes];
              attributes[i] = null;
              userForm.setFieldValue('attributes', attributes)
            }}
            onRemove={() => setAttributeCount(attributeCount - 1)}
            key={i}
            canRemove={i >= 2}
            availableAttributes={getFilterAttributes(userForm.values.attributes[i]?.url_name || "")}
            attribute={userForm.values.attributes[i] == null ? undefined : userForm.values.attributes[i] as Wfm.RivenAttributeDto}
            index={i}
            onChange={(attribute) => {
              const attributes = [...userForm.values.attributes];
              if (attribute)
                attributes[i] = attribute;
              else
                attributes.splice(i, 1);
              userForm.setFieldValue('attributes', attributes)
            }}
          />
        )
      })}
      {attributeCount < 4 &&
        <ActionIcon mt={4} color="blue" onClick={() => {
          setAttributeCount(attributeCount + 1)
        }} >
          <FontAwesomeIcon icon={faArrowsRotate} />
        </ActionIcon>
      }
      <Group grow >
        <Select
          label={useTranslateUserForm("mod_name")}
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
        <Select
          label={useTranslateUserForm("polarity")}
          value={userForm.values.polarity}
          onChange={(event) => userForm.setFieldValue('polarity', event || "")}
          data={[
            { value: "madurai", label: "Madurai" },
            { value: "naramon", label: "Naramon" },
            { value: "vazarin", label: "Vazarin" },
          ]}
        />
      </Group>
    </form>
  );
}