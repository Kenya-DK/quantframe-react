import { Group, useMantineTheme } from '@mantine/core';
import classes from './Header.module.css';
import { Logo } from '@components/Logo';
import { Clock } from '@components/Clock';
import { UserMenu } from '@components/UserMenu';

export type HeaderProps = {
}

export function Header({ }: HeaderProps) {
	const theme = useMantineTheme();
	return (
		<Group ml={"sm"} mr={"sm"} justify='space-between' className={classes.header}>
			<Logo color={theme.colors.blue[7]} />
			<Clock />
			<UserMenu />
		</Group>
	);
}