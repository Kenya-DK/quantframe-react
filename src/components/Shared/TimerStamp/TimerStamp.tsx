import { Tooltip, Text } from "@mantine/core";
import dayjs from "dayjs";
import { useTranslateComponent } from "@hooks/useTranslate.hook";
export interface TimerStampProps {
  text?: string;
  date: Date;
}

export const TimerStamp = (props: TimerStampProps) => {
  const useTranslateSearch = (key: string, context?: { [key: string]: any }) => useTranslateComponent(`timer_stamp.${key}`, { ...context });
  let { text, date } = props;
  let tooText = "";
  const now = new Date();
  if (typeof date === "string") date = new Date(date);
  const difference = now.getTime() - date.getTime();
  let months = Math.floor(difference / 1000 / 60 / 60 / 24 / 31);
  let days = Math.floor(difference / (1000 * 60 * 60 * 24));
  let hours = Math.floor((difference / (1000 * 60 * 60)) % 24);
  let minutes = Math.floor((difference / 1000 / 60) % 60);
  let seconds = Math.floor((difference / 1000) % 60);
  if (months > 0) tooText = useTranslateSearch("months", { months });
  else if (days > 0) tooText = useTranslateSearch("days", { days });
  else if (hours > 0) tooText = useTranslateSearch("hours", { hours });
  else if (minutes > 0) tooText = useTranslateSearch("minutes", { minutes });
  else tooText = useTranslateSearch("seconds", { seconds });
  return (
    <Tooltip label={`${dayjs(date).format("DD-MM-YYYY HH:mm")}`}>
      <Text fz="md" component="span" c="gray.6">
        {text}
        {tooText}
      </Text>
    </Tooltip>
  );
};
