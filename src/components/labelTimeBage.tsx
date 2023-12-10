
import { format } from "date-fns";
import { useTranslateComponent } from "@hooks/index";
import { Tooltip, Text } from "@mantine/core";
interface TimerStampProps {
  text?: string;
  date: Date;
}

export const LabelTimeBage = (props: TimerStampProps) => {
  const useTranslateSearch = (key: string, context?: { [key: string]: any }) => useTranslateComponent(`labelTimeBage.${key}`, { ...context })
  let { text, date } = props;
  let tooTiptext = "";
  const now = new Date();
  if (typeof date === "string")
    date = new Date(date);
  const difference = now.getTime() - date.getTime();
  let months = Math.floor(difference / 1000 / 60 / 60 / 24 / 31);
  let days = Math.floor(difference / (1000 * 60 * 60 * 24));
  let hours = Math.floor((difference / (1000 * 60 * 60)) % 24);
  let minutes = Math.floor((difference / 1000 / 60) % 60);
  let seconds = Math.floor((difference / 1000) % 60);
  if (months > 0)
    tooTiptext = useTranslateSearch('months', { months });
  else if (days > 0)
    tooTiptext = useTranslateSearch('days', { days });
  else if (hours > 0)
    tooTiptext = useTranslateSearch('hours', { hours });
  else if (minutes > 0)
    tooTiptext = useTranslateSearch('minutes', { minutes });
  else
    tooTiptext = useTranslateSearch('seconds', { seconds });
  return (
    <Tooltip label={`${format(date, 'dd-MM-yyyy HH:mm')}`}>
      <Text fz="md" component="span" color="gray.6">{text}{tooTiptext}</Text>
    </Tooltip>
  );

}
