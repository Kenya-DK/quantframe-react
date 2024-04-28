import { Group, NumberInput, Box, Checkbox, Collapse, Switch, Button } from '@mantine/core';
import { useTranslateForms } from '@hooks/index';
import { useForm } from '@mantine/form';
import { UpdateStockRiven } from '@api/types';

export type UpdateRivenBulkProps = {
	onSubmit: (values: UpdateStockRiven) => void;
}

export function UpdateRivenBulk({ onSubmit }: UpdateRivenBulkProps) {

	// Translate general
	const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateForms(`update_stock_riven.${key}`, { ...context }, i18Key)
	const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateForm(`fields.${key}`, { ...context }, i18Key)
	// const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateForm(`buttons.${key}`, { ...context }, i18Key)

	// User form
	const form = useForm({
		initialValues: {
			minimum_price: 0 as number | undefined,
			use_hidden: false,
			is_hidden: false as boolean | undefined,
		},
		validate: {},
	});
	return (
		<Box w={"100%"}>
			<form onSubmit={form.onSubmit((data) => {
				if (data.minimum_price == 0)
					delete data.minimum_price;
				if (!data.use_hidden)
					delete data.is_hidden;
				onSubmit(data);
			})}>
				<Group gap="md">
					<NumberInput
						required
						label={useTranslateFormFields('minimum_price.label')}
						description={useTranslateFormFields('minimum_price.description')}
						placeholder={useTranslateFormFields('minimum_price.placeholder')}
						min={0}
						value={form.values.minimum_price || 0}
						onChange={(event) => form.setFieldValue('minimum_price', Number(event))}
						error={form.errors.minimum_price && useTranslateFormFields('minimum_price.error')}
						radius="md"
					/>
				</Group>
				<Group mt={15} gap="md">
					<Checkbox
						label={useTranslateFormFields('use_hidden.label')}
						checked={form.values.use_hidden}
						onChange={(event) => form.setFieldValue('use_hidden', event.currentTarget.checked)}
					/>
					<Collapse in={form.values.use_hidden}>
						<Switch
							label={useTranslateFormFields('is_hidden.label')}
							checked={form.values.is_hidden}
							onChange={(event) => form.setFieldValue('is_hidden', event.currentTarget.checked)}
						/>
					</Collapse>
				</Group>
				<Button mt={15} type="submit" color="blue" radius="md" fullWidth>
					{useTranslateForm('buttons.submit')}
				</Button>
			</form>
		</Box>
	);
}