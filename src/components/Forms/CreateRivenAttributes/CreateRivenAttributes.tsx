import { Title, Button, Stack, Group } from "@mantine/core";
import { RivenAttribute, TauriTypes } from "$types";
import { useTranslateForms } from "@hooks/useTranslate.hook";
import { useForm } from "@mantine/form";
import { useState } from "react";
import { CreateRivenAttribute } from "../CreateRivenAttribute";

export type CreateRivenAttributesProps = {
  attributes: TauriTypes.CacheRivenAttribute[];
  maxPositive: number;
  maxNegative: number;
  value: RivenAttribute[];
  onSubmit: (values: RivenAttribute[]) => void;
};
export function CreateRivenAttributes({ maxPositive, maxNegative, attributes, onSubmit }: CreateRivenAttributesProps) {
  // State
  const defaultAttribute = { positive: true, url_name: "N/A", value: 0 };
  const [showPositiveCount, setShowPositiveCount] = useState(2);
  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`create_riven_attributes.${key}`, { ...context }, i18Key);
  const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`fields.${key}`, { ...context }, i18Key);
  const useTranslateFormButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`buttons.${key}`, { ...context }, i18Key);
  const [currentAttributes, setCurrentAttribute] = useState<RivenAttribute[]>([]);

  // Form
  const form = useForm({
    initialValues: {
      positive_attributes: Array.from({ length: maxPositive }, () => defaultAttribute as RivenAttribute),
      negative_attributes: Array.from({ length: maxNegative }, () => ({ ...defaultAttribute, positive: false } as RivenAttribute)),
    },
    onValuesChange: (values) => {
      const items = [...values.positive_attributes, ...values.negative_attributes];
      setCurrentAttribute(items);
      onSubmit && onSubmit(items.filter((item) => item.url_name != "N/A" && item.url_name != ""));
    },
  });

  const GetAvailableAttributes = (currentAttribute: RivenAttribute | undefined) => {
    if (!attributes) return [];

    const formAttributes = currentAttributes.map((item) => item.url_name);

    let avAttributes = attributes.filter((item) => !formAttributes?.includes(item.url_name));

    if (currentAttribute) {
      const attr = attributes.find((item) => item.url_name == currentAttribute.url_name);
      if (attr && !avAttributes.includes(attr)) avAttributes.push(attr);
    }
    return avAttributes;
  };

  return (
    <Stack w={"100%"} gap={"sm"}>
      <Title order={5} c={"green.7"}>
        {useTranslateFormFields("positive.title")}
      </Title>
      {form.values.positive_attributes.slice(0, showPositiveCount).map((item, index) => {
        return (
          <CreateRivenAttribute
            key={index}
            availableAttributes={GetAvailableAttributes(item)}
            positiveNumberOnly
            value={item}
            onChange={(v) => {
              form.setFieldValue(`positive_attributes.${index}`, v);
            }}
            canRemove={index >= 2}
            onRemove={() => {
              form.setFieldValue(`positive_attributes.${index}`, defaultAttribute);
              setShowPositiveCount(showPositiveCount - 1);
            }}
          />
        );
      })}
      {showPositiveCount < maxPositive && (
        <Group>
          <Button
            onClick={() => {
              if (showPositiveCount < maxPositive) {
                setShowPositiveCount(showPositiveCount + 1);
              }
            }}
          >
            {useTranslateFormButtons("add")}
          </Button>
        </Group>
      )}
      <Title order={5} c={"red.7"}>
        {useTranslateFormFields("negative.title")}
      </Title>
      {form.values.negative_attributes.map((item, index) => {
        return (
          <CreateRivenAttribute
            key={index}
            availableAttributes={GetAvailableAttributes(item)}
            negativeNumberOnly
            value={item}
            onChange={(v) => {
              form.setFieldValue(`negative_attributes.${index}`, v);
            }}
          />
        );
      })}
    </Stack>
  );
}
