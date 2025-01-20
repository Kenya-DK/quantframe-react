import { Text, Card, Group, Button, TextInput, Box } from '@mantine/core';
import { useMutation } from '@tanstack/react-query';
import api from '@api/index';
import { notifications } from '@mantine/notifications';
import { useTranslatePages } from '@hooks/useTranslate.hook';
import { useState } from 'react';
import { open } from '@tauri-apps/plugin-dialog';
import { faFile } from '@fortawesome/free-solid-svg-icons';
import { ActionWithTooltip } from '@components/ActionWithTooltip';

export function ImportATraderCard() {
	const [selected, setSelected] = useState<string>("");


	// Translate general
	const useTranslateImportATraderCard = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslatePages(`debug.tabs.database.cards.import_algo_trader.${key}`, { ...context }, i18Key)
	const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateImportATraderCard(`fields.${key}`, { ...context }, i18Key)
	const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateImportATraderCard(`buttons.${key}`, { ...context }, i18Key)
	const useTranslateErrors = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateImportATraderCard(`errors.${key}`, { ...context }, i18Key)
	const useTranslateSuccess = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateImportATraderCard(`success.${key}`, { ...context }, i18Key)

	// Mutations
	const importDataMutation = useMutation({
		mutationFn: (target: string) => api.debug.importAlgoTrader(target),
		onSuccess: () => {
			notifications.show({ title: useTranslateSuccess("import.title"), message: useTranslateSuccess("import.message"), color: "green.7" });
		},
		onError: () => notifications.show({ title: useTranslateErrors("import.title"), message: useTranslateErrors("import.message"), color: "red.7" })
	})

	const openDialog = async () => {
		const selected = await open({
			multiple: false,
			filters: [{
				name: 'Database',
				extensions: ['db']
			}]
		});
		setSelected(selected as string);
	}

	return (
		<Card withBorder shadow="sm" radius="md">
			<Card.Section withBorder inheritPadding py="xs">
				<Group justify="center">
					<Text fw={500}>{useTranslateImportATraderCard("title")}</Text>
				</Group>
			</Card.Section>

			<Card.Section inheritPadding mt="sm" pb="md">
				<Box >
					<TextInput
						required
						label={useTranslateFormFields('db_path.label')}
						placeholder={useTranslateFormFields('db_path.placeholder')}
						value={selected}
						onChange={(event) => setSelected(event.currentTarget.value)}
						radius="md"
						rightSection={
							<ActionWithTooltip
								tooltip={useTranslateButtons(`open_file.tooltip`)}
								icon={faFile}
								color='green.7'
								onClick={() => openDialog()}
							/>
						}
					/>
					<Button mt={"md"} loading={importDataMutation.isPending} onClick={async () => importDataMutation.mutateAsync(selected)} color="red" >
						{useTranslateButtons("import.title")}
					</Button>
				</Box>
			</Card.Section>
		</Card>
	);
}