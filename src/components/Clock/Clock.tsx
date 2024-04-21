import { Group } from '@mantine/core';
import classes from './Clock.module.css';
import { useEffect, useState } from 'react';
import { useTranslateComponent } from '../../hooks';
import { TextTranslate } from '..';

export function Clock() {
	const [currentTime, setCurrentTime] = useState("");
	const [timeUntilMidnight, setTimeUntilMidnight] = useState("");


	// Translate general
	const useTranslateClock = (key: string, context?: { [key: string]: any }, i18Key?: boolean) => useTranslateComponent(`clock.${key}`, { ...context }, i18Key)

	useEffect(() => {
		const interval = setInterval(() => {
			const date = new Date();
			const hours = date.getUTCHours().toString().padStart(2, "0");
			const minutes = date.getUTCMinutes().toString().padStart(2, "0");
			const seconds = date.getUTCSeconds().toString().padStart(2, "0");
			const time = `${hours}:${minutes}:${seconds}`;
			setCurrentTime(time);

			// Calculate time until midnight
			const midnight = new Date(date);
			midnight.setUTCHours(24, 0, 0, 0);
			const timeUntilMidnightMS = midnight.valueOf() - date.valueOf();
			const hoursUntilMidnight = Math.floor(timeUntilMidnightMS / 3600000);
			const minutesUntilMidnight = Math.floor(
				(timeUntilMidnightMS % 3600000) / 60000
			);
			const secondsUntilMidnight = Math.floor(
				(timeUntilMidnightMS % 60000) / 1000
			);
			// Pad the time values with leading zeros
			const paddedHours = hoursUntilMidnight.toString().padStart(2, "0");
			const paddedMinutes = minutesUntilMidnight.toString().padStart(2, "0");
			const paddedSeconds = secondsUntilMidnight.toString().padStart(2, "0");

			const timeUntilMidnight = `${paddedHours}:${paddedMinutes}:${paddedSeconds}`;
			setTimeUntilMidnight(timeUntilMidnight);
		}, 1000); // Update every second

		return () => {
			clearInterval(interval);
		};
	}, []);
	return (
		<Group className={classes.clock}>
			<TextTranslate size='md' i18nKey={useTranslateClock("gmt", undefined, true)} values={{ time: currentTime }} />
			<TextTranslate size='md' i18nKey={useTranslateClock("time_until_midnight", undefined, true)} values={{ time: timeUntilMidnight }} />
		</Group>
	);
}