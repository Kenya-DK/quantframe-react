import { Button } from '@mantine/core';
import classes from './ButtonInterval.module.css';

export type ButtonIntervalProps = {
	intervals: number[];
	prefix: string;
	color: string;
	OnClick: (interval: number) => void;
}

export function ButtonInterval({ color, prefix, OnClick, intervals }: ButtonIntervalProps) {

	return (
		<>
			{intervals.map((interval) => (
				<Button
					key={interval}
					onClick={() => OnClick(interval)}
					variant="filled"
					color={color}
					className={classes.button}
				>
					{prefix}{interval}
				</Button>
			))}
		</>
	);
}