import { Title, Box, Group, NumberInput, Select } from '@mantine/core';
import { useTranslateForms } from '@hooks/useTranslate.hook';
import { useForm } from '@mantine/form';
import { RivenAttribute, CacheRivenAttribute } from '@api/types';
import { useEffect, useState } from 'react';



export type CreateRivenAttributeProps = {
	availableAttributes: CacheRivenAttribute[];
	value: RivenAttribute;
	onChange?: (values: RivenAttribute) => void;
}
export function CreateRivenAttribute({ availableAttributes, onChange, value }: CreateRivenAttributeProps) {
	// Translate general
	const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateForms(`create_riven_attribute.${key}`, { ...context }, i18Key)
	const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateForm(`fields.${key}`, { ...context }, i18Key)

	// User form
	const form = useForm({
		initialValues: {
			...value,
		},
		validate: {},
		onValuesChange: (values) => {
			onChange && onChange(values);
		}
	});

	// Helper functions
	const getAvailableAttributes = () => {
		return availableAttributes.map((item) => ({ label: item.effect, value: item.url_name }));
	}

	return (
		<Box w={"100%"} mt={"md"}>
			<Group gap={"xs"}>
				<Select
					searchable
					clearable
					limit={5}
					w={"80%"}
					value={form.values.url_name || ""}
					onChange={(event) => form.setFieldValue('url_name', event || "")}
					data={getAvailableAttributes()}
				/>
				<NumberInput
					w={"15%"}
					value={form.values.value || 0}
					onChange={(event) => form.setFieldValue('value', Number(event))}
					error={form.errors.value && useTranslateFormFields('value.error')}
					radius="md"
				/>
			</Group>
		</Box>
	);
}

export type CreateRivenAttributesProps = {
	attributes: CacheRivenAttribute[];
	value: RivenAttribute[];
	onSubmit: (values: RivenAttribute[]) => void;
}
export function CreateRivenAttributes({ attributes, onSubmit }: CreateRivenAttributesProps) {
	// State
	const [, setTotalPositive] = useState(0);
	const defaultAttribute = { positive: true, url_name: "N/A", value: 0 };

	// Translate general
	const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateForms(`create_riven_attributes.${key}`, { ...context }, i18Key)
	const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateForm(`fields.${key}`, { ...context }, i18Key)
	const [currentAttributes, setCurrentAttribute] = useState<RivenAttribute[]>([]);


	// User form
	const form = useForm({
		initialValues: {
			att1: defaultAttribute as RivenAttribute,
			att2: defaultAttribute as RivenAttribute,
			att3: defaultAttribute as RivenAttribute,
			att4: { ...defaultAttribute, positive: false } as RivenAttribute,
		},
		validate: {},
		onValuesChange: (values) => {
			const items = [
				values.att1,
				values.att2,
				values.att3,
				values.att4,
			];
			setCurrentAttribute(items);
			onSubmit && onSubmit(items.filter((item) => item.url_name != "N/A" && item.url_name != ""));
		}
	});

	// Effects 	
	useEffect(() => {
		const positive = currentAttributes.filter((item) => item.positive && item.url_name != "N/A" && item.url_name != "");
		setTotalPositive(positive.length);
	}, [form.values]);

	const GetAvailableAttributes = (currentAttribute: RivenAttribute | undefined) => {
		if (!attributes)
			return [];

		const formAttributes = currentAttributes.map((item) => item.url_name);

		let avAttributes = attributes.filter((item) => !formAttributes?.includes(item.url_name));

		if (currentAttribute) {
			const attr = attributes.find((item) => item.url_name == currentAttribute.url_name);
			if (attr && !avAttributes.includes(attr))
				avAttributes.push(attr);
		}
		return avAttributes;
	}


	const SetAttribute = (index: number, value: RivenAttribute, _remove?: boolean) => {
		switch (index) {
			case 0:
				form.setFieldValue("att1", value);
				break;
			case 1:
				form.setFieldValue("att2", value);
				break;
			case 2:
				form.setFieldValue("att3", value);
				break;
			case 3:
				form.setFieldValue("att4", value);
				break;
			default:
				break;
		}
	}


	return (
		<Box w={"100%"}>
			<Title order={5} c={"green.7"} >{useTranslateFormFields("positive.title")}</Title>
			<CreateRivenAttribute availableAttributes={GetAvailableAttributes(form.values.att1)} value={form.values.att1} onChange={(v) => SetAttribute(0, v)} />
			<CreateRivenAttribute availableAttributes={GetAvailableAttributes(form.values.att2)} value={form.values.att2} onChange={(v) => SetAttribute(1, v)} />
			<CreateRivenAttribute availableAttributes={GetAvailableAttributes(form.values.att3)} value={form.values.att3} onChange={(v) => SetAttribute(2, v)} />
			<Group>
			</Group>
			<Title order={5} c={"red.7"}>{useTranslateFormFields("negative.title")}</Title>
			<Group>
				<CreateRivenAttribute availableAttributes={GetAvailableAttributes(form.values.att4)} value={form.values.att4} onChange={(v) => SetAttribute(3, v)} />
			</Group>
		</Box>
	);
}