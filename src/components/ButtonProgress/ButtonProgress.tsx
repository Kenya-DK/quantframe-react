import { Button, ButtonProps, Progress, rgba, useMantineTheme } from '@mantine/core';
import classes from './ButtonProgress.module.css';
import { useEffect, useState } from 'react';

export type ButtonProgressProps = {
	interval: number;
	max: number;
	text: string;
	onClick: () => void;
	color?: string;
	progressColor?: string;
	buttonProps?: ButtonProps;
}

export function ButtonProgress({ buttonProps, text, interval, max, onClick, color, progressColor }: ButtonProgressProps) {
	const theme = useMantineTheme();
	const [loaded, setLoaded] = useState(false);

	useEffect(() => {
		if (interval === max)
			setLoaded(true);
		else
			setLoaded(false);
	}, [interval, max]);
	return (
		<Button
			{...buttonProps}
			fullWidth
			className={classes.button}
			onClick={() => (
				!loaded && onClick()
			)
			}
			color={loaded ? (progressColor || 'teal') : (color || theme.primaryColor)}
		>
			<div className={classes.label}>
				{text}
			</div>
			{interval !== 0 && (
				<Progress
					value={interval / max}
					className={classes.progress}
					color={rgba((color || theme.colors.blue[2]), 0.35)}
					radius="sm"
				/>
			)}
		</Button>
	);
}