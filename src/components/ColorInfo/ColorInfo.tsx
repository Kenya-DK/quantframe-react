import { Box, Group, Text, Tooltip, UnstyledButton } from '@mantine/core';
import classes from './ColorInfo.module.css';

export type ColorInfoProps = {
	color?: string;
	text: string;
	infoProps?: Record<string, any>;
	tooltip?: string;
	active?: boolean;
	onClick?: () => void;
}
export function ColorInfo({ active, onClick, text, tooltip, infoProps }: ColorInfoProps) {
	return (
		<Tooltip disabled={!text} label={tooltip} position="top" >
			<UnstyledButton disabled={!onClick} className={classes.button} onClick={onClick}>
				<Group gap={5} >
					<Box w={16} h={16} className={classes.box} {...infoProps} />
					<Text td={active ? "line-through" : ""} >{text}</Text>
				</Group>
			</UnstyledButton>
		</Tooltip>
	);
}