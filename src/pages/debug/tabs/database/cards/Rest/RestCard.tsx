import { Text, Card, Group, Button, Select } from '@mantine/core';
import { useMutation } from '@tanstack/react-query';
import api from '@api/index';
import { notifications } from '@mantine/notifications';
import { useTranslatePages } from '@hooks/index';
import { useState } from 'react';

export function RestCard() {
	const [selected, setSelected] = useState<string>("stock_item");


	// Translate general
	const useTranslateRestCard = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslatePages(`debug.tabs.database.cards.reset.${key}`, { ...context }, i18Key)
	const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateRestCard(`buttons.${key}`, { ...context }, i18Key)
	const useTranslateErrors = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateRestCard(`errors.${key}`, { ...context }, i18Key)
	const useTranslateSuccess = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateRestCard(`success.${key}`, { ...context }, i18Key)

	// Mutations
	const resetDataMutation = useMutation({
		mutationFn: (target: string) => api.debug.reset(target),
		onSuccess: () => {
			notifications.show({ title: useTranslateSuccess("rest.title"), message: useTranslateSuccess("rest.message"), color: "green.7" });
		},
		onError: () => notifications.show({ title: useTranslateErrors("rest.title"), message: useTranslateErrors("rest.message"), color: "green.7" })
	})

	return (
		<Card withBorder shadow="sm" radius="md">
			<Card.Section withBorder inheritPadding py="xs">
				<Group justify="center">
					<Text fw={500}>{useTranslateRestCard("title")}</Text>
				</Group>
			</Card.Section>

			<Card.Section inheritPadding mt="sm" pb="md">
				<Group>
					<Select
						value={selected}
						onChange={(value) => setSelected(value as string)}
						data={[
							{ value: "all", label: "All" },
							{ value: "stock_item", label: "Stock Items" },
							{ value: "stock_riven", label: "Stock Rivens" },
							{ value: "transaction", label: "Transactions" },
						]}
					/>
					<Button loading={resetDataMutation.isPending} onClick={async () => resetDataMutation.mutateAsync(selected)} color="red" >
						{useTranslateButtons("rest.title")}
					</Button>
				</Group>
			</Card.Section>
		</Card>
	);
}