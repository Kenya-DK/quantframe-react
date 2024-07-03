import { Text, Group, Menu, Avatar, Button, Indicator } from '@mantine/core';
import { useAppContext, useAuthContext, useWFMSocketContext } from '@contexts/index';
import api, { SendTauriDataEvent, WFMThumbnail } from '@api/index';
import { QfSocketEvent, QfSocketEventOperation, Settings, UserStatus } from '@api/types';
import classes from './UserMenu.module.css';
import { useTranslateComponent, useTranslateEnums } from '@hooks/index';
import { faGear, faRightFromBracket } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { useMutation } from '@tanstack/react-query';
import { notifications } from '@mantine/notifications';
import { useNavigate } from 'react-router-dom';
import { SettingsForm } from '../forms';
import { modals } from '@mantine/modals';
export function UserMenu() {
	// States
	const navigate = useNavigate();
	const { user } = useAuthContext();
	const { settings } = useAppContext();
	const { isConnected, inErrorState } = useWFMSocketContext();

	// Translate general
	const useTranslateUserMenu = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateComponent(`user_menu.${key}`, { ...context }, i18Key)
	const useTranslateUserStatus = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateEnums(`user_status.${key}`, { ...context }, i18Key)
	const useTranslateErrors = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateUserMenu(`errors.${key}`, { ...context }, i18Key)
	const useTranslateSuccess = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateUserMenu(`success.${key}`, { ...context }, i18Key)

	// Mutations
	const logOutMutation = useMutation({
		mutationFn: () => api.auth.logout(),
		onSuccess: () => {
			notifications.show({ title: useTranslateSuccess("logout.title"), message: useTranslateSuccess("logout.message"), color: "green.7" });
			SendTauriDataEvent(QfSocketEvent.UpdateUser, QfSocketEventOperation.SET, undefined);
			navigate('/');
		},
		onError: () => notifications.show({ title: useTranslateErrors("logout.title"), message: useTranslateErrors("logout.message"), color: "green.7" })
	})
	const updateSettingsMutation = useMutation({
		mutationFn: (s: Settings) => api.app.updateSettings(s),
		onSuccess: () => {
			notifications.show({ title: useTranslateSuccess("update_settings.title"), message: useTranslateSuccess("update_settings.message"), color: "green.7" });
		},
		onError: () => notifications.show({ title: useTranslateErrors("update_settings.title"), message: useTranslateErrors("update_settings.message"), color: "green.7" })
	})


	return (
		<Menu shadow="md" width={200}
			transitionProps={{ transition: "fade-down", duration: 150 }}
		>
			<Menu.Target>
				<Group>
					{(isConnected && !inErrorState) ? (
						<Indicator
							inline size={16} offset={7} position="bottom-start" withBorder
							classNames={classes}
							disabled={!user}
							data-user-status={user?.status || UserStatus.Invisible}
						>
							<Avatar
								variant="subtle"
								src={WFMThumbnail(user?.avatar || "")}
								alt={user?.ingame_name}
								radius="xl"
								size="48px"
							/>
						</Indicator>
					) : (
						<Text tt="uppercase" fw={500}>
							Err
						</Text>
					)}
				</Group>
			</Menu.Target>

			<Menu.Dropdown>
				<Menu.Item leftSection={<Avatar variant="subtle" src={WFMThumbnail(user?.avatar || "")} alt={user?.ingame_name} radius="xl" size={"md"} />}>
					{user?.ingame_name || "Unknown"}
				</Menu.Item>
				<Menu.Divider />
				<Group gap={3} mt="xs" classNames={{ root: classes.user_status }}>
					{Object.values(UserStatus).map((status) => (
						<Button key={status} p={3} fullWidth variant="subtle" data-active={status == user?.status} onClick={() => api.auth.update_status(status)}>
							<Text tt="uppercase" data-color-mode='text' data-user-status={status} fw={500}>
								{useTranslateUserStatus(status)}
							</Text>
						</Button>
					))}
				</Group>
				<Menu.Divider />
				<Menu.Label>{useTranslateUserMenu("items.app_label")}</Menu.Label>
				<Menu.Item leftSection={<FontAwesomeIcon icon={faGear} />} onClick={() => {
					if (!settings) return;
					modals.open({
						size: "100%",
						withCloseButton: false,
						children: <SettingsForm value={settings} onSubmit={async (s) => {
							await updateSettingsMutation.mutateAsync(s);
							modals.closeAll();
						}} />,
					})
				}}>{useTranslateUserMenu("items.settings")}</Menu.Item>
				<Menu.Item leftSection={<FontAwesomeIcon icon={faRightFromBracket} />} onClick={async () => { await logOutMutation.mutateAsync() }}>{useTranslateUserMenu("items.logout")}</Menu.Item>
			</Menu.Dropdown>
		</Menu>
	);
}
