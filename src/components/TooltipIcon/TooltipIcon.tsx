import { faInfoCircle } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { Center, Text, Tooltip } from "@mantine/core";

export type TooltipIconProps = {
	label: string;
}

export function TooltipIcon({ label }: TooltipIconProps) {

	return (
		<Tooltip
			label={label}
			position="top-end"
			withArrow
			transitionProps={{ transition: 'pop-bottom-right' }}
		>
			<Text component="div" c="dimmed" style={{ cursor: 'help' }}>
				<Center>
					<FontAwesomeIcon icon={faInfoCircle} />
				</Center>
			</Text>
		</Tooltip>
	);
}